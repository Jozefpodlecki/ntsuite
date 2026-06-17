use core::ffi::c_void;
use core::ptr;

use crate::ntdef::*;
use crate::ntioapi::FILE_INFORMATION_CLASS;
use crate::ntpebteb::*;
use crate::ntpsapi::PINITIAL_TEB;

pub const RTL_IMAGE_MAX_DOS_HEADER: ULONG = 256 * 1024 * 1024;

pub const ANSI_DOS_STAR: CHAR = '<' as CHAR;
pub const ANSI_DOS_STAR_W: WCHAR = '<' as WCHAR;
pub const ANSI_DOS_QM: CHAR = '>' as CHAR;
pub const ANSI_DOS_QM_W: WCHAR = '>' as WCHAR;
pub const ANSI_DOS_DOT: CHAR = '"' as CHAR;
pub const ANSI_DOS_DOT_W: WCHAR = '"' as WCHAR;

pub const RTL_NANOSEC_PER_TICK: u64 = 100;
pub const RTL_TICKS_PER_MICROSEC: u64 = 10;
pub const RTL_TICKS_PER_MILLISEC: u64 = RTL_TICKS_PER_MICROSEC * 1000;
pub const RTL_TICKS_PER_SEC: u64 = RTL_TICKS_PER_MILLISEC * 1000;
pub const RTL_TICKS_PER_MIN: u64 = RTL_TICKS_PER_SEC * 60;
pub const RTL_TICKS_PER_HOUR: u64 = RTL_TICKS_PER_MIN * 60;
pub const RTL_TICKS_PER_DAY: u64 = RTL_TICKS_PER_HOUR * 24;
pub const RTL_TICKS_PER_WEEK: u64 = RTL_TICKS_PER_DAY * 7;
pub const RTL_TICKS_PER_MONTH: u64 = RTL_TICKS_PER_DAY * 30;
pub const RTL_TICKS_PER_YEAR: u64 = RTL_TICKS_PER_DAY * 365;
pub const RTL_TICKS_PER_LEAP_YEAR: u64 = RTL_TICKS_PER_DAY * 366;

pub const RTL_NANOSEC_PER_SEC: u64 = 1_000_000_000;
pub const RTL_NANOSEC_PER_MILLISEC: u64 = 1_000_000;
pub const RTL_100NANOSEC_PER_SEC: u64 = 10_000_000;
pub const RTL_100NANOSEC_PER_MILLISEC: u64 = 10_000;
pub const RTL_MILLISEC_PER_SEC: u64 = 1_000;

pub const RTL_SEC_PER_HOUR: u64 = 3_600;
pub const RTL_SEC_PER_DAY: u64 = 86_400;
pub const RTL_SEC_PER_WEEK: u64 = 604_800;
pub const RTL_SEC_PER_MONTH: u64 = 2_592_000;
pub const RTL_SEC_PER_YEAR: u64 = 31_536_000;

#[macro_export]
macro_rules! RTL_NUM_ALIGN_DOWN {
    ($Number:expr, $Alignment:expr) => {
        ($Number) - (($Number) & (($Alignment) - 1))
    };
}

#[macro_export]
macro_rules! RTL_NUM_ALIGN_UP {
    ($Number:expr, $Alignment:expr) => {
        $crate::RTL_NUM_ALIGN_DOWN!(($Number) + ($Alignment) - 1, $Alignment)
    };
}

#[macro_export]
macro_rules! RTL_IS_POWER_OF_TWO {
    ($Value:expr) => {
        ($Value != 0) && (($Value & ($Value - 1)) == 0)
    };
}

#[macro_export]
macro_rules! RTL_OFFSET_TO_POINTER {
    ($Base:expr, $Offset:expr) => {
        ((($Base) as *const u8).wrapping_add(($Offset) as usize)) as *mut core::ffi::c_void
    };
}

#[macro_export]
macro_rules! RTL_POINTER_TO_OFFSET {
    ($Base:expr, $Pointer:expr) => {
        (($Pointer) as usize - ($Base) as usize) as ULONG
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SLIST_HEADER {
    pub Alignment: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SLIST_ENTRY {
    pub Next: *mut SLIST_ENTRY,
}

pub type PLIST_ENTRY = *mut LIST_ENTRY;
pub type PSINGLE_LIST_ENTRY = *mut SINGLE_LIST_ENTRY;
pub type PSLIST_HEADER = *mut SLIST_HEADER;
pub type PSLIST_ENTRY = *mut SLIST_ENTRY;

#[inline]
pub fn InitializeListHead(ListHead: PLIST_ENTRY) {
    unsafe {
        (*ListHead).Flink = ListHead;
        (*ListHead).Blink = ListHead;
    }
}

pub fn IsListEmpty(ListHead: PLIST_ENTRY) -> BOOLEAN {
    unsafe { ((*ListHead).Flink == ListHead) as BOOLEAN }
}

#[inline]
pub fn RemoveEntryList(Entry: PLIST_ENTRY) -> BOOLEAN {
    unsafe {
        let NextEntry = (*Entry).Flink;
        let PrevEntry = (*Entry).Blink;
        (*PrevEntry).Flink = NextEntry;
        (*NextEntry).Blink = PrevEntry;
        (NextEntry == PrevEntry) as BOOLEAN
    }
}

#[inline]
pub unsafe fn RemoveHeadList(ListHead: PLIST_ENTRY) -> PLIST_ENTRY {
    let Entry = (*ListHead).Flink;
    let NextEntry = (*Entry).Flink;
    (*ListHead).Flink = NextEntry;
    (*NextEntry).Blink = ListHead;
    Entry
}

#[inline]
pub unsafe fn RemoveTailList(ListHead: PLIST_ENTRY) -> PLIST_ENTRY {
    let Entry = (*ListHead).Blink;
    let PrevEntry = (*Entry).Blink;
    (*ListHead).Blink = PrevEntry;
    (*PrevEntry).Flink = ListHead;
    Entry
}

#[inline]
pub unsafe fn InsertHeadList(ListHead: PLIST_ENTRY, Entry: PLIST_ENTRY) {
    let NextEntry = (*ListHead).Flink;
    (*Entry).Flink = NextEntry;
    (*Entry).Blink = ListHead;
    (*NextEntry).Blink = Entry;
    (*ListHead).Flink = Entry;
}

#[inline]
pub unsafe fn InsertTailList(ListHead: PLIST_ENTRY, Entry: PLIST_ENTRY) {
    let PrevEntry = (*ListHead).Blink;
    (*Entry).Flink = ListHead;
    (*Entry).Blink = PrevEntry;
    (*PrevEntry).Flink = Entry;
    (*ListHead).Blink = Entry;
}

#[inline]
pub unsafe fn PushEntryList(ListHead: PSINGLE_LIST_ENTRY, Entry: PSINGLE_LIST_ENTRY) {
    (*Entry).Next = (*ListHead).Next;
    (*ListHead).Next = Entry;
}

#[inline]
pub unsafe fn PopEntryList(ListHead: PSINGLE_LIST_ENTRY) -> PSINGLE_LIST_ENTRY {
    let FirstEntry = (*ListHead).Next;
    if !FirstEntry.is_null() {
        (*ListHead).Next = (*FirstEntry).Next;
    }
    FirstEntry
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_BALANCED_LINKS {
    pub Parent: *mut RTL_BALANCED_LINKS,
    pub LeftChild: *mut RTL_BALANCED_LINKS,
    pub RightChild: *mut RTL_BALANCED_LINKS,
    pub Balance: CHAR,
    pub Reserved: [UCHAR; 3],
}

pub type PRTL_BALANCED_LINKS = *mut RTL_BALANCED_LINKS;
pub type PRTL_BALANCED_NODE = *mut RTL_BALANCED_LINKS;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_AVL_TABLE {
    pub BalancedRoot: RTL_BALANCED_LINKS,
    pub OrderedPointer: PVOID,
    pub WhichOrderedElement: ULONG,
    pub NumberGenericTableElements: ULONG,
    pub DepthOfTree: ULONG,
    pub RestartKey: PRTL_BALANCED_LINKS,
    pub DeleteCount: ULONG,
    pub CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_AVL_FREE_ROUTINE,
    pub TableContext: PVOID,
}

pub type PRTL_AVL_TABLE = *mut RTL_AVL_TABLE;

pub type PRTL_AVL_COMPARE_ROUTINE = Option<unsafe extern "system" fn(Table: PRTL_AVL_TABLE, FirstStruct: PVOID, SecondStruct: PVOID) -> RTL_GENERIC_COMPARE_RESULTS>;
pub type PRTL_AVL_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(Table: PRTL_AVL_TABLE, ByteSize: CLONG) -> PVOID>;
pub type PRTL_AVL_FREE_ROUTINE = Option<unsafe extern "system" fn(Table: PRTL_AVL_TABLE, Buffer: PVOID)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_SPLAY_LINKS {
    pub Parent: *mut RTL_SPLAY_LINKS,
    pub LeftChild: *mut RTL_SPLAY_LINKS,
    pub RightChild: *mut RTL_SPLAY_LINKS,
}

pub type PRTL_SPLAY_LINKS = *mut RTL_SPLAY_LINKS;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_GENERIC_TABLE {
    pub TableRoot: PRTL_SPLAY_LINKS,
    pub InsertOrderList: LIST_ENTRY,
    pub OrderedPointer: PLIST_ENTRY,
    pub WhichOrderedElement: ULONG,
    pub NumberGenericTableElements: ULONG,
    pub CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
    pub TableContext: PVOID,
}

pub type PRTL_GENERIC_TABLE = *mut RTL_GENERIC_TABLE;
pub type PRTL_GENERIC_COMPARE_ROUTINE = Option<unsafe extern "system" fn(Table: PRTL_GENERIC_TABLE, FirstStruct: PVOID, SecondStruct: PVOID) -> RTL_GENERIC_COMPARE_RESULTS>;
pub type PRTL_GENERIC_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(Table: PRTL_GENERIC_TABLE, ByteSize: CLONG) -> PVOID>;
pub type PRTL_GENERIC_FREE_ROUTINE = Option<unsafe extern "system" fn(Table: PRTL_GENERIC_TABLE, Buffer: PVOID)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_RB_TREE {
    pub Root: PRTL_BALANCED_NODE,
    pub Min: PRTL_BALANCED_NODE,
}

pub type PRTL_RB_TREE = *mut RTL_RB_TREE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENTRY {
    pub Linkage: LIST_ENTRY,
    pub Signature: ULONG_PTR,
}

pub type PRTL_DYNAMIC_HASH_TABLE_ENTRY = *mut RTL_DYNAMIC_HASH_TABLE_ENTRY;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_DYNAMIC_HASH_TABLE_CONTEXT {
    pub ChainHead: PLIST_ENTRY,
    pub PrevLinkage: PLIST_ENTRY,
    pub Signature: ULONG_PTR,
}

pub type PRTL_DYNAMIC_HASH_TABLE_CONTEXT = *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    pub HashEntry: RTL_DYNAMIC_HASH_TABLE_ENTRY,
    pub ChainHead: PLIST_ENTRY,
    pub BucketIndex: ULONG,
}

pub type PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR = *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_DYNAMIC_HASH_TABLE {
    pub Flags: ULONG,
    pub Shift: ULONG,
    pub TableSize: ULONG,
    pub Pivot: ULONG,
    pub DivisorMask: ULONG,
    pub NumEntries: ULONG,
    pub NonEmptyBuckets: ULONG,
    pub NumEnumerators: ULONG,
    pub Directory: PVOID,
}

pub type PRTL_DYNAMIC_HASH_TABLE = *mut RTL_DYNAMIC_HASH_TABLE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: USHORT,
    pub CreatorBackTraceIndex: USHORT,
    pub CriticalSection: *mut RTL_CRITICAL_SECTION,
    pub ProcessLocksList: LIST_ENTRY,
    pub EntryCount: ULONG,
    pub ContentionCount: ULONG,
    pub Flags: ULONG,
    pub CreatorBackTraceIndexHigh: USHORT,
    pub Identifier: USHORT,
}

pub type PRTL_CRITICAL_SECTION_DEBUG = *mut RTL_CRITICAL_SECTION_DEBUG;

#[repr(C, packed(8))]
#[derive(Copy, Clone)]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: PRTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: LONG,
    pub RecursionCount: LONG,
    pub OwningThread: HANDLE,
    pub LockSemaphore: HANDLE,
    pub SpinCount: ULONG_PTR,
}

pub type PRTL_CRITICAL_SECTION = *mut RTL_CRITICAL_SECTION;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: PVOID,
}

