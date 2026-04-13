pub mod lectures;
pub mod login;
pub mod not_found;
pub mod preflight;

pub mod error_messages {
    pub const LOGIN_UNAVAILABLE: &str = "Login temporarily unavailable";
    pub const TOO_MANY_REQUESTS: &str = "Too many requests";
    pub const INVALID_CREDENTIALS: &str = "Invalid credentials";
    pub const LOGIN_ERROR: &str = "Unknown login Error";
    pub const NO_CREDENTIALS: &str = "No credentials";
    pub const MAIN_PAGE_UNAVAILABLE: &str = "Main page temporarily unavailable";
    pub const SESSION_EXPIRED: &str = "Session Expired";
    pub const TODO_PAGE_UNAVAILABLE: &str = "Todo page temporarily unavailable";
}
