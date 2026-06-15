use core::ffi::c_void;

use crate::{ntdef::*, ntioapi::*, ntlpcapi::PPORT_MESSAGE, ntpoapi::PCOUNTED_REASON_CONTEXT, ntrtl::CLIENT_ID};

pub const EVENT_QUERY_STATE: u32 = 0x0001;
pub const EVENT_MODIFY_STATE: u32 = 0x0002;
pub const EVENT_ALL_ACCESS: u32 = EVENT_QUERY_STATE | EVENT_MODIFY_STATE | 0x000F0000 | 0x00100000;

#[repr(u32)]
pub enum EVENT_INFORMATION_CLASS {
    EventBasicInformation,
}

#[repr(C)]
pub struct EVENT_BASIC_INFORMATION {
    pub EventType: EVENT_TYPE,
    pub EventState: LONG,
}

pub type PEVENT_BASIC_INFORMATION = *mut EVENT_BASIC_INFORMATION;

pub const MUTANT_QUERY_STATE: u32 = 0x0001;
pub const MUTANT_ALL_ACCESS: u32 = MUTANT_QUERY_STATE | 0x000F0000 | 0x00100000;

#[repr(u32)]
pub enum MUTANT_INFORMATION_CLASS {
    MutantBasicInformation,
    MutantOwnerInformation,
}

#[repr(C)]
pub struct MUTANT_BASIC_INFORMATION {
    pub CurrentCount: LONG,
    pub OwnedByCaller: BOOLEAN,
    pub AbandonedState: BOOLEAN,
}

pub type PMUTANT_BASIC_INFORMATION = *mut MUTANT_BASIC_INFORMATION;

#[repr(C)]
pub struct MUTANT_OWNER_INFORMATION {
    pub ClientId: CLIENT_ID,
}

pub type PMUTANT_OWNER_INFORMATION = *mut MUTANT_OWNER_INFORMATION;

pub const SEMAPHORE_QUERY_STATE: u32 = 0x0001;
pub const SEMAPHORE_MODIFY_STATE: u32 = 0x0002;
pub const SEMAPHORE_ALL_ACCESS: u32 = SEMAPHORE_QUERY_STATE | SEMAPHORE_MODIFY_STATE | 0x000F0000 | 0x00100000;

#[repr(u32)]
pub enum SEMAPHORE_INFORMATION_CLASS {
    SemaphoreBasicInformation,
}

#[repr(C)]
pub struct SEMAPHORE_BASIC_INFORMATION {
    pub CurrentCount: LONG,
    pub MaximumCount: LONG,
}

pub type PSEMAPHORE_BASIC_INFORMATION = *mut SEMAPHORE_BASIC_INFORMATION;

pub const TIMER_QUERY_STATE: u32 = 0x0001;
pub const TIMER_MODIFY_STATE: u32 = 0x0002;
pub const TIMER_ALL_ACCESS: u32 = TIMER_QUERY_STATE | TIMER_MODIFY_STATE | 0x000F0000 | 0x00100000;

#[repr(u32)]
pub enum TIMER_INFORMATION_CLASS {
    TimerBasicInformation,
}

#[repr(u32)]
pub enum TIMER_SET_INFORMATION_CLASS {
    TimerSetCoalescableTimer,
    MaxTimerInfoClass,
}

#[repr(C)]
pub struct TIMER_BASIC_INFORMATION {
    pub RemainingTime: LARGE_INTEGER,
    pub TimerState: BOOLEAN,
}

pub type PTIMER_BASIC_INFORMATION = *mut TIMER_BASIC_INFORMATION;

pub type PTIMER_APC_ROUTINE = Option<unsafe extern "system" fn(TimerContext: PVOID, TimerLowValue: ULONG, TimerHighValue: LONG)>;

#[repr(C)]
pub struct TIMER_SET_COALESCABLE_TIMER_INFO {
    pub DueTime: LARGE_INTEGER,
    pub TimerApcRoutine: PTIMER_APC_ROUTINE,
    pub TimerContext: PVOID,
    pub WakeContext: PCOUNTED_REASON_CONTEXT,
    pub Period: ULONG,
    pub TolerableDelay: ULONG,
    pub PreviousState: PBOOLEAN,
}

pub type PTIMER_SET_COALESCABLE_TIMER_INFO = *mut TIMER_SET_COALESCABLE_TIMER_INFO;

pub const KEYEDEVENT_WAIT: u32 = 0x0001;
pub const KEYEDEVENT_WAKE: u32 = 0x0002;
pub const KEYEDEVENT_ALL_ACCESS: u32 = 0x000F0000 | KEYEDEVENT_WAIT | KEYEDEVENT_WAKE;

pub const WORKER_FACTORY_RELEASE_WORKER: u32 = 0x0001;
pub const WORKER_FACTORY_WAIT: u32 = 0x0002;
pub const WORKER_FACTORY_SET_INFORMATION: u32 = 0x0004;
pub const WORKER_FACTORY_QUERY_INFORMATION: u32 = 0x0008;
pub const WORKER_FACTORY_READY_WORKER: u32 = 0x0010;
pub const WORKER_FACTORY_SHUTDOWN: u32 = 0x0020;
pub const WORKER_FACTORY_ALL_ACCESS: u32 = 0x000F0000
    | WORKER_FACTORY_RELEASE_WORKER
    | WORKER_FACTORY_WAIT
    | WORKER_FACTORY_SET_INFORMATION
    | WORKER_FACTORY_QUERY_INFORMATION
    | WORKER_FACTORY_READY_WORKER
    | WORKER_FACTORY_SHUTDOWN;

#[repr(u32)]
pub enum WORKERFACTORYINFOCLASS {
    WorkerFactoryTimeout,
    WorkerFactoryRetryTimeout,
    WorkerFactoryIdleTimeout,
    WorkerFactoryBindingCount,
    WorkerFactoryThreadMinimum,
    WorkerFactoryThreadMaximum,
    WorkerFactoryPaused,
    WorkerFactoryBasicInformation,
    WorkerFactoryAdjustThreadGoal,
    WorkerFactoryCallbackType,
    WorkerFactoryStackInformation,
    WorkerFactoryThreadBasePriority,
    WorkerFactoryTimeoutWaiters,
    WorkerFactoryFlags,
    WorkerFactoryThreadSoftMaximum,
    WorkerFactoryThreadCpuSets,
    MaxWorkerFactoryInfoClass,
}

