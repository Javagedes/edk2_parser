// #[cfg(test)]
// mod dsc_integration_tests {
//     use edk2_parser::{
//         dsc::Dsc,
//         section::{
//             BuildOptionEntry, FeaturePcdEntry, LibraryClassEntry, PackageEntry, PcdEntry,
//             SourceEntry,
//         },
//         *,
//     };

//     #[test]
//     fn test_prm_pkg() {
//         let data = include_str!("data/PrmPkg.dsc").to_string();
//         let mut dscp = ConfigParser::<Dsc>::new();
//         dscp.parse(data).unwrap();
//     }
// }