# NomCup

![NomCup the mascot](nomcup.png)

This is a Rust library that parses `PKGBUILD` files used by pacman the package manager.

There are three main sources of inspiration for name:
* [namcap](https://projects.archlinux.org/namcap.git/) — a Pacman package analyzer
* [nOm](https://github.com/Geal/nom) — an awesome Rust library to create parsers
* [rUst](https://www.rust-lang.org) — language this library is written in

Now it's a hungry cup, waiting to eat all the tokens from your `PKGBUILD`.

## Goals

* Parse PKGBUILD file into a reasonable Rust structure
* Understand and render variables
* Generate back a file with original formatting

## Secondary goals

* Generate file with opinionated formatting
* Provide warnings and recommendations

# Notes

This library is created to provide a proper way to work with PKGBUILD in `pacops` tool.
One of key `pacops` features is updating a package.
Let's take [https://aur.archlinux.org/cgit/aur.git/tree/PKGBUILD?h=microsoft-edge-dev-bin](https://aur.archlinux.org/cgit/aur.git/tree/PKGBUILD?h=microsoft-edge-dev-bin) as an example.
It has `source` array with `https://packages.microsoft.com/repos/edge/pool/main/m/microsoft-edge-dev/${_pkgname}_${pkgver}-1_amd64.deb` in it.
`pacops` should be able to understand that it's a remote source (not a file a package directory), that it's a `.deb` file.
If it's a `.deb` it means it's probably a repository, so it should try to get a list of files in the repo.
From file name it should be able to tell their versions and compare to the current one.
If newer version is found, `pacops` should update version and hash in PKGBUILD.

We don't need to have this functionality baked into this lib, but we have to provide reasonable interface to make it easy in `pacops`.

# Ideas
Following stuff should be discussed before implementing

## Storing token tree next to a Rusty struct

It's a good thing to provide developers with an easy to use interface, but there is a goal with something about "original formatting".
It means it not enough to just parse PKGBUILD and then build it back in a general way.
I'm not sure how well it's going to work but in a process of parsing we are building a tree of tokens which also represent formatting.
It might be possible to build a file back from this tree modifying only relevant pieces.

## Metadata
Parsed result is going to look something like `.SRCINFO` meaning that all the values are already rendered:
```
https://packages.microsoft.com/repos/edge/pool/main/m/microsoft-edge-dev/${_pkgname}_${pkgver}-1_amd64.deb
```
is going to look like this

```
https://packages.microsoft.com/repos/edge/pool/main/m/microsoft-edge-dev/microsoft-edge-dev_89.0.760.0-1_amd64.deb
```

While it makes a link usable as a link it's now harder to understand where the version is inside it.
Maybe it's going to be easier to go through the repo when we know how the link was build.
For this we can have several ways to represent source in out API: rendered, template with links to `PKGBUILD` variables.
Or something like that.

It's possible to look for occurrences of version and pkgname in versions but there can be some extra logic (`-git` or anything else).

# Links

* [libalpm](https://git.archlinux.org/pacman.git/tree/lib/libalpm) - Arch Linux Package Management (ALPM) library
* [Pacman homepage](https://archlinux.org/pacman/)
