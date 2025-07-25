# rpgsave-filter

Git filter driver for RPG maker MV/MZ save files!

## Usage

Add this filter driver to your `.gitconfig` or the `.git/config` file of the project:
```
[filter "rpgsave"]
    clean = <PATH TO RPGSAVE-FILTER EXECUTABLE> clean
    smudge = <PATH TO RPGSAVE-FILTER EXECUTABLE> smudge
```
and set `.rpgsave` files to use it by appending to .gitattributes:
```
*.rpgsave filter=rpgsave
```
