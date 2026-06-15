#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::{ffi::c_void, mem::ManuallyDrop};

use crate::ntseapi::SECURITY_IMPERSONATION_LEVEL;

#[repr(C, align(16))]
pub struct CONTEXT {
    pub P1Home: DWORD64,
    pub P2Home: DWORD64,
    pub P3Home: DWORD64,
    pub P4Home: DWORD64,
    pub P5Home: DWORD64,
    pub P6Home: DWORD64,
    pub ContextFlags: DWORD,
    pub MxCsr: DWORD,
    pub SegCs: WORD,
    pub SegDs: WORD,
    pub SegEs: WORD,
    pub SegFs: WORD,
    pub SegGs: WORD,
    pub SegSs: WORD,
    pub EFlags: DWORD,
    pub Dr0: DWORD64,
    pub Dr1: DWORD64,
    pub Dr2: DWORD64,
    pub Dr3: DWORD64,
    pub Dr6: DWORD64,
    pub Dr7: DWORD64,
    pub Rax: DWORD64,
    pub Rcx: DWORD64,
    pub Rdx: DWORD64,
    pub Rbx: DWORD64,
    pub Rsp: DWORD64,
    pub Rbp: DWORD64,
    pub Rsi: DWORD64,
    pub Rdi: DWORD64,
    pub R8: DWORD64,
    pub R9: DWORD64,
    pub R10: DWORD64,
    pub R11: DWORD64,
    pub R12: DWORD64,
    pub R13: DWORD64,
    pub R14: DWORD64,
    pub R15: DWORD64,
    pub Rip: DWORD64,
    pub u: CONTEXT_u,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: DWORD64,
    pub DebugControl: DWORD64,
    pub LastBranchToRip: DWORD64,
    pub LastBranchFromRip: DWORD64,
    pub LastExceptionToRip: DWORD64,
    pub LastExceptionFromRip: DWORD64,
}

#[repr(C)]
pub union CONTEXT_u {
    pub FltSave: ManuallyDrop<XSAVE_FORMAT>,
    pub Dummy: [DWORD; 1],
}

#[repr(C)]
pub struct XSAVE_FORMAT {
    pub ControlWord: WORD,
    pub StatusWord: WORD,
    pub TagWord: WORD,
    pub Reserved1: WORD,
    pub ErrorOpcode: WORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: WORD,
    pub Reserved2: WORD,
    pub DataOffset: DWORD,
    pub DataSelector: WORD,
    pub Reserved3: WORD,
    pub MxCsr: DWORD,
    pub MxCsr_Mask: DWORD,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [UCHAR; 96],
}

#[repr(C, align(16))]
pub struct M128A {
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}

