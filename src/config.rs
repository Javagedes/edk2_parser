use std::collections::HashMap;

use crate::{ParseFn, section::*};

pub trait Config: Default {
    fn has_conditionals(&self) -> bool;
    fn supported_sections(&self) -> HashMap<&str, ParseFn>;
}

#[derive(Default)]
pub struct Inf;

impl Config for Inf {
    fn has_conditionals(&self) -> bool {
        false
    }
    fn supported_sections(&self) -> HashMap<&str, ParseFn> {
        HashMap::from([
            ("defines", Box::new(defines::parse) as ParseFn),
            ("sources", Box::new(sources::parse) as ParseFn),
            ("buildoptions", Box::new(buildoptions::parse) as ParseFn),
            ("binaries", Box::new(binaries::parse) as ParseFn),
            ("depex", Box::new(depex::parse) as ParseFn),
            ("fixedpcd", Box::new(pcds::parse) as ParseFn),
            ("guids", Box::new(guids::parse) as ParseFn),
            ("featurepcd", Box::from(pcds::parse)),
            ("libraryclasses", Box::new(libraryclasses::parse) as ParseFn),
            ("packages", Box::new(name_only::parse) as ParseFn),
            ("patchpcd", Box::new(patchpcd::parse) as ParseFn),
            ("pcd", Box::new(pcds::parse)),
            ("pcdex", Box::new(pcds::parse)),
            ("ppis", Box::new(name_expression::parse) as ParseFn),
            ("protocols", Box::new(name_expression::parse) as ParseFn),
            ("userextensions", Box::new(name_only::parse) as ParseFn), 
        ])
    }
}

#[derive(Default)]
pub struct Dsc;

impl Config for Dsc {
    fn has_conditionals(&self) -> bool {
        true
    }

    fn supported_sections(&self) -> HashMap<&str, ParseFn>{
        HashMap::new()
    }
    
}
