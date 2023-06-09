mod core_config;
pub use self::core_config::*;
mod session_key;
pub use self::session_key::*;
mod user_session;
pub use self::user_session::*;
mod redis;
pub use self::redis::*;
mod postgres;
pub use self::postgres::*;

pub mod cacerts;

pub const APP_NAME: &str = "Scytta";