#[repr(C)]
pub struct LUID {
    pub LowPart: ULONG,
    pub HighPart: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GUID {
    pub Data1: ULONG,
    pub Data2: USHORT,
    pub Data3: USHORT,
    pub Data4: [UCHAR; 8],
}

pub type PGUID = *mut GUID;
pub type LPCGUID = *const GUID;
pub type PCGUID = *const GUID;
pub type VOID = c_void;
pub type PCHAR = *mut CHAR;
pub type LPCH = *mut CHAR;
pub type PCH = *mut CHAR;
pub type LPCCH = *const CHAR;
pub type PCCH = *const CHAR;
pub type NPSTR = *mut CHAR;
pub type LPSTR = *mut CHAR;
pub type PSTR = *mut CHAR;
pub type PZPSTR = *mut PSTR;
pub type PCZPSTR = *const PSTR;
pub type LPCSTR = *const CHAR;
pub type PCSTR = *const CHAR;
pub type PZPCSTR = *mut PCSTR;
pub type PCZPCSTR = *const PCSTR;
pub type PZZSTR = *mut CHAR;
pub type PCZZSTR = *const CHAR;
pub type PNZCH = *mut CHAR;
pub type PCNZCH = *const CHAR;

pub type PLUID = *mut LUID;
pub type DWORD64 = u64;
pub type PCONTEXT = *mut CONTEXT;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
pub type PCZZWSTR = *const u16;

pub type PLONG = *mut LONG;
pub type LOGICAL = ULONG;
pub type PLOGICAL = *mut ULONG;
pub type CSHORT = i16;
pub type CLONG = ULONG;
pub type ULONG = u32;
pub type ULONG64 = u64;
pub type PLONG64 = *mut i64;
pub type PULONG64 = *mut u64;
pub type PULONG_PTR = *mut usize;
pub type PULONG = *mut ULONG;
pub type USHORT = u16;
pub type PUSHORT = *mut USHORT;
pub type UCHAR = u8;
pub type PUCHAR = *mut u8;
pub type CHAR = i8;
pub type PSZ = *mut CHAR;

pub type PVOID = *mut c_void;
pub type PCVOID = *const c_void;
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
pub type UINT32 = u32;
pub type BOOLEAN = u8;
pub type PBOOLEAN = *mut BOOLEAN;
pub type KTMOBJECT_TYPE = i32;

pub type SIZE_T = ULONG_PTR;
pub type PSIZE_T = *mut SIZE_T;
pub type SSIZE_T = LONG_PTR;
pub type PSSIZE_T = *mut SSIZE_T;
pub type LONGLONG = i64;
pub type ULONGLONG = u64;
pub type PLONGLONG = *mut LONGLONG;
pub type PULONGLONG = *mut ULONGLONG;
pub type PCRM_PROTOCOL_ID = *const c_void;
pub type TRANSACTION_INFORMATION_CLASS = i32;
pub type PLCID = *mut u32;
pub type PTRANSACTION_NOTIFICATION = *mut c_void;

pub const MAXLONGLONG: LONGLONG = 0x7fffffffffffffff;

pub type USN = LONGLONG;

pub type HANDLE = *mut c_void;
pub type ACCESS_MASK = u32;
pub type PACCESS_TOKEN = PVOID;
pub type NTSTATUS = i32;
pub type PNTSTATUS = *mut NTSTATUS;
pub type PSID = *mut c_void;
pub type PSECURITY_DESCRIPTOR = *mut c_void;
pub type SECURITY_INFORMATION = u32;
pub type LPGUID = *mut GUID;
pub type LANGID = u16;
pub type RESOURCEMANAGER_INFORMATION_CLASS = i32;
pub type PPRIVILEGE_SET = *mut c_void;
pub type TRANSACTIONMANAGER_INFORMATION_CLASS = i32;

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
#[derive(Copy, Clone)]
pub struct UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: *mut u16,
}

pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;

#[repr(C)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [BYTE; IMAGE_SIZEOF_SHORT_NAME],
    pub Misc: IMAGE_SECTION_HEADER_Misc,
    pub VirtualAddress: DWORD,
    pub SizeOfRawData: DWORD,
    pub PointerToRawData: DWORD,
    pub PointerToRelocations: DWORD,
    pub PointerToLinenumbers: DWORD,
    pub NumberOfRelocations: WORD,
    pub NumberOfLinenumbers: WORD,
    pub Characteristics: DWORD,
}

#[repr(C)]
pub union IMAGE_SECTION_HEADER_Misc {
    pub PhysicalAddress: DWORD,
    pub VirtualSize: DWORD,
}

pub type PIMAGE_SECTION_HEADER = *mut IMAGE_SECTION_HEADER;

#[repr(C)]
pub struct STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PCHAR,
}

pub type PSTRING = *mut STRING;
pub type PANSI_STRING = *mut STRING;
pub type PUNICODE_STRING = *mut UNICODE_STRING;
pub type PCUNICODE_STRING = *const UNICODE_STRING;
pub type PCANSI_STRING = PSTRING;

#[repr(C)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: ULONG,
    pub RootDirectory: HANDLE,
    pub ObjectName: PUNICODE_STRING,
    pub Attributes: ULONG,
    pub SecurityDescriptor: PVOID,
    pub SecurityQualityOfService: PVOID,
}

pub type PCOBJECT_ATTRIBUTES = *const OBJECT_ATTRIBUTES;
pub type POBJECT_ATTRIBUTES = *mut OBJECT_ATTRIBUTES;

