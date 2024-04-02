#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridBuffer<T, const WIDTH: usize, const HEIGHT: usize> {
    pub content: [[T; WIDTH]; HEIGHT],
}

impl<T, const WIDTH: usize, const HEIGHT: usize> GridBuffer<T, WIDTH, HEIGHT> {
    pub fn new_with_default(default: T) -> Self
    where
        T: Copy,
    {
        Self {
            content: [[default; WIDTH]; HEIGHT],
        }
    }

    pub fn from_array(array: [[T; WIDTH]; HEIGHT]) -> Self
    where
        T: Copy,
    {
        Self { content: array }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> GridBuffer<T, WIDTH, HEIGHT>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            content: [[T::default(); WIDTH]; HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.shift((0, HEIGHT as isize));
    }

    pub fn shift(&mut self, count: (isize, isize)) {
        let (horizontal, vertical) = count;
        self.shift_horizontal(horizontal);
        self.shift_vertical(vertical);
    }

    fn shift_vertical(&mut self, count: isize) {
        let (left, start, end, right) = Self::get_shift_ranges(count, HEIGHT);
        let sign = (((left >= 0) && (start >= 0) && (end >= 0) && (right >= 0)) as isize) * 2 - 1;

        for line_pos in left..start {
            for char_pos in 0..WIDTH {
                let line_pos = (line_pos * sign) as usize;
                self.content[line_pos][char_pos] = T::default();
            }
        }

        for line_pos in start..end {
            for char_pos in 0..WIDTH {
                let line_pos = (line_pos * sign) as usize;
                let new_pos = Self::get_source_pos((line_pos, char_pos), (count, 0));
                self.content[line_pos][char_pos] = self.content[new_pos.0][new_pos.1];
            }
        }

        for line_pos in end..right {
            for char_pos in 0..WIDTH {
                let line_pos = (line_pos * sign) as usize;
                self.content[line_pos][char_pos] = T::default();
            }
        }
    }

    fn shift_horizontal(&mut self, count: isize) {
        let (left, start, end, right) = Self::get_shift_ranges(count, WIDTH);
        let sign = (((left >= 0) && (start >= 0) && (end >= 0) && (right >= 0)) as isize) * 2 - 1;

        for line_pos in 0..HEIGHT {
            for char_pos in start..end {
                let char_pos = (char_pos * sign) as usize;
                let new_pos = Self::get_source_pos((line_pos, char_pos), (0, count));
                self.content[line_pos][char_pos] = self.content[new_pos.0][new_pos.1];
            }

            for char_pos in left..start {
                let char_pos = (char_pos * sign) as usize;
                self.content[line_pos][char_pos] = T::default();
            }

            for char_pos in end..right {
                let char_pos = (char_pos * sign) as usize;
                self.content[line_pos][char_pos] = T::default();
            }
        }
    }

    fn get_shift_ranges(count: isize, bound: usize) -> (isize, isize, isize, isize) {
        let count = Self::clip_shift(count, bound);
        let bound = bound as isize;

        let start = core::cmp::max(Self::shift_pos(0, -count), 0);
        let end = core::cmp::min(Self::shift_pos(bound as isize, -count), bound as isize);

        if count <= 0 {
            (0, start, end, bound)
        } else {
            (-bound + 1, -end + 1, -start + 1, 1)
        }
    }

    fn clip_shift(count: isize, bound: usize) -> isize {
        let count = core::cmp::min(count, bound as isize);
        let count = core::cmp::max(count, -(bound as isize));
        count
    }

    fn get_source_pos(pos: (usize, usize), count: (isize, isize)) -> (usize, usize) {
        let (line_pos, char_pos) = pos;
        let (vertical, horizontal) = count;

        (
            Self::shift_pos(line_pos as isize, vertical) as usize,
            Self::shift_pos(char_pos as isize, horizontal) as usize,
        )
    }

    fn shift_pos(pos: isize, count: isize) -> isize {
        (pos as isize) - count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_shift() {
        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift((2, 2));

        assert_eq!(
            buffer,
            GridBuffer::<u8, 5, 4>::from_array([
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 1, 2, 3],
                [0, 0, 6, 7, 8],
            ])
        );
    }

    #[test_case]
    fn test_shift_vertical() {
        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_vertical(2);

        assert_eq!(
            buffer,
            GridBuffer::<u8, 5, 4>::from_array([
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [1, 2, 3, 4, 5],
                [6, 7, 8, 9, 10],
            ])
        );

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_vertical(-2);

        assert_eq!(
            buffer,
            GridBuffer::<u8, 5, 4>::from_array([
                [11, 12, 13, 14, 15],
                [16, 17, 18, 19, 20],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ])
        );

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_vertical(0);

        assert_eq!(
            buffer,
            GridBuffer::<u8, 5, 4>::from_array([
                [1, 2, 3, 4, 5],
                [6, 7, 8, 9, 10],
                [11, 12, 13, 14, 15],
                [16, 17, 18, 19, 20],
            ])
        );

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_vertical(10);

        assert_eq!(buffer, GridBuffer::<u8, 5, 4>::new_with_default(0));

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_vertical(-10);

        assert_eq!(buffer, GridBuffer::<u8, 5, 4>::new_with_default(0));
    }

    #[test_case]
    fn test_shift_horizontal() {
        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_horizontal(2);

        assert_eq!(
            buffer,
            GridBuffer::<u8, 5, 4>::from_array([
                [0, 0, 1, 2, 3],
                [0, 0, 6, 7, 8],
                [0, 0, 11, 12, 13],
                [0, 0, 16, 17, 18],
            ])
        );

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_horizontal(-2);

        assert_eq!(
            buffer,
            GridBuffer::<u8, 5, 4>::from_array([
                [3, 4, 5, 0, 0],
                [8, 9, 10, 0, 0],
                [13, 14, 15, 0, 0],
                [18, 19, 20, 0, 0],
            ])
        );

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_horizontal(0);

        assert_eq!(
            buffer,
            GridBuffer::<u8, 5, 4>::from_array([
                [1, 2, 3, 4, 5],
                [6, 7, 8, 9, 10],
                [11, 12, 13, 14, 15],
                [16, 17, 18, 19, 20],
            ])
        );

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_horizontal(10);

        assert_eq!(buffer, GridBuffer::<u8, 5, 4>::new_with_default(0));

        let mut buffer = GridBuffer::<u8, 5, 4>::from_array([
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
        ]);

        buffer.shift_horizontal(-10);

        assert_eq!(buffer, GridBuffer::<u8, 5, 4>::new_with_default(0));
    }

    #[test_case]
    fn test_get_shift_ends() {
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_shift_ranges(5, 10),
            (-9, -9, -4, 1)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_shift_ranges(-5, 10),
            (0, 0, 5, 10)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_shift_ranges(15, 10),
            (-9, -9, -9, 1)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_shift_ranges(-15, 10),
            (0, 0, 0, 10)
        );
    }

