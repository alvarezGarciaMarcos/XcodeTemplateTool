
extern crate git2;
extern crate keyring;

use crate::config::{get_configuration, save_repo_url};
use crate::error::XTTError;
use crate::security::get_credentials;
use crate::repo::{clone_repo, fetch_repo, list_files_in_repo};
use crate::storage::{create_symlink, get_xcode_file_templates_dir, get_templates_dir, get_data_dir, create_project_dir, create_dir_if_missing, get_dir_for_template, remove_file, expand_user};

pub fn add_remote(remote: &String, destination: &Option<String>) -> Result<(String, String), XTTError> {
    let url = remote;
    // let default_path = "./repo".to_string();
    let default_path = get_data_dir().ok_or(XTTError::DataDirNotFound)?;
    let default_path_str = default_path.as_path().as_os_str().to_str().ok_or(XTTError::NonUTF8Path)?;
    let default_path_string = default_path_str.to_owned();

    _ = create_project_dir(&default_path);

    let path = destination
    .as_ref()
    .cloned()
    .unwrap_or(default_path_string);


    let credentials = get_credentials().unwrap();
    let remote = url.clone();
    let local = path.clone();

    match clone_repo(remote, local.to_owned(), credentials) {
        Ok((remote, local)) => {
            save_repo_url(remote.clone(), local.clone());
            return Ok((remote, local));
        }
        Err(error) => return Err(error)
    }
}

pub fn show_configuration() -> Result<String, XTTError> {
    match get_configuration() {
        Ok(config) => {
            let string = toml::ser::to_string_pretty(&config).unwrap();
            return Ok(string);
        }
        Err(error) => Err(error)
    }
}

pub fn remove_remote(_remote: &String) {
}

pub fn list() {
    let configuration = get_configuration().unwrap();

    let templates = list_files_in_repo(configuration);

    if templates.is_err() {
    }

    for template in templates.unwrap() {
        println!("{}", template);
    }
}

pub fn install(template: &String, _as_file: bool) -> Result<(), XTTError> {
    let configuration = get_configuration()?;
    let templates = list_files_in_repo(configuration)?;

    if !templates.contains(template) {
        return Err(XTTError::NonExistingTemplate);
    }

    match (get_templates_dir(), get_xcode_file_templates_dir()) {
        (Some(from), Some(to)) => {
            // create_symlink(path, to)
            let template_name = template.clone();
            let repo_template_dir = from.as_path().join(&template_name);
            let xcode_template_dir = to.as_path().join(&template_name);
            // create_dir_if_missing(&xcode_template_dir);

            if let Some(str_path) = xcode_template_dir.as_path().to_str() {
                if let Some(expanded) = expand_user(str_path) {
                    create_dir_if_missing(&expanded);
                }
            }


            match create_symlink(&repo_template_dir, &xcode_template_dir) {
                Ok(_) => Ok(()),
                Err(_) => Err(XTTError::CannotCreateSymlink)
            }
        }

        _ => {
            Err(XTTError::CannotCreateSymlink)
        }
    }
}


pub fn install_all() {
    println!("Installing all")
}

pub fn uninstall(template: &String) -> Result<(), XTTError> {
    if let Some(template_dir) = get_dir_for_template(template) {
        let str_path = template_dir.as_path().to_str().unwrap();
        // let expand_user(&template_dir.as_path().to_str())

        let expanded = expand_user(str_path).unwrap();
        remove_file(&expanded);

        return Ok(());
    }
    return Err(XTTError::NonExistingTemplate);
}

pub fn uninstall_all() -> Result<(), XTTError> {
    Ok(())
}

pub fn fetch() {
    let credentials = get_credentials().unwrap();
    _ = fetch_repo(credentials).unwrap();
}