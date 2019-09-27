//! Toggle backlight devices.
//!
//! # Usage
//!
//! Just run `rust-lcd` at your terminal (with superuser permissions).
//!
//! # Examples
//!
//! You can run it as root:
//! ```bash
//! root@host# rust-lcd
//! ```
//! or as a simple user:
//! ```bash
//! user@host$ sudo rust-lcd
//! ```

use rust_lcd::{iterate_devices, BACKLIGHT_PATH};
use std::io;
// use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    for device in iterate_devices(BACKLIGHT_PATH)? {
        println!("{:?}", device);
        device.toggle()?;
    }

    Ok(())
}
