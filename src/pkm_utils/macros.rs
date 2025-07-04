/*
 * GTS-RS - Rust tool for downloading/uploading Pokémon to Gen IV/V games via the in-game GTS.
 * (Rust re-implementation of IR-GTS-MG: https://github.com/ScottehMax/IR-GTS-MG/tree/gen-5)
 * Copyright (C) 2025  Bolu <bolu@tuta.io>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
//! Macros used throughout the library.

#[cfg(debug_assertions)]
/// Macro to indicate that certain code is intended to never be execute in any circumstances.
///
/// In debug builds, this is substituted with the `unreachable!`macro. That is, if executed,
/// this macro panics with the given message.
///
/// In release builds, this is substituted with an unsafe `unreachable_unchecked` call. That is, if
/// executed, this macro produces undefined behavior.
#[macro_export]
macro_rules! should_not_happen {
    ($($msg:expr),*) => {
        unreachable!($($msg),*)
    };
}

#[cfg(not(debug_assertions))]
/// Macro to indicate that certain code is intended to never be execute in any circumstances.
///
/// In release builds, this is substituted with an unsafe `unreachable_unchecked` call. That is, if
/// executed, this macro produces undefined behavior.
#[macro_export]
macro_rules! should_not_happen {
    ($($msg:expr),*) => {
        unsafe { std::hint::unreachable_unchecked() }
    };
}

/// Macro to indicate that certain function returning `Result` is expected to always succeed.
///
/// In debug builds, this is substituted to an `unwrap_or_else` call that calls
/// `unreachable!` macro. That is, if the result is not the expected one, this
/// macro panics with the given message.
///
/// In release builds, this is substituted to an `unwrap_or_else` call that calls
/// an unsafe `unreachable_unchecked`. That is, if the result is not the expected one, this
/// macro produces undefined behavior.
#[macro_export]
macro_rules! should_be_ok {
    ($func:expr, $($msg:expr),*) => {
        $func.unwrap_or_else(|_| { crate::should_not_happen!($($msg),*) })
    };
}

/// Macro to indicate that certain function returning `Option` is expected to always be something.
///
/// In debug builds, this is substituted to an `unwrap_or_else` call that calls
/// `unreachable!` macro. That is, if the result is not the expected one, this
/// macro panics with the given message.
///
/// In release builds, this is substituted to an `unwrap_or_else` call that calls
/// an unsafe `unreachable_unchecked`. That is, if the result is not the expected one, this
/// macro produces undefined behavior.
#[macro_export]
macro_rules! should_be_some {
    ($func:expr, $($msg:expr),*) => {
        $func.unwrap_or_else(|| { crate::should_not_happen!($($msg),*) })
    };
}
