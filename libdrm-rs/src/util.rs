pub const SM_SIZE: usize = 32;

pub use libc::{ioctl, c_void, c_char};
pub use std::io::Error as IoctlError;
pub use std::mem;

pub type Buffer<T> = [T; SM_SIZE];

macro_rules! ioctl {
    ( $card:expr, $code:expr, $obj:expr ) => ( unsafe {
        if ioctl($card.as_raw_fd(), $code as u64, $obj) != 0 {
            return Err(IoctlError::last_os_error().into());
        }
    })
}

macro_rules! ptr_buffers {
    ( $($buf:ident = ($ptr:expr, $count:expr, $max:expr, $bty:ty);)* ) => (
        $(
            let bsize = if $count > $max {
                $max
            } else {
                $count
            };

            let mut $buf: Buffer<$bty> = Default::default();

            *$ptr = unsafe {
                mem::transmute($buf.as_mut_ptr())
            };
        )*
    )
}

