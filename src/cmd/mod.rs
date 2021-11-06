mod connect;
mod init;
mod query;
mod read;
mod send;
mod send_bulk;
mod simple_query;

pub use connect::{connect, connect_args};
pub use init::{init, init_args};
pub use query::{query, query_args};
pub use read::{read, read_args};
pub use send::{send, send_args};
pub use send_bulk::{send_bulk, send_bulk_args};
pub use simple_query::{simple_query, simple_query_args};

// Available subcommands
pub const INIT: &str = "init";
pub const CONNECT: &str = "connect";
pub const QUERY: &str = "query";
pub const SIMPLE_QUERY: &str = "simple-query";
pub const READ: &str = "read";
pub const SEND: &str = "send";
pub const SEND_BULK: &str = "send-bulk";