#[repr(C)]
pub struct WORKER_FACTORY_BASIC_INFORMATION {
    pub Timeout: LARGE_INTEGER,
    pub RetryTimeout: LARGE_INTEGER,
    pub IdleTimeout: LARGE_INTEGER,
    pub Paused: BOOLEAN,
    pub TimerSet: BOOLEAN,
    pub QueuedToExWorker: BOOLEAN,
    pub MayCreate: BOOLEAN,
    pub CreateInProgress: BOOLEAN,
    pub InsertedIntoQueue: BOOLEAN,
    pub Shutdown: BOOLEAN,
    pub BindingCount: ULONG,
    pub ThreadMinimum: ULONG,
    pub ThreadMaximum: ULONG,
    pub PendingWorkerCount: ULONG,
    pub WaitingWorkerCount: ULONG,
    pub TotalWorkerCount: ULONG,
    pub ReleaseCount: ULONG,
    pub InfiniteWaitGoal: LONGLONG,
    pub StartRoutine: PVOID,
    pub StartParameter: PVOID,
    pub ProcessId: HANDLE,
    pub StackReserve: SIZE_T,
    pub StackCommit: SIZE_T,
    pub LastThreadCreationStatus: NTSTATUS,
}

pub type PWORKER_FACTORY_BASIC_INFORMATION = *mut WORKER_FACTORY_BASIC_INFORMATION;

#[repr(C)]
pub struct WORKER_FACTORY_DEFERRED_WORK {
    pub AlpcSendMessage: PPORT_MESSAGE,
    pub AlpcSendMessagePort: PVOID,
    pub AlpcSendMessageFlags: ULONG,
    pub Flags: ULONG,
}

pub type PWORKER_FACTORY_DEFERRED_WORK = *mut WORKER_FACTORY_DEFERRED_WORK;

pub const ATOM_FLAG_NONE: u32 = 0x0;
pub const ATOM_FLAG_GLOBAL: u32 = 0x2;

#[repr(u32)]
pub enum ATOM_INFORMATION_CLASS {
    AtomBasicInformation,
    AtomTableInformation,
}

#[repr(C)]
pub struct ATOM_BASIC_INFORMATION {
    pub UsageCount: USHORT,
    pub Flags: USHORT,
    pub NameLength: USHORT,
    pub Name: [WCHAR; 1],
}

pub type PATOM_BASIC_INFORMATION = *mut ATOM_BASIC_INFORMATION;

#[repr(C)]
pub struct ATOM_TABLE_INFORMATION {
    pub NumberOfAtoms: ULONG,
    pub Atoms: [RTL_ATOM; 1],
}

pub type PATOM_TABLE_INFORMATION = *mut ATOM_TABLE_INFORMATION;

pub const FLG_STOP_ON_EXCEPTION: u32 = 0x00000001;
pub const FLG_SHOW_LDR_SNAPS: u32 = 0x00000002;
pub const FLG_DEBUG_INITIAL_COMMAND: u32 = 0x00000004;
pub const FLG_STOP_ON_HUNG_GUI: u32 = 0x00000008;
pub const FLG_HEAP_ENABLE_TAIL_CHECK: u32 = 0x00000010;
pub const FLG_HEAP_ENABLE_FREE_CHECK: u32 = 0x00000020;
pub const FLG_HEAP_VALIDATE_PARAMETERS: u32 = 0x00000040;
pub const FLG_HEAP_VALIDATE_ALL: u32 = 0x00000080;
pub const FLG_APPLICATION_VERIFIER: u32 = 0x00000100;
pub const FLG_MONITOR_SILENT_PROCESS_EXIT: u32 = 0x00000200;
pub const FLG_POOL_ENABLE_TAGGING: u32 = 0x00000400;
pub const FLG_HEAP_ENABLE_TAGGING: u32 = 0x00000800;
pub const FLG_USER_STACK_TRACE_DB: u32 = 0x00001000;
pub const FLG_KERNEL_STACK_TRACE_DB: u32 = 0x00002000;
pub const FLG_MAINTAIN_OBJECT_TYPELIST: u32 = 0x00004000;
pub const FLG_HEAP_ENABLE_TAG_BY_DLL: u32 = 0x00008000;
pub const FLG_DISABLE_STACK_EXTENSION: u32 = 0x00010000;
pub const FLG_ENABLE_CSRDEBUG: u32 = 0x00020000;
pub const FLG_ENABLE_KDEBUG_SYMBOL_LOAD: u32 = 0x00040000;
pub const FLG_DISABLE_PAGE_KERNEL_STACKS: u32 = 0x00080000;
pub const FLG_ENABLE_SYSTEM_CRIT_BREAKS: u32 = 0x00100000;
pub const FLG_HEAP_DISABLE_COALESCING: u32 = 0x00200000;
pub const FLG_ENABLE_CLOSE_EXCEPTIONS: u32 = 0x00400000;
pub const FLG_ENABLE_EXCEPTION_LOGGING: u32 = 0x00800000;
pub const FLG_ENABLE_HANDLE_TYPE_TAGGING: u32 = 0x01000000;
pub const FLG_HEAP_PAGE_ALLOCS: u32 = 0x02000000;
pub const FLG_DEBUG_INITIAL_COMMAND_EX: u32 = 0x04000000;
pub const FLG_DISABLE_DBGPRINT: u32 = 0x08000000;
pub const FLG_CRITSEC_EVENT_CREATION: u32 = 0x10000000;
pub const FLG_LDR_TOP_DOWN: u32 = 0x20000000;
pub const FLG_ENABLE_HANDLE_EXCEPTIONS: u32 = 0x40000000;
pub const FLG_DISABLE_PROTDLLS: u32 = 0x80000000;

#[repr(u32)]
pub enum SHUTDOWN_ACTION {
    ShutdownNoReboot,
    ShutdownReboot,
    ShutdownPowerOff,
    ShutdownRebootForRecovery,
}

#[repr(u32)]
pub enum HOT_PATCH_INFORMATION_CLASS {
    ManageHotPatchLoadPatch,
    ManageHotPatchUnloadPatch,
    ManageHotPatchQueryPatches,
    ManageHotPatchLoadPatchForUser,
    ManageHotPatchUnloadPatchForUser,
    ManageHotPatchQueryPatchesForUser,
    ManageHotPatchQueryActivePatches,
    ManageHotPatchApplyImagePatch,
    ManageHotPatchQuerySinglePatch,
    ManageHotPatchCheckEnabled,
    ManageHotPatchCreatePatchSection,
    ManageHotPatchMax,
}

