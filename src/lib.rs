//! Rusty WASI type definitions based on
//! [the spec](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md)


#![cfg_attr(feature = "sgx", no_std)]

#[cfg(feature = "sgx")]
#[macro_use]
extern crate sgx_tstd as std;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate proper;
use std::cmp::Ordering;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use err_derive::Error;

/// File or memory access pattern advisory information.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Prim, Debug)]
pub enum Advice {
    /// The application has no advice to give on its behavior with respect to the specified data.
    Normal,

    /// The application expects to access the data sequentially from lower to higher offsets.
    Sequential,

    /// The application expects to access the specified data in a random order.
    Random,

    /// The application expects that it will not access the specified data in the near future.
    DontNeed,

    /// The application expects to access the specified data once and then not reuse it thereafter.
    NoReuse,

    /// The application expects to access the specified data in the near future.
    WillNeed,
}

impl From<Advice> for u8 {
    #[inline]
    fn from(advice: Advice) -> Self {
        advice as u8
    }
}

/// Identifiers for clocks.
#[repr(u32)]
#[prim(ty = "u32")]
#[derive(Clone, Copy, PartialEq, Debug, Prim)]
pub enum ClockId {
    /// The clock measuring real time. Time value zero corresponds with 1970-01-01T00:00:00Z.
    RealTime,

    /// The store-wide monotonic clock, which is defined as a clock measuring real time, whose
    /// value cannot be adjusted and which cannot have negative clock jumps.
    ///
    /// The epoch of this clock is undefined. The absolute time value of this clock therefore
    /// has no meaning.
    Monotonic,

    /// The CPU-time clock associated with the current process.
    ProcessCpuTime,

    /// The CPU-time clock associated with the current thread.
    ThreadCpuTime,
}

impl From<ClockId> for u32 {
    #[inline]
    fn from(clockid: ClockId) -> Self {
        clockid as u32
    }
}

/// Identifier for a device containing a file system. Can be used in combination with `Inode`
/// to uniquely identify a file or directory in the filesystem.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim)]
pub struct Device(u64);

/// A reference to the offset of a directory entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim)]
pub struct DirCookie(pub u64);

impl DirCookie {
    /// Creates a new `DirCookie` repreenting a permanent reference to the first directory entry
    /// within a directory.
    pub fn start() -> Self {
        DirCookie(0)
    }
}

/// A directory entry.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DirEnt {
    /// The offset of the next directory entry stored in this directory.
    pub next: DirCookie,

    /// The serial number of the file referred to by this directory entry.
    pub inode: Inode,

    /// The length of the name of the directory entry.
    pub name_len: u32,

    /// The type of the file referred to by this directory entry.
    pub file_type: FileType,
}

/// Error codes returned by functions.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim, Serialize, Deserialize, Error)]
#[prim(ty = "u16")]
#[non_exhaustive]
pub enum ErrNo {
    /// No error occurred. System call completed successfully.
    #[error(display = "Success")]
    Success,

    /// Argument list too long.
    #[error(display = "TooBig")]
    TooBig,

    /// Permission denied.
    #[error(display = "Access")]
    Access,

    /// Address in use.
    #[error(display = "AddrInUse")]
    AddrInUse,

    /// Address not available.
    #[error(display = "AddrNotAvail")]
    AddrNotAvail,

    /// Address family not supported.
    #[error(display = "AfNoSupport")]
    AfNoSupport,

    /// Resource unavailable, or operation would block.
    #[error(display = "Again")]
    Again,

    /// Connection already in progress.
    #[error(display = "Already")]
    Already,

    /// Bad file descriptor.
    #[error(display = "BadF")]
    BadF,

    /// Bad message.
    #[error(display = "BadMsg")]
    BadMsg,

    /// Device or resource busy.
    #[error(display = "Busy")]
    Busy,

    /// Operation canceled.
    #[error(display = "Canceled")]
    Canceled,

    /// No child processes.
    #[error(display = "Child")]
    Child,

    /// Connection aborted.
    #[error(display = "ConnAborted")]
    ConnAborted,

    /// Connection refused.
    #[error(display = "ConnRefused")]
    ConnRefused,

    /// Connection reset.
    #[error(display = "ConnReset")]
    ConnReset,

    /// Resource deadlock would occur.
    #[error(display = "Deadlk")]
    Deadlk,

    /// Destination address required.
    #[error(display = "DestAddrReq")]
    DestAddrReq,

    /// Mathematics argument out of domain of function.
    #[error(display = "Domain")]
    Domain,

