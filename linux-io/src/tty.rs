use linux_unsafe::{int, ulong, ushort};

use crate::fd::ioctl::{
    ioctl_read, ioctl_write, ioctl_writeread, IoctlReqRead, IoctlReqWrite, IoctlReqWriteRead, _IOR,
    _IOW,
};

/// `ioctl` request for retrieving the current window size of a tty.
// NOTE: This ioctl number isn't valid for all Linux architectures, but is valid
// for all of the ones linux-unsafe supoorts at the time of writing.
pub const TIOCGWINSZ: IoctlReqRead<TtyDevice, WindowSize> = unsafe { ioctl_read(0x5413) };

/// `ioctl` request for changing the window size of a tty.
// NOTE: This ioctl number isn't valid for all Linux architectures, but is valid
// for all of the ones linux-unsafe supoorts at the time of writing.
pub const TIOCSWINSZ: IoctlReqWrite<TtyDevice, WindowSize> = unsafe { ioctl_write(0x5414) };

/// Represents the size of the window (or equivalent) that a tty is presented
/// through.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct WindowSize {
    /// The number of rows in the window.
    pub ws_row: ushort,

    /// The number of columns in the window.
    pub ws_col: ushort,

    /// Not actually used.
    pub ws_xpixel: ushort,

    /// Not actually used.
    pub ws_ypixel: ushort,
}

/// A marker type for [`super::File`] objects that represent tty devices.
pub struct TtyDevice;

impl super::fd::ioctl::IoDevice for TtyDevice {}

/// A marker type for [`super::File`] objects that represent pseudoterminal
/// controllers, as are typically opened from `/dev/ptmx`.
pub struct PtyControllerDevice;

impl super::fd::ioctl::IoDevice for PtyControllerDevice {}

const IOCTL_TTY: ulong = b'T' as ulong;

/// `ioctl` request for setting or removing the lock on the terminal device
/// associated with a [`PtyControllerDevice`].
// NOTE: This ioctl number isn't valid for all Linux architectures, but is valid
// for all of the ones linux-unsafe supoorts at the time of writing.
pub const TIOCSPTLCK: IoctlReqWrite<PtyControllerDevice, int> =
    unsafe { ioctl_write(_IOW(IOCTL_TTY, 0x31, core::mem::size_of::<int>() as ulong)) };

/// `ioctl` request for finding the device unit number for a pseudoterminal.
/// Use this with a [`PtyControllerDevice`] to determine which node to open
/// from the `/dev/pts` directory.
// NOTE: This ioctl number isn't valid for all Linux architectures, but is valid
// for all of the ones linux-unsafe supoorts at the time of writing.
pub const TIOCGPTN: IoctlReqWriteRead<PtyControllerDevice, int> =
    unsafe { ioctl_writeread(_IOR(IOCTL_TTY, 0x30, core::mem::size_of::<int>() as ulong)) };
