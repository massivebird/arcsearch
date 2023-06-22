# arcsearch
> short for "archive search"

My command line querying utility for video game archives!

🦀 written in Rust

## why

I wanted to search for games without dealing with the slowness and unhelpful presentation of normal file system query methods.

Arcsearch delivers:

+ Clean titles without their [codes](https://www.emuparadise.me/help/romnames.php) such as `(USA, Europe) [!] (Disk 2)`
+ Systems with visually distinct colors
+ Querying with regular expressions

## getting started

```cmd
git clone https://github.com/massivebird/arcsearch
cd arcsearch
```

## usage

Arcsearch finds the root of your archive using the environment variable `VG_ARCHIVE`. You can set this during testing like so:

```rust
VG_ARCHIVE="path/to/archive/root" cargo run
```

Systems and their directory names are currently hard-coded [here](https://github.com/massivebird/archive_systems/blob/2dc96e2ac6191741f9995cd6a1c337d9bbf01a79/src/lib.rs#L28) inside the `archive_systems` dependency. I'll probably make this more modular later.

## demos

![demo-metal-gear](https://imgur.com/b8hfzFN.gif)
