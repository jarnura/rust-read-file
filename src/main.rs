use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use std::env;

fn main() {
    let secrets = read_file();
    println!("{secrets:?}");
}

#[derive(Debug, Deserialize)]
struct Sensitive {
    key: String,
}

#[derive(Debug, Deserialize)]
struct Secrets {
    foo: Sensitive,
}

fn read_file() -> Secrets {
    let path = env::var("MY_FILE_PATH").unwrap();
    println!("{path}");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    toml::from_str(&std::fs::read_to_string(path).expect("auth file not found"))
        .expect("Failed to read the file")
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn read_check() {
        main();
    }
}
