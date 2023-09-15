use anyhow::{anyhow, Context};
use regex::{self, Regex};
use std::fmt::Debug;
use std::str::FromStr;

pub struct Captures<'a>(regex::Captures<'a>);

impl<'a> Captures<'a> {
    pub fn new<'b: 'a>(regex: &Regex, s: &'b str) -> Result<Captures<'a>, anyhow::Error> {
        let c = regex
            .captures(s)
            .with_context(|| format!("{regex} did not match {s}"))?;
        Ok(Captures(c))
    }

    pub fn get<A: From<&'a str>>(&self, n: usize) -> Result<A, anyhow::Error> {
        self.0
            .get(n)
            .with_context(|| format!("Could not find capture group {n}"))
            .map(|m| m.as_str().into())
    }

    pub fn parse<A>(&self, n: usize) -> Result<A, anyhow::Error>
    where
        A: FromStr,
        A::Err: Debug,
    {
        let value = self
            .0
            .get(n)
            .with_context(|| format!("Could not find capture group {n}"))?
            .as_str();

        value
            .parse()
            .map_err(|e| anyhow!("Could not parse {value} in capture group {n}, error: {e:?}"))
    }

    pub fn try_get<A>(&self, n: usize) -> Result<A, anyhow::Error>
    where
        A: TryFrom<&'a str>,
        A::Error: Debug,
    {
        let value = self
            .0
            .get(n)
            .with_context(|| format!("Could not find capture group {n}"))?;
        let value = value.as_str();

        value
            .try_into()
            .map_err(|e| anyhow!("Could not parse {value} in capture group {n}, error: {e:?}"))
    }

    pub fn name<A: From<&'a str>>(&self, name: &'static str) -> Result<A, anyhow::Error> {
        self.0
            .name(name)
            .with_context(|| format!("Could not find capture group {name}"))
            .map(|m| m.as_str().into())
    }

    pub fn parse_name<A>(&self, name: &'static str) -> Result<A, anyhow::Error>
    where
        A: FromStr,
        A::Err: Debug,
    {
        let value = self
            .0
            .name(name)
            .with_context(|| format!("Could not find capture group {name}"))?
            .as_str();

        value
            .parse()
            .map_err(|e| anyhow!("Could not parse {value} in capture group {name}, error: {e:?}"))
    }

    pub fn try_name<A>(&self, name: &'static str) -> Result<A, anyhow::Error>
    where
        A: TryFrom<&'a str>,
        A::Error: Debug,
    {
        let value = self
            .0
            .name(name)
            .with_context(|| format!("Could not find capture group {name}"))?
            .as_str();

        value
            .try_into()
            .map_err(|e| anyhow!("Could not parse {value} in capture group {name}, error: {e:?}"))
    }
}
