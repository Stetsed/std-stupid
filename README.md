# std-stupid

Self-written library for rust to use... being very stupid

This is a project I am using to learn how to write and use the rust language, I have at the time of writing implemented a HTTP Server and some basic functions like finding a sub-string with bytes and a String(Same thing really, but I learned this later). Due to this learning goal I have decided to only use std/core libraries inside of the project and not use any crates etc to force myself to think more about stuff like implementing my own error types etc. And not just "Add crate X cuz it gives us Y", so the HTTP Server is only implemented using standard functions with std net etc.*

This library is NOT ment for any type of production deployment and is purely meant for me to learn, if you wish to use it for this purpose I would recommend going through commit history as you can see quiet a few times where I realized there was a better way, but besides that pick a project you find cool and do it. I started with a D&D random dice generator but eventually ended up with the HTTP Server which I am slowly writing.

* Due to rust having a relativley small STD on purpose. Crates which are "defacto std", things that would usually be in a standard library in C++ for example like random generator is excluded from this, so for example rand or other such things are used here and there.

Very helpful sources:

* [Rust-Lang-Book](https://doc.rust-lang.org/book/title-page.html) - Generally has info on alot of basic concepts within rust
* [Rustlings](https://github.com/rust-lang/rustlings/) - Good exercises to get standard, used them in the beginning but found that for me working on smth I find cool and figuring it out along the way kept me motivated.
* [The Internet](https://duckduckgo.com) /s, learning how to search is a big one and knowing what parts of errors you should/shouldn't use for a search is trial and error. But once you can use it you can usually find an answer to most questions, and if not you can maybe find something to help you on you're way.
