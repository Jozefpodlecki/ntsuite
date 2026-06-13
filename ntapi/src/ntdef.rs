
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_void;

pub type ULONG = u32;
pub type PULONG = *mut ULONG;
pub type USHORT = u16;
pub type PUSHORT = *mut USHORT;
pub type UCHAR = u8;
pub type PUCHAR = *mut u8;
pub type CHAR = i8;
pub type PSZ = *mut CHAR;

pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type LPBOOL = *mut BOOL;
pub type LPBYTE = *mut BYTE;
pub type LPINT = *mut i32;
pub type LPWORD = *mut WORD;
pub type LPLONG = *mut i32;
pub type LPDWORD = *mut DWORD;

pub type DWORD = u32;
pub type BOOL = i32;
pub type BYTE = u8;
pub type WORD = u16;
pub type FLOAT = f32;
pub type PFLOAT = *mut f32;
pub type PBOOL = *mut BOOL;
pub type PBYTE = *mut BYTE;
pub type PWORD = *mut WORD;
pub type PINT = *mut i32;
pub type PDWORD = *mut DWORD;
pub type INT = i32;
pub type LONG = i32;
pub type UINT = u32;
pub type PUINT = *mut u32;
pub type BOOLEAN = u8;
pub type PBOOLEAN = *mut BOOLEAN;

pub type SIZE_T = ULONG_PTR;
pub type PSIZE_T = *mut SIZE_T;
pub type SSIZE_T = LONG_PTR;
pub type PSSIZE_T = *mut SSIZE_T;
pub type LONGLONG = i64;
pub type ULONGLONG = u64;
pub type PLONGLONG = *mut LONGLONG;
pub type PULONGLONG = *mut ULONGLONG;

pub const MAXLONGLONG: LONGLONG = 0x7fffffffffffffff;

pub type USN = LONGLONG;

pub type HANDLE = *mut c_void;
pub type ACCESS_MASK = u32;
pub type NTSTATUS = i32;
pub type PSID = *mut c_void;
pub type PSECURITY_DESCRIPTOR = *mut c_void;
pub type SECURITY_INFORMATION = u32;

#[cfg(target_pointer_width = "64")]
pub type INT_PTR = i64;
#[cfg(target_pointer_width = "32")]
pub type INT_PTR = i32;

#[cfg(target_pointer_width = "64")]
pub type UINT_PTR = u64;
#[cfg(target_pointer_width = "32")]
pub type UINT_PTR = u32;

#[cfg(target_pointer_width = "64")]
pub type LONG_PTR = i64;
#[cfg(target_pointer_width = "32")]
pub type LONG_PTR = i32;

#[cfg(target_pointer_width = "64")]
pub type ULONG_PTR = u64;
#[cfg(target_pointer_width = "32")]
pub type ULONG_PTR = u32;

#[cfg(target_pointer_width = "64")]
pub type DWORD_PTR = u64;
#[cfg(target_pointer_width = "32")]
pub type DWORD_PTR = u32;

pub type PHANDLE = *mut HANDLE;
pub type HGLOBAL = HANDLE;
pub type HLOCAL = HANDLE;
pub type GLOBALHANDLE = HANDLE;
pub type LOCALHANDLE = HANDLE;
pub type HMODULE = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HRGN = HANDLE;
pub type HRSRC = HANDLE;
pub type HSPRITE = HANDLE;
pub type HLSURF = HANDLE;
pub type HSTR = HANDLE;
pub type HTASK = HANDLE;
pub type HWINSTA = HANDLE;
pub type HKL = HANDLE;
pub type HKEY = HANDLE;
pub type PHKEY = *mut HKEY;
pub type HMETAFILE = HANDLE;
pub type SPHANDLE = *mut HANDLE;
pub type LPHANDLE = *mut HANDLE;

pub type HFILE = i32;

pub type WCHAR = u16;

pub const ANSI_NULL: CHAR = 0;
pub const UNICODE_NULL: WCHAR = 0;
pub const UNICODE_STRING_MAX_BYTES: WORD = 65534;
pub const UNICODE_STRING_MAX_CHARS: i32 = 32767;

