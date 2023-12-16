# arcsearch
> short for "archive search"

My command line querying utility for video game archives!

🦀 written in Rust

<p align="center">
  <img width="75%" src="https://imgur.com/b8hfzFN.gif" />
</p>

## What does arcsearch do?

Arcsearch allows you to query your game collection with ease! Some features include:

+ Regular expression support
+ Displays game titles without their [codes](https://www.emuparadise.me/help/romnames.php) such as `(USA, Europe)`, `[!]`, and `(Disk 2)`
+ Displays game systems with distinct, customizable names

## Building

To manually build the project, you must first [install Rust](https://www.rust-lang.org/tools/install).

Once you have Rust installed, run the following commands:

```bash
git clone https://github.com/massivebird/arcsearch
cd arcsearch
cargo run # runs unoptimized build
```

### Adding arcsearch to your PATH

If you want to add arcsearch to your PATH, I recommend building it in release mode for better optimization.

```bash
cd arcsearch
# build release mode
cargo build --release
# add arcsearch to your PATH
ln -s ./target/release/arcsearch <dir-in-PATH>
# run arcsearch
arcsearch
```

## Usage

Basic arcsearch syntax is as follows:

```bash
# if running with cargo
cargo run [query-regex]

# if added to your path
arcsearch [query-regex]
```

Omitting the query argument displays all games in your collection.


### Locating your archive

Arcsearch finds the root of your archive using the environment variable `VG_ARCHIVE`.

You can temporarily define this variable during testing like so:

```bash
VG_ARCHIVE="path/to/archive/root" cargo run [query-regex]
```

### Customization

Arcsearch looks for a `config.yaml` file in the root of your archive. This configuration file tells arcsearch where and how to look for games!

> For a quickstart on YAML syntax, click [here](https://docs.ansible.com/ansible/latest/reference_appendices/YAMLSyntax.html).

Here is an example configuration:

```yaml
systems:
  ds: # system "label" — call it whatever you want!
    display_name: "DS"
    color: [135,215,255]
    path: "ds" # path relative to archive root
    games_are_directories: false # are games stored as directories?
  snes:
    display_name: "SNES"
    color: [95,0,255]
    path: "snes"
    games_are_directories: false
  wii:
    display_name: "WII"
    color: [0,215,255]
    path: "wbfs"
    games_are_directories: true
```
