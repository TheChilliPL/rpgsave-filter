# rpgsave-filter

Git filter driver for RPG maker MV/MZ save files!

## Installation

### Automatic

You need to build this tool from source (`cargo build`) or otherwise get the executable for your system.
You can use [`cargo binstall rpgsave-filter`](<https://github.com/cargo-bins/cargo-binstall?tab=readme-ov-file#installation>)
to get the executable without building it locally.

Then, in the directory of the repo you want to install it to, simply run:
```shell
rpgsave-filter install
```

This will automatically add this driver to the project's `.git/config` with the same
path that you used to call it.

### Manual

First, add this filter driver to your `.gitconfig` or the `.git/config` file of the project:
```
[filter "rpgsave"]
    clean = <PATH TO RPGSAVE-FILTER EXECUTABLE> clean
    smudge = <PATH TO RPGSAVE-FILTER EXECUTABLE> smudge
    required = true
```

If this is done in a repository that already had it used, be sure to clear the index and
check out the repository again:
```
rm .git/index
git checkout HEAD -- .
```

## Usage

You can set `.rpgsave` files to use it by appending to `.gitattributes`:
```
*.rpgsave filter=rpgsave
```

Git will automatically use this filter whenever it's necessary.
