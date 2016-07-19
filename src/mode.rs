use super::ffi;

use std::ffi::CStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Mode {
    name: String,
    clock: u32,
    display: (u16, u16),
    hsync: (u16, u16),
    vsync: (u16, u16),
    hskew: u16,
    vscan: u16,
    htotal: u16,
    vtotal: u16,
    vrefresh: u32,
    flags: u32,
    mode_type: u32,
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