#[repr(C)]
pub struct HOT_PATCH_IMAGE_INFO {
    pub CheckSum: ULONG,
    pub TimeDateStamp: ULONG,
}

pub type PHOT_PATCH_IMAGE_INFO = *mut HOT_PATCH_IMAGE_INFO;

pub const MANAGE_HOT_PATCH_LOAD_PATCH_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_LOAD_PATCH {
    pub Version: ULONG,
    pub PatchPath: UNICODE_STRING,
    pub UserSid: MANAGE_HOT_PATCH_LOAD_PATCH_USER_SID,
    pub BaseInfo: HOT_PATCH_IMAGE_INFO,
}

#[repr(C)]
pub union MANAGE_HOT_PATCH_LOAD_PATCH_USER_SID {
    pub Sid: core::mem::ManuallyDrop<SID>,
    pub Buffer: [UCHAR; SECURITY_MAX_SID_SIZE],
}

pub type PMANAGE_HOT_PATCH_LOAD_PATCH = *mut MANAGE_HOT_PATCH_LOAD_PATCH;

pub const MANAGE_HOT_PATCH_UNLOAD_PATCH_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_UNLOAD_PATCH {
    pub Version: ULONG,
    pub BaseInfo: HOT_PATCH_IMAGE_INFO,
    pub UserSid: MANAGE_HOT_PATCH_UNLOAD_PATCH_USER_SID,
}

#[repr(C)]
pub union MANAGE_HOT_PATCH_UNLOAD_PATCH_USER_SID {
    pub Sid: core::mem::ManuallyDrop<SID>,
    pub Buffer: [UCHAR; SECURITY_MAX_SID_SIZE],
}

pub type PMANAGE_HOT_PATCH_UNLOAD_PATCH = *mut MANAGE_HOT_PATCH_UNLOAD_PATCH;

pub const MANAGE_HOT_PATCH_QUERY_PATCHES_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_QUERY_PATCHES {
    pub Version: ULONG,
    pub UserSid: MANAGE_HOT_PATCH_QUERY_PATCHES_USER_SID,
    pub PatchCount: ULONG,
    pub PatchPathStrings: PUNICODE_STRING,
    pub BaseInfos: PHOT_PATCH_IMAGE_INFO,
}

#[repr(C)]
pub union MANAGE_HOT_PATCH_QUERY_PATCHES_USER_SID {
    pub Sid: core::mem::ManuallyDrop<SID>,
    pub Buffer: [UCHAR; SECURITY_MAX_SID_SIZE],
}

pub type PMANAGE_HOT_PATCH_QUERY_PATCHES = *mut MANAGE_HOT_PATCH_QUERY_PATCHES;

pub const MANAGE_HOT_PATCH_QUERY_ACTIVE_PATCHES_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_QUERY_ACTIVE_PATCHES {
    pub Version: ULONG,
    pub ProcessHandle: HANDLE,
    pub PatchCount: ULONG,
    pub PatchPathStrings: PUNICODE_STRING,
    pub BaseInfos: PHOT_PATCH_IMAGE_INFO,
    pub PatchSequenceNumbers: PULONG,
}

pub type PMANAGE_HOT_PATCH_QUERY_ACTIVE_PATCHES = *mut MANAGE_HOT_PATCH_QUERY_ACTIVE_PATCHES;

pub const MANAGE_HOT_PATCH_APPLY_IMAGE_PATCH_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_APPLY_IMAGE_PATCH {
    pub Version: ULONG,
    pub ApplyReversePatches: u32,
    pub ApplyForwardPatches: u32,
    pub Spare: u32,
    pub ProcessHandle: HANDLE,
    pub BaseImageAddress: PVOID,
    pub PatchImageAddress: PVOID,
}

pub type PMANAGE_HOT_PATCH_APPLY_IMAGE_PATCH = *mut MANAGE_HOT_PATCH_APPLY_IMAGE_PATCH;

pub const MANAGE_HOT_PATCH_QUERY_SINGLE_PATCH_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_QUERY_SINGLE_PATCH {
    pub Version: ULONG,
    pub ProcessHandle: HANDLE,
    pub BaseAddress: PVOID,
    pub Flags: ULONG,
    pub PatchPathString: UNICODE_STRING,
}

pub type PMANAGE_HOT_PATCH_QUERY_SINGLE_PATCH = *mut MANAGE_HOT_PATCH_QUERY_SINGLE_PATCH;

pub const MANAGE_HOT_PATCH_CHECK_ENABLED_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_CHECK_ENABLED {
    pub Version: ULONG,
    pub Flags: ULONG,
}

pub type PMANAGE_HOT_PATCH_CHECK_ENABLED = *mut MANAGE_HOT_PATCH_CHECK_ENABLED;

pub const MANAGE_HOT_PATCH_CREATE_PATCH_SECTION_VERSION: u32 = 1;

#[repr(C)]
pub struct MANAGE_HOT_PATCH_CREATE_PATCH_SECTION {
    pub Version: ULONG,
    pub Flags: ULONG,
    pub DesiredAccess: ACCESS_MASK,
    pub PageProtection: ULONG,
    pub AllocationAttributes: ULONG,
    pub BaseImageAddress: PVOID,
    pub SectionHandle: HANDLE,
}

pub type PMANAGE_HOT_PATCH_CREATE_PATCH_SECTION = *mut MANAGE_HOT_PATCH_CREATE_PATCH_SECTION;

pub const PROCESSOR_FEATURE_MAX: usize = 64;
pub const MAX_WOW64_SHARED_ENTRIES: usize = 16;

pub const NX_SUPPORT_POLICY_ALWAYSOFF: u32 = 0;
pub const NX_SUPPORT_POLICY_ALWAYSON: u32 = 1;
pub const NX_SUPPORT_POLICY_OPTIN: u32 = 2;
pub const NX_SUPPORT_POLICY_OPTOUT: u32 = 3;

pub const SEH_VALIDATION_POLICY_ON: u32 = 0;
pub const SEH_VALIDATION_POLICY_OFF: u32 = 1;
pub const SEH_VALIDATION_POLICY_TELEMETRY: u32 = 2;
pub const SEH_VALIDATION_POLICY_DEFER: u32 = 3;

