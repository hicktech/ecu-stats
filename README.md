ECU statistics
===

Capture and generate statistics from vehicle electronic control units (ECUs).

Captures events from the CAN bus using [SocketCAN](https://docs.kernel.org/networking/can.html).

Journals events to disk with [sled](https://github.com/spacejam/sled).

## crate: ECU Stats (ecustats)

Accumulate and watch ECU data.

## tool: ECU Flight Recorder (ecufr)

A flight recorder for ECU data.

### Commands

- Record
- Play
- Dump
- Count

## About the DBC file

Not provided here.

## Cross compile

```
rustup target add armv7-unknown-linux-gnueabihf
sudo apt install gcc-arm-linux-gnueabihf
```

Add the following entry to the Cargo config toml

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

## License

GPL v3
