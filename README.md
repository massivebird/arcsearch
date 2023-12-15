# arcsearch
> short for "archive search"

My command line querying utility for video game archives!

🦀 written in Rust

<p align="center">
  <img width="75%" src="https://imgur.com/b8hfzFN.gif" />
</p>

## What does arcsearch do?

I wanted to search for games without dealing with the slowness and unhelpful presentation of normal file system query methods.

Some features include:

+ Displaying game titles without their [codes](https://www.emuparadise.me/help/romnames.php) such as `(USA, Europe)`, `[!]`, and `(Disk 2)`
+ Displaying systems with visually distinct colors
+ Querying with regular expressions

## Getting started

```bash
git clone https://github.com/massivebird/arcsearch
cd arcsearch
cargo run
```

## Usage

Arcsearch finds the root of your archive using the environment variable `VG_ARCHIVE`. You can set this during testing like so:

```bash
VG_ARCHIVE="path/to/archive/root" cargo run
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