pub const SHARED_GLOBAL_FLAGS_ERROR_PORT_V: u32 = 0;
pub const SHARED_GLOBAL_FLAGS_ERROR_PORT: u32 = 1 << SHARED_GLOBAL_FLAGS_ERROR_PORT_V;
pub const SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED_V: u32 = 1;
pub const SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_VIRT_ENABLED_V: u32 = 2;
pub const SHARED_GLOBAL_FLAGS_VIRT_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_VIRT_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED_V: u32 = 3;
pub const SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_LKG_ENABLED_V: u32 = 4;
pub const SHARED_GLOBAL_FLAGS_LKG_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_LKG_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED_V: u32 = 5;
pub const SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED_V: u32 = 6;
pub const SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED_V: u32 = 7;
pub const SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU_V: u32 = 8;
pub const SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU: u32 = 1 << SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU_V;
pub const SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU_V: u32 = 9;
pub const SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU: u32 = 1 << SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU_V;
pub const SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED_V: u32 = 10;
pub const SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED: u32 = 1 << SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED_V;
pub const SHARED_GLOBAL_FLAGS_SET_GLOBAL_DATA_FLAG: u32 = 0x40000000;
pub const SHARED_GLOBAL_FLAGS_CLEAR_GLOBAL_DATA_FLAG: u32 = 0x80000000;

pub const SYSTEM_CALL_SYSCALL: u32 = 0;
pub const SYSTEM_CALL_INT_2E: u32 = 1;

pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_ENABLED: u32 = 0x01;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_HV_PAGE: u32 = 0x02;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_DISABLE_32BIT: u32 = 0x04;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_MFENCE: u32 = 0x10;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_LFENCE: u32 = 0x20;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_A73_ERRATA: u32 = 0x40;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_RDTSCP: u32 = 0x80;

pub const KF64_SMEP: u64 = 0x0000000000000001;
pub const KF64_RDTSC: u64 = 0x0000000000000002;
pub const KF64_CR4: u64 = 0x0000000000000004;
pub const KF64_CMOV: u64 = 0x0000000000000008;
pub const KF64_GLOBAL_PAGE: u64 = 0x0000000000000010;
pub const KF64_LARGE_PAGE: u64 = 0x0000000000000020;
pub const KF64_MTRR: u64 = 0x0000000000000040;
pub const KF64_CMPXCHG8B: u64 = 0x0000000000000080;
pub const KF64_MMX: u64 = 0x0000000000000100;
pub const KF64_DTS: u64 = 0x0000000000000200;
pub const KF64_PAT: u64 = 0x0000000000000400;
pub const KF64_FXSR: u64 = 0x0000000000000800;
pub const KF64_FAST_SYSCALL: u64 = 0x0000000000001000;
pub const KF64_XMMI: u64 = 0x0000000000002000;
pub const KF64_3DNOW: u64 = 0x0000000000004000;
pub const KF64_AMDK6MTRR: u64 = 0x0000000000008000;
pub const KF64_XMMI64: u64 = 0x0000000000010000;
pub const KF64_BRANCH: u64 = 0x0000000000020000;
pub const KF64_XSTATE: u64 = 0x0000000000800000;
pub const KF64_RDRAND: u64 = 0x0000000100000000;
pub const KF64_SMAP: u64 = 0x0000000200000000;
pub const KF64_RDTSCP: u64 = 0x0000000400000000;
pub const KF64_HUGEPAGE: u64 = 0x0000002000000000;
pub const KF64_XSAVES: u64 = 0x0000004000000000;
pub const KF64_FPU_LEAKAGE: u64 = 0x0000020000000000;
pub const KF64_CAT: u64 = 0x0000100000000000;
pub const KF64_CET_SS: u64 = 0x0000400000000000;
pub const KF64_SSSE3: u64 = 0x0000800000000000;
pub const KF64_SSE4_1: u64 = 0x0001000000000000;
pub const KF64_SSE4_2: u64 = 0x0002000000000000;
pub const KF64_XFD: u64 = 0x0080000000000000;

#[repr(u32)]
pub enum SYSDBG_COMMAND {
    SysDbgQueryModuleInformation,
    SysDbgQueryTraceInformation,
    SysDbgSetTracepoint,
    SysDbgSetSpecialCall,
    SysDbgClearSpecialCalls,
    SysDbgQuerySpecialCalls,
    SysDbgBreakPoint,
    SysDbgQueryVersion,
    SysDbgReadVirtual,
    SysDbgWriteVirtual,
    SysDbgReadPhysical,
    SysDbgWritePhysical,
    SysDbgReadControlSpace,
    SysDbgWriteControlSpace,
    SysDbgReadIoSpace,
    SysDbgWriteIoSpace,
    SysDbgReadMsr,
    SysDbgWriteMsr,
    SysDbgReadBusData,
    SysDbgWriteBusData,
    SysDbgCheckLowMemory,
    SysDbgEnableKernelDebugger,
    SysDbgDisableKernelDebugger,
    SysDbgGetAutoKdEnable,
    SysDbgSetAutoKdEnable,
    SysDbgGetPrintBufferSize,
    SysDbgSetPrintBufferSize,
    SysDbgGetKdUmExceptionEnable,
    SysDbgSetKdUmExceptionEnable,
    SysDbgGetTriageDump,
    SysDbgGetKdBlockEnable,
    SysDbgSetKdBlockEnable,
    SysDbgRegisterForUmBreakInfo,
    SysDbgGetUmBreakPid,
    SysDbgClearUmBreakPid,
    SysDbgGetUmAttachPid,
    SysDbgClearUmAttachPid,
    SysDbgGetLiveKernelDump,
    SysDbgKdPullRemoteFile,
    SysDbgMaxInfoClass,
}

#[repr(C)]
pub struct SYSDBG_VIRTUAL {
    pub Address: PVOID,
    pub Buffer: PVOID,
    pub Request: ULONG,
}

pub type PSYSDBG_VIRTUAL = *mut SYSDBG_VIRTUAL;

#[repr(C)]
pub struct PHYSICAL_ADDRESS {
    pub QuadPart: ULONGLONG,
}

pub type PPHYSICAL_ADDRESS = *mut PHYSICAL_ADDRESS;

