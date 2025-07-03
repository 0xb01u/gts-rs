# pkm\_utils library

This is a small library of utilities for working with Generation IV and V Pokémon and GTS data.

This library is intended to be used as support for the `gts-rs` application.  It is not as fully-fledged and it does not contain as many functionalities to be considered a general-purpose, standalone library; but it contains more functionalities than those required by `gts-rs`.

The main reasons to have developped this library are:

 * To have ported all the code/functionalities of the orginal IR-GTS-MG python script to Rust, which included many functions that were unused.
 * To avoid the Rust compiler from complaining about unused code in the `gts-rs` application, in a way that is somewhat elegant and makes sense.
