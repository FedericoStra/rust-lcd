//! Iteration over backlight devices.
//!
//! # Usage
//!
//! The default directory for backlight devices is [`BACKLIGHT_PATH`],
//! whose value is `"/sys/class/backlight"`.
//! The `DeviceIter` iterates over...
//!
//! [`BACKLIGHT_PATH`]: constant.BACKLIGHT_PATH.html
//!
//! # Examples
//!
//! ```
//! # use rust_lcd::*;
//! use std::path::Path;
//! for device in DeviceIter::default() {
//!      assert_eq!(
//!         Path::new(BACKLIGHT_PATH),
//!         device.path().parent().unwrap());
//! }
//! ```

#![deny(missing_docs)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The default directory where to look for devices.
///
/// The value under Linux is `"/sys/class/backlight"`.
pub const BACKLIGHT_PATH: &str = "/sys/class/backlight";

/// The default name of the file controlling the power of the device.
///
/// The value under Linux is `"bl_power"`.
pub const BL_POWER: &str = "bl_power";

/// A single backlight device that can be toggled ON and OFF.
///
/// # Examples
///
/// ```
/// # use rust_lcd::{BL_POWER, Device};
/// use std::path::Path;
/// let path = Path::new("/sys/class/backlight/intel_backlight");
/// let dev = Device::new(path);
/// assert_eq!(dev.bl_power(), path.join(BL_POWER));
/// assert!(dev.toggle().is_err()); // we don't have permission
/// ```
#[derive(Debug)]
pub struct Device {
    path: PathBuf,
    bl_power: PathBuf,
}

impl Device {
    /// Creates a new device located at `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path: &Path = path.as_ref();
        Self {
            path: path.to_path_buf(),
            bl_power: path.join(BL_POWER),
        }
    }

    // This is not exposed in the public API at the moment.
    #[allow(unused)]
    fn custom<P: AsRef<Path>, Q: AsRef<Path>>(path: P, bl_power: Option<Q>) -> Self {
        let path: &Path = path.as_ref();
        match bl_power {
            Some(q) => Device {
                path: path.to_path_buf(),
                bl_power: path.join(q),
            },
            None => Device {
                path: path.to_path_buf(),
                bl_power: path.join(BL_POWER),
            },
        }
    }

    /// Returns the path of the device.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the path of the device power controller.
    pub fn bl_power(&self) -> &Path {
        &self.bl_power
    }

    /// Toggles the state of the device ON and OFF.
    ///
    /// The return value is either a [`std::io::Error`] or the new state of the device.
    ///
    /// [`std::io::Error`]: https://doc.rust-lang.org/stable/std/io/struct.Error.html
    pub fn toggle(&self) -> io::Result<i32> {
        let old_value = read_i32(&self.bl_power)?;
        let new_value = if old_value == 0 { 1 } else { 0 };
        write_i32(&self.bl_power, new_value)?;
        Ok(new_value)
    }
}

/// An iterator over the devices found in a given folder.
pub struct DeviceIter {
    readdir: Option<fs::ReadDir>,
}

impl DeviceIter {
    /// Create a new iterator over the devices found in `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let readdir = match fs::read_dir(&path) {
            Ok(iter) => Some(iter),
            Err(_) => None,
        };
        Self { readdir }
    }
}

impl Default for DeviceIter {
    fn default() -> Self {
        Self::new(Path::new(BACKLIGHT_PATH))
    }
}

impl Iterator for DeviceIter {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        match self.readdir {
            Some(ref mut diriter) => diriter
                .filter(|entry| entry.is_ok())
                .map(|entry| entry.unwrap().path())
                .filter(|path| bl_power(path).is_file())
                .next()
                .map(|p| Device::new(&p)),
            _ => None,
        }
    }
}

/// Iterate over devices in `dir`.
///
/// If successful, it returns an iterator over [`Device`]s.
///
/// The function can fail, returning `std::io::Error`,
/// if `std::fs::read_dir` cannot open the directory.
///
/// [`Device`]: struct.Device.html
pub fn iterate_devices<P: AsRef<Path>>(dir: P) -> io::Result<impl Iterator<Item = Device>> {
    let diriter = fs::read_dir(dir)?;
    Ok(diriter
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|path| bl_power(path).is_file())
        .map(|p| Device::new(&p)))
}

fn bl_power(path: &Path) -> PathBuf {
    path.join(BL_POWER)
}

fn read_i32(path: &Path) -> io::Result<i32> {
    fs::read_to_string(path)?
        .trim()
        .parse::<i32>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "cannot parse i32"))
}

fn write_i32(path: &Path, value: i32) -> io::Result<()> {
    fs::write(path, value.to_string())
}
