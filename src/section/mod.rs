mod common;
mod specific;
pub use common::*;
pub use specific::*;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum SectionType {
    Defines,
    Sources,
    Binaries,
    Protocols,
    Ppis,
    Guids,
    LibraryClasses,
    Packages,
    FeaturePcd,
    FixedPcd,
    PatchPcd,
    Pcd,
    PcdEx,
    Depex,
    UserExtensions,
    BuildOptions,
}
