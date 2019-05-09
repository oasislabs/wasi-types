//! Rusty WASI type definitions based on
//! [the spec](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md)

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate proper;

/// File or memory access pattern advisory information.
#[derive(Prim)]
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

/// Identifiers for clocks.
#[derive(Prim)]
#[prim(ty = "u8")]
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

/// Identifier for a device containing a file system. Can be used in combination with `Inode`
/// to uniquely identify a file or directory in the filesystem.
#[derive(Prim)]
pub struct Device(u64);

/// A reference to the offset of a directory entry.
#[derive(Prim)]
pub struct DirCookie(u64);

impl DirCookie {
    /// Creates a new `DirCookie` repreenting a permanent reference to the first directory entry
    /// within a directory.
    pub fn start() -> Self {
        DirCookie(0)
    }
}

/// A directory entry.
pub struct DirEnt {
    /// The offset of the next directory entry stored in this directory.
    next: DirCookie,

    /// The serial number of the file referred to by this directory entry.
    inode: Inode,

    /// The length of the name of the directory entry.
    name_len: u32,

    /// The type of the file referred to by this directory entry.
    file_type: FileType,
}

impl DirEnt {
    pub fn next(&self) -> &DirCookie {
        &self.next
    }

    pub fn inode(&self) -> &Inode {
        &self.inode
    }

    pub fn name_len(&self) -> u32 {
        self.name_len
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }
}

/// Error codes returned by functions.
#[derive(Prim)]
#[prim(ty = "u16")]
pub enum ErrNo {
    /// No error occurred. System call completed successfully.
    Success,

    /// Argument list too long.
    TooBig,

    /// Permission denied.
    Access,

    /// Address in use.
    AddrInUse,

    /// Address not available.
    AddrNotAvail,

    /// Address family not supported.
    AfNoSupport,

    /// Resource unavailable, or operation would block.
    Again,

    /// Connection already in progress.
    Already,

    /// Bad file descriptor.
    BadF,

    /// Bad message.
    BadMsg,

    /// Device or resource busy.
    Busy,

    /// Operation canceled.
    Canceled,

    /// No child processes.
    Child,

    /// Connection aborted.
    ConnAborted,

    /// Connection refused.
    ConnRefused,

    /// Connection reset.
    ConnReset,

    /// Resource deadlock would occur.
    Deadlk,

    /// Destination address required.
    DestAddrReq,

    /// Mathematics argument out of domain of function.
    Domain,

    /// Reserved. (Quota exceeded.)
    DQuot,

    /// File exists.
    Exist,

    /// Bad address.
    Fault,

    /// File too large.
    FBig,

    /// Host is unreachable.
    HostUnreach,

    /// Identifier removed.
    IdRm,

    /// Illegal byte sequence.
    IlSeq,

    /// Operation in progress.
    InProgress,

    /// Interrupted function.
    Intr,

    /// Invalid argument.
    Inval,

    /// I/O error.
    Io,

    /// Socket is connected.
    IsConn,

    /// Is a directory.
    IsDir,

    /// Too many levels of symbolic links.
    Loop,

    /// File descriptor value too large.
    MFile,

    /// Too many links.
    MLink,

    /// Message too large.
    MsgSize,

    /// Reserved. (Multihop attempted.)
    Multihop,

    /// Filename too long.
    NameTooLong,

    /// Network is down.
    Netdown,

    /// Connection aborted by network.
    NetReset,

    /// Network unreachable.
    NetUnreach,

    /// Too many files open in system.
    NFile,

    /// No buffer space available.
    NoBufS,

    /// No such device.
    NoDev,

    /// No such file or directory.
    NoEnt,

    /// Executable file format error.
    NoExec,

    /// No locks available.
    NoLock,

    /// Reserved. (Link has been severed.)
    NoLink,

    /// Not enough space.
    NoMem,

    /// No message of the desired type.
    NoMsg,

    /// Protocol not available.
    NoProtoOpt,

    /// No space left on device.
    NoSpace,

    /// Function not supported.
    NoSys,

    /// The socket is not connected.
    NotConn,