#[repr(C)]
pub struct SYSDBG_PHYSICAL {
    pub Address: PHYSICAL_ADDRESS,
    pub Buffer: PVOID,
    pub Request: ULONG,
}

pub type PSYSDBG_PHYSICAL = *mut SYSDBG_PHYSICAL;

#[repr(C)]
pub struct SYSDBG_CONTROL_SPACE {
    pub Address: ULONG64,
    pub Buffer: PVOID,
    pub Request: ULONG,
    pub Processor: ULONG,
}

pub type PSYSDBG_CONTROL_SPACE = *mut SYSDBG_CONTROL_SPACE;

#[repr(C)]
pub struct SYSDBG_IO_SPACE {
    pub Address: ULONG64,
    pub Buffer: PVOID,
    pub Request: ULONG,
    pub InterfaceType: INTERFACE_TYPE,
    pub BusNumber: ULONG,
    pub AddressSpace: ULONG,
}

pub type PSYSDBG_IO_SPACE = *mut SYSDBG_IO_SPACE;

#[repr(C)]
pub struct SYSDBG_MSR {
    pub Msr: ULONG,
    pub Data: ULONG64,
}

pub type PSYSDBG_MSR = *mut SYSDBG_MSR;

#[repr(C)]
pub struct SYSDBG_BUS_DATA {
    pub Address: ULONG,
    pub Buffer: PVOID,
    pub Request: ULONG,
    pub BusDataType: BUS_DATA_TYPE,
    pub BusNumber: ULONG,
    pub SlotNumber: ULONG,
}

pub type PSYSDBG_BUS_DATA = *mut SYSDBG_BUS_DATA;

#[repr(C)]
pub struct SYSDBG_TRIAGE_DUMP {
    pub Flags: ULONG,
    pub BugCheckCode: ULONG,
    pub BugCheckParam1: ULONG_PTR,
    pub BugCheckParam2: ULONG_PTR,
    pub BugCheckParam3: ULONG_PTR,
    pub BugCheckParam4: ULONG_PTR,
    pub ProcessHandles: ULONG,
    pub ThreadHandles: ULONG,
    pub Handles: PHANDLE,
}

pub type PSYSDBG_TRIAGE_DUMP = *mut SYSDBG_TRIAGE_DUMP;

#[repr(C)]
pub union SYSDBG_LIVEDUMP_CONTROL_FLAGS {
    pub AsUlong: ULONG,
    pub UseDumpStorageStack: u32,
    pub CompressMemoryPagesData: u32,
    pub IncludeUserSpaceMemoryPages: u32,
    pub AbortIfMemoryPressure: u32,
    pub SelectiveDump: u32,
    pub Reserved: u32,
}

pub type PSYSDBG_LIVEDUMP_CONTROL_FLAGS = *mut SYSDBG_LIVEDUMP_CONTROL_FLAGS;

#[repr(C)]
pub union SYSDBG_LIVEDUMP_CONTROL_ADDPAGES {
    pub AsUlong: ULONG,
    pub HypervisorPages: u32,
    pub NonEssentialHypervisorPages: u32,
    pub Reserved: u32,
}

pub type PSYSDBG_LIVEDUMP_CONTROL_ADDPAGES = *mut SYSDBG_LIVEDUMP_CONTROL_ADDPAGES;

pub const SYSDBG_LIVEDUMP_SELECTIVE_CONTROL_VERSION: u32 = 1;

#[repr(C)]
pub struct SYSDBG_LIVEDUMP_SELECTIVE_CONTROL {
    pub Version: ULONG,
    pub Size: ULONG,
    pub Flags: ULONGLONG,
    pub Reserved: [ULONGLONG; 4],
}

pub type PSYSDBG_LIVEDUMP_SELECTIVE_CONTROL = *mut SYSDBG_LIVEDUMP_SELECTIVE_CONTROL;

pub const SYSDBG_LIVEDUMP_CONTROL_VERSION_1: u32 = 1;
pub const SYSDBG_LIVEDUMP_CONTROL_VERSION_2: u32 = 2;

#[repr(C)]
pub struct SYSDBG_LIVEDUMP_CONTROL_V1 {
    pub Version: ULONG,
    pub BugCheckCode: ULONG,
    pub BugCheckParam1: ULONG_PTR,
    pub BugCheckParam2: ULONG_PTR,
    pub BugCheckParam3: ULONG_PTR,
    pub BugCheckParam4: ULONG_PTR,
    pub DumpFileHandle: HANDLE,
    pub CancelEventHandle: HANDLE,
    pub Flags: SYSDBG_LIVEDUMP_CONTROL_FLAGS,
    pub AddPagesControl: SYSDBG_LIVEDUMP_CONTROL_ADDPAGES,
}

pub type PSYSDBG_LIVEDUMP_CONTROL_V1 = *mut SYSDBG_LIVEDUMP_CONTROL_V1;

#[repr(C)]
pub struct SYSDBG_LIVEDUMP_CONTROL {
    pub Version: ULONG,
    pub BugCheckCode: ULONG,
    pub BugCheckParam1: ULONG_PTR,
    pub BugCheckParam2: ULONG_PTR,
    pub BugCheckParam3: ULONG_PTR,
    pub BugCheckParam4: ULONG_PTR,
    pub DumpFileHandle: HANDLE,
    pub CancelEventHandle: HANDLE,
    pub Flags: SYSDBG_LIVEDUMP_CONTROL_FLAGS,
    pub AddPagesControl: SYSDBG_LIVEDUMP_CONTROL_ADDPAGES,
    pub SelectiveControl: PSYSDBG_LIVEDUMP_SELECTIVE_CONTROL,
}

pub type PSYSDBG_LIVEDUMP_CONTROL = *mut SYSDBG_LIVEDUMP_CONTROL;

#[repr(C)]
pub struct SYSDBG_KD_PULL_REMOTE_FILE {
    pub ImageFileName: UNICODE_STRING,
}

pub type PSYSDBG_KD_PULL_REMOTE_FILE = *mut SYSDBG_KD_PULL_REMOTE_FILE;

#[repr(u32)]
pub enum HARDERROR_RESPONSE_OPTION {
    OptionAbortRetryIgnore,
    OptionOk,
    OptionOkCancel,
    OptionRetryCancel,
    OptionYesNo,
    OptionYesNoCancel,
    OptionShutdownSystem,
    OptionOkNoWait,
    OptionCancelTryContinue,
}