    /// Reserved. (Quota exceeded.)
    #[error(display = "DQuot")]
    DQuot,

    /// File exists.
    #[error(display = "Exist")]
    Exist,

    /// Bad address.
    #[error(display = "Fault")]
    Fault,

    /// File too large.
    #[error(display = "FBig")]
    FBig,

    /// Host is unreachable.
    #[error(display = "HostUnreach")]
    HostUnreach,

    /// Identifier removed.
    #[error(display = "IdRm")]
    IdRm,

    /// Illegal byte sequence.
    #[error(display = "IlSeq")]
    IlSeq,

    /// Operation in progress.
    #[error(display = "InProgress")]
    InProgress,

    /// Interrupted function.
    #[error(display = "Intr")]
    Intr,

    /// Invalid argument.
    #[error(display = "Inval")]
    Inval,

    /// I/O error.
    #[error(display = "Io")]
    Io,

    /// Socket is connected.
    #[error(display = "IsConn")]
    IsConn,

    /// Is a directory.
    #[error(display = "IsDir")]
    IsDir,

    /// Too many levels of symbolic links.
    #[error(display = "Loop")]
    Loop,

    /// File descriptor value too large.
    #[error(display = "MFile")]
    MFile,

    /// Too many links.
    #[error(display = "MLink")]
    MLink,

    /// Message too large.
    #[error(display = "MsgSize")]
    MsgSize,

    /// Reserved. (Multihop attempted.)
    #[error(display = "Multihop")]
    Multihop,

    /// Filename too long.
    #[error(display = "NameTooLong")]
    NameTooLong,

    /// Network is down.
    #[error(display = "NetDown")]
    NetDown,

    /// Connection aborted by network.
    #[error(display = "NetReset")]
    NetReset,

    /// Network unreachable.
    #[error(display = "NetUnreach")]
    NetUnreach,

    /// Too many files open in system.
    #[error(display = "NFile")]
    NFile,

    /// No buffer space available.
    #[error(display = "NoBufS")]
    NoBufS,

    /// No such device.
    #[error(display = "NoDev")]
    NoDev,

    /// No such file or directory.
    #[error(display = "NoEnt")]
    NoEnt,

    /// Executable file format error.
    #[error(display = "NoExec")]
    NoExec,

    /// No locks available.
    #[error(display = "NoLock")]
    NoLock,

    /// Reserved. (Link has been severed.)
    #[error(display = "NoLink")]
    NoLink,

    /// Not enough space.
    #[error(display = "NoMem")]
    NoMem,

    /// No message of the desired type.
    #[error(display = "NoMsg")]
    NoMsg,

    /// Protocol not available.
    #[error(display = "NoProtoOpt")]
    NoProtoOpt,

    /// No space left on device.
    #[error(display = "NoSpace")]
    NoSpace,

    /// Function not supported. (Always unsupported.)
    #[error(display = "NoSys")]
    NoSys,

    /// The socket is not connected.
    #[error(display = "NotConn")]
    NotConn,

    /// Not a directory or a symbolic link to a directory.
    #[error(display = "NotDir")]
    NotDir,

    /// Directory not empty.
    #[error(display = "NotEmpty")]
    NotEmpty,

    /// State not recoverable.
    #[error(display = "NotRecoverable")]
    NotRecoverable,

    /// Not a socket.
    #[error(display = "NotSock")]
    NotSock,

    /// Not supported, or operation not supported on socket. (Transient unsupported.)
    #[error(display = "NotSup")]
    NotSup,

    /// Inappropriate I/O control operation.
    #[error(display = "NoTty")]
    NoTty,

    /// No such device or address.
    #[error(display = "NxIo")]
    NxIo,

    /// Value too large to be stored in data type.
    #[error(display = "Overflow")]
    Overflow,

    /// Previous owner died.
    #[error(display = "OwnerDead")]
    OwnerDead,

    /// Operation not permitted.
    #[error(display = "Perm")]
    Perm,

    /// Broken pipe.
    #[error(display = "Pipe")]
    Pipe,

    /// Protocol error.
    #[error(display = "Proto")]
    Proto,

    /// Protocol not supported.
    #[error(display = "ProtoNoSupport")]
    ProtoNoSupport,

    /// Protocol wrong type for socket.
    #[error(display = "ProtoType")]
    ProtoType,

    /// Result too large.
    #[error(display = "Range")]
    Range,

    /// Read-only file system.
    #[error(display = "RoFs")]
    RoFs,

    /// Invalid seek.
    #[error(display = "SPipe")]
    SPipe,

