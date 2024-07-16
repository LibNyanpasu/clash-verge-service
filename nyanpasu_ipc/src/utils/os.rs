#![allow(unused_variables)]
use std::io::Result;

pub const NYANPASU_USER_GROUP: &str = "nyanpasu";

pub(crate) fn change_socket_group(placeholder: &str) -> Result<()> {
    #[cfg(not(windows))]
    {
        use std::{
            io::{Error as IoError, ErrorKind},
            process::Command,
        };
        let output = Command::new("chown")
            .arg(format!("root:{}", NYANPASU_USER_GROUP))
            .arg(format!("/var/run/{placeholder}.sock"))
            .output()?;
        if !output.status.success() {
            return Err(IoError::new(
                ErrorKind::Other,
                "failed to change socket group",
            ));
        }
    }
    Ok(())
}

pub(crate) fn change_socket_mode(placeholder: &str) -> Result<()> {
    #[cfg(not(windows))]
    {
        use std::{
            io::{Error as IoError, ErrorKind},
            process::Command,
        };
        let output = Command::new("chmod")
            .arg("664")
            .arg(format!("/var/run/{placeholder}.sock"))
            .output()?;
        if !output.status.success() {
            return Err(IoError::new(
                ErrorKind::Other,
                "failed to change socket mode",
            ));
        }
    }
    Ok(())
}
