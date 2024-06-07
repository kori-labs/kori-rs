use thiserror::Error;

#[derive(Error, Debug)]
pub enum KoriError {
    #[error("Authentication is required.")]
    AuthenticationRequired,
}
