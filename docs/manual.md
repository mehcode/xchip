# xCHIP

## About

The CHIP-8 is a simple, interpreted language which was designed for use in the COSMAC VIP (but was
later ported to several other computers such as the DREAM 6800, and ETI 660).

## Specifications

The standard CHIP-8 language can address up to 4 KiB (4,096 bytes) of RAM. The first 512 bytes are where the original
interpreter was located and should not be used by programs.

The COSMAC VIP that the CHIP-8 language was designed to run on normally had either 2 KiB or 4 KiB of RAM.

### Memory Map

|  |  |
| --- | --- |
| `$000` - `$1FF` | Reserved for the interpreter program and contains the standard font sprites. |
| `$200` - `$Y9F` | User program; CHIP-8 programs are loaded (and begin execution) at `$200`. |
| `$YA0` - `$YCF` | Return stack (max of 12) |
| `$YD0` - `$YEF` | Reserved for interpreter work area |
| `$YF0` - `$YFF` | V0 - VF |
| `$X00` - `$XFF` | Video RAM |

 - `X` is the highest 256-byte page of RAM (eg. on a 4 KiB system it would be `8` and on a 2 KiB system, `7`).
 - `Y` is `X - 1`.
 - A game could have made use of these details to have significantly faster
   rendering among other benefits (SAVE/RESTORE could blit N bytes in 2
   instructions) but I'm unaware of any game that did.
 - The SUPER-CHIP, MEGA-CHIP, and XO-CHIP do not store the stack,
   work area, V registers, nor video RAM in program-accessible memory.
 - The SUPER-CHIP increased the return stack size to 16 (as did the MEGA-CHIP but not the XO-CHIP).
 - Overflowing the stack does not wrap around (at the right boundary, it does eventually wrap around as the internal stack pointer is 8-bits) and can have funny consequences as it will roll over into space reserved for internal operations and general registers.

## Instructions

| Opcode | Description |
| --- | --- |
| `00E0` | Clear the screen |

## Extensions

### [CHIP-10] — VIPER 1.7 by Ben H. Hutchinson, Jr. (1979)

A modified version of CHIP-8 providing an expanded screen resolution
of 128 (horizontally) x 64 (vertically).

No observable differences in instructions beyond the usage of a doubled display.

**Incompatible** with HIRES CHIP-8, SUPER-CHIP, MEGA-CHIP8, and XO-CHIP.

[CHIP-10]: http://www.mattmik.com/files/viper/Volume1Issue07.pdf

### [CHIP-8X] — VIPER 1.3 by Rick Simpson (1978)

An "expanded version of the original CHIP-8 interpreter" meant for use with a series
of expansion modules marketed by RCA for the COSMAC VIP system:

 - VP-590 Color board
 - VP-595 Simple Sound board
 - VP-580 Expansion Keypad

The CHIP-8X language adds:

 - Color graphics
 - Extended sound capabilities
 - Support for a second keypad

**Incompatible** with technically all other variants. However nothing ever made use of `BNNN` so if that instruction is ignored then CHIP-8X is only incompatible with XO-CHIP, MEGA-CHIP, and CHIP-8E.

[CHIP-8x]: http://www.mattmik.com/files/viper/Volume1Issue03.pdf

### Instructions (in addition to CHIP-8)

| Opcode | Description |
| --- | --- |
| `02A0` | Steps the background color (-> Blue -> Black -> Green -> Red ->); the background color starts at Black. |
| `5XY1` | Set `VX` equal to `VX` plus `VY` |
| `BXY0` | Set foreground color of 1 or more 8x4 dot zones (*) |
| `BXYN` | Set foreground color of 1 or more 8x1 dot zones (*) |

(*) Defined below as these instructions require more explanation than would fit in the table.

#### `BXY0` — `COL VX, VY`

This operates on the 64x32 CHIP-8 display divided into 8x8 zones of 8x4 dots each.

The lower 4 bits of `VX` is the horizontal zone index (0-7). The upper 4 bits of `VX` is the horizontal width minus 1 (from 1 to 8 zones).

The lower 4 bits of `V[X+1]` is the vertical zone index (0-7). The upper 4 bits of `V[X+1]` is the horizontal width minus 1 (from 1 to 8 zones).

The lower 4 bits of `VY` are used to chose a color from this palette:

| Value | Color |
| --- | --- | --- |
| `0` | Black |
| `1` | Red |
| `2` | Blue |
| `3` | Violet |
| `4` | Green |
| `5` | Yellow |
| `6` | Aqua |
| `7` | White |

#### `BXYN` — `COL VX, VY, N`

The horizontal **dot index** is `VX` (0-3F) and the vertical **dot index** is `V[X+1]` (0-1F).

The height is defined by `N`.

The width is always 8 dots.

The color is chosen with `VY` as in `BXY0`.

### [SUPER-CHIP] — `comp.sys.handhelds` by Erik Bryntse (1991)

[SUPER-CHIP]: http://devernay.free.fr/hacks/chip8/schip.txt

### Instructions (in addition to CHIP-8)

| Opcode | Description |
| --- | --- |
| `00CN` | Scroll display `N` lines down. When in _normal_ (64x32) display mode; the display is scrolled by half-dots. |
| `00FB` | Scroll display 4 dots right. When in _normal_ (64x32) display mode; the display is scrolled by half-dots. |
| `00FC` | Scroll display 4 dots left. When in _normal_ (64x32) display mode; the display is scrolled by half-dots. |
| `00FD` | Exit the interpreter. Modern interpreters should simply halt operation.
| `00FE` | Enable _extended_, 128x64 display mode. This should act as if the existing 64x32 screen buffer is divided to double the number of dots accessible (rather than increasing resolution in any direction). |
| `00FF` | Disable _extended_ display mode and revert to _normal_, 64x32 display mode. The existing screen buffer should be left unchanged. |
| `DXY0` | Show 16x16 sprite from `I` at coordinates (`VX`, `VY`). `VF` is still used for collision.
| `FX30` | Point I to 10-byte font sprite for digit `VX` (originally this was restricted to `<= 9` but as there is no harm in extending that to the full hex range, this is what xCHIP does). |
| `FX75` | Store `V0`..`VX` in interpreter memory (`X` <= 7) |
| `FX85` | Store `V0`..`VX` in interpreter memory (`X` <= 7) |