#[repr(C)]
#[derive(Copy, Clone)]
pub union LARGE_INTEGER {
    pub u: LARGE_INTEGER_STRUCT,
    pub QuadPart: LONGLONG,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LARGE_INTEGER_STRUCT {
    pub LowPart: DWORD,
    pub HighPart: LONG,
}

pub type PLARGE_INTEGER = *mut LARGE_INTEGER;
pub type PFILE_SEGMENT_ELEMENT = *mut core::ffi::c_void;

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESSOR_NUMBER {
    pub Group: USHORT,
    pub Number: UCHAR,
    pub Reserved: UCHAR,
}

pub type PPROCESSOR_NUMBER = *mut PROCESSOR_NUMBER;

#[repr(C)]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [BYTE; 6],
}

pub type PSID_IDENTIFIER_AUTHORITY = *mut SID_IDENTIFIER_AUTHORITY;

#[repr(C)]
pub struct _IMAGE_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: DWORD,
    pub EndAddress: DWORD,
    pub u: IMAGE_RUNTIME_FUNCTION_ENTRY_u,
}

#[repr(C)]
pub union IMAGE_RUNTIME_FUNCTION_ENTRY_u {
    pub UnwindInfoAddress: DWORD,
    pub UnwindData: DWORD,
}

pub type _PIMAGE_RUNTIME_FUNCTION_ENTRY = *mut _IMAGE_RUNTIME_FUNCTION_ENTRY;
pub type IMAGE_RUNTIME_FUNCTION_ENTRY = _IMAGE_RUNTIME_FUNCTION_ENTRY;
pub type PRUNTIME_FUNCTION = *mut IMAGE_RUNTIME_FUNCTION_ENTRY;

pub const PROCESS_CREATE_FLAGS_LARGE_PAGE_SYSTEM_DLL: ULONG = 0x00000020;
pub const PROCESS_CREATE_FLAGS_PROTECTED_PROCESS: ULONG = 0x00000040;
pub const PROCESS_CREATE_FLAGS_CREATE_SESSION: ULONG = 0x00000080;
pub const PROCESS_CREATE_FLAGS_INHERIT_FROM_PARENT: ULONG = 0x00000100;
pub const PROCESS_CREATE_FLAGS_SUSPENDED: ULONG = 0x00000200;
pub const PROCESS_CREATE_FLAGS_EXTENDED_UNKNOWN: ULONG = 0x00000400;

pub const EXCEPTION_MAXIMUM_PARAMETERS: usize = 15;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: DWORD,
    pub ExceptionFlags: DWORD,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: PVOID,
    pub NumberParameters: DWORD,
    pub ExceptionInformation: [ULONG_PTR; EXCEPTION_MAXIMUM_PARAMETERS],
}
pub type PEXCEPTION_RECORD = *mut EXCEPTION_RECORD;

#[repr(C)]
pub struct EXCEPTION_RECORD32 {
    pub ExceptionCode: DWORD,
    pub ExceptionFlags: DWORD,
    pub ExceptionRecord: DWORD,
    pub ExceptionAddress: DWORD,
    pub NumberParameters: DWORD,
    pub ExceptionInformation: [DWORD; EXCEPTION_MAXIMUM_PARAMETERS],
}
pub type PEXCEPTION_RECORD32 = *mut EXCEPTION_RECORD32;

#[repr(C)]
pub struct EXCEPTION_RECORD64 {
    pub ExceptionCode: DWORD,
    pub ExceptionFlags: DWORD,
    pub ExceptionRecord: DWORD64,
    pub ExceptionAddress: DWORD64,
    pub NumberParameters: DWORD,
    pub __unusedAlignment: DWORD,
    pub ExceptionInformation: [DWORD64; EXCEPTION_MAXIMUM_PARAMETERS],
}
pub type PEXCEPTION_RECORD64 = *mut EXCEPTION_RECORD64;

#[repr(C)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: PEXCEPTION_RECORD,
    pub ContextRecord: PCONTEXT,
}

pub const SID_MAX_SUB_AUTHORITIES: usize = 15;
pub const SECURITY_MAX_SID_SIZE: usize = core::mem::size_of::<SID>() - core::mem::size_of::<u32>() + (SID_MAX_SUB_AUTHORITIES * core::mem::size_of::<u32>());

