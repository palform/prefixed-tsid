use diesel::{deserialize::FromSql, pg::Pg, serialize::ToSql, sql_types::BigInt};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<Resource: TSIDResource> ToSql<BigInt, Pg> for TSIDDatabaseID<Resource> {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let val = self.to_raw_number() as i64;
        <i64 as ToSql<BigInt, Pg>>::to_sql(&val, &mut out.reborrow())
    }
}

impl<Resource: TSIDResource> FromSql<BigInt, Pg> for TSIDDatabaseID<Resource> {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let num = i64::from_sql(bytes)?;
        Ok(TSIDDatabaseID::<Resource>::from_raw_number(num as u64))
    }
}
