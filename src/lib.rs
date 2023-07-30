mod database;
mod auth_utils;

use database::Status;
use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}