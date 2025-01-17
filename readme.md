[![Build status](https://img.shields.io/github/actions/workflow/status/eum2o/fair_next/build.yml?branch=master)](https://github.com/eum2o/fair_next/actions)
[![Latest version](https://img.shields.io/crates/v/fair_next.svg)](https://crates.io/crates/fair_next)
[![GitHub Release](https://img.shields.io/github/v/release/eum2o/fair_next?label=download&link=https%3A%2F%2Fgithub.com%2Feum2o%2Fftree%2Freleases)](https://github.com/eum2o/fair_next/releases)
![Crates.io License](https://img.shields.io/crates/l/fair_next?color=%238b55d7)


# Fair Next

A simple command line tool that displays a list. When a list item is selected, it's moved to the bottom. The list is
persisted in a `names.txt` file next to the executable.

Motivation: I attend online meetings (e.g. daily standup) where I have to select the next speaker. I use this tool for a
fair selection of the next speaker.

![screenshot.png](assets/screenshot.png)

## Installation

You can either download the executable manually or use Cargo to install this tool.

### Cargo

You can install `fn` using [Cargo](https://doc.rust-lang.org/cargo/):

```
cargo install fair_next
```

For more information about the crate, visit [https://crates.io/crates/fair_next](https://crates.io/crates/fair_next).

### Pre-built Executables

Alternatively, you can download pre-built executables for various platforms from the GitHub releases page:

[https://github.com/eum2o/fair_next/releases](https://github.com/eum2o/fair_next/releases)

## How to Use

- Run the downloaded executable via terminal or by double-clicking it (if downloaded with Cargo, run `fn` in your terminal).
- If you run it the first time, you'll see a message that you need to add names (or arbitrary list items) to
  the `names.txt` file next to the executable. The path is displayed in the UI.
- Select a user with the arrow keys and hit `Enter` to confirm. The user will be moved to the bottom.
- Save to save without quitting.
- Quit to quit without saving.

