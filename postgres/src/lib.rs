// pub trait GetProviders<'a, T: 'a> {
//     fn get_providers(npis: impl Iterator<Item = &'a T>) -> dyn IntoIterator<Item = Provider, IntoIter = Provider>;
// }

// impl GetProviders<'_, &str> for providers {
//     fn get_providers<'a>(npis: impl Iterator<Item = &'a str>) -> dyn IntoIterator<Item = Provider, IntoIter = Provider> {
//         todo!("Get providers from a collection of strs");
//     }
// }

pub mod get_providers;
pub mod insert_providers;
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use configuration::{Config, GetEnvVar};
use get_providers::{GetProvider, GetProviders};
use models::Provider;

pub struct ProviderOperations {
    pub get_providers: GetProviders,
    pub postgres_connection_string: String,
}

impl ProviderOperations {
    pub fn new(get_providers: GetProviders) -> Self {
        let config = Config::new();
        let (username, password, connection) = ProviderOperations::get_connection(&config);

        let postgres_connection_string = format!("postgres://{username}:{password}@{connection}");

        ProviderOperations {
            get_providers,
            postgres_connection_string,
        }
    }

    pub fn get_provider_from_id(&self, id: &i32) -> Option<Provider> {
        let mut pg_connection = PgConnection::establish(&self.postgres_connection_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &self.postgres_connection_string));

        match self.get_providers.get_provider(id, &mut pg_connection) {
            Ok(providers) => providers.into_iter().next(),
            Err(e) => {
                println!("{e}");
                None
            }
        }
    }

    fn get_connection(config: &Config) -> (&String, &String, &String) {
        (
            config
                .get_var("db_username")
                .expect("db_username is a required env var"),
            config
                .get_var("db_password")
                .expect("db_password is a required env var"),
            config
                .get_var("db_connection")
                .expect("db_connection is a required env var"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
