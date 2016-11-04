/*!
  High-level access to modesetting functionality.

  # Overview

  Modesetting is how you control the display functionality on your computer.
  In systems that provide Kernel Modesetting (KMS), this functionality can be
  accessed by opening a character block device and controlling it through
  various ioctls provided by your graphics driver.

  Modesetting consists of opening a UnprivilegedDevice and using four types of resources:

  - Connectors: The physical interfaces on your GPU, such as HDMI, VGA, and
  LVDS ports.
  - Encoders: These modify and deliver the pixel data to the connectors.
  - Display Controllers: Controls the scanout of a Framebuffer to one or more
  Connectos.
  - Framebuffer: Pixel data that can be used by a Display Controller

  The standard procedure to do this is to first open the device and select the
  Connectors you will use. For each Connector, decide on a mode you will use
  and attach the proper Encoders. Create the Framebuffers you wish to display
  and set up the Display Controllers for proper scanout.

  For more information, see the `drm-kms` man page.
  */

#[macro_use]
extern crate error_chain;
extern crate libc;

mod ffi;
pub mod result;
pub mod mode;

#[cfg(feature="dumbbuffer")]
pub mod dumbbuffer;

use result::{Result, ErrorKind};
use mode::Mode;

use std::os::unix::io::AsRawFd;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use std::mem::transmute;
use std::vec::IntoIter;
use std::ops::Deref;

pub type ResourceId = u32;
pub type ConnectorId = ResourceId;
pub type EncoderId = ResourceId;
pub type ControllerId = ResourceId;
pub type FramebufferId = ResourceId;

/// An object that implements `MasterLock` allows itself to acquire and
/// release the master lock for modesetting actions.
pub trait MasterLock<'a, T> {
    /// Acquire the master control lock.
    fn lock_master(&'a self) -> Result<T>;
    /// Release the master control lock.
    fn release_master(&'a self, guard: T);
}

/// A `Device` is an unprivileged handle to the character device file that
/// provides modesetting capabilities.
pub struct UnprivilegedDevice {
    file: File,
    master_lock: Mutex<()>
}

impl AsRef<File> for UnprivilegedDevice {
    fn as_ref(&self) -> &File {
        &self.file
    }
}

impl<'a> MasterLock<'a, MutexGuard<'a, ()>> for UnprivilegedDevice {
    fn lock_master(&'a self) -> Result<MutexGuard<'a, ()>> {
        let guard = self.master_lock.lock().unwrap();
        try!(ffi::set_master(self.file.as_raw_fd()));
        Ok(guard)
    }

    #[allow(unused_variables)]
    fn release_master(&'a self, guard: MutexGuard<'a, ()>) {
        let _ = ffi::drop_master(self.file.as_raw_fd());
    }
}

impl From<File> for UnprivilegedDevice {
    fn from(file: File) -> UnprivilegedDevice {
        UnprivilegedDevice {
            file: file,
            master_lock: Mutex::new(())
        }
    }
}

impl UnprivilegedDevice {
    /// Attempt to open the file specified at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Self::from(file);
        Ok(dev)
    }

    /// Acquire the master lock and create a `MasterDevice`
    pub fn master(&self) -> Result<MasterDevice> {
        MasterDevice::from_device(self)
    }
}

/// A `PrivilegedDevice` is identical to an `UnprivilegedDevice`, but does not
/// set or drop the DRM master. This is useful on platforms where the program
/// is granted the privileges by another program, such as a display server or a
/// session manager like logind.
pub struct PrivilegedDevice<F> where F: AsRef<File> {
    file: F,
    master_lock: Mutex<()>,
}

impl<F> AsRef<File> for PrivilegedDevice<F> where F: AsRef<File> {
    fn as_ref(&self) -> &File {
        self.file.as_ref()
    }
}

impl<'a, F> MasterLock<'a, MutexGuard<'a, ()>> for PrivilegedDevice<F> where F: AsRef<File> {
    fn lock_master(&'a self) -> Result<MutexGuard<'a, ()>> {
        let guard = self.master_lock.lock().unwrap();
        Ok(guard)
    }

    #[allow(unused_variables)]
    fn release_master(&'a self, guard: MutexGuard<'a, ()>) {
        // Simply consumes the guard and returns it to its mutex.
    }
}

impl<'a, F> PrivilegedDevice<F> where F: AsRef<File> {
    /// Create a `PrivilegedDevice` from an opened file.
    pub fn from_file_ref(file: F) -> PrivilegedDevice<F> {
        PrivilegedDevice {
            file: file,
            master_lock: Mutex::new(())
        }
    }
}

/// A `MasterDevice` is an privileged handle to the character device file that
/// provides full modesetting capabilities.
///
/// Unlike a `Device`, a `MasterDevice` does not own the file descriptor used.
/// It is the responsibility of the program to open and close the file
/// descriptor.
///
/// A `MasterDevice` can be used to access various modesetting resources. It
/// also prevents dual ownership of any single resource in multiple locations.
pub struct MasterDevice<'a> {
    handle: &'a File,
    _guard: MutexGuard<'a, ()>,
}

impl<'a> AsRef<File> for MasterDevice<'a> {
    fn as_ref(&self) -> &File {
        self.handle
    }
}

impl<'a> MasterDevice<'a> {
    pub fn from_device<T: MasterLock<'a, MutexGuard<'a, ()>> + AsRef<File>>(device: &'a T) -> Result<Self> {
        let file = device.as_ref();
        let fd = file.as_raw_fd();
        let dev = MasterDevice {
            handle: file,
            _guard: try!(device.lock_master()),
        };
        Ok(dev)
    }
}

/// An object that implements the `Buffer` trait allows it to be used as a part
/// of a `Framebuffer`.
pub trait Buffer {
    /// The width and height of the buffer.
    fn size(&self) -> (u32, u32);
    /// The depth size of the buffer.
    fn depth(&self) -> u8;
    /// The number of 'bits per pixel'
    fn bpp(&self) -> u8;
    /// The pitch of the buffer.
    fn pitch(&self) -> u32;
    /// A handle provided by your graphics driver that can be used to reference
    /// the buffer, such as a dumb buffer handle or a handle provided by mesa's
    /// libgbm.
    fn handle(&self) -> u32;
}