#[repr(u32)]
pub enum HARDERROR_RESPONSE {
    ResponseReturnToCaller,
    ResponseNotHandled,
    ResponseAbort,
    ResponseCancel,
    ResponseIgnore,
    ResponseNo,
    ResponseOk,
    ResponseRetry,
    ResponseYes,
    ResponseTryAgain,
    ResponseContinue,
}

pub const HARDERROR_OVERRIDE_ERRORMODE: u32 = 0x10000000;

pub const CMF_ACCESS_DIRECTORY: u32 = 0x00000002;
pub const CMF_ACCESS_SEGMENT: u32 = 0x00000004;
pub const CMF_ACCESS_HITS: u32 = 0x00000008;
pub const CMF_OP_DIRECTORY: u32 = 0x00000010;
pub const CMF_OP_SEGMENT: u32 = 0x00000020;
pub const CMF_OP_HITS: u32 = 0x00000100;
pub const CMF_PROTECT_SPECIAL: u32 = 0x00000040;
pub const CMF_UPDATE_FLAGS: u32 = 0x00020000;
pub const CMF_FLAG_A: u32 = 0x00040000;
pub const CMF_FLAG_B: u32 = 0x00080000;
pub const CMF_FLAG_C: u32 = 0x00100000;
pub const CMF_ALLOWED_MASK: u32 = 0xFFFFFECF;

pub const MUI_REGINFO_QUERY: u32 = 0x1;
pub const MUI_REGINFO_CLEAR: u32 = 0x2;
pub const MUI_REGINFO_COMMIT: u32 = 0x8;

#[repr(C)]
pub union WNF_STATE_NAME {
    pub Value: ULONGLONG,
    pub Data: [ULONG; 2],
    pub Version: u64,
    pub NameLifetime: u64,
    pub DataScope: u64,
    pub PermanentData: u64,
    pub Unique: u64,
}

pub type PWNF_STATE_NAME = *mut WNF_STATE_NAME;
pub type PCWNF_STATE_NAME = *const WNF_STATE_NAME;

#[repr(u32)]
pub enum WNF_STATE_NAME_INFORMATION_CLASS {
    WnfInfoStateNameExist,
    WnfInfoSubscribersPresent,
    WnfInfoIsQuiescent,
}

#[repr(u32)]
pub enum WNF_STATE_NAME_LIFETIME {
    WnfWellKnownStateName,
    WnfPermanentStateName,
    WnfPersistentStateName,
    WnfTemporaryStateName,
}

#[repr(u32)]
pub enum WNF_STATE_NAME_INFORMATION {
    WnfInfoStateNameExist,
    WnfInfoSubscribersPresent,
    WnfInfoIsQuiescent,
}

#[repr(u32)]
pub enum WNF_DATA_SCOPE {
    WnfDataScopeSystem,
    WnfDataScopeSession,
    WnfDataScopeUser,
    WnfDataScopeProcess,
    WnfDataScopeMachine,
    WnfDataScopePhysicalMachine,
}

#[repr(C)]
pub struct WNF_TYPE_ID {
    pub TypeId: GUID,
}

#[repr(C)]
pub struct WNF_DELIVERY_DESCRIPTOR {
    pub SubscriptionId: ULONGLONG,
    pub StateName: WNF_STATE_NAME,
    pub ChangeStamp: WNF_CHANGE_STAMP,
    pub StateDataSize: ULONG,
    pub EventMask: ULONG,
    pub TypeId: WNF_TYPE_ID,
    pub StateDataOffset: ULONG,
}

