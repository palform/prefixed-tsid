use std::borrow::Cow;

use schemars::{json_schema, JsonSchema, Schema, SchemaGenerator};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> JsonSchema for TSIDDatabaseID<Resource> {
    fn schema_id() -> Cow<'static, str> {
        format!(
            "{}::TSID<{}>",
            module_path!(),
            Resource::prefix().unwrap_or("unknown".to_owned())
        )
        .into()
    }

    fn schema_name() -> Cow<'static, str> {
        format!(
            "TSID_{}",
            Resource::prefix().unwrap_or("unknown".to_owned())
        )
        .into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        match Resource::prefix() {
            Some(p) => json_schema!({
                "type": "string",
                "description": format!("TSID with prefix {}_", p),
                "pattern": format!("^({}_)(\\w|\\d)+$", p)
            }),
            None => json_schema!({
                "type": "string",
                "description": "TSID without prefix"
            }),
        }
    }
}
