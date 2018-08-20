pub use arena::*;
pub use cards::*;

use crate::server::validators::length_validator;

mod arena;
mod cards;

pub fn clash_royale_tag_validator(tag: &str) -> bool {
    if length_validator(tag.len(), 6, 15, "").is_err() {
        return false;
    }

    for char in tag.chars() {
        if !matches!(char, '0'..='9' | 'A'..='Z') {
            return false;
        }
    }

    true
}