#[cfg(target_pointer_width = "64")]
pub type FARPROC = Option<unsafe extern "system" fn() -> INT_PTR>;
#[cfg(target_pointer_width = "32")]
pub type FARPROC = Option<unsafe extern "system" fn() -> i32>;

#[cfg(target_pointer_width = "64")]
pub type NEARPROC = Option<unsafe extern "system" fn() -> INT_PTR>;
#[cfg(target_pointer_width = "32")]
pub type NEARPROC = Option<unsafe extern "system" fn() -> i32>;

#[cfg(target_pointer_width = "64")]
pub type PROC = Option<unsafe extern "system" fn() -> INT_PTR>;
#[cfg(target_pointer_width = "32")]
pub type PROC = Option<unsafe extern "system" fn() -> i32>;

pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;

pub type ATOM = WORD;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILETIME {
    pub dwLowDateTime: DWORD,
    pub dwHighDateTime: DWORD,
}

pub type PFILETIME = *mut FILETIME;
pub type LPFILETIME = *mut FILETIME;

pub const MAX_PATH: usize = 260;

pub const FALSE: BOOL = 0;
pub const TRUE: BOOL = 1;

pub const NULL: *mut c_void = core::ptr::null_mut();

#[inline]
pub const fn MAKEWORD(a: u8, b: u8) -> WORD {
    ((a as WORD) & 0xFF) | (((b as WORD) & 0xFF) << 8)
}

#[inline]
pub const fn MAKELONG(a: WORD, b: WORD) -> LONG {
    (((a as DWORD) & 0xFFFF) | (((b as DWORD) & 0xFFFF) << 16)) as LONG
}

#[inline]
pub const fn LOWORD(l: DWORD_PTR) -> WORD {
    (l & 0xFFFF) as WORD
}

#[inline]
pub const fn HIWORD(l: DWORD_PTR) -> WORD {
    ((l >> 16) & 0xFFFF) as WORD
}

#[inline]
pub const fn LOBYTE(w: DWORD_PTR) -> BYTE {
    (w & 0xFF) as BYTE
}

#[inline]
pub const fn HIBYTE(w: DWORD_PTR) -> BYTE {
    ((w >> 8) & 0xFF) as BYTE
}

#[inline]
pub fn max<T: core::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[inline]
pub fn min<T: core::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

#[macro_export]
macro_rules! DECLARE_HANDLE {
    ($name:ident) => {
        pub type $name = HANDLE;
    };
}

pub const STATUS_SUCCESS: NTSTATUS = 0x00000000;
pub const STATUS_WAIT_0: NTSTATUS = 0x00000000;
pub const STATUS_ABANDONED_WAIT_0: NTSTATUS = 0x00000080;
pub const STATUS_USER_APC: NTSTATUS = 0x000000C0;
pub const STATUS_TIMEOUT: NTSTATUS = 0x00000102;
pub const STATUS_PENDING: NTSTATUS = 0x00000103;
pub const STATUS_SEGMENT_NOTIFICATION: NTSTATUS = 0x40000005;

pub const STATUS_UNSUCCESSFUL: NTSTATUS = 0xC0000001u32 as _;
pub const STATUS_ACCESS_VIOLATION: NTSTATUS = 0xC0000005u32 as _;
pub const STATUS_DATATYPE_MISALIGNMENT: NTSTATUS = 0xC0000006u32 as _;
pub const STATUS_BREAKPOINT: NTSTATUS = 0xC0000003u32 as _;
pub const STATUS_SINGLE_STEP: NTSTATUS = 0xC0000004u32 as _;
pub const STATUS_ILLEGAL_INSTRUCTION: NTSTATUS = 0xC000001Du32 as _;
pub const STATUS_PRIVILEGED_INSTRUCTION: NTSTATUS = 0xC0000096u32 as _;
pub const STATUS_PAGE_FAULT: NTSTATUS = 0xC000000Eu32 as _;
pub const STATUS_INVALID_HANDLE: NTSTATUS = 0xC0000008u32 as _;
pub const STATUS_NOT_IMPLEMENTED: NTSTATUS = 0xC0000002u32 as _;
pub const STATUS_INFO_LENGTH_MISMATCH: NTSTATUS = 0xC0000004u32 as _;
pub const STATUS_INVALID_PARAMETER: NTSTATUS = 0xC000000Du32 as _;

