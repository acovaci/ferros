#!/usr/bin/env bash

qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-ferros.bin
