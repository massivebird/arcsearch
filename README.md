# arcsearch has been archived!

I'm archiving arcsearch in favor of the new and improved [bsrc](https://github.com/massivebird/bsrc). Please check it out!

Arcsearch is awesome, and bsrc will grow on the foundation of everything that arcsearch did right. I want this version of arcsearch to remain as a reminder of how far I've come as a developer.

> This project is still fully functional, so try it out if you want :3 I'll see you in bsrc!

---

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

### What is a valid video game archive?

A valid archive file system structure should follow these general rules:

+ Contains a `config.yaml` in the archive root (see: [Customization](#customization))
+ Immediate root subdirectories represent individual game systems
+ Files in system directories represent individual games
  + These files can either be normal files or directories

Here is an example of a valid archive structure:

```bash
/game/archive/root
├── ds
│   ├── game-1.nds
│   ├── game-2.nds
│   └── game-3.nds
├── wii
│   ├── game-1-dir
│   │   └── game-1.wbfs
│   └── game-2-dir
│       └── game-2.wbfs
└── config.yaml
```

> [!tip]
> While it is possible to place system directories multiple levels below the archive root (such as in `root/systems/consoles/ps2`), __I do not recommend nesting system directories.__ This may generate undesirable results.

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
ln -rs ./target/release/arcsearch <dir-in-PATH>/arcsearch
# run arcsearch
arcsearch
```

## Usage

Basic arcsearch syntax is as follows:

```bash
arcsearch <query>
```

For more information, run `arcsearch --help`.


### Locating your archive

To find your archive, arcsearch defaults to reading the environment variable `VG_ARCHIVE`.

You can also provide this path from the command line:

```bash
arcsearch --archive-path /path/to/archive "some query"
```

<h3 id="customization">Customization</h3>

Arcsearch looks for a `config.yaml` file in the root of your archive. This configuration file tells arcsearch where and how to look for games!

> For a quickstart on YAML syntax, click [here](https://docs.ansible.com/ansible/latest/reference_appendices/YAMLSyntax.html).

Here is an example configuration:

```yaml
# config.yaml
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

## Other arcosystem projects

Arcsearch belongs to a family of projects called the arcosystem!

Check out some other arcosystem projects:

+ [arcstat](https://github.com/massivebird/arcstat): game archive stats
+ [arcconfig](https://github.com/massivebird/arcconfig): backbone of the arcosystem
