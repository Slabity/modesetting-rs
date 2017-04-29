pub use libc::{ioctl, c_void, c_char};
pub use std::io::Error as IoctlError;
pub use std::mem;

// We will use this type as a buffer.
//
// This is a temporary solution until alloca support is enabled:
// https://github.com/rust-lang/rfcs/pull/1909
pub const SM_SIZE: usize = 32;
pub type Array<T> = Vec<T>;

pub type RawId = u32;

macro_rules! ioctl {
    ( $card:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($card.as_raw_fd(), $code as u64, $obj) != 0 {
            return Err(IoctlError::last_os_error().into());
        }
    })
}

/// Creates a buffer to be modified by an FFI function.
///
/// An buffer of $sz length is created and initialized with zeros. The address
/// of the buffer is then assigned to the variable $ptr, which can be passed
/// into an FFI function to be modified.
macro_rules! ffi_buf {
    ( $ptr:expr, $sz:expr ) => (
        {
            let mut buf = unsafe { vec![mem::zeroed(); $sz as usize] };
            *(&mut $ptr) = unsafe { mem::transmute(buf.as_mut_ptr()) };
            buf
        }
    )
}
