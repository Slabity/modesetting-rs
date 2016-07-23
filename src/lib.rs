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

extern crate libc;
extern crate errno;

mod ffi;
pub mod error;
pub mod resource;
pub mod mode;

use error::Result;
use self::resource::*;

use std::os::unix::io::AsRawFd;
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(Debug)]
pub struct Device {
    file: File
}

impl Device {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Device> {
        let file = try!(OpenOptions::new().read(true).write(true).open(path));
        let dev = Device {
            file: file,
        };
        Ok(dev)
    }

    pub fn manager(&self) -> Result<Manager> {
        Manager::from_device(self)
    }

    fn resources(&self) -> Result<ffi::DrmModeCardRes> {
        ffi::DrmModeCardRes::new(self.file.as_raw_fd())
    }

    fn connector(&self, id: ConnectorId) -> Result<ffi::DrmModeGetConnector> {
        ffi::DrmModeGetConnector::new(self.file.as_raw_fd(), id)
    }

    fn encoder(&self, id: EncoderId) -> Result<ffi::DrmModeGetEncoder> {
        ffi::DrmModeGetEncoder::new(self.file.as_raw_fd(), id)
    }

    fn crtc(&self, id: CrtcId) -> Result<ffi::DrmModeGetCrtc> {
        ffi::DrmModeGetCrtc::new(self.file.as_raw_fd(), id)
    }

    fn framebuffer(&self, id: FramebufferId) -> Result<ffi::DrmModeGetFb> {
        ffi::DrmModeGetFb::new(self.file.as_raw_fd(), id)
    }
}