pub type PWNF_DELIVERY_DESCRIPTOR = *mut WNF_DELIVERY_DESCRIPTOR;
pub type PWNF_USER_SUBSCRIPTION = *mut c_void;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SYSTEM_INFORMATION_CLASS {
    SystemBasicInformation = 0,
    SystemProcessorInformation = 1,
    SystemPerformanceInformation = 2,
    SystemTimeOfDayInformation = 3,
    SystemPathInformation = 4,
    SystemProcessInformation = 5,
    SystemCallCountInformation = 6,
    SystemDeviceInformation = 7,
    SystemProcessorPerformanceInformation = 8,
    SystemFlagsInformation = 9,
    SystemCallTimeInformation = 10,
    SystemModuleInformation = 11,
    SystemLocksInformation = 12,
    SystemStackTraceInformation = 13,
    SystemPagedPoolInformation = 14,
    SystemNonPagedPoolInformation = 15,
    SystemHandleInformation = 16,
    SystemObjectInformation = 17,
    SystemPageFileInformation = 18,
    SystemVdmInstemulInformation = 19,
    SystemVdmBopInformation = 20,
    SystemFileCacheInformation = 21,
    SystemPoolTagInformation = 22,
    SystemInterruptInformation = 23,
    SystemDpcBehaviorInformation = 24,
    SystemFullMemoryInformation = 25,
    SystemLoadGdiDriverInformation = 26,
    SystemUnloadGdiDriverInformation = 27,
    SystemTimeAdjustmentInformation = 28,
    SystemSummaryMemoryInformation = 29,
    SystemMirrorMemoryInformation = 30,
    SystemPerformanceTraceInformation = 31,
    SystemObsolete0 = 32,
    SystemExceptionInformation = 33,
    SystemCrashDumpStateInformation = 34,
    SystemKernelDebuggerInformation = 35,
    SystemContextSwitchInformation = 36,
    SystemRegistryQuotaInformation = 37,
    SystemExtendServiceTableInformation = 38,
    SystemPrioritySeparation = 39,
    SystemVerifierAddDriverInformation = 40,
    SystemVerifierRemoveDriverInformation = 41,
    SystemProcessorIdleInformation = 42,
    SystemLegacyDriverInformation = 43,
    SystemCurrentTimeZoneInformation = 44,
    SystemLookasideInformation = 45,
    SystemTimeSlipNotification = 46,
    SystemSessionCreate = 47,
    SystemSessionDetach = 48,
    SystemSessionInformation = 49,
    SystemRangeStartInformation = 50,
    SystemVerifierInformation = 51,
    SystemVerifierThunkExtend = 52,
    SystemSessionProcessInformation = 53,
    SystemLoadGdiDriverInSystemSpace = 54,
    SystemNumaProcessorMap = 55,
    SystemPrefetcherInformation = 56,
    SystemExtendedProcessInformation = 57,
    SystemRecommendedSharedDataAlignment = 58,
    SystemComPlusPackage = 59,
    SystemNumaAvailableMemory = 60,
    SystemProcessorPowerInformation = 61,
    SystemEmulationBasicInformation = 62,
    SystemEmulationProcessorInformation = 63,
    SystemExtendedHandleInformation = 64,
    SystemLostDelayedWriteInformation = 65,
    SystemBigPoolInformation = 66,
    SystemSessionPoolTagInformation = 67,
    SystemSessionMappedViewInformation = 68,
    SystemHotpatchInformation = 69,
    SystemObjectSecurityMode = 70,
    SystemWatchdogTimerHandler = 71,
    SystemWatchdogTimerInformation = 72,
    SystemLogicalProcessorInformation = 73,
    SystemWow64SharedInformationObsolete = 74,
    SystemRegisterFirmwareTableInformationHandler = 75,
    SystemFirmwareTableInformation = 76,
    SystemModuleInformationEx = 77,
    SystemVerifierTriageInformation = 78,
    SystemSuperfetchInformation = 79,
    SystemMemoryListInformation = 80,
    SystemFileCacheInformationEx = 81,
    SystemThreadPriorityClientIdInformation = 82,
    SystemProcessorIdleCycleTimeInformation = 83,
    SystemVerifierCancellationInformation = 84,
    SystemProcessorPowerInformationEx = 85,
    SystemRefTraceInformation = 86,
    SystemSpecialPoolInformation = 87,
    SystemProcessIdInformation = 88,
    SystemErrorPortInformation = 89,
    SystemBootEnvironmentInformation = 90,
    SystemHypervisorInformation = 91,
    SystemVerifierInformationEx = 92,
    SystemTimeZoneInformation = 93,
    SystemImageFileExecutionOptionsInformation = 94,
    SystemCoverageInformation = 95,
    SystemPrefetchPatchInformation = 96,
    SystemVerifierFaultsInformation = 97,
    SystemSystemPartitionInformation = 98,
    SystemSystemDiskInformation = 99,
    SystemProcessorPerformanceDistribution = 100,
    SystemNumaProximityNodeInformation = 101,
    SystemDynamicTimeZoneInformation = 102,
    SystemCodeIntegrityInformation = 103,
    SystemProcessorMicrocodeUpdateInformation = 104,
    SystemProcessorBrandString = 105,
    SystemVirtualAddressInformation = 106,
    SystemLogicalProcessorAndGroupInformation = 107,
    SystemProcessorCycleTimeInformation = 108,
    SystemStoreInformation = 109,
    SystemRegistryAppendString = 110,
    SystemAitSamplingValue = 111,
    SystemVhdBootInformation = 112,
    SystemCpuQuotaInformation = 113,
    SystemNativeBasicInformation = 114,
    SystemErrorPortTimeouts = 115,
    SystemLowPriorityIoInformation = 116,
    SystemTpmBootEntropyInformation = 117,
    SystemVerifierCountersInformation = 118,
    SystemPagedPoolInformationEx = 119,
    SystemSystemPtesInformationEx = 120,
    SystemNodeDistanceInformation = 121,
    SystemAcpiAuditInformation = 122,
    SystemBasicPerformanceInformation = 123,
    SystemQueryPerformanceCounterInformation = 124,
    SystemSessionBigPoolInformation = 125,
    SystemBootGraphicsInformation = 126,
    SystemScrubPhysicalMemoryInformation = 127,
    SystemBadPageInformation = 128,
    SystemProcessorProfileControlArea = 129,
    SystemCombinePhysicalMemoryInformation = 130,
    SystemEntropyInterruptTimingInformation = 131,
    SystemConsoleInformation = 132,
    SystemPlatformBinaryInformation = 133,
    SystemPolicyInformation = 134,
    SystemHypervisorProcessorCountInformation = 135,
    SystemDeviceDataInformation = 136,
    SystemDeviceDataEnumerationInformation = 137,
    SystemMemoryTopologyInformation = 138,
    SystemMemoryChannelInformation = 139,
    SystemBootLogoInformation = 140,
    SystemProcessorPerformanceInformationEx = 141,
    SystemCriticalProcessErrorLogInformation = 142,
    SystemSecureBootPolicyInformation = 143,
    SystemPageFileInformationEx = 144,
    SystemSecureBootInformation = 145,
    SystemEntropyInterruptTimingRawInformation = 146,
    SystemPortableWorkspaceEfiLauncherInformation = 147,
    SystemFullProcessInformation = 148,
    SystemKernelDebuggerInformationEx = 149,
    SystemBootMetadataInformation = 150,
    SystemSoftRebootInformation = 151,
    SystemElamCertificateInformation = 152,
    SystemOfflineDumpConfigInformation = 153,
    SystemProcessorFeaturesInformation = 154,
    SystemRegistryReconciliationInformation = 155,
    SystemEdidInformation = 156,
    SystemManufacturingInformation = 157,
    SystemEnergyEstimationConfigInformation = 158,
    SystemHypervisorDetailInformation = 159,
    SystemProcessorCycleStatsInformation = 160,
    SystemVmGenerationCountInformation = 161,
    SystemTrustedPlatformModuleInformation = 162,
    SystemKernelDebuggerFlags = 163,
    SystemCodeIntegrityPolicyInformation = 164,
    SystemIsolatedUserModeInformation = 165,
    SystemHardwareSecurityTestInterfaceResultsInformation = 166,
    SystemSingleModuleInformation = 167,
    SystemAllowedCpuSetsInformation = 168,
    SystemVsmProtectionInformation = 169,
    SystemInterruptCpuSetsInformation = 170,
    SystemSecureBootPolicyFullInformation = 171,
    SystemCodeIntegrityPolicyFullInformation = 172,
    SystemAffinitizedInterruptProcessorInformation = 173,
    SystemRootSiloInformation = 174,
    SystemCpuSetInformation = 175,
    SystemCpuSetTagInformation = 176,
    SystemWin32WerStartCallout = 177,
    SystemSecureKernelProfileInformation = 178,
    SystemCodeIntegrityPlatformManifestInformation = 179,
    SystemInterruptSteeringInformation = 180,
    SystemSupportedProcessorArchitectures = 181,
    SystemMemoryUsageInformation = 182,
    SystemCodeIntegrityCertificateInformation = 183,
    SystemPhysicalMemoryInformation = 184,
    SystemControlFlowTransition = 185,
    SystemKernelDebuggingAllowed = 186,
    SystemActivityModerationExeState = 187,
    SystemActivityModerationUserSettings = 188,
    SystemCodeIntegrityPoliciesFullInformation = 189,
    SystemCodeIntegrityUnlockInformation = 190,
    SystemIntegrityQuotaInformation = 191,
    SystemFlushInformation = 192,
    SystemProcessorIdleMaskInformation = 193,
    SystemSecureDumpEncryptionInformation = 194,
    SystemWriteConstraintInformation = 195,
    SystemKernelVaShadowInformation = 196,
    SystemHypervisorSharedPageInformation = 197,
    SystemFirmwareBootPerformanceInformation = 198,
    SystemCodeIntegrityVerificationInformation = 199,
    SystemFirmwarePartitionInformation = 200,
    SystemSpeculationControlInformation = 201,
    SystemDmaGuardPolicyInformation = 202,
    SystemEnclaveLaunchControlInformation = 203,
    SystemWorkloadAllowedCpuSetsInformation = 204,
    SystemCodeIntegrityUnlockModeInformation = 205,
    SystemLeapSecondInformation = 206,
    SystemFlags2Information = 207,
    SystemSecurityModelInformation = 208,
    SystemCodeIntegritySyntheticCacheInformation = 209,
    SystemFeatureConfigurationInformation = 210,
    SystemFeatureConfigurationSectionInformation = 211,
    SystemFeatureUsageSubscriptionInformation = 212,
    SystemSecureSpeculationControlInformation = 213,
    SystemSpacesBootInformation = 214,
    SystemFwRamdiskInformation = 215,
    SystemWheaIpmiHardwareInformation = 216,
    SystemDifSetRuleClassInformation = 217,
    SystemDifClearRuleClassInformation = 218,
    SystemDifApplyPluginVerificationOnDriver = 219,
    SystemDifRemovePluginVerificationOnDriver = 220,
    SystemShadowStackInformation = 221,
    SystemBuildVersionInformation = 222,
    SystemPoolLimitInformation = 223,
    SystemCodeIntegrityAddDynamicStore = 224,
    SystemCodeIntegrityClearDynamicStores = 225,
    SystemDifPoolTrackingInformation = 226,
    SystemPoolZeroingInformation = 227,
    SystemDpcWatchdogInformation = 228,
    SystemDpcWatchdogInformation2 = 229,
    SystemSupportedProcessorArchitectures2 = 230,
    SystemSingleProcessorRelationshipInformation = 231,
    SystemXfgCheckFailureInformation = 232,
    SystemIommuStateInformation = 233,
    SystemHypervisorMinrootInformation = 234,
    SystemHypervisorBootPagesInformation = 235,
    SystemPointerAuthInformation = 236,
    SystemSecureKernelDebuggerInformation = 237,
    SystemOriginalImageFeatureInformation = 238,
    SystemMemoryNumaInformation = 239,
    SystemMemoryNumaPerformanceInformation = 240,
    SystemCodeIntegritySignedPoliciesFullInformation = 241,
    SystemSecureCoreInformation = 242,
    SystemTrustedAppsRuntimeInformation = 243,
    SystemBadPageInformationEx = 244,
    SystemResourceDeadlockTimeout = 245,
    SystemBreakOnContextUnwindFailureInformation = 246,
    SystemOslRamdiskInformation = 247,
    SystemCodeIntegrityPolicyManagementInformation = 248,
    SystemMemoryNumaCacheInformation = 249,
    SystemProcessorFeaturesBitMapInformation = 250,
    SystemRefTraceInformationEx = 251,
    SystemBasicProcessInformation = 252,
    SystemHandleCountInformation = 253,
    SystemRuntimeAttestationReport = 254,
    SystemPoolTagInformation2 = 255,
    MaxSystemInfoClass = 256,
}

