use super::*;

mod json;
mod toml;
mod yaml;

#[inline]
fn is_non_alphanumeric(c: Option<char>) -> bool {
    match c {
        None => true,
        Some(c) => !c.is_alphanumeric(),
    }
}