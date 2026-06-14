use core::mem::ManuallyDrop;

use crate::{ntdef::*, ntrtl::*};

#[repr(C)]
pub union TEB_u {
    pub CurrentIdealProcessor: ManuallyDrop<PROCESSOR_NUMBER>,
    pub IdealProcessorValue: ULONG,
    pub s: ManuallyDrop<TEB_u_s>,
}

#[repr(C)]
pub struct TEB_u_s {
    pub BypassProcessFree: ULONG,
    pub Reserved: ULONG,
}

#[repr(C)]
pub struct NT_TIB {
    pub ExceptionList: *mut EXCEPTION_REGISTRATION_RECORD,
    pub StackBase: PVOID,
    pub StackLimit: PVOID,
    pub SubSystemTib: PVOID,
    pub u: NT_TIB_u,
    pub ArbitraryUserPointer: PVOID,
    pub _Self: *mut NT_TIB,
}

#[repr(C)]
pub union NT_TIB_u {
    pub FiberData: PVOID,
    pub Version: ULONG,
}

pub type PNT_TIB = *mut NT_TIB;
pub type PEXCEPTION_REGISTRATION_RECORD = *mut EXCEPTION_REGISTRATION_RECORD;

#[repr(C)]
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: PEXCEPTION_REGISTRATION_RECORD,
    pub Handler: PVOID,
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
pub struct TEB {
    pub NtTib: NT_TIB,
    pub EnvironmentPointer: PVOID,
    pub ClientId: CLIENT_ID,
    pub ActiveRpcHandle: PVOID,
    pub ThreadLocalStoragePointer: PVOID,
    pub ProcessEnvironmentBlock: PPEB,
    pub LastErrorValue: ULONG,
    pub CountOfOwnedCriticalSections: ULONG,
    pub CsrClientThread: PVOID,
    pub Win32ThreadInfo: PVOID,
    pub User32Reserved: [ULONG; 26],
    pub UserReserved: [ULONG; 5],
    pub WOW32Reserved: PVOID,
    pub CurrentLocale: LCID,
    pub FpSoftwareStatusRegister: ULONG,
    pub ReservedForDebuggerInstrumentation: [PVOID; 16],
    pub SystemReserved1: [PVOID; 30],
    pub PlaceholderCompatibilityMode: CHAR,
    pub PlaceholderReserved: [CHAR; 11],
    pub ProxiedProcessId: ULONG,
    pub ActivationStack: ACTIVATION_CONTEXT_STACK,
    pub WorkingOnBehalfTicket: [UCHAR; 8],
    pub ExceptionCode: NTSTATUS,
    pub ActivationContextStackPointer: PACTIVATION_CONTEXT_STACK,
    pub InstrumentationCallbackSp: ULONG_PTR,
    pub InstrumentationCallbackPreviousPc: ULONG_PTR,
    pub InstrumentationCallbackPreviousSp: ULONG_PTR,
    pub TxFsContext: ULONG,
    pub InstrumentationCallbackDisabled: BOOLEAN,
    pub GdiTebBatch: GDI_TEB_BATCH,
    pub RealClientId: CLIENT_ID,
    pub GdiCachedProcessHandle: HANDLE,
    pub GdiClientPID: ULONG,
    pub GdiClientTID: ULONG,
    pub GdiThreadLocalInfo: PVOID,
    pub Win32ClientInfo: [ULONG_PTR; 62],
    pub glDispatchTable: [PVOID; 233],
    pub glReserved1: [ULONG_PTR; 29],
    pub glReserved2: PVOID,
    pub glSectionInfo: PVOID,
    pub glSection: PVOID,
    pub glTable: PVOID,
    pub glCurrentRC: PVOID,
    pub glContext: PVOID,
    pub LastStatusValue: NTSTATUS,
    pub StaticUnicodeString: UNICODE_STRING,
    pub StaticUnicodeBuffer: [WCHAR; 261],
    pub DeallocationStack: PVOID,
    pub TlsSlots: [PVOID; 64],
    pub TlsLinks: LIST_ENTRY,
    pub Vdm: PVOID,
    pub ReservedForNtRpc: PVOID,
    pub DbgSsReserved: [PVOID; 2],
    pub HardErrorMode: ULONG,
    pub Instrumentation: [PVOID; 11],
    pub ActivityId: GUID,
    pub SubProcessTag: PVOID,
    pub PerflibData: PVOID,
    pub EtwTraceData: PVOID,
    pub WinSockData: PVOID,
    pub GdiBatchCount: ULONG,
    pub u: TEB_u,
    pub GuaranteedStackBytes: ULONG,
    pub ReservedForPerf: PVOID,
    pub ReservedForOle: PVOID,
    pub WaitingOnLoaderLock: ULONG,
    pub SavedPriorityState: PVOID,
    pub ReservedForCodeCoverage: ULONG_PTR,
    pub ThreadPoolData: PVOID,
    pub TlsExpansionSlots: *mut PVOID,
    pub DeallocationBStore: PVOID,
    pub BStoreLimit: PVOID,
    pub MuiGeneration: ULONG,
    pub IsImpersonating: ULONG,
    pub NlsCache: PVOID,
    pub pShimData: PVOID,
    pub HeapVirtualAffinity: USHORT,
    pub LowFragHeapDataSlot: USHORT,
    pub CurrentTransactionHandle: HANDLE,
    pub ActiveFrame: PTEB_ACTIVE_FRAME,
    pub FlsData: PVOID,
    pub PreferredLanguages: PVOID,
    pub UserPrefLanguages: PVOID,
    pub MergedPrefLanguages: PVOID,
    pub MuiImpersonation: ULONG,
    pub CrossTebFlags: USHORT,
    pub SameTebFlags: USHORT,
    pub TxnScopeEnterCallback: PVOID,
    pub TxnScopeExitCallback: PVOID,
    pub TxnScopeContext: PVOID,
    pub LockCount: ULONG,
    pub WowTebOffset: LONG,
    pub ResourceRetValue: PVOID,
    pub ReservedForWdf: PVOID,
    pub ReservedForCrt: ULONGLONG,
    pub EffectiveContainerId: GUID,
}