#[repr(C)]
pub struct BOOT_ENTRY {
    pub Version: ULONG,
    pub Length: ULONG,
    pub Id: ULONG,
    pub Attributes: ULONG,
    pub FriendlyNameOffset: ULONG,
    pub BootFilePathOffset: ULONG,
    pub OsOptionsLength: ULONG,
    pub OsOptions: [UCHAR; 1],
}
pub type PBOOT_ENTRY = *mut BOOT_ENTRY;

#[repr(C)]
pub struct BOOT_ENTRY_LIST {
    pub NextEntryOffset: ULONG,
    pub BootEntry: BOOT_ENTRY,
}
pub type PBOOT_ENTRY_LIST = *mut BOOT_ENTRY_LIST;

#[repr(C)]
pub struct BOOT_OPTIONS {
    pub Version: ULONG,
    pub Length: ULONG,
    pub Timeout: ULONG,
    pub CurrentBootEntryId: ULONG,
    pub NextBootEntryId: ULONG,
    pub HeadlessRedirection: [u16; 1],
}
pub type PBOOT_OPTIONS = *mut BOOT_OPTIONS;

#[repr(C)]
pub struct FILE_PATH {
    pub Version: ULONG,
    pub Length: ULONG,
    pub Type: ULONG,
    pub FilePath: [UCHAR; 1],
}
pub type PFILE_PATH = *mut FILE_PATH;

#[repr(C)]
pub struct EFI_DRIVER_ENTRY {
    pub Version: ULONG,
    pub Length: ULONG,
    pub Id: ULONG,
    pub FriendlyNameOffset: ULONG,
    pub DriverFilePathOffset: ULONG,
}
pub type PEFI_DRIVER_ENTRY = *mut EFI_DRIVER_ENTRY;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FILTER_BOOT_OPTION_OPERATION {
    FilterBootOptionOperationOpenSystemStore = 0,
    FilterBootOptionOperationSetElement = 1,
    FilterBootOptionOperationDeleteElement = 2,
    FilterBootOptionOperationMax = 3,
}

#[repr(C)]
pub struct T2_SET_PARAMETERS_V0 {
    pub Version: ULONG,
    pub Reserved: ULONG,
    pub NoWakeTolerance: LONGLONG,
}
pub type PT2_SET_PARAMETERS = *mut T2_SET_PARAMETERS_V0;