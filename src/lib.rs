#![allow(dead_code, unused_variables)]


mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        //mock setup
        return Status::Connected;
    }

    pub fn get_user() {
        //get user from db
    }

}

mod auth_utils {
    
    pub fn login (creds: models::Credentials) {
        //authenticate
        crate::database::get_user();
    }
    
    fn logout () {
        //log user out
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }

}


pub fn authenticate(creds: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}