#[repr(C)]
pub struct ACTIVATION_CONTEXT_STACK {
    pub ActiveFrame: *mut RTL_ACTIVATION_CONTEXT_STACK_FRAME,
    pub FrameListCache: LIST_ENTRY,
    pub Flags: ULONG,
    pub NextCookieSequenceNumber: ULONG,
    pub StackId: ULONG,
}

pub type PACTIVATION_CONTEXT_STACK = *mut ACTIVATION_CONTEXT_STACK;

pub type PGDI_TEB_BATCH = *mut GDI_TEB_BATCH;

#[repr(C)]
pub struct TEB_ACTIVE_FRAME_CONTEXT {
    pub Flags: ULONG,
    pub FrameName: PSTR,
}

pub type PTEB_ACTIVE_FRAME_CONTEXT = *mut TEB_ACTIVE_FRAME_CONTEXT;

#[repr(C)]
pub struct TEB_ACTIVE_FRAME {
    pub Flags: ULONG,
    pub Previous: *mut TEB_ACTIVE_FRAME,
    pub Context: PTEB_ACTIVE_FRAME_CONTEXT,
}

pub type PTEB_ACTIVE_FRAME = *mut TEB_ACTIVE_FRAME;

#[repr(C)]
pub struct ACTIVATION_CONTEXT {
    pub dummy: PVOID,
}

#[repr(C)]
pub struct GDI_TEB_BATCH {
    pub Offset: u32,
    pub InProcessing: u32,
    pub HasRenderingCommand: u32,
    pub HDC: ULONGLONG,
    pub Buffer: [ULONG; 310],
}

