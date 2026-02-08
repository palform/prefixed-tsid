use std::borrow::Cow;

use apistos::{
    ApiComponent, InstanceType, Schema, SchemaObject, SingleOrVec, reference_or::ReferenceOr,
};
use apistos_schemars::{JsonSchema, SchemaGenerator, r#gen::SchemaSettings};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> ApiComponent for TSIDDatabaseID<Resource> {
    fn child_schemas() -> Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        vec![]
    }

    fn schema() -> Option<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        None
    }

    fn raw_schema() -> Option<apistos::reference_or::ReferenceOr<apistos::Schema>> {
        let generator = SchemaSettings::openapi3().into_generator();
        let schema: ReferenceOr<apistos::Schema> =
            apistos::Schema::Object(generator.into_root_schema_for::<String>().schema).into();
        Some(schema)
    }
}

/**
* Reimplement schemars for version <1 which is needed for Apistos.
* Version 1 implementation is in [schemars.rs].
*/
impl<Resource: TSIDResource> JsonSchema for TSIDDatabaseID<Resource> {
    fn schema_id() -> std::borrow::Cow<'static, str> {
        Cow::Owned(format!(
            "{}::TSID<{}>",
            module_path!(),
            Resource::prefix().unwrap_or("unknown".to_owned())
        ))
    }

    fn schema_name() -> String {
        format!(
            "TSID_{}",
            Resource::prefix().unwrap_or("unknown".to_owned())
        )
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
        })
    }
}
