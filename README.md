# pngme

Implementation of <https://jrdngr.github.io/pngme_book/>
<https://jrdngr.github.io/pngme_book/>

## Build Instructions

### Dev

`cargo build`

### Release

`cargo build --release`

## CLI Help output

Hide secret message in png files

```sh
Usage: pngme <COMMAND>

Commands:
  encode
  decode
  remove
  print
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Sample commands with outputs

```sh
$ cargo run encode dice.png ruSt "this is secret"
$ cargo run decode dice.png ruSt
this is secret
$ cargo run remove dice.png ruSt
$ cargo run decode dice.png ruSt
No secret message found
$ cargo run print dict.png
Chunk {
  Length: 13
  Type: IHDR
  Data: 13 bytes
  Crc: 804134823
}

Chunk {
  Length: 4
  Type: gAMA
  Data: 4 bytes
  Crc: 201089285
}

Chunk {
  Length: 32
  Type: cHRM
  Data: 32 bytes
  Crc: 2629456188
}

Chunk {
  Length: 6
  Type: bKGD
  Data: 6 bytes
  Crc: 4181965695
}

Chunk {
  Length: 7
  Type: tIME
  Data: 7 bytes
  Crc: 3906383041
}

Chunk {
  Length: 32768
  Type: IDAT
  Data: 32768 bytes
  Crc: 1053753753
}

Chunk {
  Length: 32768
  Type: IDAT
  Data: 32768 bytes
  Crc: 2711955290
}

Chunk {
  Length: 32768
  Type: IDAT
  Data: 32768 bytes
  Crc: 1661320401
}

Chunk {
  Length: 32768
  Type: IDAT
  Data: 32768 bytes
  Crc: 864217832
}

Chunk {
  Length: 12435
  Type: IDAT
  Data: 12435 bytes
  Crc: 1720023695
}

Chunk {
  Length: 37
  Type: tEXt
  Data: 37 bytes
  Crc: 2834148272
}

Chunk {
  Length: 37
  Type: tEXt
  Data: 37 bytes
  Crc: 3652195084
}

Chunk {
  Length: 0
  Type: IEND
  Data: 0 bytes
  Crc: 2923585666
}
```

### Known Errors

```sh
➜ cargo run encode dice.png rust "this is secret"
Error: Supplied chunk type value: rust is not valid
```

### TODO

- [ ] Update `lib` to use `snafu` / `thiserror`
- [ ] Map custom errors to relevant Python errors
- [ ] Add test workflow for python build
- [ ] Fix coverage to include pytest coverage for `pngme-python`
