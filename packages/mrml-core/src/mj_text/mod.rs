use crate::mj_body::MJBodyChild;
use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-text";

#[derive(Debug, Default)]
pub struct MJText {
    attributes: Map<String, String>,
    children: Vec<MJBodyChild>,
}
