# xCHIP

## CHIP-8 (1978)

## [CHIP-10] — VIPER 1.7 by Ben H. Hutchinson, Jr. (1979)

A modified version of CHIP-8 providing an expanded screen resolution
of 128 (horizontally) x 64 (vertically).

No observable differences in instructions beyond the usage of a doubled display.

**Incompatible** with HIRES CHIP-8, SUPER-CHIP, MEGA-CHIP8, and XO-CHIP.

[CHIP-10]: http://www.mattmik.com/files/viper/Volume1Issue07.pdf

## [CHIP-8X] — VIPER 1.3 by Rick Simpson (1978)

An "expanded version of the original CHIP-8 interpreter" meant for use with a series
of expansion modules marketed by RCA for the COSMAC VIP system:

 - VP-590 Color board
 - VP-595 Simple Sound board
 - VP-580 Expansion Keypad

The CHIP-8X language adds:

 - Color graphics
 - Extended sound capabilities
 - Support for a second keypad

[CHIP-8x]: http://www.mattmik.com/files/viper/Volume1Issue03.pdf

### Instructions (in addition to CHIP-8)

| Opcode | Mnemonic | Description |
| --- | --- | --- |
| `02A0` | `STEPCOL` | Steps the background color (-> Blue -> Black -> Green -> Red ->); the background color starts at Black. |
| `5XY1` | `ADD VX, VY` | Set `VX` equal to `VX` plus `VY` |
| `BXY0` | `COL VX, VY` | Set foreground color of 1 or more 8x4 dot zones (*) |
| `BXYN` | `COL VX, VY, N` | Set foreground color of 1 or more 8x1 dot zones (*) |

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

## [SUPER-CHIP] — `comp.sys.handhelds` by Erik Bryntse (1991)

[SUPER-CHIP]: http://devernay.free.fr/hacks/chip8/schip.txt

### Instructions (in addition to CHIP-8)

| Opcode | Mnemonic | Description |
| --- | --- | --- |
| `00CN` | `SCDOWN N` | Scroll display `N` lines down. When in _normal_ (64x32) display mode; the display is scrolled by half-dots. |
| `00FB` | `SCRIGHT` | Scroll display 4 dots right. When in _normal_ (64x32) display mode; the display is scrolled by half-dots. |
| `00FC` | `SCLEFT` | Scroll display 4 dots left. When in _normal_ (64x32) display mode; the display is scrolled by half-dots. |
| `00FD` | `EXIT` | Exit the interpreter. Modern interpreters should simply halt operation.
| `00FE` |  | Enable _extended_, 128x64 display mode. This should act as if the existing 64x32 screen buffer is divided to double the number of dots accessible (rather than increasing resolution in any direction). |
| `00FF` |  | Disable _extended_ display mode and revert to _normal_, 64x32 display mode. The existing screen buffer should be left unchanged. |
| `DXY0` | `SHOW VX, VY` | Show 16x16 sprite from `I` at coordinates (`VX`, `VY`). `VF` is still used for collision.
| `FX30` | `LD [I], SFONT VX` | Point I to 10-byte font sprite for digit `VX` (originally this was restricted to `<= 9` but as there is no harm in extending that to the full hex range, this is what xCHIP does). |
| `FX75` | `SAVE V0..VX` | Store `V0`..`VX` in interpreter memory (`X` <= 7) |
| `FX85` | `RESTORE V0..VX` | Store `V0`..`VX` in interpreter memory (`X` <= 7) |
