# Privod (оптический привод)

Eject CD/DVD/BD drive at random time.

Linux only.

## Usage

```shell
cargo run <PATH_TO_DRIVE> [RANGE_START] [RANGE_END]
```

`PATH_TO_DRIVE` is usually `/dev/sr0`, `RANGE_START` (in secs) and `RANGE_END` (in secs) are optional (default is 5 and 300).

## License

The project is licensed under the terms of the [MIT license](./LICENSE).
