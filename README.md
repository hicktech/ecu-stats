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

- Record - journal events from can to disk
- Play   - play events from disk to can
- Dump   - dump event description to stdout
- Count  - count events and pgns

## About the DBC file

It is not provided.

## Cross compile

```
rustup target add armv7-unknown-linux-gnueabihf
sudo apt install gcc-arm-linux-gnueabihf
```

## License

GPL v3
