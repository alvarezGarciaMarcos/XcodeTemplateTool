use crate::{config::{save_username, get_username}, error::XTTError};

pub struct Credentials {
    pub username: String,
    pub password: String
}

pub fn get_credentials() -> Result<Credentials, XTTError> {
    let username = get_username().unwrap();
    match keyring::Entry::new("xtt", username.as_str()).unwrap().get_password() {
        Ok(password) => return Ok(Credentials {
            username: username,
            password: password
        }),
        Err(_) => return Err(XTTError::PasswordNotConfigured)
    }
}

pub fn setup_credentials(username: &String, password: &String) {
    let result = keyring::Entry::new("xtt", &username);

    match result {
        Ok(entry) => match entry.set_password(password) {
            Ok(_) => {
                save_username(username);
            }
            Err(error) => eprintln!("Error setting the password: {}", error)
        },
        Err(error) => println!("{}", error)
    }
}

pub fn remove_credentials(username: &String) {
    let result = keyring::Entry::new("xtt", &username).unwrap();
    _ = result.delete_password().unwrap();
}