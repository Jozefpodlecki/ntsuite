use core::ptr;

use crate::ntdef::*;
use crate::ntioapi::IO_STATUS_BLOCK;

pub const PAGE_NOACCESS: u32 = 0x01;
pub const PAGE_READONLY: u32 = 0x02;
pub const PAGE_READWRITE: u32 = 0x04;
pub const PAGE_WRITECOPY: u32 = 0x08;
pub const PAGE_EXECUTE: u32 = 0x10;
pub const PAGE_EXECUTE_READ: u32 = 0x20;
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;
pub const PAGE_GUARD: u32 = 0x100;
pub const PAGE_NOCACHE: u32 = 0x200;
pub const PAGE_WRITECOMBINE: u32 = 0x400;

pub const PAGE_REVERT_TO_FILE_MAP: u32 = 0x80000000;
pub const PAGE_ENCLAVE_THREAD_CONTROL: u32 = 0x80000000;
pub const PAGE_TARGETS_NO_UPDATE: u32 = 0x40000000;
pub const PAGE_TARGETS_INVALID: u32 = 0x40000000;
pub const PAGE_ENCLAVE_UNVALIDATED: u32 = 0x20000000;
pub const PAGE_ENCLAVE_NO_CHANGE: u32 = 0x20000000;
pub const PAGE_ENCLAVE_MASK: u32 = 0x10000000;
pub const PAGE_ENCLAVE_DECOMMIT: u32 = PAGE_ENCLAVE_MASK | 0;
pub const PAGE_ENCLAVE_SS_FIRST: u32 = PAGE_ENCLAVE_MASK | 1;
pub const PAGE_ENCLAVE_SS_REST: u32 = PAGE_ENCLAVE_MASK | 2;

pub const MEM_COMMIT: u32 = 0x00001000;
pub const MEM_RESERVE: u32 = 0x00002000;
pub const MEM_DECOMMIT: u32 = 0x00004000;
pub const MEM_RELEASE: u32 = 0x00008000;
pub const MEM_FREE: u32 = 0x00010000;
pub const MEM_PRIVATE: u32 = 0x00020000;
pub const MEM_MAPPED: u32 = 0x00040000;
pub const MEM_RESET: u32 = 0x00080000;
pub const MEM_TOP_DOWN: u32 = 0x00100000;
pub const MEM_WRITE_WATCH: u32 = 0x00200000;
pub const MEM_PHYSICAL: u32 = 0x00400000;
pub const MEM_ROTATE: u32 = 0x00800000;
pub const MEM_DIFFERENT_IMAGE_BASE_OK: u32 = 0x00800000;
pub const MEM_RESET_UNDO: u32 = 0x01000000;
pub const MEM_LARGE_PAGES: u32 = 0x20000000;
pub const MEM_DOS_LIM: u32 = 0x40000000;
pub const MEM_4MB_PAGES: u32 = 0x80000000;
pub const MEM_64K_PAGES: u32 = MEM_LARGE_PAGES | MEM_PHYSICAL;

pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: u32 = 0x00000001;
pub const MEM_COALESCE_PLACEHOLDERS: u32 = 0x00000001;
pub const MEM_PRESERVE_PLACEHOLDER: u32 = 0x00000002;
pub const MEM_REPLACE_PLACEHOLDER: u32 = 0x00004000;
pub const MEM_RESERVE_PLACEHOLDER: u32 = 0x00040000;

pub const SEC_HUGE_PAGES: u32 = 0x00020000;
pub const SEC_PARTITION_OWNER_HANDLE: u32 = 0x00040000;
pub const SEC_64K_PAGES: u32 = 0x00080000;
pub const SEC_DRIVER_IMAGE: u32 = 0x00100000;
pub const SEC_BASED: u32 = 0x00200000;
pub const SEC_NO_CHANGE: u32 = 0x00400000;
pub const SEC_FILE: u32 = 0x00800000;
pub const SEC_IMAGE: u32 = 0x01000000;
pub const SEC_PROTECTED_IMAGE: u32 = 0x02000000;
pub const SEC_RESERVE: u32 = 0x04000000;
pub const SEC_COMMIT: u32 = 0x08000000;
pub const SEC_NOCACHE: u32 = 0x10000000;
pub const SEC_GLOBAL: u32 = 0x20000000;
pub const SEC_WRITECOMBINE: u32 = 0x40000000;
pub const SEC_LARGE_PAGES: u32 = 0x80000000;
pub const SEC_IMAGE_NO_EXECUTE: u32 = SEC_IMAGE | SEC_NOCACHE;

pub const MEM_IMAGE: u32 = SEC_IMAGE;

pub type VIRTUAL_ALLOCATION_TYPE = u32;
pub type PAGE_PROTECTION_FLAGS = u32;
pub type PAGE_TYPE = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}

#[repr(u32)]
pub enum MEMORY_INFORMATION_CLASS {
    MemoryBasicInformation = 0,
    MemoryWorkingSetInformation = 1,
    MemoryMappedFilenameInformation = 2,
    MemoryRegionInformation = 3,
    MemoryWorkingSetExInformation = 4,
    MemorySharedCommitInformation = 5,
    MemoryImageInformation = 6,
    MemoryRegionInformationEx = 7,
    MemoryPrivilegedBasicInformation = 8,
    MemoryEnclaveImageInformation = 9,
    MemoryBasicInformationCapped = 10,
    MemoryPhysicalContiguityInformation = 11,
    MemoryBadInformation = 12,
    MemoryBadInformationAllProcesses = 13,
    MemoryImageExtensionInformation = 14,
}

