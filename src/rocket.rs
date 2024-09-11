use std::borrow::Cow;

use rocket::{
    data::ToByteUnit,
    form::{error::ErrorKind, FromFormField},
    request::FromParam,
    tokio::io::AsyncReadExt,
};

use crate::{resources::TSIDResource, tsid::TSIDDatabaseID};

impl<'d, Resource: TSIDResource> FromParam<'d> for TSIDDatabaseID<Resource> {
    type Error = anyhow::Error;
    fn from_param(param: &'d str) -> Result<Self, Self::Error> {
        Self::from_str(param)
    }
}

#[rocket::async_trait]
impl<'d, Resource: TSIDResource> FromFormField<'d> for TSIDDatabaseID<Resource> {
    fn from_value(field: rocket::form::ValueField<'d>) -> rocket::form::Result<'d, Self> {
        Self::from_str(field.value)
            .map_err(|e| ErrorKind::Validation(Cow::Owned(e.to_string())).into())
    }

    async fn from_data(field: rocket::form::DataField<'d, '_>) -> rocket::form::Result<'d, Self> {
        let mut value = "".to_string();
        field
            .data
            .open(5.kibibytes())
            .read_to_string(&mut value)
            .await?;
        Self::from_str(&value).map_err(|e| ErrorKind::Validation(Cow::Owned(e.to_string())).into())
    }
}
