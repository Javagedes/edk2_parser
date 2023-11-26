use crate::Config;

#[derive(Default)]
pub struct Inf;

impl Config for Inf {
    fn has_conditionals(&self) -> bool {
        false
    }
    fn no_fail_mode(&self) -> bool {
        false
    }
}
