A GNU-less system... what a dream. I mean a system without GNU software, not the GNU `less` pager utility.

# The journey so far

While it is possible to use LLVM and clang as system compilers and LLVM's libc and libc++ as C standard libraries 
there are some notable problems:
1. Not all software can be compiled with clang,
2. Not all software can be linked with ld.lld,
3. libc and libc++ are slightly incompatible with libstdc and libstdc++ from GNU,
4. Some software developers do not care for compatibility with anything else than GCC.

# What is available?

The very first cut of GNU software on every Linux system is the GNU Coreutils. They can be replaced by uutils written in Rust 
(I love Rust). The people behind the uutils project also offer replacements for other GNU utilities, which ticks off more 
points on our todo list.

Gentoo already uses alternatives to the GNU Autotools, but this is up to developers to use those alternatives.
