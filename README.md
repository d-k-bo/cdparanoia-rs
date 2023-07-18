# cdparanoia-rs

<!-- cargo-rdme start -->

High-level bindings for libcdio-paranoia/cdparanoia-3.

By default, this library uses
[libcdio-paranoia](https://github.com/rocky/libcdio-paranoia)
under the hood. If you want to use the original
[cdparanoia-3](https://xiph.org/paranoia/) library
instead, you need to disable the default features and enable
`cdparanoia-3` instead.

```bash
cargo add cdparanoia --no-default-features --features cdparanoia-3
```

## Example

The following example uses [hound](https://lib.rs/crates/hound) to write
the first track of a CD in a default drive to `/tmp/example.wav`.

```rust
let drive = cdparanoia::Drive::find()?;
let mut paranoia = drive.paranoia();

let mut writer = hound::WavWriter::create(
    "/tmp/example.wav",
    hound::WavSpec {
        channels: drive.track_channels(1).unwrap_or(2).into(),
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    },
)?;

for sector_result in paranoia.read_track(1)? {
    let sector = sector_result?;
    let mut writer = writer.get_i16_writer(sector.len() as u32);
    for sample in sector {
        writer.write_sample(sample);
    }
    writer.flush()?;
}
```

<!-- cargo-rdme end -->

# License

This project is licensed under GNU General Public License version 3 or later (GPL-3.0-or-later).
