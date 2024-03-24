mod connect;
mod init;
mod query;
mod read;
mod send;
mod send_bulk;
mod simple_query;

pub use connect::connect;
pub use init::init;
pub use query::query;
pub use read::read;
pub use send::send;
pub use send_bulk::send_bulk;
pub use simple_query::simple_query;

// Binary name
pub const BIN: &str = "pigeon";

// Available subcommands
pub const INIT: &str = "init";
pub const CONNECT: &str = "connect";
pub const QUERY: &str = "query";
pub const SIMPLE_QUERY: &str = "simple-query";
pub const READ: &str = "read";
pub const SEND: &str = "send";
pub const SEND_BULK: &str = "send-bulk";
