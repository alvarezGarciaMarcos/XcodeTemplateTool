- [XcodeTemplateTool](#xcodetemplatetool)
- [Usage](#usage)
  - [Setup](#setup)
    - [Credential](#credential)
      - [Add](#add)
      - [Remove](#remove)
    - [Remote](#remote)
      - [Add](#add-1)
      - [Remove](#remove-1)
    - [Show](#show)
  - [Template](#template)
    - [install](#install)
    - [Uninstall](#uninstall)
    - [Fetch](#fetch)
    - [List](#list)

# XcodeTemplateTool
A simple CLI tool to manage your Xcode templates.



# Usage
XTT has a simple interface and it is divided in subcommands (along the lines that GIT CLI uses). This are all the different commands that the tool accepts.

## Setup
All commands related to setting up the tool to be able to work with your own repository

### Credential
Allows you to manage your credentials to be able to work with your repository.

#### Add
Adds a new credential. In case that the username already exists, it will be substituted. The password will be saved on your keychain for security reasons.
``` shell
xtt setup credential add <USERNAME> <PASSWORD>
```

#### Remove
Removes a credential.
``` shell
xtt setup credential remove <USERNAME> 
```


### Remote
Allows you to manage your current remote so that it points to your repository.

#### Add
Adds a remote. If the remote has already been set, it will be replaced by the new one provided with this command. This commands accepts an optional parameter to tell the tool where the repository will be cloned.

``` shell
xtt setup remote add <REMOTE> [DESTINATION]
```

#### Remove
Removes a remote.

``` shell
xtt setup remote remove <REMOTE> 
```

### Show
Shows the configuration of the tool.

``` shell
xtt setup show 
```

## Template
Allows you to manage all your templates.

### install
Enables you to install a new template (provided that has already been fetched from the repository). This command accepts an optional flag to tell the tool whether the template is for files or not. This flag is true by default as no other templates are supported by now.
``` shell
xtt template install <TEMPLATE> 
```

### Uninstall
Allows you to uninstall a template that has already been installed previously.

``` shell
xtt template uninstall <TEMPLATE> 
```
### Fetch
Fetches your remote to get the most up-to-date templates, enabling you to install them.

``` shell
xtt template fetch 
```

### List
Lists all templates that can be installed (from your local repository)

``` shell
xtt template list 
```