    #[test_case]
    fn test_get_source_pos() {
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (3, 5)),
            (7, 15)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (-3, -5)),
            (13, 25)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (3, -5)),
            (7, 25)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (-3, 5)),
            (13, 15)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (0, 0)),
            (10, 20)
        );
    }

    #[test_case]
    fn test_get_source_pos__odd() {
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos(
                GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (3, 5)),
                (-3, -5)
            ),
            (10, 20)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos(
                GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (-3, -5)),
                (3, 5)
            ),
            (10, 20)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos(
                GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (3, -5)),
                (-3, 5)
            ),
            (10, 20)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::get_source_pos(
                GridBuffer::<u8, 0, 0>::get_source_pos((10, 20), (-3, 5)),
                (3, -5)
            ),
            (10, 20)
        );
    }

    #[test_case]
    fn test_clip_shift() {
        assert_eq!(GridBuffer::<u8, 0, 0>::clip_shift(5, 10), 5);
        assert_eq!(GridBuffer::<u8, 0, 0>::clip_shift(-5, 10), -5);
        assert_eq!(GridBuffer::<u8, 0, 0>::clip_shift(15, 10), 10);
        assert_eq!(GridBuffer::<u8, 0, 0>::clip_shift(-15, 10), -10);
    }

    #[test_case]
    fn test_clip_shift__idempotent() {
        assert_eq!(
            GridBuffer::<u8, 0, 0>::clip_shift(GridBuffer::<u8, 0, 0>::clip_shift(5, 10), 10),
            GridBuffer::<u8, 0, 0>::clip_shift(5, 10)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::clip_shift(GridBuffer::<u8, 0, 0>::clip_shift(-5, 10), 10),
            GridBuffer::<u8, 0, 0>::clip_shift(-5, 10)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::clip_shift(GridBuffer::<u8, 0, 0>::clip_shift(15, 10), 10),
            GridBuffer::<u8, 0, 0>::clip_shift(15, 10)
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::clip_shift(GridBuffer::<u8, 0, 0>::clip_shift(-15, 10), 10),
            GridBuffer::<u8, 0, 0>::clip_shift(-15, 10)
        );
    }

    #[test_case]
    fn test_shift_pos() {
        assert_eq!(GridBuffer::<u8, 0, 0>::shift_pos(5, 2), 3);
        assert_eq!(GridBuffer::<u8, 0, 0>::shift_pos(5, -2), 7);
        assert_eq!(GridBuffer::<u8, 0, 0>::shift_pos(5, 0), 5);
    }

    #[test_case]
    fn test_shift_pos__odd() {
        assert_eq!(
            GridBuffer::<u8, 0, 0>::shift_pos(GridBuffer::<u8, 0, 0>::shift_pos(5, 2), -2),
            5
        );
        assert_eq!(
            GridBuffer::<u8, 0, 0>::shift_pos(GridBuffer::<u8, 0, 0>::shift_pos(5, -2), 2),
            5
        );
    }
}
