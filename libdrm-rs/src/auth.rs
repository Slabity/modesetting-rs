use drm_sys::*;
use super::*;
use super::util::*;

use std::ptr::null;

impl<T> Card<T> where T: AsRawFd {
    pub fn set_master(&self, enable: bool) -> Result<()> {
        if enable {
            ioctl!(self, MACRO_DRM_IOCTL_SET_MASTER, null() as *const c_void);
        } else {
            ioctl!(self, MACRO_DRM_IOCTL_DROP_MASTER, null() as *const c_void);
        }
        Ok(())
    }

    pub fn get_bus_id(&self) -> Result<BusId> {
        let mut raw: drm_unique_t = Default::default();
        ioctl!(self, MACRO_DRM_IOCTL_VERSION, &mut raw);
        println!("{:?}", raw);
        ptr_buffers! {
            unique = (&mut raw.unique, raw.unique_len as usize + 1, SM_SIZE, c_char);
        };
        println!("{:?}", raw);
        ioctl!(self, MACRO_DRM_IOCTL_VERSION, &mut raw);

        let busid = BusId {
            unique: unique
        };

        Ok(busid)
    }
}

#[derive(Debug)]
pub struct BusId {
    unique: Buffer<c_char>
}

