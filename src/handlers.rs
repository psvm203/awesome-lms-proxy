pub mod lectures;
pub mod login;
pub mod not_found;
pub mod preflight;
pub mod view;

pub mod error_messages {
    pub const LOGIN_UNAVAILABLE: &str = "Login temporarily unavailable";
    pub const TOO_MANY_REQUESTS: &str = "Too many requests, try again in 5 minutes";
    pub const INVALID_CREDENTIALS: &str = "Invalid credentials";
    pub const LOGIN_ERROR: &str = "Unknown login Error";
    pub const PARSE_ERROR: &str = "Text cannot be properly parsed";
    pub const NO_CREDENTIALS: &str = "No credentials";
    pub const MAIN_PAGE_UNAVAILABLE: &str = "Main page temporarily unavailable";
    pub const TODO_PAGE_UNAVAILABLE: &str = "Todo page temporarily unavailable";
    pub const SESSION_EXPIRED: &str = "Session Expired";
    pub const CONNECT_PAGE_UNAVAILABLE: &str = "Connect page temporarily unavailable";
    pub const VIEW_FORM_UNAVAILABLE: &str = "View form temporarily unavailable";
    pub const INVALID_REQUEST_BODY: &str = "Invalid request body";
    pub const NOT_FOUND: &str = "Not found";
}
