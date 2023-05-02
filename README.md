# ndice
> Role a `n`ice `dice` (or `[n|d]ice`?)

![Header image](assets/github-header.png)

# Table of Contents
<!-- BEGIN mktoc -->

- [About](#about)
- [Installation](#installation)
  - [Binary](#binary)
  - [Cargo ](#cargo)
- [Usage ](#usage)
- [Contributing](#contributing)
  - [Where to start?](#where-to-start)
  - [Tooling](#tooling)
  - [Install dev build](#install-dev-build)
- [License](#license)
<!-- END mktoc -->

## About

`ndice` is a cli dice roller. 

Dices are writen in the format `${number}d{$sides}`, e.g. `1d6` means 1 six-sided dice. There's no limit to sides so a non-existing dice like `1d13` can be rolled. 

- Each space-speparated argument is counted as one dice notation.
- Dices can be mixed, so `ndice 1d6 2d8 4d10` works.
- Invalid arguments are ignored.


## Installation
[⬆️ Back to Top](#table-of-contents)

### Binary

Download a pre-compiled binary from the [release page](https://github.com/KevinGimbel/ndice/releases)

### Cargo 

Install with cargo: 
```sh
$ cargo install ndice
```

## Usage 
[⬆️ Back to Top](#table-of-contents)

```sh
$ ndice [dice]
```

Examples:

```sh
# Roll 1 six-sided dice
$ ndice 1d6
Rolled: ["d6 => 1"]
Result: 1
# Roll 2 8 sided dice and 2 four sided dice
$ ndice 2d8 2d4
Rolled: ["d8 => 6", "d8 => 3", "d4 => 1", "d4 => 1"]
Result: 11
```


## Contributing
[⬆️ Back to Top](#table-of-contents)

We love and welcome every form of contribution.

### Where to start?

Here are some good places to start:

* Issues with label [Good first issue](https://github.com/kevingimbel/ndice/labels/good%20first%20issue)
* Issues with label [Documentation](https://github.com/kevingimbel/ndice/labels/documentation)
* Providing example implementations or usage demos

### Tooling

- [mktoc](https://github.com/KevinGimbel/mktoc) is used for table of content generation in the README.md (neat!)

### Install dev build

Sometimes it's nice to install a specific version of ndice, this can be done with the following command:

```sh
# install specific commit
cargo install --git https://github.com/KevinGimbel/ndice--force --rev $COMMIT_ID
# install branch
cargo install --git https://github.com/KevinGimbel/ndice--force --branch $BRANCH_NAME
```


## License
[⬆️ Back to Top](#table-of-contents)

MIT, see LICENSE file.