#[repr(C)]
pub struct SID {
    pub Revision: UCHAR,
    pub SubAuthorityCount: UCHAR,
    pub IdentifierAuthority: [UCHAR; 6],
    pub SubAuthority: [ULONG; 1],
}

pub type PCSID = *const SID;
pub type PISID = *mut SID;

pub type RTL_ATOM = USHORT;
pub type PRTL_ATOM = *mut RTL_ATOM;

#[repr(u32)]
pub enum EVENT_TYPE {
    NotificationEvent,
    SynchronizationEvent,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum POWER_ACTION {
    PowerActionNone = 0,
    PowerActionReserved = 1,
    PowerActionSleep = 2,
    PowerActionHibernate = 3,
    PowerActionShutdown = 4,
    PowerActionShutdownReset = 5,
    PowerActionShutdownOff = 6,
    PowerActionWarmEject = 7,
    PowerActionDisplayOff = 8,
}
pub type PPOWER_ACTION = *mut POWER_ACTION;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DEVICE_POWER_STATE {
    PowerDeviceUnspecified = 0,
    PowerDeviceD0 = 1,
    PowerDeviceD1 = 2,
    PowerDeviceD2 = 3,
    PowerDeviceD3 = 4,
    PowerDeviceMaximum = 5,
}
pub type PDEVICE_POWER_STATE = *mut DEVICE_POWER_STATE;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MONITOR_DISPLAY_STATE {
    PowerMonitorOff = 0,
    PowerMonitorOn = 1,
    PowerMonitorDim = 2,
}
pub type PMONITOR_DISPLAY_STATE = *mut MONITOR_DISPLAY_STATE;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum USER_ACTIVITY_PRESENCE {
    PowerUserPresent = 0,
    PowerUserNotPresent = 1,
    PowerUserInactive = 2,
    PowerUserMaximum = 3,
    // PowerUserInvalid = 3,
}
pub type PUSER_ACTIVITY_PRESENCE = *mut USER_ACTIVITY_PRESENCE;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ENERGY_SAVER_STATUS {
    ENERGY_SAVER_OFF = 0,
    ENERGY_SAVER_STANDARD = 1,
    ENERGY_SAVER_HIGH_SAVINGS = 2,
}
pub type PENERGY_SAVER_STATUS = *mut ENERGY_SAVER_STATUS;

pub const ES_SYSTEM_REQUIRED: u32 = 0x00000001;
pub const ES_DISPLAY_REQUIRED: u32 = 0x00000002;
pub const ES_USER_PRESENT: u32 = 0x00000004;
pub const ES_AWAYMODE_REQUIRED: u32 = 0x00000040;
pub const ES_CONTINUOUS: u32 = 0x80000000;

pub type EXECUTION_STATE = u32;
pub type PEXECUTION_STATE = *mut EXECUTION_STATE;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LATENCY_TIME {
    LT_DONT_CARE = 0,
    LT_LOWEST_LATENCY = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PORT_INFORMATION_CLASS {
    PortBasicInformation = 0,
    PortDumpInformation = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JOBOBJECTINFOCLASS {
    JobObjectBasicAccountingInformation = 1,
    JobObjectBasicLimitInformation = 2,
    JobObjectBasicProcessIdList = 3,
    JobObjectBasicUIRestrictions = 4,
    JobObjectSecurityLimitInformation = 5,
    JobObjectEndOfJobTimeInformation = 6,
    JobObjectAssociateCompletionPortInformation = 7,
    JobObjectBasicAndIoAccountingInformation = 8,
    JobObjectExtendedLimitInformation = 9,
    JobObjectJobSetInformation = 10,
    JobObjectGroupInformation = 11,
    JobObjectNotificationLimitInformation = 12,
    JobObjectLimitViolationInformation = 13,
    JobObjectGroupInformationEx = 14,
    JobObjectCpuRateControlInformation = 15,
    JobObjectCompletionFilter = 16,
    JobObjectCompletionCounter = 17,
    JobObjectReserved1Information = 18,
    JobObjectReserved2Information = 19,
    JobObjectReserved3Information = 20,
    JobObjectReserved4Information = 21,
    JobObjectReserved5Information = 22,
    JobObjectReserved6Information = 23,
    JobObjectReserved7Information = 24,
    JobObjectReserved8Information = 25,
    JobObjectReserved9Information = 26,
    JobObjectReserved10Information = 27,
    JobObjectReserved11Information = 28,
    JobObjectReserved12Information = 29,
    JobObjectReserved13Information = 30,
    JobObjectReserved14Information = 31,
    JobObjectNetRateControlInformation = 32,
    JobObjectNotificationLimitInformation2 = 33,
    JobObjectLimitViolationInformation2 = 34,
    JobObjectCreateSilo = 35,
    JobObjectSiloBasicInformation = 36,
    JobObjectReserved15Information = 37,
    JobObjectReserved16Information = 38,
    JobObjectReserved17Information = 39,
    JobObjectReserved18Information = 40,
    JobObjectReserved19Information = 41,
    JobObjectReserved20Information = 42,
    JobObjectReserved21Information = 43,
    JobObjectReserved22Information = 44,
    JobObjectReserved23Information = 45,
    JobObjectReserved24Information = 46,
    JobObjectReserved25Information = 47,
    JobObjectReserved26Information = 48,
    JobObjectReserved27Information = 49,
    JobObjectReserved28Information = 50,
    JobObjectNetworkAccountingInformation = 51,
    MaxJobObjectInfoClass = 52,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TIMER_TYPE {
    NotificationTimer = 0,
    SynchronizationTimer = 1,
}

pub type SE_SIGNING_LEVEL = u8;
pub type PSE_SIGNING_LEVEL = *mut u8;

pub const SE_SIGNING_LEVEL_UNCHECKED: u8 = 0x00;
pub const SE_SIGNING_LEVEL_UNSIGNED: u8 = 0x01;
pub const SE_SIGNING_LEVEL_ENTERPRISE: u8 = 0x02;
pub const SE_SIGNING_LEVEL_CUSTOM_1: u8 = 0x03;
pub const SE_SIGNING_LEVEL_DEVELOPER: u8 = 0x03;
pub const SE_SIGNING_LEVEL_AUTHENTICODE: u8 = 0x04;
pub const SE_SIGNING_LEVEL_CUSTOM_2: u8 = 0x05;
pub const SE_SIGNING_LEVEL_STORE: u8 = 0x06;
pub const SE_SIGNING_LEVEL_CUSTOM_3: u8 = 0x07;
pub const SE_SIGNING_LEVEL_ANTIMALWARE: u8 = 0x07;
pub const SE_SIGNING_LEVEL_MICROSOFT: u8 = 0x08;
pub const SE_SIGNING_LEVEL_CUSTOM_4: u8 = 0x09;
pub const SE_SIGNING_LEVEL_CUSTOM_5: u8 = 0x0A;
pub const SE_SIGNING_LEVEL_DYNAMIC_CODEGEN: u8 = 0x0B;
pub const SE_SIGNING_LEVEL_WINDOWS: u8 = 0x0C;
pub const SE_SIGNING_LEVEL_CUSTOM_7: u8 = 0x0D;
pub const SE_SIGNING_LEVEL_WINDOWS_TCB: u8 = 0x0E;
pub const SE_SIGNING_LEVEL_CUSTOM_6: u8 = 0x0F;

pub type WNF_CHANGE_STAMP = ULONG;
pub type PWNF_CHANGE_STAMP = *mut WNF_CHANGE_STAMP;

#[repr(C)]
pub struct WNF_TYPE_ID {
    pub TypeId: GUID,
}

pub type PWNF_TYPE_ID = *mut WNF_TYPE_ID;
pub type PCWNF_TYPE_ID = *const WNF_TYPE_ID;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SYSTEM_POWER_STATE {
    PowerSystemUnspecified = 0,
    PowerSystemWorking = 1,
    PowerSystemSleeping1 = 2,
    PowerSystemSleeping2 = 3,
    PowerSystemSleeping3 = 4,
    PowerSystemHibernate = 5,
    PowerSystemShutdown = 6,
    PowerSystemMaximum = 7,
}
pub type PSYSTEM_POWER_STATE = *mut SYSTEM_POWER_STATE;

pub const POWER_SYSTEM_MAXIMUM: u32 = 7;

pub type PT2_CANCEL_PARAMETERS = PVOID;
pub type SECURITY_CONTEXT_TRACKING_MODE = BOOLEAN;
pub type PSECURITY_CONTEXT_TRACKING_MODE = *mut SECURITY_CONTEXT_TRACKING_MODE;

#[repr(C)]
pub struct SECURITY_QUALITY_OF_SERVICE {
    pub Length: DWORD,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub ContextTrackingMode: SECURITY_CONTEXT_TRACKING_MODE,
    pub EffectiveOnly: BOOLEAN,
}
pub type PSECURITY_QUALITY_OF_SERVICE = *mut SECURITY_QUALITY_OF_SERVICE;

#[repr(C)]
pub struct SE_IMPERSONATION_STATE {
    pub Token: PACCESS_TOKEN,
    pub CopyOnOpen: BOOLEAN,
    pub EffectiveOnly: BOOLEAN,
    pub Level: SECURITY_IMPERSONATION_LEVEL,
}
pub type PSE_IMPERSONATION_STATE = *mut SE_IMPERSONATION_STATE;

#[repr(C)]
pub struct RTL_SRWLOCK {
    pub Ptr: PVOID,
}
pub type PRTL_SRWLOCK = *mut RTL_SRWLOCK;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AUDIT_EVENT_TYPE {
    AuditEventObjectAccess = 0,
    AuditEventDirectoryServiceAccess = 1,
}
pub type PAUDIT_EVENT_TYPE = *mut AUDIT_EVENT_TYPE;

#[repr(C)]
pub struct JOB_SET_ARRAY {
    pub JobHandle: HANDLE,
    pub MemberLevel: DWORD,
    pub Flags: DWORD,
}
pub type PJOB_SET_ARRAY = *mut JOB_SET_ARRAY;

#[repr(C)]
pub struct KTMOBJECT_CURSOR {
    pub LastQuery: GUID,
    pub ObjectIdCount: DWORD,
    pub ObjectIds: [GUID; 1],
}
pub type PKTMOBJECT_CURSOR = *mut KTMOBJECT_CURSOR;

pub type NOTIFICATION_MASK = ULONG;

pub const TRANSACTION_NOTIFY_MASK: ULONG = 0x3FFFFFFF;
pub const TRANSACTION_NOTIFY_PREPREPARE: ULONG = 0x00000001;
pub const TRANSACTION_NOTIFY_PREPARE: ULONG = 0x00000002;
pub const TRANSACTION_NOTIFY_COMMIT: ULONG = 0x00000004;
pub const TRANSACTION_NOTIFY_ROLLBACK: ULONG = 0x00000008;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: ULONG = 0x00000010;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: ULONG = 0x00000020;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: ULONG = 0x00000040;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: ULONG = 0x00000080;
pub const TRANSACTION_NOTIFY_RECOVER: ULONG = 0x00000100;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: ULONG = 0x00000200;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: ULONG = 0x00000400;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: ULONG = 0x00000800;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: ULONG = 0x00001000;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: ULONG = 0x00002000;
pub const TRANSACTION_NOTIFY_INDOUBT: ULONG = 0x00004000;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: ULONG = 0x00008000;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: ULONG = 0x00010000;
pub const TRANSACTION_NOTIFY_MARSHAL: ULONG = 0x00020000;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: ULONG = 0x00040000;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: ULONG = 0x01000000;
pub const TRANSACTION_NOTIFY_TM_ONLINE: ULONG = 0x02000000;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: ULONG = 0x04000000;
pub const TRANSACTION_NOTIFY_PROMOTE: ULONG = 0x08000000;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: ULONG = 0x10000000;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: ULONG = 0x20000000;

pub const CONTEXT_AMD64: u32 = 0x00100000;
pub const CONTEXT_CONTROL: u32 = CONTEXT_AMD64 | 0x00000001;
pub const CONTEXT_INTEGER: u32 = CONTEXT_AMD64 | 0x00000002;
pub const CONTEXT_SEGMENTS: u32 = CONTEXT_AMD64 | 0x00000004;
pub const CONTEXT_FULL: u32 = CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_SEGMENTS;