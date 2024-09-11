use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryFromU64, TryGetError, TryGetable, Value,
};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> TryGetable for TSIDDatabaseID<Resource> {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        <i64 as TryGetable>::try_get_by(res, index).map(|v| TSIDDatabaseID::from_integer(v))
    }
}

impl<Resource: TSIDResource> ValueType for TSIDDatabaseID<Resource> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        <i64 as ValueType>::try_from(v).map(|v| TSIDDatabaseID::from_integer(v))
    }

    fn type_name() -> String {
        stringify!(PalformDatabaseID).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::BigUnsigned
    }

    fn column_type() -> ColumnType {
        ColumnType::BigUnsigned
    }
}

impl<Resource: TSIDResource> From<TSIDDatabaseID<Resource>> for Value {
    fn from(value: TSIDDatabaseID<Resource>) -> Self {
        Value::BigUnsigned(Some(value.number()))
    }
}

impl<Resource: TSIDResource> TryFromU64 for TSIDDatabaseID<Resource> {
    fn try_from_u64(n: u64) -> Result<Self, DbErr> {
        Ok(Self::from_raw_number(n))
    }
}

impl<Resource: TSIDResource> Nullable for TSIDDatabaseID<Resource> {
    fn null() -> Value {
        Value::BigUnsigned(None)
    }
}
