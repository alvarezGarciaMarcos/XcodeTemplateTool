use git2::MergeOptions;
use std::fs::read_dir;

use crate::config::{get_configuration, Config};
use crate::error::XTTError;
use crate::security::Credentials;

pub fn clone_repo(remote: String, local: String, credentials: Credentials) -> Result<(String, String), XTTError> {
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, _, _allowed_types| {
        git2::Cred::userpass_plaintext(credentials.username.as_str(), &credentials.password)
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    match builder.clone(remote.as_ref(), local.as_ref()) {
        Ok(_) =>  {
            // save_repo_url(url.to_owned(), path.to_owned());
            Ok((remote, local))
            }
        ,
        Err(_) => return Err(XTTError::CloningError)
    }
}

pub fn fetch_repo(credentials: Credentials) -> Result<(), XTTError> {
    match get_configuration() {
        Ok(config) => {
            if let (Some(_), Some(local_repo_url)) = (config.remote_repo_url, config.local_repo_url) {
                match fetch(local_repo_url, credentials) {
                    Ok(_) => println!("Templates updated!"),
                    Err(error) => print!("Fetching error: {}", error)
                }

                return Ok(());

            } else {
                return Err(XTTError::NoRepoConfigured);
            }
        }
        Err(error) => Err(error)
    }
}

pub fn list_files_in_repo(configuration: Config) -> Result<Vec<String>, XTTError> {
    if let Some(local_repo_url) = configuration.local_repo_url {
        let path = std::path::Path::new(&local_repo_url);
        let vector: Vec<String> = read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            entry
            .ok().and_then(|e| {
                let file_name = e.file_name().to_string_lossy().into_owned();
                if file_name.starts_with('.') {
                    None
                } else {
                    Some(file_name)
                }
            })
        })
        .collect();

        return Ok(vector);
    }

    return Ok(Vec::new());
}

fn fetch(local_url: String, credentials: Credentials) -> Result<(), git2::Error> {
    let repo = git2::Repository::open(local_url).unwrap();


    // Fetching changes from the remote
    let mut remote = repo.find_remote("origin").unwrap();
    let mut callbacks = git2::RemoteCallbacks::new();
    let mut fetch_options = git2::FetchOptions::new();
    callbacks.credentials(|_url, _, _allowed_types| {
        git2::Cred::userpass_plaintext(credentials.username.as_str(), &credentials.password)
    });

    fetch_options.remote_callbacks(callbacks);

    remote.fetch(&["master"], Some(&mut fetch_options), None)?;

    // Finding the branches
    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    // let master = repo.find_branch("master", git2::BranchType::Local)?.get();

     // Creating and configuring a CheckoutBuilder
     let mut merge_options = MergeOptions::new();
     merge_options.fail_on_conflict(true);


    // Merging the changes
    repo.merge(&[&fetch_commit], Some(&mut merge_options), None)?;
    repo.cleanup_state().unwrap();

    Ok(())
}