pub const MEMORY_BLOCK_NOT_ACCESSED: u32 = 0;
pub const MEMORY_BLOCK_READONLY: u32 = 1;
pub const MEMORY_BLOCK_EXECUTABLE: u32 = 2;
pub const MEMORY_BLOCK_EXECUTABLE_READONLY: u32 = 3;
pub const MEMORY_BLOCK_READWRITE: u32 = 4;
pub const MEMORY_BLOCK_COPYONWRITE: u32 = 5;
pub const MEMORY_BLOCK_EXECUTABLE_READWRITE: u32 = 6;
pub const MEMORY_BLOCK_EXECUTABLE_COPYONWRITE: u32 = 7;
pub const MEMORY_BLOCK_NOT_ACCESSED_2: u32 = 8;
pub const MEMORY_BLOCK_NON_CACHEABLE_READONLY: u32 = 9;
pub const MEMORY_BLOCK_NON_CACHEABLE_EXECUTABLE: u32 = 10;
pub const MEMORY_BLOCK_NON_CACHEABLE_EXECUTABLE_READONLY: u32 = 11;
pub const MEMORY_BLOCK_NON_CACHEABLE_READWRITE: u32 = 12;
pub const MEMORY_BLOCK_NON_CACHEABLE_COPYONWRITE: u32 = 13;
pub const MEMORY_BLOCK_NON_CACHEABLE_EXECUTABLE_READWRITE: u32 = 14;
pub const MEMORY_BLOCK_NON_CACHEABLE_EXECUTABLE_COPYONWRITE: u32 = 15;
pub const MEMORY_BLOCK_NOT_ACCESSED_3: u32 = 16;
pub const MEMORY_BLOCK_GUARD_READONLY: u32 = 17;
pub const MEMORY_BLOCK_GUARD_EXECUTABLE: u32 = 18;
pub const MEMORY_BLOCK_GUARD_EXECUTABLE_READONLY: u32 = 19;
pub const MEMORY_BLOCK_GUARD_READWRITE: u32 = 20;
pub const MEMORY_BLOCK_GUARD_COPYONWRITE: u32 = 21;
pub const MEMORY_BLOCK_GUARD_EXECUTABLE_READWRITE: u32 = 22;
pub const MEMORY_BLOCK_GUARD_EXECUTABLE_COPYONWRITE: u32 = 23;
pub const MEMORY_BLOCK_NOT_ACCESSED_4: u32 = 24;
pub const MEMORY_BLOCK_NON_CACHEABLE_GUARD_READONLY: u32 = 25;
pub const MEMORY_BLOCK_NON_CACHEABLE_GUARD_EXECUTABLE: u32 = 26;
pub const MEMORY_BLOCK_NON_CACHEABLE_GUARD_EXECUTABLE_READONLY: u32 = 27;
pub const MEMORY_BLOCK_NON_CACHEABLE_GUARD_READWRITE: u32 = 28;
pub const MEMORY_BLOCK_NON_CACHEABLE_GUARD_COPYONWRITE: u32 = 29;
pub const MEMORY_BLOCK_NON_CACHEABLE_GUARD_EXECUTABLE_READWRITE: u32 = 30;
pub const MEMORY_BLOCK_NON_CACHEABLE_GUARD_EXECUTABLE_COPYONWRITE: u32 = 31;

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct MEMORY_WORKING_SET_BLOCK {
    pub Protection: u64,
    pub ShareCount: u64,
    pub Shared: u64,
    pub Node: u64,
    pub VirtualPage: u64,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct MEMORY_WORKING_SET_BLOCK {
    pub Protection: u32,
    pub ShareCount: u32,
    pub Shared: u32,
    pub Node: u32,
    pub VirtualPage: u32,
}

#[repr(C)]
pub struct MEMORY_WORKING_SET_INFORMATION {
    pub NumberOfEntries: ULONG_PTR,
    pub WorkingSetInfo: [MEMORY_WORKING_SET_BLOCK; 0],
}

#[repr(C)]
pub union MEMORY_REGION_INFORMATION_TYPE {
    pub RegionType: u32,
    pub bits: MEMORY_REGION_INFORMATION_TYPE_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_REGION_INFORMATION_TYPE_BITS {
    pub Private: u32,
    pub MappedDataFile: u32,
    pub MappedImage: u32,
    pub MappedPageFile: u32,
    pub MappedPhysical: u32,
    pub DirectMapped: u32,
    pub SoftwareEnclave: u32,
    pub PageSize64K: u32,
    pub PlaceholderReservation: u32,
    pub MappedAwe: u32,
    pub MappedWriteWatch: u32,
    pub PageSizeLarge: u32,
    pub PageSizeHuge: u32,
    pub Reserved: u32,
}

#[repr(C)]
pub struct MEMORY_REGION_INFORMATION {
    pub AllocationBase: PVOID,
    pub AllocationProtect: ULONG,
    pub RegionType: ULONG,
    pub RegionSize: SIZE_T,
    pub CommitSize: SIZE_T,
}

#[repr(C)]
pub struct MEMORY_REGION_INFORMATION_EX {
    pub AllocationBase: PVOID,
    pub AllocationProtect: ULONG,
    pub RegionType: ULONG,
    pub RegionSize: SIZE_T,
    pub CommitSize: SIZE_T,
    pub PartitionId: ULONG_PTR,
    pub NodePreference: ULONG_PTR,
}

#[repr(u32)]
pub enum MEMORY_WORKING_SET_EX_LOCATION {
    MemoryLocationInvalid = 0,
    MemoryLocationResident = 1,
    MemoryLocationPagefile = 2,
    MemoryLocationReserved = 3,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub union MEMORY_WORKING_SET_EX_BLOCK {
    pub Flags: ULONG_PTR,
    pub Valid: MEMORY_WORKING_SET_EX_BLOCK_VALID,
    pub Invalid: MEMORY_WORKING_SET_EX_BLOCK_INVALID,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_WORKING_SET_EX_BLOCK_VALID {
    pub Valid: u64,
    pub ShareCount: u64,
    pub Win32Protection: u64,
    pub Shared: u64,
    pub Node: u64,
    pub Locked: u64,
    pub LargePage: u64,
    pub Priority: u64,
    pub Reserved: u64,
    pub SharedOriginal: u64,
    pub Bad: u64,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_WORKING_SET_EX_BLOCK_INVALID {
    pub Valid: u64,
    pub Reserved0: u64,
    pub Shared: u64,
    pub Reserved1: u64,
    pub PageTable: u64,
    pub Location: u64,
    pub Priority: u64,
    pub ModifiedList: u64,
    pub Reserved2: u64,
    pub SharedOriginal: u64,
    pub Bad: u64,
}

#[repr(C)]
pub struct MEMORY_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: PVOID,
    pub VirtualAttributes: MEMORY_WORKING_SET_EX_BLOCK,
}

#[repr(C)]
pub struct MEMORY_SHARED_COMMIT_INFORMATION {
    pub CommitSize: SIZE_T,
}

#[repr(C)]
pub union MEMORY_IMAGE_INFORMATION_FLAGS {
    pub ImageFlags: ULONG,
    pub bits: MEMORY_IMAGE_INFORMATION_FLAGS_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_IMAGE_INFORMATION_FLAGS_BITS {
    pub ImagePartialMap: u32,
    pub ImageNotExecutable: u32,
    pub ImageSigningLevel: u32,
    pub Reserved: u32,
}

#[repr(C)]
pub struct MEMORY_IMAGE_INFORMATION {
    pub ImageBase: PVOID,
    pub SizeOfImage: SIZE_T,
    pub flags: MEMORY_IMAGE_INFORMATION_FLAGS,
}

#[repr(C)]
pub struct MEMORY_ENCLAVE_IMAGE_INFORMATION {
    pub ImageInfo: MEMORY_IMAGE_INFORMATION,
    pub UniqueID: [UCHAR; 32],
    pub AuthorID: [UCHAR; 32],
}

#[repr(u32)]
pub enum MEMORY_PHYSICAL_CONTIGUITY_UNIT_STATE {
    MemoryNotContiguous = 0,
    MemoryAlignedAndContiguous = 1,
    MemoryNotResident = 2,
    MemoryNotEligibleToMakeContiguous = 3,
    MemoryContiguityStateMax = 4,
}

#[repr(C)]
pub union MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION {
    pub AllInformation: ULONG,
    pub bits: MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION_BITS {
    pub State: u32,
    pub Reserved: u32,
}

#[repr(C)]
pub struct MEMORY_PHYSICAL_CONTIGUITY_INFORMATION {
    pub VirtualAddress: PVOID,
    pub Size: ULONG_PTR,
    pub ContiguityUnitSize: ULONG_PTR,
    pub Flags: ULONG,
    pub ContiguityUnitInformation: *mut MEMORY_PHYSICAL_CONTIGUITY_UNIT_INFORMATION,
}

#[repr(C)]
pub struct MEMORY_BAD_INFORMATION {
    pub BadAddress: PVOID,
    pub Length: ULONG_PTR,
    pub Flags: ULONG,
    pub Reserved: ULONG,
}

#[repr(C)]
pub struct RTL_SCP_CFG_ARM64_HEADER {
    pub EcInvalidCallHandlerRva: ULONG,
    pub EcCfgCheckRva: ULONG,
    pub EcCfgCheckESRva: ULONG,
    pub EcCallCheckRva: ULONG,
    pub CpuInitializationCompleteLoadRva: ULONG,
    pub LdrpValidateEcCallTargetInitRva: ULONG,
    pub SyscallFfsSizeRva: ULONG,
    pub SyscallFfsBaseRva: ULONG,
}

#[repr(u32)]
pub enum RTL_SCP_CFG_PAGE_TYPE {
    RtlScpCfgPageTypeNop = 0,
    RtlScpCfgPageTypeDefault = 1,
    RtlScpCfgPageTypeExportSuppression = 2,
    RtlScpCfgPageTypeFptr = 3,
    RtlScpCfgPageTypeMax = 4,
    RtlScpCfgPageTypeNone = 5,
}

#[repr(C)]
pub struct RTL_SCP_CFG_COMMON_HEADER {
    pub CfgDispatchRva: ULONG,
    pub CfgDispatchESRva: ULONG,
    pub CfgCheckRva: ULONG,
    pub CfgCheckESRva: ULONG,
    pub InvalidCallHandlerRva: ULONG,
    pub FnTableRva: ULONG,
}

#[repr(C)]
pub struct RTL_SCP_CFG_HEADER {
    pub Common: RTL_SCP_CFG_COMMON_HEADER,
}

#[repr(C)]
pub struct RTL_SCP_CFG_REGION_BOUNDS {
    pub StartAddress: PVOID,
    pub EndAddress: PVOID,
}

#[repr(C)]
pub struct RTL_SCP_CFG_NTDLL_EXPORTS {
    pub ScpRegions: [RTL_SCP_CFG_REGION_BOUNDS; 4],
    pub CfgDispatchFptr: PVOID,
    pub CfgDispatchESFptr: PVOID,
    pub CfgCheckFptr: PVOID,
    pub CfgCheckESFptr: PVOID,
    pub IllegalCallHandler: PVOID,
}

#[repr(C)]
pub struct RTL_SCP_CFG_NTDLL_EXPORTS_ARM64EC {
    pub EcInvalidCallHandler: PVOID,
    pub EcCfgCheckFptr: PVOID,
    pub EcCfgCheckESFptr: PVOID,
    pub EcCallCheckFptr: PVOID,
    pub CpuInitializationComplete: PVOID,
    pub LdrpValidateEcCallTargetInit: PVOID,
    pub SyscallFfsSize: PVOID,
    pub SyscallFfsBase: PVOID,
}

#[repr(C)]
pub struct RTL_RETPOLINE_ROUTINES {
    pub SwitchtableJump: [ULONG; 16],
    pub CfgIndirectRax: ULONG,
    pub NonCfgIndirectRax: ULONG,
    pub ImportR10: ULONG,
    pub JumpHpat: ULONG,
}

#[repr(C)]
pub struct RTL_KSCP_ROUTINES {
    pub UnwindDataOffset: ULONG,
    pub RetpolineRoutines: RTL_RETPOLINE_ROUTINES,
    pub CfgDispatchSmep: ULONG,
    pub CfgDispatchNoSmep: ULONG,
}

#[repr(u32)]
pub enum MEMORY_IMAGE_EXTENSION_TYPE {
    MemoryImageExtensionCfgScp = 0,
    MemoryImageExtensionCfgEmulatedScp = 1,
    MemoryImageExtensionTypeMax = 2,
}

#[repr(C)]
pub struct MEMORY_IMAGE_EXTENSION_INFORMATION {
    pub ExtensionType: MEMORY_IMAGE_EXTENSION_TYPE,
    pub Flags: ULONG,
    pub ExtensionImageBaseRva: PVOID,
    pub ExtensionSize: SIZE_T,
}

pub const MMPFNLIST_ZERO: u32 = 0;
pub const MMPFNLIST_FREE: u32 = 1;
pub const MMPFNLIST_STANDBY: u32 = 2;
pub const MMPFNLIST_MODIFIED: u32 = 3;
pub const MMPFNLIST_MODIFIEDNOWRITE: u32 = 4;
pub const MMPFNLIST_BAD: u32 = 5;
pub const MMPFNLIST_ACTIVE: u32 = 6;
pub const MMPFNLIST_TRANSITION: u32 = 7;

pub const MMPFNUSE_PROCESSPRIVATE: u32 = 0;
pub const MMPFNUSE_FILE: u32 = 1;
pub const MMPFNUSE_PAGEFILEMAPPED: u32 = 2;
pub const MMPFNUSE_PAGETABLE: u32 = 3;
pub const MMPFNUSE_PAGEDPOOL: u32 = 4;
pub const MMPFNUSE_NONPAGEDPOOL: u32 = 5;
pub const MMPFNUSE_SYSTEMPTE: u32 = 6;
pub const MMPFNUSE_SESSIONPRIVATE: u32 = 7;
pub const MMPFNUSE_METAFILE: u32 = 8;
pub const MMPFNUSE_AWEPAGE: u32 = 9;
pub const MMPFNUSE_DRIVERLOCKPAGE: u32 = 10;
pub const MMPFNUSE_KERNELSTACK: u32 = 11;

#[repr(C)]
pub struct MEMORY_FRAME_INFORMATION {
    pub UseDescription: u64,
    pub ListDescription: u64,
    pub Cold: u64,
    pub Pinned: u64,
    pub DontUse: u64,
    pub Priority: u64,
    pub NonTradeable: u64,
    pub Reserved: u64,
}

#[repr(C)]
pub struct FILEOFFSET_INFORMATION {
    pub DontUse: u64,
    pub Offset: u64,
    pub Reserved: u64,
}

#[repr(C)]
pub struct PAGEDIR_INFORMATION {
    pub DontUse: u64,
    pub PageDirectoryBase: u64,
    pub Reserved: u64,
}

#[repr(C)]
pub struct UNIQUE_PROCESS_INFORMATION {
    pub DontUse: u64,
    pub UniqueProcessKey: u64,
    pub Reserved: u64,
}

#[repr(C)]
pub struct MMPFN_IDENTITY {
    pub u1: MMPFN_IDENTITY_U1,
    pub PageFrameIndex: ULONG_PTR,
    pub u2: MMPFN_IDENTITY_U2,
}

#[repr(C)]
pub union MMPFN_IDENTITY_U1 {
    pub e1: core::mem::ManuallyDrop<MEMORY_FRAME_INFORMATION>,
    pub e2: core::mem::ManuallyDrop<FILEOFFSET_INFORMATION>,
    pub e3: core::mem::ManuallyDrop<PAGEDIR_INFORMATION>,
    pub e4: core::mem::ManuallyDrop<UNIQUE_PROCESS_INFORMATION>,
}

#[repr(C)]
pub union MMPFN_IDENTITY_U2 {
    pub e1: core::mem::ManuallyDrop<MMPFN_IDENTITY_U2_E1>,
    pub e2: core::mem::ManuallyDrop<MMPFN_IDENTITY_U2_E2>,
    pub FileObject: ULONG_PTR,
    pub UniqueFileObjectKey: ULONG_PTR,
    pub ProtoPteAddress: ULONG_PTR,
    pub VirtualAddress: ULONG_PTR,
}

#[repr(C)]
pub struct MMPFN_IDENTITY_U2_E1 {
    pub Image: ULONG_PTR,
    pub Mismatch: ULONG_PTR,
}

#[repr(C)]
pub struct MMPFN_IDENTITY_U2_E2 {
    pub CombinedPage: ULONG_PTR,
}

#[repr(C)]
pub struct MMPFN_MEMSNAP_INFORMATION {
    pub InitialPageFrameIndex: ULONG_PTR,
    pub Count: ULONG_PTR,
}

pub const MEM_EXECUTE_OPTION_DISABLE: u32 = 0x1;
pub const MEM_EXECUTE_OPTION_ENABLE: u32 = 0x2;
pub const MEM_EXECUTE_OPTION_DISABLE_THUNK_EMULATION: u32 = 0x4;
pub const MEM_EXECUTE_OPTION_PERMANENT: u32 = 0x8;
pub const MEM_EXECUTE_OPTION_EXECUTE_DISPATCH_ENABLE: u32 = 0x10;
pub const MEM_EXECUTE_OPTION_IMAGE_DISPATCH_ENABLE: u32 = 0x20;
pub const MEM_EXECUTE_OPTION_DISABLE_EXCEPTION_CHAIN_VALIDATION: u32 = 0x40;
pub const MEM_EXECUTE_OPTION_VALID_FLAGS: u32 = 0x7f;

#[repr(u32)]
pub enum VIRTUAL_MEMORY_INFORMATION_CLASS {
    VmPrefetchInformation = 0,
    VmPagePriorityInformation = 1,
    VmCfgCallTargetInformation = 2,
    VmPageDirtyStateInformation = 3,
    VmImageHotPatchInformation = 4,
    VmPhysicalContiguityInformation = 5,
    VmVirtualMachinePrepopulateInformation = 6,
    VmRemoveFromWorkingSetInformation = 7,
}

pub const VM_PREFETCH_TO_WORKING_SET: u32 = 0x1;

#[repr(C)]
pub struct MEMORY_PREFETCH_INFORMATION {
    pub Flags: ULONG,
}

pub const MEMORY_PRIORITY_LOWEST: u32 = 0;
pub const MEMORY_PRIORITY_VERY_LOW: u32 = 1;
pub const MEMORY_PRIORITY_LOW: u32 = 2;
pub const MEMORY_PRIORITY_MEDIUM: u32 = 3;
pub const MEMORY_PRIORITY_BELOW_NORMAL: u32 = 4;
pub const MEMORY_PRIORITY_NORMAL: u32 = 5;
pub const MEMORY_PRIORITY_ABOVE_NORMAL: u32 = 6;
pub const MEMORY_PRIORITY_HIGH: u32 = 7;

#[repr(C)]
pub struct MEMORY_PAGE_PRIORITY_INFORMATION {
    pub PagePriority: ULONG,
}

#[repr(C)]
pub struct CFG_CALL_TARGET_LIST_INFORMATION {
    pub NumberOfEntries: ULONG,
    pub Reserved: ULONG,
    pub NumberOfEntriesProcessed: PULONG,
    pub CallTargetInfo: *mut CFG_CALL_TARGET_INFO,
    pub Section: PVOID,
    pub FileOffset: ULONGLONG,
}

#[repr(C)]
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: ULONGLONG,
    pub Flags: ULONGLONG,
}

#[repr(C)]
pub struct MEMORY_PAGE_DIRTY_STATE_INFORMATION {
    pub Flags: ULONG,
}

#[repr(C)]
pub struct MEMORY_REMOVE_WORKING_SET_INFORMATION {
    pub Flags: ULONG,
}

#[repr(C)]
pub struct MEMORY_RANGE_ENTRY {
    pub VirtualAddress: PVOID,
    pub NumberOfBytes: SIZE_T,
}

pub const MAP_PROCESS: u32 = 1;
pub const MAP_SYSTEM: u32 = 2;

#[repr(u32)]
pub enum SECTION_INFORMATION_CLASS {
    SectionBasicInformation = 0,
    SectionImageInformation = 1,
    SectionRelocationInformation = 2,
    SectionOriginalBaseInformation = 3,
    SectionInternalImageInformation = 4,
}

#[repr(C)]
pub struct SECTION_BASIC_INFORMATION {
    pub BaseAddress: PVOID,
    pub AllocationAttributes: ULONG,
    pub MaximumSize: LARGE_INTEGER,
}

#[repr(C)]
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
pub struct SECTION_INTERNAL_IMAGE_INFORMATION {
    pub SectionInformation: SECTION_IMAGE_INFORMATION,
    pub ExtendedFlags: ULONG,
}

#[repr(u32)]
pub enum SECTION_INHERIT {
    ViewShare = 1,
    ViewUnmap = 2,
}

pub const MEMORY_CURRENT_PARTITION_HANDLE: HANDLE = -1isize as HANDLE;
pub const MEMORY_SYSTEM_PARTITION_HANDLE: HANDLE = -2isize as HANDLE;
pub const MEMORY_EXISTING_VAD_PARTITION_HANDLE: HANDLE = -3isize as HANDLE;

pub const MEMORY_PARTITION_QUERY_ACCESS: u32 = 0x0001;
pub const MEMORY_PARTITION_MODIFY_ACCESS: u32 = 0x0002;
pub const MEMORY_PARTITION_ALL_ACCESS: u32 = 0x001F0003;

#[repr(u32)]
pub enum PARTITION_INFORMATION_CLASS {
    SystemMemoryPartitionInformation = 0,
    SystemMemoryPartitionMoveMemory = 1,
    SystemMemoryPartitionAddPagefile = 2,
    SystemMemoryPartitionCombineMemory = 3,
    SystemMemoryPartitionInitialAddMemory = 4,
    SystemMemoryPartitionGetMemoryEvents = 5,
    SystemMemoryPartitionSetAttributes = 6,
    SystemMemoryPartitionNodeInformation = 7,
    SystemMemoryPartitionCreateLargePages = 8,
    SystemMemoryPartitionDedicatedMemoryInformation = 9,
    SystemMemoryPartitionOpenDedicatedMemory = 10,
    SystemMemoryPartitionMemoryChargeAttributes = 11,
    SystemMemoryPartitionClearAttributes = 12,
    SystemMemoryPartitionSetMemoryThresholds = 13,
    SystemMemoryPartitionMemoryListCommand = 14,
}

#[repr(C)]
pub struct MEMORY_PARTITION_CONFIGURATION_INFORMATION {
    pub Flags: ULONG,
    pub NumaNode: ULONG,
    pub Channel: ULONG,
    pub NumberOfNumaNodes: ULONG,
    pub ResidentAvailablePages: SIZE_T,
    pub CommittedPages: SIZE_T,
    pub CommitLimit: SIZE_T,
    pub PeakCommitment: SIZE_T,
    pub TotalNumberOfPages: SIZE_T,
    pub AvailablePages: SIZE_T,
    pub ZeroPages: SIZE_T,
    pub FreePages: SIZE_T,
    pub StandbyPages: SIZE_T,
    pub StandbyPageCountByPriority: [SIZE_T; 8],
    pub RepurposedPagesByPriority: [SIZE_T; 8],
    pub MaximumCommitLimit: SIZE_T,
    pub Reserved: SIZE_T,
    pub PartitionId: ULONG,
}

#[repr(C)]
pub struct MEMORY_PARTITION_TRANSFER_INFORMATION {
    pub NumberOfPages: SIZE_T,
    pub NumaNode: ULONG,
    pub Flags: ULONG,
}

#[repr(C)]
pub struct MEMORY_PARTITION_PAGEFILE_INFORMATION {
    pub PageFileName: UNICODE_STRING,
    pub MinimumSize: LARGE_INTEGER,
    pub MaximumSize: LARGE_INTEGER,
    pub Flags: ULONG,
}

#[repr(C)]
pub struct MEMORY_PARTITION_PAGE_COMBINE_INFORMATION {
    pub StopHandle: HANDLE,
    pub Flags: ULONG,
    pub TotalNumberOfPages: SIZE_T,
}

#[repr(C)]
pub struct MEMORY_PARTITION_PAGE_RANGE {
    pub StartPage: ULONG_PTR,
    pub NumberOfPages: ULONG_PTR,
}

#[repr(C)]
pub struct MEMORY_PARTITION_INITIAL_ADD_INFORMATION {
    pub Flags: ULONG,
    pub NumberOfRanges: ULONG,
    pub NumberOfPagesAdded: SIZE_T,
    pub PartitionRanges: [MEMORY_PARTITION_PAGE_RANGE; 1],
}

#[repr(C)]
pub struct MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION {
    pub Flags: MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_FLAGS,
    pub HandleAttributes: ULONG,
    pub DesiredAccess: ACCESS_MASK,
    pub LowCommitCondition: HANDLE,
    pub HighCommitCondition: HANDLE,
    pub MaximumCommitCondition: HANDLE,
}

#[repr(C)]
pub union MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_FLAGS {
    pub AllFlags: ULONG,
    pub bits: MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_FLAGS_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION_FLAGS_BITS {
    pub CommitEvents: u32,
    pub Spare: u32,
}

pub const TERMINATE_ENCLAVE_VALID_FLAGS: u32 = 0x00000005;
pub const TERMINATE_ENCLAVE_FLAG_NO_WAIT: u32 = 0x00000001;
pub const TERMINATE_ENCLAVE_FLAG_WAIT_ERROR: u32 = 0x00000004;

pub const ENCLAVE_CALL_VALID_FLAGS: u32 = 0x00000001;
pub const ENCLAVE_CALL_FLAG_NO_WAIT: u32 = 0x00000001;

pub type PENCLAVE_ROUTINE = Option<unsafe extern "system" fn() -> ()>;

unsafe extern "system" {
    pub fn NtAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        RegionSize: *mut SIZE_T,
        AllocationType: ULONG,
        PageProtection: ULONG,
    ) -> NTSTATUS;

    pub fn NtAllocateVirtualMemoryEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: *mut SIZE_T,
        AllocationType: ULONG,
        PageProtection: ULONG,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn NtFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: *mut SIZE_T,
        FreeType: ULONG,
    ) -> NTSTATUS;

    pub fn NtReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        NumberOfBytesToRead: SIZE_T,
        NumberOfBytesRead: *mut SIZE_T,
    ) -> NTSTATUS;

    pub fn NtWow64ReadVirtualMemory64(
        ProcessHandle: HANDLE,
        BaseAddress: ULONGLONG,
        Buffer: PVOID,
        NumberOfBytesToRead: ULONGLONG,
        NumberOfBytesRead: *mut ULONGLONG,
    ) -> NTSTATUS;

    pub fn NtReadVirtualMemoryEx(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        NumberOfBytesToRead: SIZE_T,
        NumberOfBytesRead: *mut SIZE_T,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn NtWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        NumberOfBytesToWrite: SIZE_T,
        NumberOfBytesWritten: *mut SIZE_T,
    ) -> NTSTATUS;

    pub fn NtWow64WriteVirtualMemory64(
        ProcessHandle: HANDLE,
        BaseAddress: ULONGLONG,
        Buffer: PVOID,
        NumberOfBytesToWrite: ULONGLONG,
        NumberOfBytesWritten: *mut ULONGLONG,
    ) -> NTSTATUS;

    pub fn NtProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: *mut SIZE_T,
        NewProtection: ULONG,
        OldProtection: *mut ULONG,
    ) -> NTSTATUS;

    pub fn NtQueryVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: PVOID,
        MemoryInformationLength: SIZE_T,
        ReturnLength: *mut SIZE_T,
    ) -> NTSTATUS;

    pub fn NtWow64QueryVirtualMemory64(
        ProcessHandle: HANDLE,
        BaseAddress: ULONGLONG,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: PVOID,
        MemoryInformationLength: ULONGLONG,
        ReturnLength: *mut ULONGLONG,
    ) -> NTSTATUS;

    pub fn NtFlushVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: *mut SIZE_T,
        IoStatus: *mut IO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn NtSetInformationVirtualMemory(
        ProcessHandle: HANDLE,
        VmInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS,
        NumberOfEntries: SIZE_T,
        VirtualAddresses: *mut MEMORY_RANGE_ENTRY,
        VmInformation: PVOID,
        VmInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: *mut SIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;

    pub fn NtUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: *mut SIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;

    pub fn NtCreateSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *const OBJECT_ATTRIBUTES,
        MaximumSize: *mut LARGE_INTEGER,
        SectionPageProtection: ULONG,
        AllocationAttributes: ULONG,
        FileHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn NtCreateSectionEx(
        SectionHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *const OBJECT_ATTRIBUTES,
        MaximumSize: *mut LARGE_INTEGER,
        SectionPageProtection: ULONG,
        AllocationAttributes: ULONG,
        FileHandle: HANDLE,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn NtOpenSection(
        SectionHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *const OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtMapViewOfSection(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        CommitSize: SIZE_T,
        SectionOffset: *mut LARGE_INTEGER,
        ViewSize: *mut SIZE_T,
        InheritDisposition: SECTION_INHERIT,
        AllocationType: ULONG,
        PageProtection: ULONG,
    ) -> NTSTATUS;

    pub fn NtMapViewOfSectionEx(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        SectionOffset: *mut LARGE_INTEGER,
        ViewSize: *mut SIZE_T,
        AllocationType: ULONG,
        PageProtection: ULONG,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn NtUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
    ) -> NTSTATUS;

    pub fn NtUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn NtExtendSection(
        SectionHandle: HANDLE,
        NewSectionSize: *mut LARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: PVOID,
        SectionInformationLength: SIZE_T,
        ReturnLength: *mut SIZE_T,
    ) -> NTSTATUS;

    pub fn NtAreMappedFilesTheSame(
        File1MappedAsAnImage: PVOID,
        File2MappedAsFile: PVOID,
    ) -> NTSTATUS;

    pub fn NtCreatePartition(
        ParentPartitionHandle: HANDLE,
        PartitionHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *const OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtOpenPartition(
        PartitionHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *const OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtManagePartition(
        TargetHandle: HANDLE,
        SourceHandle: HANDLE,
        PartitionInformationClass: PARTITION_INFORMATION_CLASS,
        PartitionInformation: PVOID,
        PartitionInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtMapUserPhysicalPages(
        VirtualAddress: PVOID,
        NumberOfPages: SIZE_T,
        UserPfnArray: *mut ULONG_PTR,
    ) -> NTSTATUS;

    pub fn NtMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut PVOID,
        NumberOfPages: SIZE_T,
        UserPfnArray: *mut ULONG_PTR,
    ) -> NTSTATUS;

    pub fn NtAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut SIZE_T,
        UserPfnArray: *mut ULONG_PTR,
    ) -> NTSTATUS;

    pub fn NtAllocateUserPhysicalPagesEx(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut ULONG_PTR,
        UserPfnArray: *mut ULONG_PTR,
        ExtendedParameters: *mut MEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn NtFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: *mut ULONG_PTR,
        UserPfnArray: *mut ULONG_PTR,
    ) -> NTSTATUS;

    pub fn NtGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: ULONG,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
        UserAddressArray: *mut PVOID,
        EntriesInUserAddressArray: *mut ULONG_PTR,
        Granularity: *mut ULONG,
    ) -> NTSTATUS;

    pub fn NtResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
    ) -> NTSTATUS;

    pub fn NtCreatePagingFile(
        PageFileName: *const UNICODE_STRING,
        MinimumSize: *mut LARGE_INTEGER,
        MaximumSize: *mut LARGE_INTEGER,
        Priority: ULONG,
    ) -> NTSTATUS;

    pub fn NtFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
    ) -> NTSTATUS;

    pub fn NtFlushWriteBuffer() -> NTSTATUS;

    pub fn NtFlushProcessWriteBuffers() -> NTSTATUS;

    pub fn NtCreateEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        Size: SIZE_T,
        InitialCommitment: SIZE_T,
        EnclaveType: ULONG,
        EnclaveInformation: PVOID,
        EnclaveInformationLength: ULONG,
        EnclaveError: *mut ULONG,
    ) -> NTSTATUS;

    pub fn NtLoadEnclaveData(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        Protect: ULONG,
        PageInformation: PVOID,
        PageInformationLength: ULONG,
        NumberOfBytesWritten: *mut SIZE_T,
        EnclaveError: *mut ULONG,
    ) -> NTSTATUS;

    pub fn NtInitializeEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        EnclaveInformation: PVOID,
        EnclaveInformationLength: ULONG,
        EnclaveError: *mut ULONG,
    ) -> NTSTATUS;

    pub fn NtTerminateEnclave(
        BaseAddress: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn NtCallEnclave(
        Routine: PENCLAVE_ROUTINE,
        Reserved: PVOID,
        Flags: ULONG,
        RoutineParamReturn: *mut PVOID,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct MEM_EXTENDED_PARAMETER {
    pub Type: u64,
    pub Reserved: u64,
    pub Pointer: PVOID,
    pub Size: u64,
    pub Handle: HANDLE,
    pub ULong: ULONG,
}

pub type PMEMORY_WORKING_SET_BLOCK = *mut MEMORY_WORKING_SET_BLOCK;
pub type PMEMORY_WORKING_SET_INFORMATION = *mut MEMORY_WORKING_SET_INFORMATION;
pub type PMEMORY_REGION_INFORMATION = *mut MEMORY_REGION_INFORMATION;
pub type PMEMORY_REGION_INFORMATION_EX = *mut MEMORY_REGION_INFORMATION_EX;
pub type PMEMORY_WORKING_SET_EX_BLOCK = *mut MEMORY_WORKING_SET_EX_BLOCK;
pub type PMEMORY_WORKING_SET_EX_INFORMATION = *mut MEMORY_WORKING_SET_EX_INFORMATION;
pub type PMEMORY_SHARED_COMMIT_INFORMATION = *mut MEMORY_SHARED_COMMIT_INFORMATION;
pub type PMEMORY_IMAGE_INFORMATION = *mut MEMORY_IMAGE_INFORMATION;
pub type PMEMORY_ENCLAVE_IMAGE_INFORMATION = *mut MEMORY_ENCLAVE_IMAGE_INFORMATION;
pub type PMEMORY_RANGE_ENTRY = *mut MEMORY_RANGE_ENTRY;
pub type PSECTION_BASIC_INFORMATION = *mut SECTION_BASIC_INFORMATION;
pub type PSECTION_IMAGE_INFORMATION = *mut SECTION_IMAGE_INFORMATION;
pub type PSECTION_INTERNAL_IMAGE_INFORMATION = *mut SECTION_INTERNAL_IMAGE_INFORMATION;
pub type PMEMORY_PARTITION_CONFIGURATION_INFORMATION = *mut MEMORY_PARTITION_CONFIGURATION_INFORMATION;
pub type PMEMORY_PARTITION_TRANSFER_INFORMATION = *mut MEMORY_PARTITION_TRANSFER_INFORMATION;
pub type PMEMORY_PARTITION_PAGEFILE_INFORMATION = *mut MEMORY_PARTITION_PAGEFILE_INFORMATION;
pub type PMEMORY_PARTITION_PAGE_COMBINE_INFORMATION = *mut MEMORY_PARTITION_PAGE_COMBINE_INFORMATION;
pub type PMEMORY_PARTITION_PAGE_RANGE = *mut MEMORY_PARTITION_PAGE_RANGE;
pub type PMEMORY_PARTITION_INITIAL_ADD_INFORMATION = *mut MEMORY_PARTITION_INITIAL_ADD_INFORMATION;
pub type PMEMORY_PARTITION_MEMORY_EVENTS_INFORMATION = *mut MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION;
pub type PMEM_EXTENDED_PARAMETER = *mut MEM_EXTENDED_PARAMETER;