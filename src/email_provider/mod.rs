mod aws;

pub use aws::{send_email, send_raw_email, setup_ses_client};
