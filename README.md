# xCHIP
> Accurate CHIP-8, CHIP-10, HIRES CHIP-8, CHIP-8X, SUPER-CHIP, and XO-CHIP Interpreter in Rust.

## Features
 - Simple **flicker reduction** ­— 10-20 instruction delay from a pixel being turned off to it actually turning off

## Mode

The file extension is normally looked at to determine the operation mode of the xCHIP
interpreter. To force the selection of a specific mode, use `-m <mode>` at the command
line.

| Name         | Mode                    | File Extension  |
| ------------ | ----------------------- | --------------- |
| CHIP-8       | `chip-8`, `8`           | ---             |
| HIRES CHIP-8 | `hires-chip-8`, `hires` | ---             |
| CHIP-10      | `chip-10`, `10`         | `.ch10`         |
| CHIP-8X      | `chip-8x`, `8x`         | `.c8x`          |
| SUPER-CHIP   | `super-chip`, `sc`      | ---             |
| XO-CHIP      | `xo-chip`, `xo`         | `.ch8`          |

 - `CHIP-8` and `SUPER-CHIP` are subsets of `XO-CHIP`

 - `HIRES CHIP-8` is detected by checksumming the bytes from
   `$200` to `$240` as HIRES CHIP-8 ROMs officially started at `$244` (memory
   before is for the interpreter but is included in all known ROM
   distributions for ease of loading in CHIP-8 interpreters)
