use sea_orm::{
    ColIdx, ColumnType, DbErr, QueryResult, TryFromU64, TryGetError, TryGetable, Value,
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> TryGetable for TSIDDatabaseID<Resource> {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        <i64 as TryGetable>::try_get_by(res, index).map(|v| TSIDDatabaseID::from_integer(v))
    }
}

impl<Resource: TSIDResource> ValueType for TSIDDatabaseID<Resource> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::BigUnsigned(Some(n)) => Ok(TSIDDatabaseID::from_raw_number(n)),
            v => <i64 as ValueType>::try_from(v).map(TSIDDatabaseID::from_integer),
        }
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
