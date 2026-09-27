# Building CalcTUI

## Requirements

- Rust 1.85 or newer (the crate uses edition 2024)
- GNU make, for the shortcuts below — every target is a plain `cargo` command
- On Arch: `sudo pacman -S rust make`

## Clone

```sh
git clone https://github.com/YOUR_USER/CalcTUI.git
cd CalcTUI
```

## Build

```sh
make build      # cargo build --release  ->  target/release/calctui
make test       # 26 tests: parser, app state, rendering
make clean
```

## Run

```sh
make run        # cargo run
```

or, once built, run the binary directly:

```sh
./target/release/calctui
```

`q` quits.

## Install the command

```sh
make install    # cargo install --path .  ->  ~/.cargo/bin/calctui
make uninstall  # cargo uninstall
```

`~/.cargo/bin` has to be on your `PATH`. With fish: `fish_add_path ~/.cargo/bin`.

## Build the Arch package

The `PKGBUILD` in this directory builds a native package that installs
`/usr/bin/calctui`:

```sh
make package    # -> packaging/calctui-0.1.0-1-x86_64.pkg.tar.zst
sudo pacman -U packaging/calctui-0.1.0-1-x86_64.pkg.tar.zst
calctui
```

Two things to know before it will build:

- `source=` points at a git repository and a `v<version>` tag, so the code has to
  be pushed and tagged first (`git tag v0.1.0 && git push origin v0.1.0`).
  Until then, point it at a local clone:
  `source=("$pkgname-$pkgver::git+file:///path/to/CalcTUI#tag=v0.1.0")`
- `make package` runs the tests before packaging (the PKGBUILD's `check()`), so if
  it builds, the 26 tests passed.

## Notes

- `depends=()` is intentionally empty: the binary only links against `libc`,
  `libm` and `libgcc_s`, which are already in the base system.
  `makedepends=('rust')` is all that's needed to build it.
- `sha256sums=('SKIP')` is normal for VCS sources: the code is pinned by the tag.
- The ASCII logo needs a terminal of **21 lines or more** and **41 columns**; if
  it doesn't fit, the app hides it on its own rather than squeezing the buttons.
