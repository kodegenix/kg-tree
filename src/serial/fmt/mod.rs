use super::*;

pub mod json;
pub mod toml;
pub mod yaml;

#[inline]
fn is_non_alphanumeric(c: Option<char>) -> bool {
    match c {
        None => true,
        Some(c) => !c.is_alphanumeric(),
    }
}