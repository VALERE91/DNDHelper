use std::env;
extern crate dotenv;
use dotenv::dotenv;

pub struct Config {
    pub port: u16,
    pub host: String,
    pub db_uri: String,
    pub redis_uri: String,
    pub log_config_file: String
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        let log_config_file = match env::var("LOG_CONFIG") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let redis_uri = match env::var("REDIS_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let server_host = match env::var("SERVER_HOST") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let server_port = match env::var("SERVER_PORT") {
            Ok(v) => {
                match v.to_string().parse::<u16>(){
                    Ok(v) => v,
                    Err(_) => {
                        println!("Error parsing SERVER_PORT, using default value (8080)");
                        8080
                    }
                }
            },
            Err(_) => {
                println!("Error parsing SERVER_PORT, using default value (8080)");
                8080
            },
        };

        let mongo_uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        Config { 
            port: server_port, 
            host: server_host, 
            db_uri: mongo_uri, 
            redis_uri: redis_uri, 
            log_config_file: log_config_file 
        }
    }
}