pub trait NtStatusExt {
    fn is_success(&self) -> bool;
    fn is_error(&self) -> bool;
    fn is_information(&self) -> bool;
    fn is_warning(&self) -> bool;
}

impl NtStatusExt for NTSTATUS {
    #[inline]
    fn is_success(&self) -> bool {
        *self >= 0
    }

    #[inline]
    fn is_error(&self) -> bool {
        *self < 0
    }

    #[inline]
    fn is_information(&self) -> bool {
        (*self as u32) & 0xC0000000 == 0x40000000
    }

    #[inline]
    fn is_warning(&self) -> bool {
        (*self as u32) & 0xC0000000 == 0x80000000
    }
}

#[repr(C)]
pub struct UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: *mut u16,
}

#[repr(C)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: ULONG,
    pub RootDirectory: HANDLE,
    pub ObjectName: *mut UNICODE_STRING,
    pub Attributes: ULONG,
    pub SecurityDescriptor: PVOID,
    pub SecurityQualityOfService: PVOID,
}

#[repr(C)]
pub union LARGE_INTEGER {
    pub u: core::mem::ManuallyDrop<LARGE_INTEGER_STRUCT>,
    pub QuadPart: LONGLONG,
}

#[repr(C)]
pub struct LARGE_INTEGER_STRUCT {
    pub LowPart: DWORD,
    pub HighPart: LONG,
}

pub type PLARGE_INTEGER = *mut LARGE_INTEGER;

#[repr(C)]
pub union ULARGE_INTEGER {
    pub u: core::mem::ManuallyDrop<ULARGE_INTEGER_STRUCT>,
    pub QuadPart: ULONGLONG,
}

#[repr(C)]
pub struct ULARGE_INTEGER_STRUCT {
    pub LowPart: DWORD,
    pub HighPart: DWORD,
}

pub type PULARGE_INTEGER = *mut ULARGE_INTEGER;

pub type FILE_ACCESS_RIGHTS = u32;
pub const SYNCHRONIZE: FILE_ACCESS_RIGHTS = 1048576u32;
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_RIGHTS = 1179808u32;
pub const FILE_GENERIC_READ: FILE_ACCESS_RIGHTS = 1179785u32;
pub const FILE_GENERIC_WRITE: FILE_ACCESS_RIGHTS = 1179926u32;
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_RIGHTS = 128u32;
pub const FILE_READ_DATA: FILE_ACCESS_RIGHTS = 1u32;
pub const FILE_READ_EA: FILE_ACCESS_RIGHTS = 8u32;

pub const FILE_ALL_ACCESS: FILE_ACCESS_RIGHTS = 2032127u32;
pub const FILE_APPEND_DATA: FILE_ACCESS_RIGHTS = 4u32;
pub type FILE_FLAGS_AND_ATTRIBUTES = u32;
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = 32u32;
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = 2048u32;
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = 64u32;
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = 16u32;
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = 262144u32;
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = 16384u32;
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = 2u32;
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES = 32768u32;
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = 128u32;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES = 8192u32;
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES = 131072u32;
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = 4096u32;
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = 524288u32;
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = 1u32;
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = 4194304u32;
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES = 262144u32;
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = 1024u32;
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = 512u32;
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = 4u32;

pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = 4u32;
pub type FILE_SHARE_MODE = u32;
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = 0u32;
pub const FILE_SHARE_READ: FILE_SHARE_MODE = 1u32;
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = 2u32;

pub type FILE_CREATION_DISPOSITION = u32;
pub const CREATE_ALWAYS: FILE_CREATION_DISPOSITION = 2u32;
pub const CREATE_NEW: FILE_CREATION_DISPOSITION = 1u32;
pub const OPEN_ALWAYS: FILE_CREATION_DISPOSITION = 4u32;
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = 3u32;

