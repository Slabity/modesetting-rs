use super::ffi;

use std::ffi::CStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Mode {
    pub name: String,
    pub clock: u32,
    pub display: (u16, u16),
    pub hsync: (u16, u16),
    pub vsync: (u16, u16),
    pub hskew: u16,
    pub vscan: u16,
    pub htotal: u16,
    pub vtotal: u16,
    pub vrefresh: u32,
    pub flags: u32,
    pub mode_type: u32,
}

impl From<ffi::drm_mode_modeinfo> for Mode {
    fn from(raw: ffi::drm_mode_modeinfo) -> Mode {
        let name = unsafe {
            CStr::from_ptr(raw.name.as_ptr()).to_str().unwrap()
        };

        Mode {
            name: name.to_string(),
            clock: raw.clock,
            display: (raw.hdisplay, raw.vdisplay),
            hsync: (raw.hsync_start, raw.hsync_end),
            vsync: (raw.vsync_start, raw.vsync_end),
            hskew: raw.hskew,
            vscan: raw.vscan,
            htotal: raw.htotal,
            vtotal: raw.vtotal,
            vrefresh: raw.vrefresh,
            flags: raw.flags,
            mode_type: raw.type_
        }
    }
}