    /// Not a directory or a symbolic link to a directory.
    NotDir,

    /// Directory not empty.
    NotEmpty,

    /// State not recoverable.
    NotRecoverable,

    /// Not a socket.
    NotSock,

    /// Not supported, or operation not supported on socket.
    NotSup,

    /// Inappropriate I/O control operation.
    NoTty,

    /// No such device or address.
    NxIo,

    /// Value too large to be stored in data type.
    Overflow,

    /// Previous owner died.
    OwnerDead,

    /// Operation not permitted.
    Perm,

    /// Broken pipe.
    Pipe,

    /// Protocol error.
    Proto,

    /// Protocol not supported.
    ProtoNoSupport,

    /// Protocol wrong type for socket.
    ProtoType,

    /// Result too large.
    Range,

    /// Read-only file system.
    RoFs,

    /// Invalid seek.
    SPipe,

    /// No such process.
    Srch,

    /// Reserved. (Stale file handle.)
    Stale,

    /// Connection timed out.
    TimedOut,

    /// Text file busy.
    TxtBsy,

    /// Cross-device link.
    XDev,

    /// Extension: Capabilities insufficient.
    NotCapable,
}

pub struct Event {
    user_data: UserData,
    error: ErrNo,
    ty: EventType,
    fd_state: Option<EventFdState>,
}

impl Event {
    pub fn user_data(&self) -> UserData {
        self.user_data
    }

    pub fn error(&self) -> &ErrNo {
        &self.error
    }

    pub fn ty(&self) -> &EventType {
        &self.ty
    }

    pub fn fd_state(&self) -> Option<&EventFdState> {
        self.fd_state.as_ref()
    }
}

#[derive(Prim)]
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

/// The state of the file descriptor subscribed to with `EventType::FdRead` or `EventType::FdWrite`.
#[derive(Prim, Clone, Copy)]
#[prim(ty = "u16")]
pub enum EventRwFlags {
    None,
    Hangup,
}

pub type ExitCode = u32;

pub struct EventFdState {
    file_size: FileSize,
    flags: EventRwFlags,
}

impl EventFdState {
    pub fn file_size(&self) -> FileSize {
        self.file_size
    }

    pub fn flags(&self) -> EventRwFlags {
        self.flags
    }
}

/// A file descriptor number.
/// As in POSIX, 0, 1, and 2 are stdin, stdout, and stderr, respectively.
/// File descriptors are not guaranteed to be contiguous or allocated in ascending order.
/// Information about a file descriptor may be obtained through `fd_prestat_get`.
#[derive(Prim)]
pub struct Fd(u32);

#[derive(Prim)]
pub enum FdFlags {
    /// Append mode: Data written to the file is always appended to the file's end.
    Append,

    /// Write according to synchronized I/O data integrity completion.
    /// Only the data stored in the file is synchronized.
    DSync,

    /// Non-blocking mode.
    NonBlock,

    /// Synchronized read I/O operations.
    RSync,

    /// Write according to synchronized I/O file integrity completion. In addition to synchronizing
    /// the data stored in the file, the implementation may also synchronously update the file's
    /// metadata.
    Sync,
}

pub struct FdStat {
    file_type: FileType,
    flags: FdFlags,

    /// Rights that apply to this file descriptor.
    rights_base: Rights,

    /// Maximum set of rights that may be installed on new file descriptors that are created
    /// through this file descripto
    rights_inheriting: Rights,
}

impl FdStat {
    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn flags(&self) -> &FdFlags {
        &self.flags
    }

    pub fn rights_base(&self) -> &Rights {
        &self.rights_base
    }

    pub fn rights_inheriting(&self) -> &Rights {
        &self.rights_inheriting
    }
}

/// Relative offset within a file.
pub type FileDelta = i64;

/// The type of a file descriptor or file.
#[derive(Prim)]
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

pub type FileSize = u64;

/// File serial number that is unique within its file system.
#[derive(Prim)]
pub struct Inode(u64);

bitflags! {
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
    }
}

/// Timestamp in nanoseconds.
#[derive(Prim)]
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

pub type UserData = u64;