    /// No such process.
    #[error(display = "Srch")]
    Srch,

    /// Reserved. (Stale file handle.)
    #[error(display = "Stale")]
    Stale,

    /// Connection timed out.
    #[error(display = "TimedOut")]
    TimedOut,

    /// Text file busy.
    #[error(display = "TxtBsy")]
    TxtBsy,

    /// Cross-device link.
    #[error(display = "XDev")]
    XDev,

    /// Extension: Capabilities insufficient.
    #[error(display = "NotCapable")]
    NotCapable,
}

impl From<ErrNo> for u16 {
    #[inline]
    fn from(errno: ErrNo) -> Self {
        errno as u16
    }
}

impl From<std::io::Error> for ErrNo {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        use ErrNo::*;
        match err.kind() {
            ErrorKind::NotFound => NoEnt,
            ErrorKind::PermissionDenied => Access,
            ErrorKind::ConnectionRefused => ConnRefused,
            ErrorKind::ConnectionReset => ConnReset,
            ErrorKind::ConnectionAborted => ConnAborted,
            ErrorKind::NotConnected => NotConn,
            ErrorKind::AddrInUse => AddrInUse,
            ErrorKind::AddrNotAvailable => AddrNotAvail,
            ErrorKind::BrokenPipe => Pipe,
            ErrorKind::AlreadyExists => Exist,
            ErrorKind::WouldBlock => Again,
            ErrorKind::InvalidInput | ErrorKind::InvalidData => Inval,
            ErrorKind::TimedOut => TimedOut,
            ErrorKind::Interrupted => Intr,
            ErrorKind::WriteZero | ErrorKind::Other | ErrorKind::UnexpectedEof | _ => Io,
            // _ => ,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub user_data: UserData,
    pub error: ErrNo,
    pub ty: EventType,
    pub fd_state: Option<EventFdState>, // only valid when `ty \in {FdRead, FdWrite}`
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim)]
pub enum EventType {
    /// The time value of clock `SubscriptionType::clock.clock_id` has reached timestamp
    /// `Subscription::clock.timeout`.
    Clock,

    /// File descriptor `SubscriptionType::FdRw.fd` has data available for reading.
    /// This event always triggers for regular files.
    FdRead,

    /// File descriptor `SubscriptionType::FdRw.fd` has capacity available for writing.
    /// This event always triggers for regular files.
    FdWrite,
}

impl From<EventType> for u8 {
    #[inline]
    fn from(event: EventType) -> Self {
        event as u8
    }
}

/// The state of the file descriptor subscribed to with `EventType::FdRead` or `EventType::FdWrte`.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim)]
#[prim(ty = "u16")]
pub enum EventRwFlags {
    None,
    Hangup,
}

impl From<EventRwFlags> for u16 {
    #[inline]
    fn from(flags: EventRwFlags) -> Self {
        flags as u16
    }
}

pub type ExitCode = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventFdState {
    pub file_size: FileSize,
    pub flags: EventRwFlags,
}

/// A file descriptor number.
/// As in POSIX, 0, 1, and 2 are stdin, stdout, and stderr, respectively.
/// File descriptors are not guaranteed to be contiguous or allocated in ascending order.
/// Information about a file descriptor may be obtained through `fd_prestat_get`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim, Hash)]
pub struct Fd(pub u32);

impl Ord for Fd {
    fn cmp(&self, Fd(other): &Self) -> Ordering {
        self.0.cmp(other)
    }
}

impl PartialOrd for Fd {
    fn partial_cmp(&self, Fd(other): &Self) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct FdFlags: u16 {
        /// Append mode: Data written to the file is always appended to the file's end.
        const APPEND = 1 << 0;

        /// Write according to synchronized I/O data integrity completion.
        /// Only the data stored in the file is synchronized.
        const DSYNC = 1 << 1;

        /// Non-blocking mode.
        const NONBLOCK = 1 << 2;

        /// Synchronized read I/O operations.
        const RSYNC = 1 << 3;

        /// Write according to synchronized I/O file integrity completion. In addition to synchronizing
        /// the data stored in the file, the implementation may also synchronously update the file's
        /// metadata.
        const SYNC = 1 << 4;
    }
}

impl TryFrom<u16> for FdFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        FdFlags::from_bits(code).ok_or(())
    }
}

impl From<FdFlags> for u16 {
    #[inline]
    fn from(flags: FdFlags) -> Self {
        flags.bits
    }
}