pub type PRTL_CONDITION_VARIABLE = *mut RTL_CONDITION_VARIABLE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_BARRIER {
    pub Reserved: [ULONG; 4],
}

pub type PRTL_BARRIER = *mut RTL_BARRIER;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PCHAR,
}

pub type PSTRING = *mut STRING;
pub type PANSI_STRING = *mut STRING;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OEM_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PCHAR,
}

pub type POEM_STRING = *mut OEM_STRING;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UTF8_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PCHAR,
}

pub type PUTF8_STRING = *mut UTF8_STRING;
pub type PCUTF8_STRING = *const UTF8_STRING;

pub const UNICODE_STRING_MAX_BYTES: WORD = 65534;
pub const UNICODE_STRING_MAX_CHARS: i32 = 32767;

#[inline]
pub unsafe fn RtlInitEmptyUnicodeString(DestinationString: PUNICODE_STRING, Buffer: PWSTR, MaximumLength: USHORT) {
    (*DestinationString).Length = 0;
    (*DestinationString).MaximumLength = MaximumLength;
    (*DestinationString).Buffer = Buffer;
}

#[inline]
pub unsafe fn RtlInitUnicodeString(DestinationString: PUNICODE_STRING, SourceString: PCWSTR) {
    if !SourceString.is_null() {
        let mut len = 0;
        while *SourceString.add(len) != 0 {
            len += 1;
        }
        let bytes = (len * core::mem::size_of::<WCHAR>()) as USHORT;
        (*DestinationString).Length = bytes;
        (*DestinationString).MaximumLength = bytes + core::mem::size_of::<WCHAR>() as USHORT;
        (*DestinationString).Buffer = SourceString as PWSTR;
    } else {
        (*DestinationString).Length = 0;
        (*DestinationString).MaximumLength = 0;
        (*DestinationString).Buffer = ptr::null_mut();
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CURDIR {
    pub DosPath: UNICODE_STRING,
    pub Handle: HANDLE,
}

pub type PCURDIR = *mut CURDIR;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_DRIVE_LETTER_CURDIR {
    pub Flags: USHORT,
    pub Length: USHORT,
    pub TimeStamp: ULONG,
    pub DosPath: STRING,
}

pub type PRTL_DRIVE_LETTER_CURDIR = *mut RTL_DRIVE_LETTER_CURDIR;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub MaximumLength: ULONG,
    pub Length: ULONG,
    pub Flags: ULONG,
    pub DebugFlags: ULONG,
    pub ConsoleHandle: HANDLE,
    pub ConsoleFlags: ULONG,
    pub StandardInput: HANDLE,
    pub StandardOutput: HANDLE,
    pub StandardError: HANDLE,
    pub CurrentDirectory: CURDIR,
    pub DllPath: UNICODE_STRING,
    pub ImagePathName: UNICODE_STRING,
    pub CommandLine: UNICODE_STRING,
    pub Environment: PVOID,
    pub StartingX: ULONG,
    pub StartingY: ULONG,
    pub CountX: ULONG,
    pub CountY: ULONG,
    pub CountCharsX: ULONG,
    pub CountCharsY: ULONG,
    pub FillAttribute: ULONG,
    pub WindowFlags: ULONG,
    pub ShowWindowFlags: ULONG,
    pub WindowTitle: UNICODE_STRING,
    pub DesktopInfo: UNICODE_STRING,
    pub ShellInfo: UNICODE_STRING,
    pub RuntimeData: UNICODE_STRING,
    pub CurrentDirectories: [RTL_DRIVE_LETTER_CURDIR; 32],
    pub EnvironmentSize: ULONG_PTR,
    pub EnvironmentVersion: ULONG_PTR,
    pub PackageDependencyData: PVOID,
    pub ProcessGroupId: ULONG,
    pub LoaderThreads: ULONG,
    pub RedirectionDllName: UNICODE_STRING,
    pub HeapPartitionName: UNICODE_STRING,
    pub DefaultThreadpoolCpuSetMasks: PULONGLONG,
    pub DefaultThreadpoolCpuSetMaskCount: ULONG,
    pub DefaultThreadpoolThreadMaximum: ULONG,
    pub HeapMemoryTypeMask: ULONG,
}

pub type PRTL_USER_PROCESS_PARAMETERS = *mut RTL_USER_PROCESS_PARAMETERS;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_USER_PROCESS_INFORMATION {
    pub Length: ULONG,
    pub ProcessHandle: HANDLE,
    pub ThreadHandle: HANDLE,
    pub ClientId: CLIENT_ID,
    pub ImageInformation: SECTION_IMAGE_INFORMATION,
}

pub type PRTL_USER_PROCESS_INFORMATION = *mut RTL_USER_PROCESS_INFORMATION;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_UNLOAD_EVENT_TRACE {
    pub BaseAddress: PVOID,
    pub SizeOfImage: SIZE_T,
    pub Sequence: ULONG,
    pub TimeDateStamp: ULONG,
    pub CheckSum: ULONG,
    pub ImageName: [WCHAR; 32],
    pub Version: [ULONG; 2],
}

pub type PRTL_UNLOAD_EVENT_TRACE = *mut RTL_UNLOAD_EVENT_TRACE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_BITMAP {
    pub SizeOfBitMap: ULONG,
    pub Buffer: PULONG,
}

pub type PRTL_BITMAP = *mut RTL_BITMAP;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_BITMAP_RUN {
    pub StartingIndex: ULONG,
    pub NumberOfBits: ULONG,
}

pub type PRTL_BITMAP_RUN = *mut RTL_BITMAP_RUN;

pub type PFILE_INFORMATION_CLASS = *mut FILE_INFORMATION_CLASS;
// pub type PSECURITY_DESCRIPTOR = PVOID;
pub type PACL = PVOID;
pub type PACCESS_MASK = *mut ACCESS_MASK;
pub type PGENERIC_MAPPING = *mut GENERIC_MAPPING;
pub type POBJECT_TYPE_LIST = *mut OBJECT_TYPE_LIST;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GENERIC_MAPPING {
    pub GenericRead: ACCESS_MASK,
    pub GenericWrite: ACCESS_MASK,
    pub GenericExecute: ACCESS_MASK,
    pub GenericAll: ACCESS_MASK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OBJECT_TYPE_LIST {
    pub Level: WORD,
    pub Sbz: WORD,
    pub ObjectType: *mut GUID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: PWSTR,
    pub ValueType: WORD,
    pub Reserved: WORD,
    pub Flags: DWORD,
    pub ValueCount: DWORD,
    pub Values: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}

pub type PCLAIM_SECURITY_ATTRIBUTE_V1 = *mut CLAIM_SECURITY_ATTRIBUTE_V1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: PVOID,
    pub ValueLength: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: WORD,
    pub Reserved: WORD,
    pub AttributeCount: DWORD,
    pub Attribute: PCLAIM_SECURITY_ATTRIBUTE_V1,
}

pub type PCLAIM_SECURITY_ATTRIBUTES_INFORMATION = *mut CLAIM_SECURITY_ATTRIBUTES_INFORMATION;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct COMPRESSED_DATA_INFO {
    pub CompressionFormatAndEngine: USHORT,
    pub CompressionUnitShift: UCHAR,
    pub ChunkShift: UCHAR,
    pub ClusterShift: UCHAR,
    pub Reserved: UCHAR,
    pub NumberOfChunks: USHORT,
    pub CompressedChunkSizes: [ULONG; 0],
}

pub type PCOMPRESSED_DATA_INFO = *mut COMPRESSED_DATA_INFO;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TIME_FIELDS {
    pub Year: CSHORT,
    pub Month: CSHORT,
    pub Day: CSHORT,
    pub Hour: CSHORT,
    pub Minute: CSHORT,
    pub Second: CSHORT,
    pub Milliseconds: CSHORT,
    pub Weekday: CSHORT,
}

pub type PTIME_FIELDS = *mut TIME_FIELDS;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_TIME_ZONE_INFORMATION {
    pub Bias: LONG,
    pub StandardName: [WCHAR; 32],
    pub StandardStart: TIME_FIELDS,
    pub StandardBias: LONG,
    pub DaylightName: [WCHAR; 32],
    pub DaylightStart: TIME_FIELDS,
    pub DaylightBias: LONG,
}

pub type PRTL_TIME_ZONE_INFORMATION = *mut RTL_TIME_ZONE_INFORMATION;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTLP_CURDIR_REF {
    pub ReferenceCount: LONG,
    pub DirectoryHandle: HANDLE,
}

pub type PRTLP_CURDIR_REF = *mut RTLP_CURDIR_REF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_RELATIVE_NAME_U {
    pub RelativeName: UNICODE_STRING,
    pub ContainingDirectory: HANDLE,
    pub CurDirRef: PRTLP_CURDIR_REF,
}

pub type PRTL_RELATIVE_NAME_U = *mut RTL_RELATIVE_NAME_U;

pub type RTL_PATH_TYPE = ULONG;

pub const RtlPathTypeUnknown: RTL_PATH_TYPE = 0;
pub const RtlPathTypeUncAbsolute: RTL_PATH_TYPE = 1;
pub const RtlPathTypeDriveAbsolute: RTL_PATH_TYPE = 2;
pub const RtlPathTypeDriveRelative: RTL_PATH_TYPE = 3;
pub const RtlPathTypeRooted: RTL_PATH_TYPE = 4;
pub const RtlPathTypeRelative: RTL_PATH_TYPE = 5;
pub const RtlPathTypeLocalDevice: RTL_PATH_TYPE = 6;
pub const RtlPathTypeRootLocalDevice: RTL_PATH_TYPE = 7;

pub type RTL_GENERIC_COMPARE_RESULTS = ULONG;

pub const GenericLessThan: RTL_GENERIC_COMPARE_RESULTS = 0;
pub const GenericGreaterThan: RTL_GENERIC_COMPARE_RESULTS = 1;
pub const GenericEqual: RTL_GENERIC_COMPARE_RESULTS = 2;

pub type TABLE_SEARCH_RESULT = ULONG;

pub const TableEmptyTree: TABLE_SEARCH_RESULT = 0;
pub const TableFoundNode: TABLE_SEARCH_RESULT = 1;
pub const TableInsertAsLeft: TABLE_SEARCH_RESULT = 2;
pub const TableInsertAsRight: TABLE_SEARCH_RESULT = 3;

pub type HEAP_INFORMATION_CLASS = ULONG;

pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = 0;
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = 1;
pub const HeapExtendedInformation: HEAP_INFORMATION_CLASS = 2;
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = 3;
pub const HeapTaggingInformation: HEAP_INFORMATION_CLASS = 4;
pub const HeapStackDatabase: HEAP_INFORMATION_CLASS = 5;
pub const HeapMemoryLimit: HEAP_INFORMATION_CLASS = 6;
pub const HeapTag: HEAP_INFORMATION_CLASS = 7;


#[inline]
pub unsafe fn RtlGetCurrentPeb() -> PPEB {
    let teb: *mut TEB;
    core::arch::asm!("mov {}, gs:[0x30]", out(reg) teb);
    (*teb).ProcessEnvironmentBlock
}

#[inline]
pub unsafe fn RtlProcessHeap() -> HANDLE {
    (*RtlGetCurrentPeb()).ProcessHeap
}

#[inline]
pub unsafe fn RtlEqualLuid(L1: PLUID, L2: PLUID) -> BOOLEAN {
    ((*L1).LowPart == (*L2).LowPart && (*L1).HighPart == (*L2).HighPart) as BOOLEAN
}

#[inline]
pub unsafe fn RtlIsZeroLuid(L1: PLUID) -> BOOLEAN {
    (((*L1).LowPart | (*L1).HighPart as u32) == 0) as BOOLEAN
}

#[inline]
pub fn RtlConvertLongToLuid(Long: LONG) -> LUID {
    LUID {
        LowPart: Long as DWORD,
        HighPart: 0,
    }
}

#[inline]
pub fn RtlConvertUlongToLuid(Ulong: ULONG) -> LUID {
    LUID {
        LowPart: Ulong,
        HighPart: 0,
    }
}

unsafe extern "system" {
    pub fn RtlCreateEnvironmentEx(
        SourceEnv: PVOID,
        Environment: *mut PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn RtlDestroyEnvironment(
        Environment: PVOID,
    ) -> NTSTATUS;

    pub fn RtlSetCurrentEnvironment(
        Environment: PVOID,
        PreviousEnvironment: *mut PVOID,
    ) -> NTSTATUS;

    pub fn RtlSetEnvironmentVar(
        Environment: *mut PWSTR,
        Name: PWSTR,
        NameLength: SIZE_T,
        Value: PWSTR,
        ValueLength: SIZE_T,
    ) -> NTSTATUS;

    pub fn RtlSetEnvironmentVariable(
        Environment: *mut PVOID,
        Name: PUNICODE_STRING,
        Value: PUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn RtlQueryEnvironmentVariable(
        Environment: PVOID,
        Name: PWSTR,
        NameLength: SIZE_T,
        Value: PWSTR,
        ValueLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;

    pub fn RtlQueryEnvironmentVariable_U(
        Environment: PVOID,
        Name: PUNICODE_STRING,
        Value: PUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn RtlExpandEnvironmentStrings(
        Environment: PVOID,
        Src: PWSTR,
        SrcLength: SIZE_T,
        Dst: PWSTR,
        DstLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;

    pub fn RtlExpandEnvironmentStrings_U(
        Environment: PVOID,
        Source: PUNICODE_STRING,
        Destination: PUNICODE_STRING,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;

    pub fn RtlSetEnvironmentStrings(
        NewEnvironment: PWSTR,
        NewEnvironmentSize: SIZE_T,
    ) -> NTSTATUS;

    pub fn RtlInitializeGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
        AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
        FreeRoutine: PRTL_AVL_FREE_ROUTINE,
        TableContext: PVOID,
    );

    pub fn RtlInsertElementGenericTableAvl(
        Table: PRTL_AVL_TABLE,
        Buffer: PVOID,
        BufferSize: CLONG,
        NewElement: PBOOLEAN,
    ) -> PVOID;

    pub fn RtlDeleteElementGenericTableAvl(Table: PRTL_AVL_TABLE, Buffer: PVOID) -> BOOLEAN;

    pub fn RtlLookupElementGenericTableAvl(Table: PRTL_AVL_TABLE, Buffer: PVOID) -> PVOID;

    pub fn RtlNumberGenericTableElementsAvl(Table: PRTL_AVL_TABLE) -> ULONG;

    pub fn RtlIsGenericTableEmptyAvl(Table: PRTL_AVL_TABLE) -> BOOLEAN;

    pub fn RtlSplay(Links: PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS;

    pub fn RtlDelete(Links: PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS;

    pub fn RtlInitializeGenericTable(
        Table: PRTL_GENERIC_TABLE,
        CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
        AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
        FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
        TableContext: PVOID,
    );

    pub fn RtlInsertElementGenericTable(
        Table: PRTL_GENERIC_TABLE,
        Buffer: PVOID,
        BufferSize: CLONG,
        NewElement: PBOOLEAN,
    ) -> PVOID;

    pub fn RtlDeleteElementGenericTable(Table: PRTL_GENERIC_TABLE, Buffer: PVOID) -> BOOLEAN;

    pub fn RtlLookupElementGenericTable(Table: PRTL_GENERIC_TABLE, Buffer: PVOID) -> PVOID;

    pub fn RtlNumberGenericTableElements(Table: PRTL_GENERIC_TABLE) -> ULONG;

    pub fn RtlIsGenericTableEmpty(Table: PRTL_GENERIC_TABLE) -> BOOLEAN;

    pub fn RtlCreateHashTable(HashTable: *mut PRTL_DYNAMIC_HASH_TABLE, Shift: ULONG, Flags: ULONG) -> BOOLEAN;

    pub fn RtlDeleteHashTable(HashTable: PRTL_DYNAMIC_HASH_TABLE) -> LOGICAL;

    pub fn RtlInsertEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Entry: PRTL_DYNAMIC_HASH_TABLE_ENTRY,
        Signature: ULONG_PTR,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> BOOLEAN;

    pub fn RtlRemoveEntryHashTable(
        HashTable: PRTL_DYNAMIC_HASH_TABLE,
        Entry: PRTL_DYNAMIC_HASH_TABLE_ENTRY,
        Context: PRTL_DYNAMIC_HASH_TABLE_CONTEXT,
    ) -> BOOLEAN;

    pub fn RtlInitializeCriticalSection(CriticalSection: PRTL_CRITICAL_SECTION) -> NTSTATUS;

    pub fn RtlInitializeCriticalSectionAndSpinCount(
        CriticalSection: PRTL_CRITICAL_SECTION,
        SpinCount: ULONG,
    ) -> NTSTATUS;

    pub fn RtlInitializeCriticalSectionEx(
        CriticalSection: PRTL_CRITICAL_SECTION,
        SpinCount: ULONG,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn RtlDeleteCriticalSection(CriticalSection: PRTL_CRITICAL_SECTION) -> NTSTATUS;

    pub fn RtlEnterCriticalSection(CriticalSection: PRTL_CRITICAL_SECTION) -> NTSTATUS;

    pub fn RtlLeaveCriticalSection(CriticalSection: PRTL_CRITICAL_SECTION) -> NTSTATUS;

    pub fn RtlTryEnterCriticalSection(CriticalSection: PRTL_CRITICAL_SECTION) -> LOGICAL;

    pub fn RtlInitializeSRWLock(SRWLock: PRTL_SRWLOCK);

    pub fn RtlAcquireSRWLockExclusive(SRWLock: PRTL_SRWLOCK);

    pub fn RtlAcquireSRWLockShared(SRWLock: PRTL_SRWLOCK);

    pub fn RtlReleaseSRWLockExclusive(SRWLock: PRTL_SRWLOCK);

    pub fn RtlReleaseSRWLockShared(SRWLock: PRTL_SRWLOCK);

    pub fn RtlTryAcquireSRWLockExclusive(SRWLock: PRTL_SRWLOCK) -> BOOLEAN;

    pub fn RtlTryAcquireSRWLockShared(SRWLock: PRTL_SRWLOCK) -> BOOLEAN;

    pub fn RtlInitializeConditionVariable(ConditionVariable: PRTL_CONDITION_VARIABLE);

    pub fn RtlWakeConditionVariable(ConditionVariable: PRTL_CONDITION_VARIABLE);

    pub fn RtlWakeAllConditionVariable(ConditionVariable: PRTL_CONDITION_VARIABLE);

    pub fn RtlWaitOnAddress(
        Address: *const VOID,
        CompareAddress: PVOID,
        AddressSize: SIZE_T,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn RtlWakeAddressAll(Address: PVOID);

    pub fn RtlWakeAddressSingle(Address: PVOID);

    pub fn RtlInitString(DestinationString: PSTRING, SourceString: PCSTR);

    pub fn RtlInitAnsiString(DestinationString: PANSI_STRING, SourceString: PCSTR);

    pub fn RtlFreeAnsiString(AnsiString: PANSI_STRING);

    pub fn RtlFreeUnicodeString(UnicodeString: PUNICODE_STRING);

    pub fn RtlCopyUnicodeString(DestinationString: PCUNICODE_STRING, SourceString: PCUNICODE_STRING);

    pub fn RtlCompareUnicodeString(
        String1: PCUNICODE_STRING,
        String2: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> LONG;

    pub fn RtlEqualUnicodeString(
        String1: PCUNICODE_STRING,
        String2: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> BOOLEAN;

    pub fn RtlPrefixUnicodeString(
        String1: PCUNICODE_STRING,
        String2: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
    ) -> BOOLEAN;

    pub fn RtlAppendUnicodeStringToString(Destination: PUNICODE_STRING, Source: PCUNICODE_STRING) -> NTSTATUS;

    pub fn RtlAppendUnicodeToString(Destination: PUNICODE_STRING, Source: PCWSTR) -> NTSTATUS;

    pub fn RtlUpcaseUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlDowncaseUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlAnsiStringToUnicodeString(
        DestinationString: PUNICODE_STRING,
        SourceString: PCANSI_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlUnicodeStringToAnsiString(
        DestinationString: PANSI_STRING,
        SourceString: PCUNICODE_STRING,
        AllocateDestinationString: BOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlUnicodeStringToInteger(String: PCUNICODE_STRING, Base: ULONG, Value: PULONG) -> NTSTATUS;

    pub fn RtlIntegerToUnicodeString(Value: ULONG, Base: ULONG, String: PUNICODE_STRING) -> NTSTATUS;

    pub fn RtlGetFullPathName_U(
        FileName: PCWSTR,
        BufferLength: ULONG,
        Buffer: PWSTR,
        FilePart: *mut PWSTR,
    ) -> ULONG;

    pub fn RtlSetCurrentDirectory_U(PathName: PCUNICODE_STRING) -> NTSTATUS;

    pub fn RtlGetCurrentDirectory_U(BufferLength: ULONG, Buffer: PWSTR) -> ULONG;

    pub fn RtlDosPathNameToNtPathName_U(
        DosFileName: PCWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> BOOLEAN;

    pub fn RtlNtPathNameToDosPathName(
        Flags: ULONG,
        Path: PRTL_UNICODE_STRING_BUFFER,
        Disposition: PULONG,
        FilePart: *mut PWSTR,
    ) -> NTSTATUS;

    pub fn RtlReleaseRelativeName(RelativeName: PRTL_RELATIVE_NAME_U);

    pub fn RtlCreateHeap(
        Flags: ULONG,
        HeapBase: PVOID,
        ReserveSize: SIZE_T,
        CommitSize: SIZE_T,
        Lock: PVOID,
        Parameters: PVOID,
    ) -> HANDLE;

    pub fn RtlDestroyHeap(HeapHandle: HANDLE) -> PVOID;
    pub fn RtlAllocateHeap(HeapHandle: HANDLE, Flags: ULONG, Size: SIZE_T) -> PVOID;
    pub fn RtlFreeHeap(HeapHandle: HANDLE, Flags: ULONG, BaseAddress: PVOID) -> BOOLEAN;
    pub fn RtlSizeHeap(HeapHandle: HANDLE, Flags: ULONG, BaseAddress: PCVOID) -> SIZE_T;
    pub fn RtlReAllocateHeap(HeapHandle: HANDLE, Flags: ULONG, BaseAddress: PVOID, Size: SIZE_T) -> PVOID;
    pub fn RtlLockHeap(HeapHandle: HANDLE) -> BOOLEAN;
    pub fn RtlUnlockHeap(HeapHandle: HANDLE) -> BOOLEAN;
    pub fn RtlValidateHeap(HeapHandle: HANDLE, Flags: ULONG, BaseAddress: PVOID) -> BOOLEAN;
    pub fn RtlGetProcessHeaps(NumberOfHeaps: ULONG, ProcessHeaps: *mut PVOID) -> ULONG;
    pub fn RtlWalkHeap(HeapHandle: HANDLE, Entry: *mut RTL_HEAP_WALK_ENTRY) -> NTSTATUS;
    pub fn RtlQueryHeapInformation(
        HeapHandle: HANDLE,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: PVOID,
        HeapInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
    pub fn RtlSetHeapInformation(
        HeapHandle: HANDLE,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: PVOID,
        HeapInformationLength: SIZE_T,
    ) -> NTSTATUS;
    pub fn RtlCreateUserProcess(
        NtImagePathName: PCUNICODE_STRING,
        ExtendedParameters: ULONG,
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
        ProcessSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ParentProcess: HANDLE,
        InheritHandles: BOOLEAN,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
        ProcessInformation: PRTL_USER_PROCESS_INFORMATION,
    ) -> NTSTATUS;
    pub fn RtlExitUserProcess(ExitStatus: NTSTATUS);
    pub fn RtlCreateUserThread(
        ProcessHandle: HANDLE,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        CreateSuspended: BOOLEAN,
        ZeroBits: ULONG,
        MaximumStackSize: SIZE_T,
        CommittedStackSize: SIZE_T,
        StartAddress: PUSER_THREAD_START_ROUTINE,
        Parameter: PVOID,
        ThreadHandle: PHANDLE,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    pub fn RtlExitUserThread(ExitStatus: NTSTATUS);
    pub fn RtlImageNtHeader(BaseOfImage: PVOID) -> PIMAGE_NT_HEADERS;
    pub fn RtlImageDirectoryEntryToData(
        BaseOfImage: PVOID,
        MappedAsImage: BOOLEAN,
        DirectoryEntry: USHORT,
        Size: PULONG,
    ) -> PVOID;
    pub fn RtlImageRvaToVa(
        NtHeaders: PIMAGE_NT_HEADERS,
        BaseOfImage: PVOID,
        Rva: ULONG,
        LastRvaSection: *mut PIMAGE_SECTION_HEADER,
    ) -> PVOID;
    pub fn RtlCompareMemory(Source1: *const VOID, Source2: *const VOID, Length: SIZE_T) -> SIZE_T;
    pub fn RtlZeroMemory(Destination: PVOID, Length: SIZE_T);
    pub fn RtlFillMemory(Destination: PVOID, Length: SIZE_T, Fill: UCHAR);
    pub fn RtlMoveMemory(Destination: PVOID, Source: *const VOID, Length: SIZE_T);
    pub fn RtlCopyMemory(Destination: PVOID, Source: *const VOID, Length: SIZE_T);

    pub fn RtlFillMemoryUlong(Destination: PVOID, Length: SIZE_T, Pattern: ULONG);

    pub fn RtlRandomEx(Seed: PULONG) -> ULONG;

    pub fn RtlUniform(Seed: PULONG) -> ULONG;

    pub fn RtlNtStatusToDosError(Status: NTSTATUS) -> ULONG;

    pub fn RtlGetLastWin32Error() -> LONG;

    pub fn RtlSetLastWin32Error(Win32Error: LONG);

    pub fn RtlRestoreLastWin32Error(Win32Error: LONG);

    pub fn RtlCaptureStackBackTrace(
        FramesToSkip: ULONG,
        FramesToCapture: ULONG,
        BackTrace: *mut PVOID,
        BackTraceHash: PULONG,
    ) -> USHORT;

    pub fn RtlCaptureContext(ContextRecord: PCONTEXT);

    pub fn RtlLookupFunctionEntry(
        ControlPc: ULONG_PTR,
        ImageBase: PULONG_PTR,
        HistoryTable: *mut UNWIND_HISTORY_TABLE,
    ) -> PRUNTIME_FUNCTION;

    pub fn RtlPcToFileHeader(PcValue: PVOID, BaseOfImage: *mut PVOID) -> PVOID;

    pub fn RtlAddVectoredExceptionHandler(First: ULONG, Handler: PVECTORED_EXCEPTION_HANDLER) -> PVOID;

    pub fn RtlRemoveVectoredExceptionHandler(Handle: PVOID) -> ULONG;

    pub fn RtlEncodePointer(Ptr: PVOID) -> PVOID;

    pub fn RtlDecodePointer(Ptr: PVOID) -> PVOID;

    pub fn RtlEncodeSystemPointer(Ptr: PVOID) -> PVOID;

    pub fn RtlDecodeSystemPointer(Ptr: PVOID) -> PVOID;

    pub fn RtlIsProcessorFeaturePresent(ProcessorFeature: ULONG) -> BOOLEAN;

    pub fn RtlGetCurrentProcessorNumber() -> ULONG;

    pub fn RtlGetCurrentProcessorNumberEx(ProcessorNumber: PPROCESSOR_NUMBER);

    pub fn RtlQueryPerformanceCounter(PerformanceCounter: PLARGE_INTEGER) -> LOGICAL;

    pub fn RtlQueryPerformanceFrequency(PerformanceFrequency: PLARGE_INTEGER) -> LOGICAL;

    pub fn RtlGetVersion(VersionInformation: PVOID) -> NTSTATUS;

    pub fn RtlGetNtVersionNumbers(
        NtMajorVersion: PULONG,
        NtMinorVersion: PULONG,
        NtBuildNumber: PULONG,
    );

    pub fn RtlVerifyVersionInfo(
        VersionInformation: PVOID,
        TypeMask: ULONG,
        ConditionMask: ULONGLONG,
    ) -> NTSTATUS;

    pub fn RtlGetEnabledExtendedFeatures(FeatureMask: ULONG64) -> ULONG64;

    pub fn RtlInitializeSListHead(ListHead: PSLIST_HEADER);

    pub fn RtlFirstEntrySList(ListHead: *const SLIST_HEADER) -> PSLIST_ENTRY;

    pub fn RtlInterlockedPopEntrySList(ListHead: PSLIST_HEADER) -> PSLIST_ENTRY;

    pub fn RtlInterlockedPushEntrySList(ListHead: PSLIST_HEADER, ListEntry: PSLIST_ENTRY) -> PSLIST_ENTRY;

    pub fn RtlQueryDepthSList(ListHead: PSLIST_HEADER) -> USHORT;

    pub fn RtlCreateSecurityDescriptor(SecurityDescriptor: PSECURITY_DESCRIPTOR, Revision: ULONG) -> NTSTATUS;

    pub fn RtlValidSecurityDescriptor(SecurityDescriptor: PSECURITY_DESCRIPTOR) -> BOOLEAN;

    pub fn RtlLengthSecurityDescriptor(SecurityDescriptor: PSECURITY_DESCRIPTOR) -> ULONG;

    pub fn RtlSetDaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DaclPresent: BOOLEAN,
        Dacl: PACL,
        DaclDefaulted: BOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlGetDaclSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DaclPresent: PBOOLEAN,
        Dacl: *mut PACL,
        DaclDefaulted: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlSetOwnerSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Owner: PSID,
        OwnerDefaulted: BOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlGetOwnerSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Owner: *mut PSID,
        OwnerDefaulted: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlSetGroupSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Group: PSID,
        GroupDefaulted: BOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlGetGroupSecurityDescriptor(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Group: *mut PSID,
        GroupDefaulted: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn RtlMapGenericMask(AccessMask: PACCESS_MASK, GenericMapping: PGENERIC_MAPPING);

    pub fn RtlEqualSid(Sid1: PSID, Sid2: PSID) -> BOOLEAN;

    pub fn RtlValidSid(Sid: PSID) -> BOOLEAN;

    pub fn RtlLengthSid(Sid: PSID) -> ULONG;

    pub fn RtlInitializeSid(Sid: PSID, IdentifierAuthority: PSID_IDENTIFIER_AUTHORITY, SubAuthorityCount: UCHAR) -> NTSTATUS;

    pub fn RtlIdentifierAuthoritySid(Sid: PSID) -> PSID_IDENTIFIER_AUTHORITY;

    pub fn RtlSubAuthoritySid(Sid: PSID, SubAuthority: ULONG) -> PULONG;

    pub fn RtlSubAuthorityCountSid(Sid: PSID) -> PUCHAR;

    pub fn RtlAllocateAndInitializeSid(
        IdentifierAuthority: PSID_IDENTIFIER_AUTHORITY,
        SubAuthorityCount: UCHAR,
        SubAuthority0: ULONG,
        SubAuthority1: ULONG,
        SubAuthority2: ULONG,
        SubAuthority3: ULONG,
        SubAuthority4: ULONG,
        SubAuthority5: ULONG,
        SubAuthority6: ULONG,
        SubAuthority7: ULONG,
        Sid: *mut PSID,
    ) -> NTSTATUS;

    pub fn RtlFreeSid(Sid: PSID) -> PVOID;

    pub fn RtlCreateAcl(Acl: PACL, AclLength: ULONG, AclRevision: ULONG) -> NTSTATUS;

    pub fn RtlValidAcl(Acl: PACL) -> BOOLEAN;

    pub fn RtlAddAccessAllowedAce(Acl: PACL, AceRevision: ULONG, AccessMask: ACCESS_MASK, Sid: PSID) -> NTSTATUS;

    pub fn RtlAddAccessDeniedAce(Acl: PACL, AceRevision: ULONG, AccessMask: ACCESS_MASK, Sid: PSID) -> NTSTATUS;

    pub fn RtlGetAce(Acl: PACL, AceIndex: ULONG, Ace: *mut PVOID) -> NTSTATUS;

    pub fn RtlStringFromGUID(Guid: PGUID, GuidString: PUNICODE_STRING) -> NTSTATUS;

    pub fn RtlGUIDFromString(GuidString: PCUNICODE_STRING, Guid: PGUID) -> NTSTATUS;

    pub fn RtlGetUnloadEventTrace() -> PRTL_UNLOAD_EVENT_TRACE;

    pub fn RtlRunOnceInitialize(RunOnce: PRTL_RUN_ONCE);

    pub fn RtlRunOnceExecuteOnce(
        RunOnce: PRTL_RUN_ONCE,
        InitFn: PRTL_RUN_ONCE_INIT_FN,
        Parameter: PVOID,
        Context: *mut PVOID,
    ) -> NTSTATUS;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_HEAP_WALK_ENTRY {
    pub DataAddress: PVOID,
    pub DataSize: SIZE_T,
    pub OverheadBytes: UCHAR,
    pub SegmentIndex: UCHAR,
    pub Flags: USHORT,
    pub Block: RTL_HEAP_WALK_ENTRY_BLOCK,
    pub Segment: RTL_HEAP_WALK_ENTRY_SEGMENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_HEAP_WALK_ENTRY_BLOCK {
    pub Settable: SIZE_T,
    pub TagIndex: USHORT,
    pub AllocatorBackTraceIndex: USHORT,
    pub Reserved: [ULONG; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_HEAP_WALK_ENTRY_SEGMENT {
    pub CommittedSize: ULONG,
    pub UnCommittedSize: ULONG,
    pub FirstEntry: PVOID,
    pub LastEntry: PVOID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IN_ADDR {
    pub S_un: IN_ADDR_S_UN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IN_ADDR_S_UN {
    pub S_un_b: IN_ADDR_S_UN_B,
    pub S_un_w: IN_ADDR_S_UN_W,
    pub S_addr: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IN_ADDR_S_UN_B {
    pub s_b1: UCHAR,
    pub s_b2: UCHAR,
    pub s_b3: UCHAR,
    pub s_b4: UCHAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IN_ADDR_S_UN_W {
    pub s_w1: USHORT,
    pub s_w2: USHORT,
}

pub type PIN_ADDR = *mut IN_ADDR;
pub type LPIN_ADDR = *mut IN_ADDR;
pub type PCIN_ADDR = *const IN_ADDR;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IN6_ADDR {
    pub u: IN6_ADDR_U,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IN6_ADDR_U {
    pub Byte: [UCHAR; 16],
    pub Word: [USHORT; 8],
}

pub type PIN6_ADDR = *mut IN6_ADDR;
pub type LPIN6_ADDR = *mut IN6_ADDR;
pub type PCIN6_ADDR = *const IN6_ADDR;

unsafe extern "system" {
    pub fn RtlIpv4AddressToStringA(Address: PCIN_ADDR, AddressString: PSTR) -> PSTR;
    pub fn RtlIpv4AddressToStringW(Address: PCIN_ADDR, AddressString: PWSTR) -> PWSTR;
    pub fn RtlIpv6AddressToStringA(Address: PCIN6_ADDR, AddressString: PSTR) -> PSTR;
    pub fn RtlIpv6AddressToStringW(Address: PCIN6_ADDR, AddressString: PWSTR) -> PWSTR;
    pub fn RtlIpv4StringToAddressA(
        AddressString: PCSTR,
        Strict: BOOLEAN,
        Terminator: *mut PCSTR,
        Address: PIN_ADDR,
    ) -> NTSTATUS;
    pub fn RtlIpv4StringToAddressW(
        AddressString: PCWSTR,
        Strict: BOOLEAN,
        Terminator: *mut PCWSTR,
        Address: PIN_ADDR,
    ) -> NTSTATUS;
    pub fn RtlIpv6StringToAddressA(
        AddressString: PCSTR,
        Terminator: *mut PCSTR,
        Address: PIN6_ADDR,
    ) -> NTSTATUS;
    pub fn RtlIpv6StringToAddressW(
        AddressString: PCWSTR,
        Terminator: *mut PCWSTR,
        Address: PIN6_ADDR,
    ) -> NTSTATUS;
}

pub type PUSER_THREAD_START_ROUTINE = Option<unsafe extern "system" fn(ThreadParameter: PVOID) -> NTSTATUS>;
pub type PVECTORED_EXCEPTION_HANDLER = Option<unsafe extern "system" fn(ExceptionInfo: *mut EXCEPTION_POINTERS) -> LONG>;

pub type PRTL_RUN_ONCE = *mut RTL_RUN_ONCE;
pub type PRTL_RUN_ONCE_INIT_FN = Option<unsafe extern "system" fn(RunOnce: PRTL_RUN_ONCE, Parameter: PVOID, Context: *mut PVOID) -> LOGICAL>;

pub type PFLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(Data: PVOID)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SECTION_IMAGE_INFORMATION {
    pub TransferAddress: PVOID,
    pub ZeroBits: ULONG,
    pub MaximumStackSize: SIZE_T,
    pub CommittedStackSize: SIZE_T,
    pub SubSystemType: ULONG,
    pub SubSystemMajorVersion: USHORT,
    pub SubSystemMinorVersion: USHORT,
    pub MajorOperatingSystemVersion: USHORT,
    pub MinorOperatingSystemVersion: USHORT,
    pub ImageCharacteristics: USHORT,
    pub DllCharacteristics: USHORT,
    pub Machine: USHORT,
    pub ImageContainsCode: BOOLEAN,
    pub ImageFlags: UCHAR,
    pub LoaderFlags: ULONG,
    pub ImageFileSize: ULONG,
    pub CheckSum: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CLIENT_ID {
    pub UniqueProcess: HANDLE,
    pub UniqueThread: HANDLE,
}

pub type PCLIENT_ID = *mut CLIENT_ID;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UNWIND_HISTORY_TABLE {
    pub Size: ULONG,
    pub Count: ULONG,
    pub LocalHint: ULONG,
    pub GlobalHint: ULONG,
    pub Search: ULONG,
    pub Table: [ULONG_PTR; 16],
}

pub type RUNLEVEL = ULONG;

pub const RUNLEVEL_UNSPECIFIED: RUNLEVEL = 0;
pub const RUNLEVEL_NONE: RUNLEVEL = 1;
pub const RUNLEVEL_APPLICATION: RUNLEVEL = 2;
pub const RUNLEVEL_HIGHESTAVAILABLE: RUNLEVEL = 3;
pub const RUNLEVEL_REQUIREADMIN: RUNLEVEL = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulRunLevel: RUNLEVEL,
    pub UILanguage: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_ELEMENT {
    pub ElementType: ULONG,
    pub Id: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_LEGACY {
    pub Count: ULONG,
    pub Elements: [ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_ELEMENT; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub Count: ULONG,
    pub bVirtualizationEnabled: BOOLEAN,
    pub bSystemAppsMitigationEnabled: BOOLEAN,
    pub Elements: [ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION_ELEMENT; 1],
}

pub type PRTL_HEAP_WALK_ENTRY = *mut RTL_HEAP_WALK_ENTRY;

#[repr(C)]
pub struct RTL_RUN_ONCE {
    pub Ptr: PVOID,
}

#[cfg(target_pointer_width = "64")]
pub type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS64;
#[cfg(target_pointer_width = "64")]
pub type PIMAGE_NT_HEADERS = *mut IMAGE_NT_HEADERS64;

#[repr(C)]
pub struct IMAGE_NT_HEADERS64 {
    pub Signature: DWORD,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}

#[repr(C)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: WORD,
    pub NumberOfSections: WORD,
    pub TimeDateStamp: DWORD,
    pub PointerToSymbolTable: DWORD,
    pub NumberOfSymbols: DWORD,
    pub SizeOfOptionalHeader: WORD,
    pub Characteristics: WORD,
}

#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: WORD,
    pub MajorLinkerVersion: BYTE,
    pub MinorLinkerVersion: BYTE,
    pub SizeOfCode: DWORD,
    pub SizeOfInitializedData: DWORD,
    pub SizeOfUninitializedData: DWORD,
    pub AddressOfEntryPoint: DWORD,
    pub BaseOfCode: DWORD,
    pub ImageBase: ULONGLONG,
    pub SectionAlignment: DWORD,
    pub FileAlignment: DWORD,
    pub MajorOperatingSystemVersion: WORD,
    pub MinorOperatingSystemVersion: WORD,
    pub MajorImageVersion: WORD,
    pub MinorImageVersion: WORD,
    pub MajorSubsystemVersion: WORD,
    pub MinorSubsystemVersion: WORD,
    pub Win32VersionValue: DWORD,
    pub SizeOfImage: DWORD,
    pub SizeOfHeaders: DWORD,
    pub CheckSum: DWORD,
    pub Subsystem: WORD,
    pub DllCharacteristics: WORD,
    pub SizeOfStackReserve: ULONGLONG,
    pub SizeOfStackCommit: ULONGLONG,
    pub SizeOfHeapReserve: ULONGLONG,
    pub SizeOfHeapCommit: ULONGLONG,
    pub LoaderFlags: DWORD,
    pub NumberOfRvaAndSizes: DWORD,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}

pub type PIMAGE_OPTIONAL_HEADER64 = *mut IMAGE_OPTIONAL_HEADER64;

#[repr(C)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: DWORD,
    pub Size: DWORD,
}

pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_RESOURCE {
    pub CriticalSection: RTL_CRITICAL_SECTION,
    pub SharedSemaphore: HANDLE,
    pub NumberOfWaitingShared: ULONG,
    pub ExclusiveSemaphore: HANDLE,
    pub NumberOfWaitingExclusive: ULONG,
    pub NumberOfActive: LONG,
    pub ExclusiveOwnerThread: HANDLE,
    pub Flags: ULONG,
    pub DebugInfo: PRTL_RESOURCE_DEBUG,
}

pub type PRTL_RESOURCE = *mut RTL_RESOURCE;
pub type PRTL_RESOURCE_DEBUG = *mut RTL_CRITICAL_SECTION_DEBUG;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RTL_BUFFER {
    pub Buffer: PUCHAR,
    pub StaticBuffer: PUCHAR,
    pub Size: SIZE_T,
    pub StaticSize: SIZE_T,
}

pub type PRTL_BUFFER = *mut RTL_BUFFER;

#[repr(C)]
pub struct RTL_UNICODE_STRING_BUFFER {
    pub String: UNICODE_STRING,
    pub ByteBuffer: RTL_BUFFER,
    pub MinimumStaticBufferForTerminalNul: [UCHAR; 2],
}
pub type PRTL_UNICODE_STRING_BUFFER = *mut RTL_UNICODE_STRING_BUFFER;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GENERATE_NAME_CONTEXT {
    pub Checksum: USHORT,
    pub CheckSumInserted: BOOLEAN,
    pub NameLength: UCHAR,
    pub NameBuffer: [WCHAR; 8],
    pub ExtensionLength: ULONG,
    pub ExtensionBuffer: [WCHAR; 4],
    pub LastIndexValue: ULONG,
}

pub type PGENERATE_NAME_CONTEXT = *mut GENERATE_NAME_CONTEXT;

pub const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: ULONG = 0x01000000;
pub const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: ULONG = 0x02000000;
pub const RTL_CRITICAL_SECTION_FLAG_STATIC_INIT: ULONG = 0x04000000;
pub const RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE: ULONG = 0x08000000;
pub const RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO: ULONG = 0x10000000;
pub const RTL_CRITICAL_SECTION_ALL_FLAG_BITS: ULONG = 0xFF000000;

pub const RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT: ULONG = 0x00000001;

pub const RTL_RESOURCE_FLAG_LONG_TERM: ULONG = 0x00000001;

pub const RTL_CONDITION_VARIABLE_INIT: RTL_CONDITION_VARIABLE = RTL_CONDITION_VARIABLE { Ptr: core::ptr::null_mut() };
pub const RTL_CONDITION_VARIABLE_LOCKMODE_SHARED: ULONG = 0x1;

pub const RTL_BARRIER_FLAGS_SPIN_ONLY: ULONG = 0x00000001;
pub const RTL_BARRIER_FLAGS_BLOCK_ONLY: ULONG = 0x00000002;
pub const RTL_BARRIER_FLAGS_NO_DELETE: ULONG = 0x00000004;

pub const RTL_REGISTRY_ABSOLUTE: ULONG = 0;
pub const RTL_REGISTRY_SERVICES: ULONG = 1;
pub const RTL_REGISTRY_CONTROL: ULONG = 2;
pub const RTL_REGISTRY_WINDOWS_NT: ULONG = 3;
pub const RTL_REGISTRY_DEVICEMAP: ULONG = 4;
pub const RTL_REGISTRY_USER: ULONG = 5;
pub const RTL_REGISTRY_MAXIMUM: ULONG = 6;
pub const RTL_REGISTRY_HANDLE: ULONG = 0x40000000;
pub const RTL_REGISTRY_OPTIONAL: ULONG = 0x80000000;

pub const RTL_QUERY_REGISTRY_SUBKEY: ULONG = 0x00000001;
pub const RTL_QUERY_REGISTRY_TOPKEY: ULONG = 0x00000002;
pub const RTL_QUERY_REGISTRY_REQUIRED: ULONG = 0x00000004;
pub const RTL_QUERY_REGISTRY_NOVALUE: ULONG = 0x00000008;
pub const RTL_QUERY_REGISTRY_NOEXPAND: ULONG = 0x00000010;
pub const RTL_QUERY_REGISTRY_DIRECT: ULONG = 0x00000020;
pub const RTL_QUERY_REGISTRY_DELETE: ULONG = 0x00000040;
pub const RTL_QUERY_REGISTRY_TYPECHECK: ULONG = 0x00000100;

pub const RTL_QUERY_REGISTRY_TYPECHECK_SHIFT: i32 = 24;

pub const COMPRESSION_FORMAT_NONE: USHORT = 0x0000;
pub const COMPRESSION_FORMAT_DEFAULT: USHORT = 0x0001;
pub const COMPRESSION_FORMAT_LZNT1: USHORT = 0x0002;
pub const COMPRESSION_FORMAT_XPRESS: USHORT = 0x0003;
pub const COMPRESSION_FORMAT_XPRESS_HUFF: USHORT = 0x0004;
pub const COMPRESSION_FORMAT_XP10: USHORT = 0x0005;
pub const COMPRESSION_FORMAT_LZ4: USHORT = 0x0006;
pub const COMPRESSION_FORMAT_DEFLATE: USHORT = 0x0007;
pub const COMPRESSION_FORMAT_ZLIB: USHORT = 0x0008;
pub const COMPRESSION_FORMAT_MAX: USHORT = 0x0008;

pub const COMPRESSION_ENGINE_STANDARD: USHORT = 0x0000;
pub const COMPRESSION_ENGINE_MAXIMUM: USHORT = 0x0100;
pub const COMPRESSION_ENGINE_HIBER: USHORT = 0x0200;
pub const COMPRESSION_ENGINE_MAX: USHORT = 0x0200;

pub const COMPRESSION_FORMAT_MASK: USHORT = 0x00FF;
pub const COMPRESSION_ENGINE_MASK: USHORT = 0xFF00;

pub const RTL_USER_PROC_PARAMS_NORMALIZED: ULONG = 0x00000001;
pub const RTL_USER_PROC_PROFILE_USER: ULONG = 0x00000002;
pub const RTL_USER_PROC_PROFILE_KERNEL: ULONG = 0x00000004;
pub const RTL_USER_PROC_PROFILE_SERVER: ULONG = 0x00000008;
pub const RTL_USER_PROC_RESERVE_1MB: ULONG = 0x00000020;
pub const RTL_USER_PROC_RESERVE_16MB: ULONG = 0x00000040;
pub const RTL_USER_PROC_CASE_SENSITIVE: ULONG = 0x00000080;
pub const RTL_USER_PROC_DISABLE_HEAP_DECOMMIT: ULONG = 0x00000100;
pub const RTL_USER_PROC_DLL_REDIRECTION_LOCAL: ULONG = 0x00001000;
pub const RTL_USER_PROC_APP_MANIFEST_PRESENT: ULONG = 0x00002000;
pub const RTL_USER_PROC_IMAGE_KEY_MISSING: ULONG = 0x00004000;
pub const RTL_USER_PROC_DEV_OVERRIDE_ENABLED: ULONG = 0x00008000;
pub const RTL_USER_PROC_OPTIN_PROCESS: ULONG = 0x00020000;
pub const RTL_USER_PROC_SESSION_OWNER: ULONG = 0x00040000;
pub const RTL_USER_PROC_HANDLE_USER_CALLBACK_EXCEPTIONS: ULONG = 0x00080000;
pub const RTL_USER_PROC_PROTECTED_PROCESS: ULONG = 0x00400000;
pub const RTL_USER_PROC_SECURE_PROCESS: ULONG = 0x80000000;

pub const WT_EXECUTEDEFAULT: ULONG = 0x00000000;
pub const WT_EXECUTEINIOTHREAD: ULONG = 0x00000001;
pub const WT_EXECUTEINUITHREAD: ULONG = 0x00000002;
pub const WT_EXECUTEINWAITTHREAD: ULONG = 0x00000004;
pub const WT_EXECUTEONLYONCE: ULONG = 0x00000008;
pub const WT_EXECUTELONGFUNCTION: ULONG = 0x00000010;
pub const WT_EXECUTEINTIMERTHREAD: ULONG = 0x00000020;
pub const WT_EXECUTEINPERSISTENTIOTHREAD: ULONG = 0x00000040;
pub const WT_EXECUTEINPERSISTENTTHREAD: ULONG = 0x00000080;
pub const WT_TRANSFER_IMPERSONATION: ULONG = 0x00000100;

pub const RTL_CLONE_PROCESS_FLAGS_CREATE_SUSPENDED: ULONG = 0x00000001;
pub const RTL_CLONE_PROCESS_FLAGS_INHERIT_HANDLES: ULONG = 0x00000002;
pub const RTL_CLONE_PROCESS_FLAGS_NO_SYNCHRONIZE: ULONG = 0x00000004;

pub const RTL_PROCESS_REFLECTION_FLAGS_CREATE_SUSPENDED: ULONG = 0x00000001;
pub const RTL_PROCESS_REFLECTION_FLAGS_INHERIT_HANDLES: ULONG = 0x00000002;
pub const RTL_PROCESS_REFLECTION_FLAGS_NO_SUSPEND: ULONG = 0x00000004;
pub const RTL_PROCESS_REFLECTION_FLAGS_NO_SYNCHRONIZE: ULONG = 0x00000008;
pub const RTL_PROCESS_REFLECTION_FLAGS_NO_CLOSE_EVENT: ULONG = 0x00000010;

pub const RTL_ERRORMODE_FAILCRITICALERRORS: ULONG = 0x0010;
pub const RTL_ERRORMODE_NOGPFAULTERRORBOX: ULONG = 0x0020;
pub const RTL_ERRORMODE_NOOPENFILEERRORBOX: ULONG = 0x0040;

pub const RTL_USER_PROC_DETACHED_PROCESS: HANDLE = -1isize as HANDLE;
pub const RTL_USER_PROC_CREATE_NEW_CONSOLE: HANDLE = -2isize as HANDLE;
pub const RTL_USER_PROC_CREATE_NO_WINDOW: HANDLE = -3isize as HANDLE;

pub const INVALID_ACTIVATION_CONTEXT: HANDLE = -1isize as HANDLE;
pub const ACTCTX_PROCESS_DEFAULT: HANDLE = 0 as HANDLE;
pub const ACTCTX_EMPTY: HANDLE = -3isize as HANDLE;
pub const ACTCTX_SYSTEM_DEFAULT: HANDLE = -4isize as HANDLE;

pub const RTL_ACTIVATE_ACTIVATION_CONTEXT_EX_FLAG_RELEASE_ON_STACK_DEALLOCATION: ULONG = 0x00000001;
pub const RTL_DEACTIVATE_ACTIVATION_CONTEXT_FLAG_FORCE_EARLY_DEACTIVATION: ULONG = 0x00000001;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_USE_ACTIVE_ACTIVATION_CONTEXT: ULONG = 0x00000001;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_ACTIVATION_CONTEXT_IS_MODULE: ULONG = 0x00000002;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_ACTIVATION_CONTEXT_IS_ADDRESS: ULONG = 0x00000004;
pub const RTL_QUERY_INFORMATION_ACTIVATION_CONTEXT_FLAG_NO_ADDREF: ULONG = 0x80000000;

pub const RTL_RUN_ONCE_CHECK_ONLY: ULONG = 0x00000001;
pub const RTL_RUN_ONCE_ASYNC: ULONG = 0x00000002;
pub const RTL_RUN_ONCE_INIT_FAILED: ULONG = 0x00000004;
pub const RTL_RUN_ONCE_CTX_RESERVED_BITS: i32 = 2;
pub const RTL_UNLOAD_EVENT_TRACE_NUMBER: usize = 64;

pub const RTL_IMAGE_MITIGATION_FLAG_RESET: ULONG = 0x1;
pub const RTL_IMAGE_MITIGATION_FLAG_REMOVE: ULONG = 0x2;
pub const RTL_IMAGE_MITIGATION_FLAG_OSDEFAULT: ULONG = 0x4;
pub const RTL_IMAGE_MITIGATION_FLAG_AUDIT: ULONG = 0x8;

unsafe extern "system" {
    pub fn RtlAcquirePebLock() -> NTSTATUS;
    pub fn RtlReleasePebLock() -> NTSTATUS;
    pub fn RtlCreateUserStack(
        CommittedStackSize: SIZE_T,
        MaximumStackSize: SIZE_T,
        ZeroBits: ULONG_PTR,
        PageSize: SIZE_T,
        ReserveAlignment: ULONG_PTR,
        InitialTeb: PINITIAL_TEB,
    ) -> NTSTATUS;
    pub fn RtlFreeUserStack(AllocationBase: PVOID) -> NTSTATUS;
    pub fn RtlCreateRegistryKey(RelativeTo: ULONG, Path: PCWSTR) -> NTSTATUS;
    pub fn RtlCheckRegistryKey(RelativeTo: ULONG, Path: PCWSTR) -> NTSTATUS;
    pub fn RtlWriteRegistryValue(
        RelativeTo: ULONG,
        Path: PCWSTR,
        ValueName: PCWSTR,
        ValueType: ULONG,
        ValueData: PVOID,
        ValueLength: ULONG,
    ) -> NTSTATUS;
    pub fn RtlDeleteRegistryValue(RelativeTo: ULONG, Path: PCWSTR, ValueName: PCWSTR) -> NTSTATUS;
    pub fn RtlOpenCurrentUser(DesiredAccess: ACCESS_MASK, CurrentUserKey: PHANDLE) -> NTSTATUS;
    pub fn RtlGetCompressionWorkSpaceSize(
        CompressionFormatAndEngine: USHORT,
        CompressBufferWorkSpaceSize: PULONG,
        CompressFragmentWorkSpaceSize: PULONG,
    ) -> NTSTATUS;
    pub fn RtlCompressBuffer(
        CompressionFormatAndEngine: USHORT,
        UncompressedBuffer: PUCHAR,
        UncompressedBufferSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        UncompressedChunkSize: ULONG,
        FinalCompressedSize: PULONG,
        WorkSpace: PVOID,
    ) -> NTSTATUS;
    pub fn RtlDecompressBuffer(
        CompressionFormat: USHORT,
        UncompressedBuffer: PUCHAR,
        UncompressedBufferSize: ULONG,
        CompressedBuffer: PUCHAR,
        CompressedBufferSize: ULONG,
        FinalUncompressedSize: PULONG,
    ) -> NTSTATUS;
    pub fn RtlDetermineDosPathNameType_U(DosFileName: PCWSTR) -> RTL_PATH_TYPE;
    pub fn RtlDoesFileExists_U(FileName: PCWSTR) -> BOOLEAN;
    pub fn RtlGetLongestNtPathLength() -> ULONG;
    pub fn RtlDosPathNameToRelativeNtPathName_U(
        DosFileName: PCWSTR,
        NtFileName: PUNICODE_STRING,
        FilePart: *mut PWSTR,
        RelativeName: PRTL_RELATIVE_NAME_U,
    ) -> BOOLEAN;
    pub fn RtlCreateTimerQueue(TimerQueueHandle: PHANDLE) -> NTSTATUS;
    pub fn RtlCreateTimer(
        TimerQueueHandle: HANDLE,
        Handle: PHANDLE,
        Function: PRTL_TIMER_CALLBACK,
        Context: PVOID,
        DueTime: ULONG,
        Period: ULONG,
        Flags: ULONG,
    ) -> NTSTATUS;
    pub fn RtlDeleteTimerQueueEx(TimerQueueHandle: HANDLE, Event: HANDLE) -> NTSTATUS;
    pub fn RtlRegisterWait(
        WaitHandle: PHANDLE,
        Handle: HANDLE,
        Function: PWAIT_CALLBACK_ROUTINE,
        Context: PVOID,
        Milliseconds: ULONG,
        Flags: ULONG,
    ) -> NTSTATUS;
    pub fn RtlDeregisterWaitEx(WaitHandle: HANDLE, CompletionEvent: HANDLE) -> NTSTATUS;
    pub fn RtlQueueWorkItem(Function: PRTL_WORK_CALLBACK, Context: PVOID, Flags: ULONG) -> NTSTATUS;
    pub fn RtlDelayExecution(Alertable: BOOLEAN, DelayInterval: PLARGE_INTEGER) -> NTSTATUS;
    pub fn RtlUshortByteSwap(Source: USHORT) -> USHORT;
    pub fn RtlUlongByteSwap(Source: ULONG) -> ULONG;
    pub fn RtlUlonglongByteSwap(Source: ULONGLONG) -> ULONGLONG;
    pub fn RtlGetNtGlobalFlags() -> ULONG;
    pub fn RtlGetNtProductType(NtProductType: PNT_PRODUCT_TYPE) -> BOOLEAN;
    pub fn RtlIsAnyDebuggerPresent() -> BOOLEAN;
    pub fn RtlGetLastNtStatus() -> NTSTATUS;
    pub fn RtlGetThreadErrorMode() -> ULONG;
    pub fn RtlSetThreadErrorMode(NewMode: ULONG, OldMode: PULONG) -> NTSTATUS;
    pub fn RtlFlsAlloc(Callback: PFLS_CALLBACK_FUNCTION, FlsIndex: PULONG) -> NTSTATUS;
    pub fn RtlFlsFree(FlsIndex: ULONG) -> NTSTATUS;
    pub fn RtlFlsGetValue(FlsIndex: ULONG, FlsData: *mut PVOID) -> NTSTATUS;
    pub fn RtlFlsSetValue(FlsIndex: ULONG, FlsData: PVOID) -> NTSTATUS;
    pub fn RtlCreateUnicodeString(DestinationString: PUNICODE_STRING, SourceString: PCWSTR) -> BOOLEAN;
    pub fn RtlDowncaseUnicodeChar(SourceCharacter: WCHAR) -> WCHAR;
    pub fn RtlUpcaseUnicodeChar(SourceCharacter: WCHAR) -> WCHAR;
    pub fn RtlHashUnicodeString(
        String: PCUNICODE_STRING,
        CaseInSensitive: BOOLEAN,
        HashAlgorithm: ULONG,
        HashValue: PULONG,
    ) -> NTSTATUS;
    
    pub fn RtlInitializeResource(Resource: PRTL_RESOURCE);
    pub fn RtlDeleteResource(Resource: PRTL_RESOURCE);
    pub fn RtlAcquireResourceShared(Resource: PRTL_RESOURCE, Wait: BOOLEAN) -> BOOLEAN;
    pub fn RtlAcquireResourceExclusive(Resource: PRTL_RESOURCE, Wait: BOOLEAN) -> BOOLEAN;
    pub fn RtlReleaseResource(Resource: PRTL_RESOURCE);
    pub fn PfxInitialize(PrefixTable: PPREFIX_TABLE);
    pub fn RtlInitializeUnicodePrefix(PrefixTable: PUNICODE_PREFIX_TABLE);
    pub fn RtlCreateAtomTable(NumberOfBuckets: ULONG, AtomTableHandle: *mut PVOID) -> NTSTATUS;
    pub fn RtlDestroyAtomTable(AtomTableHandle: PVOID) -> NTSTATUS;
    pub fn RtlInitializeHandleTable(
        MaximumNumberOfHandles: ULONG,
        SizeOfHandleTableEntry: ULONG,
        HandleTable: PRTL_HANDLE_TABLE,
    );
    pub fn RtlDestroyHandleTable(HandleTable: PRTL_HANDLE_TABLE) -> NTSTATUS;
    pub fn RtlAddResourceAttributeAce(
        Acl: PACL,
        AceRevision: ULONG,
        AceFlags: ULONG,
        AccessMask: ULONG,
        Sid: PSID,
        AttributeInfo: PCLAIM_SECURITY_ATTRIBUTES_INFORMATION,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    
    pub fn RtlCreateBoundaryDescriptor(Name: PCUNICODE_STRING, Flags: ULONG) -> POBJECT_BOUNDARY_DESCRIPTOR;
    pub fn RtlDeleteBoundaryDescriptor(BoundaryDescriptor: POBJECT_BOUNDARY_DESCRIPTOR);
    pub fn RtlAddSIDToBoundaryDescriptor(BoundaryDescriptor: *mut POBJECT_BOUNDARY_DESCRIPTOR, RequiredSid: PCSID) -> NTSTATUS;
    
    pub fn RtlQueryWnfStateData(
        ChangeStamp: PWNF_CHANGE_STAMP,
        StateName: WNF_STATE_NAME,
        Callback: PWNF_USER_CALLBACK,
        CallbackContext: PVOID,
        TypeId: PWNF_TYPE_ID,
    ) -> NTSTATUS;
    pub fn RtlPublishWnfStateData(
        StateName: WNF_STATE_NAME,
        TypeId: PCWNF_TYPE_ID,
        Buffer: *const VOID,
        Length: ULONG,
        ExplicitScope: *const VOID,
    ) -> NTSTATUS;
}

pub type PRTL_TIMER_CALLBACK = Option<unsafe extern "system" fn(Parameter: PVOID, TimerOrWaitFired: BOOLEAN)>;
pub type PWAIT_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(Parameter: PVOID, TimerOrWaitFired: BOOLEAN)>;
pub type PRTL_WORK_CALLBACK = Option<unsafe extern "system" fn(ThreadParameter: PVOID)>;

pub type POBJECT_BOUNDARY_DESCRIPTOR = PVOID;

pub type PNT_PRODUCT_TYPE = *mut NT_PRODUCT_TYPE;

pub type WNF_STATE_NAME = ULONG64;
pub type PWNF_USER_CALLBACK = Option<unsafe extern "system" fn(
    StateName: WNF_STATE_NAME,
    ChangeStamp: WNF_CHANGE_STAMP,
    TypeId: PWNF_TYPE_ID,
    CallbackContext: PVOID,
    Buffer: *const VOID,
    Length: ULONG,
) -> NTSTATUS>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WNF_TYPE_ID {
    pub TypeId: GUID,
}

pub type NT_PRODUCT_TYPE = ULONG;

pub const NtProductWinNt: NT_PRODUCT_TYPE = 1;
pub const NtProductLanManNt: NT_PRODUCT_TYPE = 2;
pub const NtProductServer: NT_PRODUCT_TYPE = 3;

pub type SECURITY_IMPERSONATION_LEVEL = ULONG;

pub const SecurityAnonymous: SECURITY_IMPERSONATION_LEVEL = 0;
pub const SecurityIdentification: SECURITY_IMPERSONATION_LEVEL = 1;
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = 2;
pub const SecurityDelegation: SECURITY_IMPERSONATION_LEVEL = 3;

pub type PSECURITY_DESCRIPTOR_CONTROL = *mut SECURITY_DESCRIPTOR_CONTROL;
pub type SECURITY_DESCRIPTOR_CONTROL = USHORT;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PS_PROTECTION {
    pub Level: UCHAR,
}

pub type PPS_PROTECTION = *mut PS_PROTECTION;

#[repr(C)]
pub struct PREFIX_TABLE {
    pub NodeTypeCode: CSHORT,
    pub NameLength: CSHORT,
    pub NextPrefixTree: PPREFIX_TABLE_ENTRY,
}

pub type PPREFIX_TABLE = *mut PREFIX_TABLE;

#[repr(C)]
pub struct UNICODE_PREFIX_TABLE {
    pub NodeTypeCode: CSHORT,
    pub NameLength: CSHORT,
    pub NextPrefixTree: PUNICODE_PREFIX_TABLE_ENTRY,
    pub LastNextEntry: PUNICODE_PREFIX_TABLE_ENTRY,
}

pub type PUNICODE_PREFIX_TABLE = *mut UNICODE_PREFIX_TABLE;

#[repr(C)]
pub struct RTL_HANDLE_TABLE {
    pub MaximumNumberOfHandles: ULONG,
    pub SizeOfHandleTableEntry: ULONG,
    pub Reserved: [ULONG; 2],
    pub FreeHandles: PRTL_HANDLE_TABLE_ENTRY,
    pub CommittedHandles: PRTL_HANDLE_TABLE_ENTRY,
    pub UnCommittedHandles: PRTL_HANDLE_TABLE_ENTRY,
    pub MaxReservedHandles: PRTL_HANDLE_TABLE_ENTRY,
}

pub type PRTL_HANDLE_TABLE = *mut RTL_HANDLE_TABLE;

pub type PPREFIX_TABLE_ENTRY = *mut PREFIX_TABLE_ENTRY;
pub type PUNICODE_PREFIX_TABLE_ENTRY = *mut UNICODE_PREFIX_TABLE_ENTRY;
pub type PRTL_HANDLE_TABLE_ENTRY = *mut RTL_HANDLE_TABLE_ENTRY;

#[repr(C)]
pub struct PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: CSHORT,
    pub NameLength: CSHORT,
    pub NextPrefixTree: PPREFIX_TABLE_ENTRY,
    pub Links: RTL_SPLAY_LINKS,
    pub Prefix: PSTRING,
}

#[repr(C)]
pub struct UNICODE_PREFIX_TABLE_ENTRY {
    pub NodeTypeCode: CSHORT,
    pub NameLength: CSHORT,
    pub NextPrefixTree: PUNICODE_PREFIX_TABLE_ENTRY,
    pub CaseMatch: PUNICODE_PREFIX_TABLE_ENTRY,
    pub Links: RTL_SPLAY_LINKS,
    pub Prefix: PUNICODE_STRING,
}

#[repr(C)]
pub struct RTL_HANDLE_TABLE_ENTRY {
    pub NextFree: *mut RTL_HANDLE_TABLE_ENTRY,
}