# Computer Science

---

## Mac

### Battery Remain on cli

`pmset -g batt` outputs battery percentage

---

## Shell

### Symbolic link tip

When you use `ln -s`, arguments have to be **absolute** path.
Make sure that not relying on current path.

### Makefile

Makefile is make's config file. Make sure that name is not MakeFile.
This is invalid name so simply not working.

### LLDB

lldb's help is very useful. Use actively.

### Useful

>***Environment Variable***

- `XDG_CONFIG_HOME` (default is $HOME/.config)

>***Useful Shell Command***

- `date`
date command prints current time, day, year, day of the week on certain time zone
- `builtin <command> [option]`
execute builtin command

---

## Rust

### `pub(path)` conjection

>see [official reference](https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself), [related error](https://doc.rust-lang.org/error-index.html#E0704), [my playground example](https://github.com/ah-y/playground/blob/master/rust/elseeee/src/main.rs)

Since 2018 Edition, `path` for `pub(path)` must start with crate, super.

---

## Vim

### Todo highlight

>see https://qiita.com/skkzsh/items/fe40e06c0d4943389be1

In rust file, TODO, FIXME, XXX, NB, NOTE in comment is highlighted.
This is defined in `$VIMRUNTIME/syntax/rust.vim`.  
**For other languages, in `$VIMRUNTIME/syntax/(language name).vim`.**
For example, in lua file, TODO, FIXME, XXX in comment is highlighted
as defined in  `$VIMRUNTIME/syntax/lua.vim`
