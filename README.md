# CalcTUI

A terminal calculator written in Rust, driven by mouse and keyboard. No runtime
dependencies beyond libc.

```
                    █████ █   █ █     █████ █████ █   █ █████
                    █     █████ █     █       █   █   █   █
                    █████ █   █ █████ █████   █   █████ █████
```

## Install

### Arch (AUR)

```sh
yay -S calctui     # or paru -S calctui
```

### From source

```sh
make install       # installs the calctui command into ~/.cargo/bin
```

## Usage

```sh
calctui
```

`q` quits.

| Key | Action |
| --- | --- |
| `0`-`9` `.` | type the number (only one dot per number) |
| `+` `-` `*` `/` | operators (they replace each other, `2*-3` is allowed) |
| `^` | exponent, right-associative: `2^3^2` = 512 |
| `%` | percentage of the previous number: `200*10%` = 20 |
| `√` | root, postfix (`9√`) or prefix (`√(9+16)`) |
| `(` `)` | parentheses |
| `±` | flips the sign of the last number |
| `⌫` or `Backspace` | delete |
| `Enter` or `=` | evaluate |
| `c` or `AC` | clear |

After an `=` you can keep operating on the result (`10+5=`, `/2` = `7.5`); typing
a number instead starts from scratch. Errors (division by zero, unclosed
parentheses, root of a negative number) show up in red on the top box without
clearing what you were typing.

Everything works with the mouse too: the buttons are `AC ⌫ % √ / 7 8 9 / 4 5 6 * /
1 2 3 - / 0 . ( ) / ± ^ = +`.

## Layout

The app needs a terminal of at least 21 lines and 41 columns to show the logo and
give the buttons some breathing room. On anything smaller the logo hides itself
and the buttons get tighter; no functionality is ever lost.

## Development

```sh
make run      # cargo run
make test     # 26 tests: parser, app state and rendering
make build    # release
make clean
```

The expression parser lives in `src/expresion_parser.rs` (recursive descent, no
extra crates) and the UI in `src/main.rs`.

## Arch packaging

See [`packaging/README.md`](packaging/README.md) for the build and install steps,
and to generate the Arch package.

## License

GPL-3.0-only. See [LICENSE](LICENSE).
