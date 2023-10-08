# Minimal LED blinky demo for the Raspberry Pi Pico

Unlike most of the demos, this is not set up to use OpenOCD, because as of this
writing the Raspberry Pi folks haven't upstreamed their OpenOCD support.
Instead, you can run this by installing `elf2uf2-rs`:

`cargo install elf2uf2-rs`

Once that's installed,

1. Hold down the Pi Pico's BOOTSEL button.
2. Connect it via USB.
3. Wait for it to appear as a drive.
4. `cargo run --release`
