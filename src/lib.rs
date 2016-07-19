/*!
  High-level access to modesetting functionality.

  # Overview

  Modesetting is how you control the display functionality on your computer.
  In systems that provide Kernel Modesetting (KMS), this functionality can be
  accessed by opening a character block device and controlling it through
  various ioctls provided by your graphics driver.

  Modesetting consists of opening a Device and using four types of resources:

  - Connectors: The physical interfaces on your GPU, such as HDMI, VGA, and
  LVDS ports.
  - Encoders: These modify and deliver the pixel data to the connectors.
  - CRTCs: Points to a scanout buffer in video memory and reads it based on the
  mode it is set to.
  - Framebuffer: Pixel data that can be used by a CRTC

  The standard procedure to do this is to first open the device. Then choose
  the connectors you wish to use. For each connector, get your desired mode and
  choose an available CRTC to use (in most situations, attaching a CRTC to a
  connector will automatically choose the preferred encoder). Once you have a
  suitable Connector, CRTC, and Mode, you can create a framebuffer for scanout.

  For more information, see the `drm-kms` man page.
  */

#[doc(hidden)]
extern crate libc;
#[doc(hidden)]
extern crate errno;

mod ffi;
pub mod error;
pub mod resources;
pub mod mode;

use error::Result;
use self::resources::*;

use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Device {
    file: Arc<File>,
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl FromRawFd for Device {
    unsafe fn from_raw_fd(fd: RawFd) -> Device {
        Device {
            file: Arc::new(File::from_raw_fd(fd)),
        }
    }
}

impl Device {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: Arc::new(file),
        };
        Ok(dev)
    }

    pub fn resources(&self) -> Result<Resources> {
        let raw = try!(ffi::DrmModeCardRes::new(self.as_raw_fd()));
        Ok(Resources::from((self, &raw)))
    }

    fn connector(&self, id: ConnectorId) -> Result<Connector> {
        let raw = try!(ffi::DrmModeGetConnector::new(self.as_raw_fd(), id));
        Ok(Connector::from((self, &raw)))
    }

    fn encoder(&self, id: EncoderId) -> Result<Encoder> {
        let raw = try!(ffi::DrmModeGetEncoder::new(self.as_raw_fd(), id));
        Ok(Encoder::from((self, &raw)))
    }

    fn crtc(&self, id: CrtcId) -> Result<Crtc> {
        let raw = try!(ffi::DrmModeGetCrtc::new(self.as_raw_fd(), id));
        Ok(Crtc::from((self, &raw)))
    }
}


