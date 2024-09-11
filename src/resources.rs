use std::hash::Hash;

pub trait TSIDResource: Eq + PartialEq + Clone + Copy + Hash + Send {
    fn prefix() -> Option<String>;
}

#[macro_export]
macro_rules! id_resource_type {
    ($struct_name: ident, $prefix: literal) => {
        #[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
        pub struct $struct_name;
        impl TSIDResource for $struct_name {
            fn prefix() -> Option<String> {
                Some($prefix.to_owned())
            }
        }
    };
}

id_resource_type!(IDOrganisation, "org_");

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct IDUnknown;
impl TSIDResource for IDUnknown {
    fn prefix() -> Option<String> {
        None
    }
}
