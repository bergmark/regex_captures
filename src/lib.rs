//! regex_captures makes it easier to work with capture groups
//! when using the [regex](https://docs.rs/regex/latest/regex/) crate.
//!
//! Debugging and type conversions using the
//! regex API can be cumbersome, this crate aims to help with this.
//!
//! All operations are fallible since we do not guarantee that the
//! requested capture groups exist in the supplied regex.
//!
//! Error-reporting is prioritized over performance. Using this crate
//! when there are a lot expected failures in matching or parsing
//! leads to extra allocations.
//!
//! Table of methods:
//!
//! | method                             | capture_type | conversion         | total? |
//! |------------------------------------|--------------|--------------------|--------|
//! | [get](Captures::get)               | numeric      | From&lt;str&gt;    | yes    |
//! | [parse_get](Captures::parse_get)   | numeric      | FromStr            | no     |
//! | [try_get](Captures::try_get)       | numeric      | TryFrom&lt;str&gt; | no     |
//! | [name](Captures::name)             | named        | From&lt;str&gt;    | yes    |
//! | [parse_name](Captures::parse_name) | named        | FromStr            | no     |
//! | [try_name](Captures::try_name)     | named        | TryFrom&lt;str&gt; | no     |
//!
//! Examples using all methods:
//! ```
#![doc = include_str!("../examples/all_methods.rs")]
//! ```

mod error;

use regex::{self, Regex};
use std::fmt::Display;
use std::str::FromStr;

pub use self::error::{CaptureError, Group, Result};

/// See examples in [the top-level documentation](index.html)
pub struct Captures<'a>(regex::Captures<'a>);

impl<'a> Captures<'a> {
    /// Collect captures on a regex
    pub fn new<'b: 'a>(regex: &'a Regex, s: &'b str) -> Result<Captures<'a>> {
        Ok(Captures(
            regex
                .captures(s)
                .ok_or_else(|| CaptureError::no_match(regex, s))?,
        ))
    }

    /// Convert a numeric capture group using `From<&str>`.
    pub fn get<A: From<&'a str>>(&'a self, n: usize) -> Result<A> {
        Ok(self.get_numeric_group(n)?.into())
    }

    /// Attempt parsing a numeric capture group using `FromStr`.
    pub fn parse_get<A>(&'a self, n: usize) -> Result<A>
    where
        A: FromStr,
        A::Err: ToString,
    {
        self.get_numeric_group(n)?
            .parse()
            .map_err(|err| CaptureError::conversion_error(n, err))
    }

    /// Try converting a numeric capture group using `TryFrom<&str>`.
    pub fn try_get<A>(&'a self, n: usize) -> Result<A>
    where
        A: TryFrom<&'a str>,
        A::Error: ToString,
    {
        self.get_numeric_group(n)?
            .try_into()
            .map_err(|err| CaptureError::conversion_error(n, err))
    }

    /// Convert a named capture group using `From<&str>`.
    pub fn name<'b, A: From<&'a str>>(&'a self, name: &'b str) -> Result<A> {
        Ok(self.get_named_group(name)?.into())
    }

    /// Attempt parsing a named capture group using `FromStr`.
    pub fn parse_name<'b, A>(&'a self, name: &'b str) -> Result<A>
    where
        A: FromStr,
        A::Err: Display,
    {
        self.get_named_group(name)?
            .parse()
            .map_err(|err| CaptureError::conversion_error(name, err))
    }

    /// Try converting a named capture group using `TryFrom<&str>`.
    pub fn try_name<'b, A>(&'a self, name: &'b str) -> Result<A>
    where
        A: TryFrom<&'a str>,
        A::Error: Display,
    {
        self.get_named_group(name)?
            .try_into()
            .map_err(|err| CaptureError::conversion_error(name, err))
    }

    fn get_numeric_group(&'a self, n: usize) -> Result<&str> {
        Ok(self
            .0
            .get(n)
            .ok_or_else(|| CaptureError::capture_group_not_found(n))?
            .as_str())
    }

    fn get_named_group<'b>(&'a self, n: &'b str) -> Result<&str> {
        Ok(self
            .0
            .name(n)
            .ok_or_else(|| CaptureError::capture_group_not_found(n))?
            .as_str())
    }
}