#[repr(C)]
pub struct RTL_ACTIVATION_CONTEXT_STACK_FRAME {
    pub Previous: PRTL_ACTIVATION_CONTEXT_STACK_FRAME,
    pub ActivationContext: *mut ACTIVATION_CONTEXT,
    pub Flags: ULONG,
}

pub type PRTL_ACTIVATION_CONTEXT_STACK_FRAME = *mut RTL_ACTIVATION_CONTEXT_STACK_FRAME;
pub type PACTIVATION_CONTEXT = *mut ACTIVATION_CONTEXT;


pub type PTEB = *mut TEB;
pub type LCID = ULONG;

#[repr(C)]
pub struct PEB {
    pub InheritedAddressSpace: BOOLEAN,
    pub ReadImageFileExecOptions: BOOLEAN,
    pub BeingDebugged: BOOLEAN,
    pub BitField: BOOLEAN,
    pub Mutant: HANDLE,
    pub ImageBaseAddress: PVOID,
    pub Ldr: PPEB_LDR_DATA,
    pub ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    pub SubSystemData: PVOID,
    pub ProcessHeap: PVOID,
    pub FastPebLock: PRTL_CRITICAL_SECTION,
    pub IFEOKey: PVOID,
    pub AtlThunkSListPtr: PSLIST_HEADER,
    pub CrossProcessFlags: ULONG,
    pub u: PEB_U,
    pub SystemReserved: [ULONG; 1],
    pub AtlThunkSListPtr32: ULONG,
    pub ApiSetMap: PAPI_SET_NAMESPACE,
    pub TlsExpansionCounter: ULONG,
    pub TlsBitmap: PVOID,
    pub TlsBitmapBits: [ULONG; 2],
    pub ReadOnlySharedMemoryBase: PVOID,
    pub SharedData: PVOID,
    pub ReadOnlyStaticServerData: *mut PVOID,
    pub AnsiCodePageData: PVOID,
    pub OemCodePageData: PVOID,
    pub UnicodeCaseTableData: PVOID,
    pub NumberOfProcessors: ULONG,
    pub NtGlobalFlag: ULONG,
    pub CriticalSectionTimeout: ULARGE_INTEGER,
    pub HeapSegmentReserve: SIZE_T,
    pub HeapSegmentCommit: SIZE_T,
    pub HeapDeCommitTotalFreeThreshold: SIZE_T,
    pub HeapDeCommitFreeBlockThreshold: SIZE_T,
    pub NumberOfHeaps: ULONG,
    pub MaximumNumberOfHeaps: ULONG,
    pub ProcessHeaps: *mut PVOID,
    pub GdiSharedHandleTable: PVOID,
    pub ProcessStarterHelper: PVOID,
    pub GdiDCAttributeList: ULONG,
    pub LoaderLock: PRTL_CRITICAL_SECTION,
    pub OSMajorVersion: ULONG,
    pub OSMinorVersion: ULONG,
    pub OSBuildNumber: USHORT,
    pub OSCSDVersion: USHORT,
    pub OSPlatformId: ULONG,
    pub ImageSubsystem: ULONG,
    pub ImageSubsystemMajorVersion: ULONG,
    pub ImageSubsystemMinorVersion: ULONG,
    pub ActiveProcessAffinityMask: ULONG_PTR,
    pub GdiHandleBuffer: GDI_HANDLE_BUFFER,
    pub PostProcessInitRoutine: PVOID,
    pub TlsExpansionBitmap: PVOID,
    pub TlsExpansionBitmapBits: [ULONG; 32],
    pub SessionId: ULONG,
    pub AppCompatFlags: ULARGE_INTEGER,
    pub AppCompatFlagsUser: ULARGE_INTEGER,
    pub pShimData: PVOID,
    pub AppCompatInfo: PVOID,
    pub CSDVersion: UNICODE_STRING,
    pub ActivationContextData: PVOID,
    pub ProcessAssemblyStorageMap: PVOID,
    pub SystemDefaultActivationContextData: PVOID,
    pub SystemAssemblyStorageMap: PVOID,
    pub MinimumStackCommit: SIZE_T,
    pub FlsCallback: *mut PVOID,
    pub FlsListHead: LIST_ENTRY,
    pub FlsBitmap: PVOID,
    pub FlsBitmapBits: [ULONG; FLS_MAXIMUM_AVAILABLE as usize / core::mem::size_of::<ULONG>() / 8],
    pub FlsHighIndex: ULONG,
    pub WerRegistrationData: PVOID,
    pub WerShipAssertPtr: PVOID,
    pub pUnused: PVOID,
    pub pImageHeaderHash: PVOID,
    pub TracingFlags: ULONG,
    pub CsrServerReadOnlySharedMemoryBase: ULONGLONG,
    pub TppWorkerpListLock: PRTL_CRITICAL_SECTION,
    pub TppWorkerpList: LIST_ENTRY,
    pub WaitOnAddressHashTable: [PVOID; 128],
    pub TelemetryCoverageHeader: PVOID,
    pub CloudFileFlags: ULONG,
    pub CloudFileDiagFlags: ULONG,
    pub PlaceholderCompatibilityMode: CHAR,
    pub PlaceholderCompatibilityModeReserved: [CHAR; 7],
    pub LeapSecondData: *mut LEAP_SECOND_DATA,
    pub LeapSecondFlags: ULONG,
    pub NtGlobalFlag2: ULONG,
}