bitflags! {
    #[derive(Default)]
    pub struct OpenFlags: u16 {
        /// Create file if it does not exist.
        const CREATE = 1 << 0;

        /// Fail if not a directory.
        const DIRECTORY = 1 << 1;

        /// Fail if file already exists.
        const EXCL = 1 << 2;

        /// Truncate file to size 0.
        const TRUNC = 1 << 3;
    }
}

impl TryFrom<u16> for OpenFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        OpenFlags::from_bits(code).ok_or(())
    }
}

impl From<OpenFlags> for u16 {
    #[inline]
    fn from(flags: OpenFlags) -> Self {
        flags.bits
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FdStat {
    pub file_type: FileType,
    pub flags: FdFlags,

    /// Rights that apply to this file descriptor.
    pub rights_base: Rights,

    /// Maximum set of rights that may be installed on new file descriptors that are created
    /// through this file descriptor.
    pub rights_inheriting: Rights,
}

/// Relative offset within a file.
pub type FileDelta = i64;

/// The type of a file descriptor or file.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim)]
pub enum FileType {
    Unknown,
    BlockDevice,
    CharacterDevice,
    Directory,
    RegularFile,
    SocketDgram,
    SocketStream,
    SymbolicLink,
}

impl From<FileType> for u8 {
    #[inline]
    fn from(ftype: FileType) -> Self {
        ftype as u8
    }
}

pub type FileSize = u64;

/// File attributes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct FileStat {
    pub device: Device,
    pub inode: Inode,
    pub file_type: FileType,
    pub num_links: LinkCount,
    pub file_size: FileSize,
    pub atime: Timestamp,
    pub mtime: Timestamp,
    pub ctime: Timestamp,
}

/// File serial number that is unique within its file system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Prim, Hash)]
pub struct Inode(pub u64);

impl Ord for Inode {
    fn cmp(&self, Inode(other): &Self) -> Ordering {
        self.0.cmp(other)
    }
}

impl PartialOrd for Inode {
    fn partial_cmp(&self, Inode(other): &Self) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

pub type Size = u32;
pub type Pointer = u32;

/// A region of memory for scatter/gather reads.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IoVec {
    pub buf: Pointer,
    pub len: Size,
}

/// Number of hard links to an inode.
pub type LinkCount = u32;

bitflags! {
    #[derive(Default)]
    pub struct LookupFlags: u32 {
        /// Follow symlinks.
        const SYMLINK_FOLLOW = 1 << 0;
    }
}

impl TryFrom<u32> for LookupFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u32) -> Result<Self, Self::Error> {
        LookupFlags::from_bits(code).ok_or(())
    }
}

impl From<LookupFlags> for u32 {
    #[inline]
    fn from(flags: LookupFlags) -> Self {
        flags.bits
    }
}

/// Information about a preopened resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prestat {
    pub resource_type: PreopenType,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreopenType {
    Dir { name_len: Size },
}

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct Rights: u64 {
        const FD_DATASYNC             = 1 << 0;
        const FD_READ                 = 1 << 1;
        const FD_SEEK                 = 1 << 2;
        const FD_FDSTAT_SET_FLAGS     = 1 << 3;
        const FD_SYNC                 = 1 << 4;
        const FD_TELL                 = 1 << 5;
        const FD_WRITE                = 1 << 6;
        const FD_ADVISE               = 1 << 7;
        const FD_ALLOCATE             = 1 << 8;
        const PATH_CREATE_DIRECTORY   = 1 << 9;
        const PATH_CREATE_FILE        = 1 << 10;
        const PATH_LINK_SOURCE        = 1 << 11;
        const PATH_LINK_TARGET        = 1 << 12;
        const PATH_OPEN               = 1 << 13;
        const FD_READDIR              = 1 << 14;
        const PATH_READLINK           = 1 << 15;
        const PATH_RENAME_SOURCE      = 1 << 16;
        const PATH_RENAME_TARGET      = 1 << 17;
        const PATH_FILESTAT_GET       = 1 << 18;
        const PATH_FILESTAT_SET_SIZE  = 1 << 19;
        const PATH_FILESTAT_SET_TIMES = 1 << 20;
        const FD_FILESTAT_GET         = 1 << 21;
        const FD_FILESTAT_SET_SIZE    = 1 << 22;
        const FD_FILESTAT_SET_TIMES   = 1 << 23;
        const PATH_SYMLINK            = 1 << 24;
        const PATH_REMOVE_DIRECTORY   = 1 << 25;
        const PATH_UNLINK_FILE        = 1 << 26;
        const POLL_FD_READWRITE       = 1 << 27;
        const SOCK_SHUTDOWN           = 1 << 28;
    }
}

