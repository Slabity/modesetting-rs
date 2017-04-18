pub use libc::{ioctl, c_void, c_char};
pub use std::io::Error as IoctlError;
pub use std::mem;

// We will use this type as a buffer.
//
// This is a temporary solution until alloca support is enabled:
// https://github.com/rust-lang/rfcs/pull/1909
pub const SM_SIZE: usize = 32;
pub type Buffer<T> = Vec<T>;

macro_rules! ioctl {
    ( $card:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($card.as_raw_fd(), $code as u64, $obj) != 0 {
            return Err(IoctlError::last_os_error().into());
        }
    })
}

macro_rules! ptr_buffers {
    ( $($buf:ident = ($ptr:expr, $sz:expr, $bty:ty);)* ) => (
        $(
            let mut $buf: Buffer<$bty> = unsafe {
                vec![mem::zeroed(); $sz]
            };

            *(&mut $ptr) = unsafe {
                mem::transmute($buf.as_mut_ptr())
            };
        )*
    )
}

