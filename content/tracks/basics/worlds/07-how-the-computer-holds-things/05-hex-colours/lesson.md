Web and game colours are written like `#FF8800`, which is orange. The painter has to turn a code like that into three numbers from 0 to 255: red, green and blue. The code is written in a number system called hexadecimal.

**Sixteen digits.** **Hexadecimal** (hex for short) counts in sixteens, so it needs sixteen digits. After 0 to 9 come the letters A to F, worth 10 to 15. `F` is 15, and `10` in hex means one sixteen and no ones: 16.

**Two digits, one byte.** One hex digit covers exactly 4 bits (0 to 15), so two hex digits cover one byte: `00` is 0 and `FF` is 15 × 16 + 15 = 255. That is why programmers like hex: it is a short, exact way to write bytes.

**In code.** `0xFF`, with `0x` in front, is a hex number in Lua, Rust, Java and C alike; it is simply 255 written another way. `tonumber("FF", 16)` reads hex text into a number, and the 16 says which counting system the text is in.

**Slicing text.** `string.sub(code, 2, 3)` takes the characters from position 2 to position 3 of a string. For `"#FF8800"` that is `"FF"`. Positions count from 1, like list slots, so position 1 is the `#`.

```lua
local code = "#1E90FF"
local blue_text = string.sub(code, 6, 7)    -- "FF"
local blue = tonumber(blue_text, 16)        -- 255
```

**Why colours use it.** A colour is three bytes: red, green, blue. Six hex digits are exactly three bytes, two digits each. `#FF8800` is red 255, green 136 and blue 0: full red, some green and no blue, which your eye sees as orange.

## Your turn

- In `hex_byte`, turn two hex digits of text into a number.
- In `to_rgb`, cut the code into its three pairs of digits and turn each into a number with `hex_byte`.
- Give back a list of the three: red, green, blue.
