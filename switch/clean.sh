#!/bin/bash

cargo clean
xargo clean
find $PWD -name "*.elf" -type f -delete
find $PWD -name "*.nro" -type f -delete
find $PWD -name "*.nso" -type f -delete