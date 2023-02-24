use diesel::prelude::*;

use crate::models::*;

pub trait GetProvider<T> {
    fn get_provider(
        &self,
        value: T,
        pg_connection: &mut PgConnection,
    ) -> Result<Vec<Provider>, diesel::result::Error>;
}

pub struct GetProviders;

impl GetProvider<&str> for GetProviders {
    /// Gets the provider using an NPI.
    fn get_provider(
        &self,
        value: &str,
        pg_connection: &mut PgConnection,
    ) -> Result<Vec<Provider>, diesel::result::Error> {
        use crate::schema::providers::dsl::*;
        providers
            .filter(npi.eq(value.to_string()))
            .limit(1)
            .load::<Provider>(pg_connection)
    }
}

impl GetProvider<&i32> for GetProviders {
    /// Gets the provider using an ID.
    fn get_provider(
        &self,
        value: &i32,
        pg_connection: &mut PgConnection,
    ) -> Result<Vec<Provider>, diesel::result::Error> {
        use crate::schema::providers::dsl::*;
        providers.find(value).load::<Provider>(pg_connection)
    }
}
