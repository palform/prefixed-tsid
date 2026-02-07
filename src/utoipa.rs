use std::borrow::Cow;

use utoipa::{
    PartialSchema, ToSchema,
    openapi::{RefOr, Schema},
};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> ToSchema for TSIDDatabaseID<Resource> {
    fn name() -> std::borrow::Cow<'static, str> {
        let prefix = Resource::prefix();
        match prefix {
            Some(prefix) => Cow::Owned(format!("TSID_{}", prefix)),
            None => Cow::Borrowed("TSID"),
        }
    }
}

impl<Resource: TSIDResource> PartialSchema for TSIDDatabaseID<Resource> {
    fn schema() -> RefOr<Schema> {
        String::schema()
    }
}
