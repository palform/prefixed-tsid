use crate::resources::{IDUnknown, TSIDResource};
use anyhow::anyhow;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Deref,
};
use tsid::{create_tsid, TSID};

/// Represents a prefixed, type-safe, resource-specific ID in your database.
/// The resource is defined by `Resource`: when deserializing, that resource must be matched when
/// reading the prefix. When serializing, that resource will be used to create the prefix.
///
/// Internally, this stores a `TSID`, which is actually a `u64`. In your database, you should
/// probably store this `u64` instead of the base32-encoded prefixed string. The numbers are
/// time-ordered so you can sort your database with great performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(type = "string", concrete(Resource = IDUnknown)))]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::expression::AsExpression, diesel::deserialize::FromSqlRow)
)]
#[cfg_attr(feature = "diesel", diesel(sql_type = diesel::sql_types::BigInt))]
pub struct TSIDDatabaseID<Resource: TSIDResource> {
    pub(crate) id: TSID,
    resource: PhantomData<Resource>,
}

impl<Resource: TSIDResource> TSIDDatabaseID<Resource> {
    /// Parse a number (e.g. from your database) into a TSID. As long as it fits into a `u64`, it
    /// will be a valid ID, so this can't error.
    pub fn from_raw_number(number: u64) -> Self {
        Self {
            id: TSID::from(number),
            resource: PhantomData,
        }
    }

    pub fn from_integer(number: i64) -> Self {
        Self {
            id: TSID::from(number as u64),
            resource: PhantomData,
        }
    }

    /// Creates a new, random TSID.
    pub fn random() -> Self {
        Self {
            id: create_tsid(),
            resource: PhantomData,
        }
    }

    /// Returns the `u64` value of the TSID stored internally. Use this to get a value you can
    /// store in your database.
    pub fn to_raw_number(&self) -> u64 {
        self.id.number()
    }

    pub fn into_unknown(&self) -> TSIDDatabaseID<IDUnknown> {
        TSIDDatabaseID::<IDUnknown> {
            id: self.id,
            resource: PhantomData,
        }
    }

    /// Attempts to parse a string into a `TSIDDatabaseID` matching the prefix of the specified
    /// resource. If the string does not contain a prefix, or it contains the wrong one, an error
    /// will be returned instead.
    ///
    /// If the resource does not require a prefix, any string is accepted, as long as it is a valid
    /// base32-encoded TSID.
    pub fn from_str(v: &str) -> Result<TSIDDatabaseID<Resource>, anyhow::Error> {
        let tsid_only = if let Some(prefix) = Resource::prefix() {
            v.strip_prefix(&format!("{}_", prefix))
                .ok_or(anyhow!("missing prefix {}_", prefix))?
        } else {
            v
        };

        let tsid = TSID::try_from(tsid_only).map_err(|_| anyhow!("invalid tsid"))?;
        Ok(TSIDDatabaseID::<Resource>::from(tsid))
    }
}

impl<Resource: TSIDResource> Display for TSIDDatabaseID<Resource> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = Resource::prefix() {
            write!(f, "{}_{}", prefix, self.id.to_string())
        } else {
            write!(f, "{}", self.id.to_string())
        }
    }
}

impl<Resource: TSIDResource> Deref for TSIDDatabaseID<Resource> {
    type Target = TSID;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<Resource: TSIDResource> From<TSID> for TSIDDatabaseID<Resource> {
    fn from(value: TSID) -> Self {
        Self {
            id: value,
            resource: PhantomData,
        }
    }
}
