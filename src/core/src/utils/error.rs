use std::fmt;

#[derive(Debug)]
pub enum Error {
    SteamNotInstalled,
    SteamNotRunning,
    SteamUserNotConnected,
    ClientNotInitialized,
    LibraryNotFound,
    FunctionNotFound,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SteamNotInstalled => {
                write!(output, "could not detect an installed version of Steam")
            }
            Error::SteamNotRunning => {
                write!(output, "could not detect a running instance of Steam")
            }
            Error::SteamUserNotConnected => {
                write!(output, "could not detect any users connected to Steam")
            }
            Error::LibraryNotFound => write!(output, "could not locate library"),
            Error::FunctionNotFound => write!(output, "could not export function"),
            Error::ClientNotInitialized => write!(output, "could not initialize the client"),
            Error::Other(msg) => write!(output, "something went wrong, {}", msg),
        }
    }
}

impl std::error::Error for Error {}
