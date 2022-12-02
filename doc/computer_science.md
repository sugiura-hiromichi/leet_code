# Mac

## Battery Remain on cli

`pmset -g batt` outputs battery percentage

## Modify Brightness & Volume Slightly

*function key* with Option+Shift.

## Sound & Display Option ShortCut

`option + f10 (or f11 or f12)` opens sound page of system settings. 
`option + f1 (or f2)` opens display page of system settings. 

***aaa***

---

# Shell

## Symbolic link tip

When you use `ln -s`, arguments have to be **absolute** path.
Make sure that not relying on current path.

## Makefile

Makefile is make's config file. Make sure that name is not MakeFile.
This is invalid name so simply not working.

## LLDB

lldb's help is very useful. Use actively.

## Useful

>***Environment Variable***

- `XDG_CONFIG_HOME` (default is $HOME/.config)

>***Useful Shell Command***

- `date`
date command prints current time, day, year, day of the week on certain time zone
- `builtin <command> [option]`
execute builtin command

---

# Rust

## `pub(path)` conjection

>see [official reference](https://doc.rust-lang.org/reference/visibility-and-privacy.htmlpubin-path-pubcrate-pubsuper-and-pubself), [related error](https://doc.rust-lang.org/error-index.html#E0704), [my playground example](https://github.com/ah-y/playground/blob/master/rust/elseeee/src/main.rs)

Since 2018 Edition, `path` for `pub(path)` must start with crate, super.

## Treatment of lib.rs

lib.rs & main.rs is treated as different crate.

## Formatting Modifier

At least rust 1.66.0(nightly), identifier shortcut & format Modifier can coexist within `{}`.

>example
```rust
let tntn="0tintin"
println!("this is tintin: {tntn}");
println!("this is raw tintin: {tntn:?}");
```

this code prints to stdout that  
`this is tintin: 0tintin  
this is raw tintin: "0tintin"`

### p formatting

>see [more detail](https://doc.rust-lang.org/core/fmt/trait.Pointer.html)

show pointer's memory location by using p formatting.

```rust
let x = &42;
let address = format!("{x:p}"); // this produces something like '0x7f06092ac6d0'
```

## Two iterator in single for loop

```rust
let x=vec![0,1,2,3,4];
let y=vec![4,3,2,1,0];
for (i,j) in x.zip(y){
   println!("{i},{j}");
}
```

as shown above, `zip()` method combines two iterator into single tuple iterator.

## `flat_map()` is not `flattern().map()`. It's `map().flattern()`

As document shows, `flat_map()` behaves not as taking flattern iterator. It behaves as flattern it's own outputs.
Thus, below code is valid

```rust
let some_vector=vec![0,1,2,3,4,5,6];
let another=vec![10,20,30,40];
some_vector.iter().flat_map(|i|{
    another.iter().map(|j| i+j)
    //this collect() has no effect to output
    //.collect()
}).collect();
```

as shown above, `flat_map()` receives iterator of i32. And takes closure which returns iterator.
`flat_map()` flatterns this **iterator of iterator**

---

# Vim

## Todo highlight

>see <https://qiita.com/skkzsh/items/fe40e06c0d4943389be1>

In rust file, TODO, FIXME, XXX, NB, NOTE in comment is highlighted.
This is defined in `$VIMRUNTIME/syntax/rust.vim`.

>**For other languages, in `$VIMRUNTIME/syntax/(language name).vim`.**

For example, in lua file, TODO, FIXME, XXX in comment is highlighted
as defined in  `$VIMRUNTIME/syntax/lua.vim`

## Mode as a Motion

In Normal mode, typing `v` enter visual mode. Then type `iw` selects word the cursor is currently on.
Other example, typing `vi"` selects inner " .. ".

## Get filetype as variable

`vim.bo.filetype` returns filetype name as string. For example, if current buffer is init.lua, vim.bo.filetype=='lua'

## user_commands arguments

If you want to pass ambiguous number of arguments to user_command, set `nargs='*'`

## Telescope Tag Filtering

Press `<c-l>` when entering a picker, some pickers provide tag filtering and it enables filtering picked list.

---

# Markdown

## code block with filename

Here is example.

```rust:code_block.rs
fn block(){
   println("block..");
}
```