pub const FILE_SUPERSEDE: ULONG = 0x00000000;
pub const FILE_OPEN: ULONG = 0x00000001;
pub const FILE_CREATE: ULONG = 0x00000002;
pub const FILE_OPEN_IF: ULONG = 0x00000003;
pub const FILE_OVERWRITE: ULONG = 0x00000004;
pub const FILE_OVERWRITE_IF: ULONG = 0x00000005;
pub const FILE_MAXIMUM_DISPOSITION: ULONG = 0x00000005;
pub const FILE_DIRECTORY_FILE: ULONG = 0x00000001;
pub const FILE_WRITE_THROUGH: ULONG = 0x00000002;
pub const FILE_SEQUENTIAL_ONLY: ULONG = 0x00000004;
pub const FILE_NO_INTERMEDIATE_BUFFERING: ULONG = 0x00000008;
pub const FILE_SYNCHRONOUS_IO_ALERT: ULONG = 0x00000010;
pub const FILE_SYNCHRONOUS_IO_NONALERT: ULONG = 0x00000020;
pub const FILE_NON_DIRECTORY_FILE: ULONG = 0x00000040;
pub const FILE_CREATE_TREE_CONNECTION: ULONG = 0x00000080;
pub const FILE_COMPLETE_IF_OPLOCKED: ULONG = 0x00000100;
pub const FILE_NO_EA_KNOWLEDGE: ULONG = 0x00000200;
pub const FILE_OPEN_FOR_RECOVERY: ULONG = 0x00000400;
pub const FILE_RANDOM_ACCESS: ULONG = 0x00000800;
pub const FILE_DELETE_ON_CLOSE: ULONG = 0x00001000;
pub const FILE_OPEN_BY_FILE_ID: ULONG = 0x00002000;
pub const FILE_OPEN_FOR_BACKUP_INTENT: ULONG = 0x00004000;
pub const FILE_NO_COMPRESSION: ULONG = 0x00008000;
pub const FILE_OPEN_REQUIRING_OPLOCK: ULONG = 0x00010000;
pub const FILE_DISALLOW_EXCLUSIVE: ULONG = 0x00020000;
pub const FILE_SESSION_AWARE: ULONG = 0x00040000;
pub const FILE_RESERVE_OPFILTER: ULONG = 0x00100000;
pub const FILE_OPEN_REPARSE_POINT: ULONG = 0x00200000;
pub const FILE_OPEN_NO_RECALL: ULONG = 0x00400000;
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: ULONG = 0x00800000;
pub const FILE_COPY_STRUCTURED_STORAGE: ULONG = 0x00000041;
pub const FILE_STRUCTURED_STORAGE: ULONG = 0x00000441;
pub const FILE_SUPERSEDED: ULONG = 0x00000000;
pub const FILE_OPENED: ULONG = 0x00000001;
pub const FILE_CREATED: ULONG = 0x00000002;
pub const FILE_OVERWRITTEN: ULONG = 0x00000003;
pub const FILE_EXISTS: ULONG = 0x00000004;
pub const FILE_DOES_NOT_EXIST: ULONG = 0x00000005;
pub const FILE_WRITE_TO_END_OF_FILE: ULONG = 0xffffffff;
pub const FILE_USE_FILE_POINTER_POSITION: ULONG = 0xfffffffe;
pub const FILE_BYTE_ALIGNMENT: ULONG = 0x00000000;
pub const FILE_WORD_ALIGNMENT: ULONG = 0x00000001;
pub const FILE_LONG_ALIGNMENT: ULONG = 0x00000003;
pub const FILE_QUAD_ALIGNMENT: ULONG = 0x00000007;
pub const FILE_OCTA_ALIGNMENT: ULONG = 0x0000000f;
pub const FILE_32_BYTE_ALIGNMENT: ULONG = 0x0000001f;
pub const FILE_64_BYTE_ALIGNMENT: ULONG = 0x0000003f;
pub const FILE_128_BYTE_ALIGNMENT: ULONG = 0x0000007f;
pub const FILE_256_BYTE_ALIGNMENT: ULONG = 0x000000ff;
pub const FILE_512_BYTE_ALIGNMENT: ULONG = 0x000001ff;
pub const MAXIMUM_FILENAME_LENGTH: u32 = 256;
pub const FILE_NEED_EA: ULONG = 0x00000080;
pub const FILE_EA_TYPE_BINARY: ULONG = 0xfffe;
pub const FILE_EA_TYPE_ASCII: ULONG = 0xfffd;
pub const FILE_EA_TYPE_BITMAP: ULONG = 0xfffb;
pub const FILE_EA_TYPE_METAFILE: ULONG = 0xfffa;
pub const FILE_EA_TYPE_ICON: ULONG = 0xfff9;
pub const FILE_EA_TYPE_EA: ULONG = 0xffee;
pub const FILE_EA_TYPE_MVMT: ULONG = 0xffdf;
pub const FILE_EA_TYPE_MVST: ULONG = 0xffde;
pub const FILE_EA_TYPE_ASN1: ULONG = 0xffdd;
pub const FILE_EA_TYPE_FAMILY_IDS: ULONG = 0xff01;
pub const FILE_REMOVABLE_MEDIA: ULONG = 0x00000001;
pub const FILE_READ_ONLY_DEVICE: ULONG = 0x00000002;
pub const FILE_FLOPPY_DISKETTE: ULONG = 0x00000004;
pub const FILE_WRITE_ONCE_MEDIA: ULONG = 0x00000008;
pub const FILE_REMOTE_DEVICE: ULONG = 0x00000010;
pub const FILE_DEVICE_IS_MOUNTED: ULONG = 0x00000020;
pub const FILE_VIRTUAL_VOLUME: ULONG = 0x00000040;
pub const FILE_AUTOGENERATED_DEVICE_NAME: ULONG = 0x00000080;
pub const FILE_DEVICE_SECURE_OPEN: ULONG = 0x00000100;
pub const FILE_CHARACTERISTIC_PNP_DEVICE: ULONG = 0x00000800;
pub const FILE_CHARACTERISTIC_TS_DEVICE: ULONG = 0x00001000;
pub const FILE_CHARACTERISTIC_WEBDAV_DEVICE: ULONG = 0x00002000;
pub const FILE_CHARACTERISTIC_CSV: ULONG = 0x00010000;
pub const FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL: ULONG = 0x00020000;
pub const FILE_PORTABLE_DEVICE: ULONG = 0x00040000;
pub const FILE_PIPE_BYTE_STREAM_TYPE: ULONG = 0x00000000;
pub const FILE_PIPE_MESSAGE_TYPE: ULONG = 0x00000001;
pub const FILE_PIPE_ACCEPT_REMOTE_CLIENTS: ULONG = 0x00000000;
pub const FILE_PIPE_REJECT_REMOTE_CLIENTS: ULONG = 0x00000002;
pub const FILE_PIPE_TYPE_VALID_MASK: ULONG = 0x00000003;
pub const FILE_PIPE_QUEUE_OPERATION: ULONG = 0x00000000;
pub const FILE_PIPE_COMPLETE_OPERATION: ULONG = 0x00000001;
pub const FILE_PIPE_BYTE_STREAM_MODE: ULONG = 0x00000000;
pub const FILE_PIPE_MESSAGE_MODE: ULONG = 0x00000001;
pub const FILE_PIPE_INBOUND: ULONG = 0x00000000;
pub const FILE_PIPE_OUTBOUND: ULONG = 0x00000001;
pub const FILE_PIPE_FULL_DUPLEX: ULONG = 0x00000002;
pub const FILE_PIPE_DISCONNECTED_STATE: ULONG = 0x00000001;
pub const FILE_PIPE_LISTENING_STATE: ULONG = 0x00000002;
pub const FILE_PIPE_CONNECTED_STATE: ULONG = 0x00000003;
pub const FILE_PIPE_CLOSING_STATE: ULONG = 0x00000004;
pub const FILE_PIPE_CLIENT_END: ULONG = 0x00000000;
pub const FILE_PIPE_SERVER_END: ULONG = 0x00000001;