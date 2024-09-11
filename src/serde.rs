use std::marker::PhantomData;

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> Serialize for TSIDDatabaseID<Resource> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct TSIDDatabaseIDVisitor<Resource: TSIDResource> {
    resource: PhantomData<Resource>,
}

impl<'de, Resource: TSIDResource> Visitor<'de> for TSIDDatabaseIDVisitor<Resource> {
    type Value = TSIDDatabaseID<Resource>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "string TSID prefixed with {}_",
            Resource::prefix().unwrap_or("nothing".to_owned())
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        TSIDDatabaseID::<Resource>::from_str(v)
            .map_err(|e| E::invalid_value(Unexpected::Str(&e.to_string()), &self))
    }
}

impl<'de, Resource: TSIDResource> Deserialize<'de> for TSIDDatabaseID<Resource> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = TSIDDatabaseIDVisitor::<Resource> {
            resource: PhantomData,
        };
        deserializer.deserialize_string(visitor)
    }
}
