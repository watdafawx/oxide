Your decoder trusts the bytes it is given. The checker reads bytes from servers it doesn't control, and a broken server, or someone poking at your checker on purpose, can send a VarInt whose "more follows" bit never turns off. Run the tests and see what the decoder does with that.

**What happens now.** The test hands over six bytes that all say "more follows" except the last. At the sixth byte, `7 * i` is 35, and a `u32` has only 32 bits to shift through. Rust stops the program:

```text
thread 'tests::malformed_input_errors' panicked at src/lib.rs:
attempt to shift left with overflow
```

**Five is the limit.** An `i32` has 32 bits, and five 7-bit groups hold 35, so a valid VarInt never needs more than five bytes. A sixth byte can only mean broken or hostile input, so the decoder should say so with an error the caller can handle: `Err(VarIntError::TooLong)`.

**Check first.** The check has to come before the byte is used: at the top of the loop, look at the position `i`. If it is already 5, that byte would be the sixth, so return the error before shifting anything.

**Why it matters.** "Never trust the length blindly" is the rule behind most network bugs. A server that sends `ff ff ff ...` forever must cost your checker five bytes and an error, never a crash and never a hang. The boss applies the same rule to whole packet lengths.

## Your turn

- In `decode_varint`, stop with `TooLong` before using a sixth byte.
