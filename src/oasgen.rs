use oasgen::{OaSchema, Schema, SchemaData, StringType};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> OaSchema for TSIDDatabaseID<Resource> {
    fn schema() -> oasgen::Schema {
        Schema {
            data: SchemaData {
                nullable: false,
                ..Default::default()
            },
            kind: oasgen::SchemaKind::Type(oasgen::Type::String(StringType::default())),
        }
    }
}