impl TryFrom<u64> for Rights {
    type Error = ();

    #[inline]
    fn try_from(code: u64) -> Result<Self, Self::Error> {
        Rights::from_bits(code).ok_or(())
    }
}

impl From<Rights> for u64 {
    #[inline]
    fn from(rights: Rights) -> Self {
        rights.bits
    }
}

/// Signal condition.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Prim)]
pub enum Signal {
    Reserved,
    Abort,
    Alarm,
    Bus,
    Child,
    Cont,
    FP,
    Hup,
    Ill,
    Int,
    Kill,
    Pipe,
    Quit,
    Seg,
    Stop,
    Sys,
    Term,
    Trap,
    TStp,
    TTIn,
    TTOut,
    Urg,
    Usr1,
    Usr2,
    VTAlrm,
    XCpu,
    XFSz,
}

impl From<Signal> for u8 {
    #[inline]
    fn from(signal: Signal) -> u8 {
        signal as u8
    }
}

/// Timestamp in nanoseconds.
#[derive(Prim, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn from_nanos(nanos: u64) -> Self {
        Timestamp(nanos)
    }

    pub fn from_sec(sec: u64) -> Self {
        Self::from_nanos(sec * 1_000_000_000)
    }

    pub fn as_nanos(&self) -> u64 {
        self.0
    }
}

bitflags! {
    pub struct SetTimeFlags: u16 {
        const ATIME     = 1 << 0;
        const ATIME_NOW = 1 << 1;
        const MTIME     = 1 << 2;
        const MTIME_NOW = 1 << 3;
    }
}

impl TryFrom<u16> for SetTimeFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        SetTimeFlags::from_bits(code).ok_or(())
    }
}

impl From<SetTimeFlags> for u16 {
    #[inline]
    fn from(flags: SetTimeFlags) -> Self {
        flags.bits
    }
}

pub type UserData = u64;

/// The position relative to which to set the offset of the file descriptor.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Prim)]
pub enum Whence {
    Start,
    Current,
    End,
}

impl From<Whence> for u8 {
    #[inline]
    fn from(whence: Whence) -> Self {
        whence as u8
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Subscription {
    pub userdata: UserData, 
    pub u: SubscriptionUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SubscriptionUnion {
    Clock(SubscriptionClock),
    FdRead(SubscriptionFdReadwrite),
    FdWrite(SubscriptionFdReadwrite),
}

pub type SubClockFlags = u16;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionClock {
    pub clock_id: ClockId,
    pub timeout: Timestamp,
    pub precision: Timestamp,
    pub flags: SubClockFlags,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SubscriptionFdReadwrite {
    pub fd: Fd,
}

bitflags! {
    #[derive(Default)]
    pub struct SdFlags: u8 {
        /// Disables further receive operations.
        const RD = 1 << 0;

        /// Disables further send operations.
        const WR = 1 << 1;
    }
}

impl TryFrom<u8> for SdFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u8) -> Result<Self, Self::Error> {
        SdFlags::from_bits(code).ok_or(())
    }
}

impl From<SdFlags> for u8 {
    #[inline]
    fn from(flags: SdFlags) -> Self {
        flags.bits
    }
}

bitflags! {
    #[derive(Default)]
    pub struct SiFlags: u16 { 
        const MUST_BE_ZERO = 0;
    }
}

impl TryFrom<u16> for SiFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        SiFlags::from_bits(code).ok_or(())
    }
}

impl From<SiFlags> for u16 {
    #[inline]
    fn from(flags: SiFlags) -> Self {
        flags.bits
    }
}


bitflags! {
    #[derive(Default)]
    pub struct RiFlags: u16 { 
        /// Returns the message without removing it.
        const RECV_PEEK = 1 << 0;

        /// Block until the full amount.
        const RECV_WAITALL = 1 << 1;
    }
}

impl TryFrom<u16> for RiFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        RiFlags::from_bits(code).ok_or(())
    }
}

impl From<RiFlags> for u16 {
    #[inline]
    fn from(flags: RiFlags) -> Self {
        flags.bits
    }
}

bitflags! {
    #[derive(Default)]
    pub struct RoFlags: u16 { 
        /// Truncate the message
        const RECV_DATA_TRUNCATED = 1 << 0;
    }
}

impl TryFrom<u16> for RoFlags {
    type Error = ();

    #[inline]
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        RoFlags::from_bits(code).ok_or(())
    }
}

impl From<RoFlags> for u16 {
    #[inline]
    fn from(flags: RoFlags) -> Self {
        flags.bits
    }
}
