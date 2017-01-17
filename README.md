# xCHIP
> Accurate CHIP-8 Interpreter in Rust

## Features
 - Simple **flicker reduction** ­— 10-20 instruction delay from a pixel being turned off to it actually turning off

## Mode

The file extension is normally looked at to determine the operation mode of the xCHIP
interpreter. To force the selection of a specific mode, use `-m <mode>` at the command
line.

| Name         | Mode                    | File Extension  | Notes                                                 |
| ------------ | ----------------------- | --------------- | ----------------------------------------------------- |
| CHIP-8       | `chip-8`, `8`           | ---             | SUPER-CHIP is 100% backwards-compatible with CHIP-8   |
| HIRES CHIP-8 | `hires-chip-8`, `hires` | ---             | Detected by checksumming the bytes from `$200` to `$240` as HIRES CHIP-8 ROMs officially started at `$244` (memory before is for the interpter but is included in all known ROM distributions for ease of loading in CHIP-8 interpters) |
| CHIP-10      | `chip-10`, `10`         | `.ch10`         |                                                       |
| CHIP-8C      | ---                     | ---             | Never released but was a subset of CHIP-8X            |
| CHIP-8X      | `chip-8x`, `8x`         | `.c8x`          |                                                       |
| SUPER-CHIP   | `super-chip`, `sc`      | ---             | XO-CHIP is 100% backwards-compatible with SUPER-CHIP  |
| XO-CHIP      | `xo-chip`, `xo`         | `.ch8`          |                                                       |

> NOTE: Only CHIP-8 is implemented right now
