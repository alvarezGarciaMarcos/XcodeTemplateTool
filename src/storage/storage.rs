use std::path::{PathBuf, Path};
use std::str::FromStr;
use directories::ProjectDirs;
use std::fs;
use std::env;

use crate::error::XTTError;
use crate::constants::{self};
use crate::config::get_configuration;
use std::os::unix::fs as unix_fs;

pub fn get_project_dir() -> Option<PathBuf> {
    if let Some(project_dirs) = ProjectDirs::from(
        "dev", 
        "alvarezGarciaMarcos", 
        "xcodeTemplateTool"
    ) {
        let config_dir = project_dirs.config_dir();
        return Some(config_dir.to_path_buf());
    }

    return None;
}

pub fn get_data_dir() -> Option<PathBuf> {
    if let Some(project_dirs) = ProjectDirs::from(
        "dev", 
        "alvarezGarciaMarcos", 
        "xcodeTemplateTool"
    ) {
        Some(project_dirs.data_dir().join("repo").to_path_buf())
    } else {
        None
    }
}

pub fn expand_user(path: &str) -> Option<PathBuf> {
    if path.starts_with('~') {
        let without_tilde = &path[1..];
        env::home_dir().map(|mut home| {
            if home == Path::new("/") {
                // Home dir is root
                home.push(without_tilde.trim_start_matches('/'));
            } else {
                home.push(without_tilde.trim_start_matches('/'));
            }
            home
        })
    } else {
        Some(PathBuf::from(path))
    }
}

pub fn create_symlink(from: &PathBuf, to: &PathBuf) -> Result<(), XTTError> {

    if let Some(expanded_to) = expand_user(to.as_path().to_str().unwrap()) {
        remove_file(&expanded_to);

        match unix_fs::symlink(&from, &expanded_to) {
            Ok(_) => Ok(()),
            Err(error) => Err(XTTError::CannotCreateSymlink)
        }
    } else {
        Err(XTTError::CannotCreateSymlink)
    }
}

pub fn remove_file(path: &PathBuf) {
    if path.as_path().exists() {
        _ = fs::remove_file(&path);
    }
}

pub fn get_xcode_file_templates_dir() -> Option<PathBuf> {
    let base_url = constants::templates::TEMPLATE_PATH;
    let path = PathBuf::from_str(base_url);

    match path {
        Ok(path) => {
            let mut cloned_path = path.clone();
            cloned_path.push("File Templates");

            return Some(cloned_path);
        },
        Err(_) => {
            return None;
        }
    };
}

pub fn get_templates_dir() -> Option<PathBuf> {
    if let Some(configuration) = get_configuration().ok() {
        let path = configuration.local_repo_url
        .map(|s| PathBuf::from_str(s.as_str()))?
        .ok();

        if let Some(path) = path {
            return Some(path);
        } else {
            return None;
        }

    }

    None
}

pub fn get_dir_for_template(template: &String) -> Option<PathBuf> {
    return get_xcode_file_templates_dir().map(|path| path.as_path().join(template));
}

pub fn create_project_dir(path: &PathBuf) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}

pub fn create_dir_if_missing(path: &PathBuf) {
    if !path.as_path().exists() {
        _ = create_project_dir(path);
    }
}

fn _get_project_dir() -> Option<ProjectDirs> {
    return ProjectDirs::from(
        "dev", 
        "alvarezGarciaMarcos",
        "xcodeTemplateTool"
    )
}