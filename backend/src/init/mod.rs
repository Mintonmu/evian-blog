mod config;
pub use config::{AppConfig, read_config};

use log::LevelFilter;
use chrono::Local;

use std::env;
use std::io::Write;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

enum BlogEnv {
    Development,
    Production,
}

impl From<&str> for BlogEnv {
    fn from(s: &str) -> BlogEnv {
        match s {
            "development" => BlogEnv::Development,
            "production" => BlogEnv::Production,
            _ => BlogEnv::Development
        }
    }
}

pub fn initialize() {
    init_logging();
    let app_config = init_config();
}

fn init_logging() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf, 
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

fn init_config() -> Result<AppConfig> {
    let blog_env_str = if let Ok(blog_env) = env::var("BLOG_ENV") {
        blog_env
    } else {
        String::from("development")
    };

    let blog_env = BlogEnv::from(blog_env_str.as_str());

    let app_configs = read_config().or_else(|config_error| Err(Error::Config(config_error)))?;
    
    Ok(match blog_env {
        BlogEnv::Development => app_configs.development,
        BlogEnv::Production => app_configs.production
    })
}

#[derive(Debug)]
pub enum Error {
    Config(config::Error),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        let message = match &self {
            Config(config_error) => format!("{}", config_error),
        };

        write!(f, "{}", message)
    }
}
