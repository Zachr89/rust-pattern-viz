// Example: Nested match with complex decision tree
enum Command {
    Read { path: String },
    Write { path: String, data: String },
    Delete { path: String },
}

enum Permission {
    Admin,
    User,
    Guest,
}

fn execute_command(cmd: Command, perm: Permission) -> Result<String, String> {
    match perm {
        Permission::Admin => match cmd {
            Command::Read { path } => Ok(format!("Reading {}", path)),
            Command::Write { path, data } => Ok(format!("Writing {} to {}", data, path)),
            Command::Delete { path } => Ok(format!("Deleting {}", path)),
        },
        Permission::User => match cmd {
            Command::Read { path } => Ok(format!("Reading {}", path)),
            Command::Write { path, data } => Ok(format!("Writing {} to {}", data, path)),
            Command::Delete { .. } => Err("Users cannot delete files".to_string()),
        },
        Permission::Guest => match cmd {
            Command::Read { path } => Ok(format!("Reading {}", path)),
            Command::Write { .. } => Err("Guests cannot write files".to_string()),
            Command::Delete { .. } => Err("Guests cannot delete files".to_string()),
        },
    }
}