#[repr(C)]
pub union PEB_U {
    pub CrossProcessFlags: ULONG,
    pub CrossProcessFlagsBits: ManuallyDrop<PEB_U_BITS>,
}

#[repr(C)]
pub struct PEB_U_BITS {
    pub ProcessInJob: ULONG,
    pub ProcessInitializing: ULONG,
    pub ProcessUsingVEH: ULONG,
    pub ProcessUsingVCH: ULONG,
    pub ProcessUsingFTH: ULONG,
    pub ProcessPreviouslyThrottled: ULONG,
    pub ProcessCurrentlyThrottled: ULONG,
    pub ProcessImagesHotPatched: ULONG,
    pub ReservedBits0: ULONG,
}

#[repr(C)]
pub struct PEB_LDR_DATA {
    pub Length: ULONG,
    pub Initialized: BOOLEAN,
    pub SsHandle: HANDLE,
    pub InLoadOrderModuleList: LIST_ENTRY,
    pub InMemoryOrderModuleList: LIST_ENTRY,
    pub InInitializationOrderModuleList: LIST_ENTRY,
    pub EntryInProgress: PVOID,
    pub ShutdownInProgress: BOOLEAN,
    pub ShutdownThreadId: HANDLE,
}

#[repr(C)]
pub struct API_SET_NAMESPACE {
    pub Version: ULONG,
    pub Size: ULONG,
    pub Flags: ULONG,
    pub Count: ULONG,
    pub EntryOffset: ULONG,
    pub HashOffset: ULONG,
    pub HashFactor: ULONG,
}

#[repr(C)]
pub struct LEAP_SECOND_DATA {
    pub Enabled: BOOLEAN,
    pub Count: ULONG,
    pub Data: [LEAP_SECOND_ENTRY; 1],
}

#[repr(C)]
pub struct LEAP_SECOND_ENTRY {
    pub LeapSecond: LARGE_INTEGER,
    pub Adjustment: LONG,
}

pub type PLEAP_SECOND_DATA = *mut LEAP_SECOND_DATA;

pub const FLS_MAXIMUM_AVAILABLE: ULONG = 128;

pub type PPEB = *mut PEB;
pub type PPEB_LDR_DATA = *mut PEB_LDR_DATA;
pub type PAPI_SET_NAMESPACE = *mut API_SET_NAMESPACE;
pub type GDI_HANDLE_BUFFER = [PVOID; 34];
pub type PRTL_CRITICAL_SECTION = *mut RTL_CRITICAL_SECTION;
pub type PSLIST_HEADER = *mut SLIST_HEADER;