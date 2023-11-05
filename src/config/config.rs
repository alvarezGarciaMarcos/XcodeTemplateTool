
use serde::{Serialize, Deserialize};
use crate::error::XTTError;
use crate::storage::{get_project_dir, create_project_dir};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub remote_repo_url: Option<String>,
    pub local_repo_url: Option<String>,
}

pub fn save_username(username: &String) {
    let config = Config { 
        username : username.to_string(), 
        remote_repo_url : None,
        local_repo_url: None
    };
    update_config(config);
}

pub fn save_repo_url(remote_repo_url: String, local_repo_url: String) {
    match get_configuration() {
        Ok(config) => {
            let new_config = Config {
                username: config.username,
                remote_repo_url: Some(remote_repo_url),
                local_repo_url: Some(local_repo_url)
            };

            update_config(new_config);
        }
        Err(error) => {
            println!("Error saving repo: {}", error)
        }
    }
}

fn update_config(config: Config) {
    let serialized = toml::ser::to_string_pretty(&config).unwrap();

    if let Some(project_dir) = get_project_dir() {
        let config_file = project_dir.as_path().join("config.toml".to_string());
        // Modify or create a new file to write to.
        println!("Will write to: {}", config_file.display());
        match create_project_dir(&project_dir) {
            Ok(_) => {
                let result = std::fs::write(config_file, serialized);
                match result {
                    Ok(_) => {}
                    Err(error) => println!("Error: {}", error)
                }
            }
            Err(error) => {
                println!("Error creating the folder: {}", error)
            }
        }
    }
}

pub fn get_username() -> Result<String, XTTError> {
    match get_configuration() {
        Ok(config) => Ok(config.username),
        Err(error) => Err(error)
    }

}

pub fn get_configuration() -> Result<Config, XTTError> {
    if let Some(project_dir) = get_project_dir() {
        let config_file = project_dir.as_path().join("config.toml".to_string());
        let content_string = std::fs::read_to_string(config_file).unwrap();
        let content_ref = content_string.as_str();
        let config: Config = toml::from_str(content_ref).unwrap();
        return Ok(config);

    } else {
        return Err(XTTError::NoConfigFile);
    }
}

