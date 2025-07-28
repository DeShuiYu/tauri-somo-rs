
use crate::scheme::NetWorkInfo;

#[cfg(target_os = "macos")]
mod macos;

pub fn get_all_connections() -> Vec<NetWorkInfo> {


    #[cfg(target_os = "macos")]
    {
        macos::get_connections()
    }
}