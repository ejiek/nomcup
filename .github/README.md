# NomCup

![NomCup the mascot](nomcup.png)

This is a Rust library that parses `PKGBUILD` files used by Pacman, the package manager.

There are three main sources of inspiration for the name:
* [namcap](https://gitlab.archlinux.org/pacman/namcap) — a Pacman package analyzer
* [nOm](https://github.com/rust-bakery/nom) — an awesome Rust library to create parsers
* [rUst](https://www.rust-lang.org) — language this library is written in

Now it's a hungry cup, waiting to eat all the tokens from your `PKGBUILD`.

## Goals

* Parse a PKGBUILD file into a reasonable Rust structure
* Understand and render variables
* Generate back a file with the original formatting

## Secondary goals

* Generate a file with opinionated formatting
* Provide warnings and recommendations

# Notes

This library is created to provide a proper way to work with PKGBUILD in the `pacops` tool.
One of the key `pacops` features is updating a package.
Let's take [microsoft-edge-dev-bin](https://aur.archlinux.org/cgit/aur.git/tree/PKGBUILD?h=microsoft-edge-dev-bin) as an example.
It has `source` array with `https://packages.microsoft.com/repos/edge/pool/main/m/microsoft-edge-dev/${_pkgname}_${pkgver}-1_amd64.deb` in it.
`pacops` should be able to understand that it's a remote source (not a file in a package directory), that it's a `.deb` file.
If it's a `.deb`, it means it's probably a repository, so it should try to get a list of files in the repo.
From the file name, it should be able to tell their versions and compare them to the current one.
If a newer version is found, `pacops` should update the version and hash fields in PKGBUILD.

We don't need to have this functionality baked into this lib, but we have to provide a reasonable interface to make it easy in `pacops`.

# Ideas
The following should be discussed before implementing.

## Storing token tree next to a Rusty struct

It's a good thing to provide developers with an easy-to-use interface, but there is a goal with something about "original formatting".
It means it is not enough to just parse PKGBUILD and then build it back in a general way.
I'm not sure how well it's going to work, but in a process of parsing, we are building a tree of tokens that also represent formatting.
It might be possible to build a file back from this tree, modifying only relevant pieces.

## Metadata
The parsed result is going to look something like `.SRCINFO`, meaning that all the values are already rendered:
```
https://packages.microsoft.com/repos/edge/pool/main/m/microsoft-edge-dev/${_pkgname}_${pkgver}-1_amd64.deb
```
is going to look like this

```
https://packages.microsoft.com/repos/edge/pool/main/m/microsoft-edge-dev/microsoft-edge-dev_89.0.760.0-1_amd64.deb
```

While it makes a link usable as a link, it's now harder to understand where the version is inside it.
Maybe it's going to be easier to go through the repo when we know how the link was built.
For this, we can have several ways to represent source in our API: rendered, template with links to `PKGBUILD` variables.
Or something like that.

It's possible to look for occurrences of version and pkgname in versions, but there can be some extra logic (`-git` or anything else).

# Relevant third-party Documentation

* [Arch Linux Wiki PKGBUILD page](https://wiki.archlinux.org/title/PKGBUILD)
* [Pacman manual](https://wiki.archlinux.org/title/PKGBUILD)
* [Pacman & libalm sources](https://gitlab.archlinux.org/pacman/pacman)
* [Bash syntax](https://www.gnu.org/software/bash/manual/bash.html#Shell-Syntax)
* [Arch Linux packaging guidlines](https://wiki.archlinux.org/title/Arch_package_guidelines)
* [Arch Linux repository sources](https://github.com/archlinux/svntogit-packages) - PKGBUILD examples
* [Arch Linux User Repository](https://aur.archlinux.org/) / [gh mirror](https://github.com/archlinux/aur) - PKGBUILD examples
