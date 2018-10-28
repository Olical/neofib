# neofib

An example Neovim plugin written in Rust using [neovim-lib][]. I learnt a lot from [scorched-earth][], the install script is adapted from [LanguageClient-neovim][].

> Disclaimer: I am extremely new to Rust and essentially learning it so I can write a Neovim -> Rust -> Clojure socket / pREPL tool.

All it does is generate the nth Fibonacci number when you execute `:echo neofib#nth(10)`, for example. You can install it with any package manager you like, you'll just need to make sure the binary is built before running it. I use this [vim-plug][] line:

```viml
Plug 'Olical/neofib', { 'do': 'bash install.sh' }
```

This entire repository is pretty much copied and adapted code in an attempt to learn from all of these wonderful original authors I have linked to. I hope this collation can help others learn too!

## Unlicenced

Find the full [unlicense][] in the `UNLICENSE` file, but here's a snippet.

>This is free and unencumbered software released into the public domain.
>
>Anyone is free to copy, modify, publish, use, compile, sell, or distribute this software, either in source code form or as a compiled binary, for any purpose, commercial or non-commercial, and by any means.

Do what you want. Learn as much as you can. Unlicense more software.

[unlicense]: http://unlicense.org/
[neovim-lib]: https://github.com/daa84/neovim-lib
[sorched-earth]: https://github.com/boxofrox/neovim-scorched-earth
[vim-plug]: https://github.com/junegunn/vim-plug
[LanguageClient-neovim]: https://github.com/autozimu/LanguageClient-neovim
