use crate::error::{Error, Result};

pub struct SUCCDeserializer<'a> {
    input: &'a str,
}

impl<'a> SUCCDeserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Self { input }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}