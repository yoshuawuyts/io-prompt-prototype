//! A prototype for `io::prompt`, `io::promptln`, and `io::read_line`.
//! # Examples
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! use io_prompt_prototype::{prompt, promptln};
//!
//! let num: u16 = prompt!("What's your favorite number? >").parse()?;
//! println!("Oh, cool: {}!", num);
//!
//! let variable = "snack";
//! let res = promptln!("What's your favorite {}? >", variable);
//! println!("We love {} too!", res);
//! # Ok(()) }
//! ```
//!
//! # Design Considerations
//!
//! A survey of existing prompt functions can be found
//! [here](https://github.com/rust-lang/rust/pull/75435#issuecomment-680278635)
//! This library makes several tradeoffs in its design:
//!
//! - Just like the `std::io::println!` family of macros, the `prompt` macros
//!     panic in the case of an error.
//! - The prompt macros don't support parsing of values in-place. Users are
//!     encouraged to `.parse` instead.
//! - The prompt family of macros only support reading a single line at the time
//!     and assigning it to a value.
//!
//! This library is split into two parts: a convenient `read_line` function
//! which is a shorthand for calling `Stdin::read_line` and reading into a new
//! string. And the `prompt!` family of macros which support reading from
//! writing to stdout/stderr, and reading a value from stdin.
//!
//! The focus for the `prompt` family of macros is on simplicity: its goal is to
//! make it convenient to write quick prompts inside Rust programs in a way that feels similar to using `println!`. It does not
//! try and cover parsing rules by introducing a new DSL. Such a DSL almost
//! certainly needs to have regex-like capabilities, and would be nearly
//! impossible to stabilize.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use std::io::{self, stdin};

/// Reads a line of input from stdin.
///
/// This is a shorthand for calling [`Stdin::read_line`] and reading
/// it into a new string.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// use io_prompt_prototype::read_line;
///
/// print!("What's your favorite number? >");
/// let num: u16 = read_line()?.parse()?;
/// println!("Oh, cool: {}!", num);
/// # Ok(()) }
/// ```
///
/// [`Stdin::read_line`]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

/// Prints to the standard output. Then reads a line of input.
///
/// This is a shorthand for calling [`print!`], [`read_line`], and removing
/// any trailing newlines from the output.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// use io_prompt_prototype::prompt;
///
/// let num: u16 = prompt!("What's your favorite number? >").parse()?;
/// println!("Oh, cool: {}!", num);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! prompt {
    ($($arg:tt)*) => {{
        use std::io::{stdout, Write};
        print!($($arg)*);
        stdout().flush().expect("failed writing to stdout");
        let mut s = $crate::read_line().expect("failed reading from stdin");
        if s.ends_with('\n') {
            s.pop();
        }
        if s.ends_with('\r') {
            s.pop();
        }
        s
    }};
}

/// Prints to the standard output, with a newline. Then reads a line of input.
///
/// This is a shorthand for calling [`println!`], [`read_line`], and removing
/// any trailing newlines from the output.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// use io_prompt_prototype::promptln;
///
/// let num: u16 = promptln!("What's your favorite number? >").parse()?;
/// println!("Oh, cool: {}!", num);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! promptln {
    ($($arg:tt)*) => {{
        use std::io::{stdout, Write};
        println!($($arg)*);
        stdout().flush().expect("failed writing to stdout");
        let mut s = $crate::read_line().expect("failed reading from stdin");
        if let Some(_) = s.strip_suffix('\n') {
            let _ = s.strip_suffix('\r');
        }
        s
    }};
}

/// Prints to the standard error. Then reads a line of input.
///
/// This is a shorthand for calling [`eprint!`], [`read_line`], and removing
/// any trailing newlines from the output.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// use io_prompt_prototype::eprompt;
///
/// let num: u16 = eprompt!("What's your favorite number? >").parse()?;
/// println!("Oh, cool: {}!", num);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! eprompt {
    ($($arg:tt)*) => {{
        use std::io::{stdout, Write};
        print!($($arg)*);
        stdout().flush().expect("failed writing to stdout");
        let mut s = $crate::read_line().expect("failed reading from stderr");
        if s.ends_with('\n') {
            s.pop();
        }
        if s.ends_with('\r') {
            s.pop();
        }
        s
    }};
}

/// Prints to the standard error, with a newline. Then reads a line of input.
///
/// This is a shorthand for calling [`eprintln!`], [`read_line`], and removing
/// any trailing newlines from the output.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// use io_prompt_prototype::epromptln;
///
/// let num: u16 = epromptln!("What's your favorite number? >").parse()?;
/// println!("Oh, cool: {}!", num);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! epromptln {
    ($($arg:tt)*) => {{
        use std::io::{stdout, Write};
        println!($($arg)*);
        stdout().flush().expect("failed writing to stdout");
        let mut s = $crate::read_line().expect("failed reading from stderr");
        if let Some(_) = s.strip_suffix('\n') {
            let _ = s.strip_suffix('\r');
        }
        s
    }};
}
