
## Reproduction

```
cargo espflash --release --monitor
```

Which will output the following:

```rust
Exception occured LoadProhibited Context { PC: 42006826, PS: 60130, A0: 82001f50, A1: 3fce2380, A2: 0, A3: 3fce767c, A4: 0, A5: 42009d4c, A6: 1, A7: 10, A8: 4, A9: 0, A10: 38329a61, A11: 3fce76a8, A12: 14, A13: 800000, A14: 0, A15: 3e99999a, SAR: 17, EXCCAUSE: 1c, EXCVADDR: 4, LBEG: 42005293, LEND: 42005321, LCOUNT: fffffffc, THREADPTR: 0, SCOMPARE1: 0, BR: 1, ACCLO: 0, ACCHI: 0, M0: 0, M1: 0, M2: 0, M3: 0, F64R_LO: 0, F64R_HI: 0, F64S: 18, FCR: 0, FSR: 80, F0: 3c5992b9, F1: 0, F2: 0, F3: 0, F4: 0, F5: 3c1a6bb2, F6: 0, F7: 0, F8: 38329a61, F9: 39962fc9, F10: 4f7fffff, F11: 43800000, F12: 0, F13: c0200000, F14: 0, F15: 0 }
0x42006826
0x42006826 - <mi_plaits_dsp::dsp::engine::noise_engine::NoiseEngine as mi_plaits_dsp::dsp::engine::Engine>::render
    at /home/mabez/.cargo/git/checkouts/mi-plaits-dsp-rs-8772d2f6d427e02a/98e82d6/src/dsp/engine/noise_engine.rs:88
0x42001f50
0x42001f50 - mi_plaits_dsp::dsp::voice::Voice::render
    at /home/mabez/.cargo/git/checkouts/mi-plaits-dsp-rs-8772d2f6d427e02a/98e82d6/src/dsp/voice.rs:314
0x4200ffa1
0x4200ffa1 - Reset
    at /home/mabez/.cargo/registry/src/github.com-1ecc6299db9ec823/xtensa-lx-rt-0.13.0/src/lib.rs:77
0x403786fd
0x403786fd - ESP32Reset
    at /home/mabez/.cargo/registry/src/github.com-1ecc6299db9ec823/esp32s3-hal-0.2.0/src/lib.rs:220
0x40000000
0x40000000 - _external_ram_end
    at ??:??
0x403bb474
0x403bb474 - __default_naked_level_7_interrupt
    at ??:??
0x403b622c
0x403b622c - __default_naked_level_7_interrupt
    at ??:??
0x40045c04
0x40045c04 - rom_config_instruction_cache_mode
    at ??:??
0x40043ab9
0x40043ab9 - rom_config_instruction_cache_mode
    at ??:??
0x40034c48
0x40034c48 - rom_config_instruction_cache_mode
    at ??:??
0x40000000
0x40000000 - _external_ram_end
    at ??:??
```