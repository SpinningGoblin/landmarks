use std::str::FromStr;

use serde::{Deserialize, Deserializer};

pub fn comma_separated_list<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(Vec::new()),
        Some(s) => Ok(s
            .split(',')
            .filter_map(|val| FromStr::from_str(val).ok())
            .collect::<Vec<T>>()),
    }
}
