use lazy_regex::regex;
use regex_captures::{Captures, Result};

#[derive(Debug, PartialEq, Eq)]
struct Components {
    prefix: bool,
    infixes: String,
    suffix: Number,
}

#[derive(Debug, PartialEq, Eq)]
struct Number(usize);
impl<'a> TryFrom<&'a str> for Number {
    type Error = std::num::ParseIntError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Number(s.parse()?))
    }
}

fn main() -> Result<()> {
    let numeric = Captures::new(regex!("^(true|false):(.+):([^:]+)$"), "true:two:three:4")?;
    let result = Components {
        prefix: numeric.parse_get(1)?,
        infixes: numeric.get(2)?,
        suffix: numeric.try_get(3)?,
    };
    assert_eq!(
        result,
        Components {
            prefix: true,
            infixes: "two:three".to_owned(),
            suffix: Number(4),
        }
    );

    let named = Captures::new(
        regex!("^(?<prefix>true|false):(?<infixes>.+):(?<suffix>[^:]+)$"),
        "true:two:three:4",
    )?;
    let result = Components {
        prefix: named.parse_name("prefix")?,
        infixes: named.name("infixes")?,
        suffix: named.try_name("suffix")?,
    };
    assert_eq!(
        result,
        Components {
            prefix: true,
            infixes: "two:three".to_owned(),
            suffix: Number(4),
        }
    );

    Ok(())
}
