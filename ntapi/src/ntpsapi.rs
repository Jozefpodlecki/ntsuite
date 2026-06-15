use crate::ntdef::*;
use crate::ntpebteb::*;
use crate::ntrtl::{CLIENT_ID, PCLIENT_ID, PRTL_USER_PROCESS_PARAMETERS, PUSER_THREAD_START_ROUTINE};

pub const STANDARD_RIGHTS_REQUIRED: u32 = 0x000F0000;
pub const SPECIFIC_RIGHTS_ALL: u32 = 0x0000FFFF;
pub const PROCESS_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | SPECIFIC_RIGHTS_ALL;
pub const THREAD_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | SPECIFIC_RIGHTS_ALL;
pub const JOB_OBJECT_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x3F;

pub const PROCESS_TERMINATE: u32 = 0x0001;
pub const PROCESS_CREATE_THREAD: u32 = 0x0002;
pub const PROCESS_SET_SESSIONID: u32 = 0x0004;
pub const PROCESS_VM_OPERATION: u32 = 0x0008;
pub const PROCESS_VM_READ: u32 = 0x0010;
pub const PROCESS_VM_WRITE: u32 = 0x0020;
pub const PROCESS_DUP_HANDLE: u32 = 0x0040;
pub const PROCESS_CREATE_PROCESS: u32 = 0x0080;
pub const PROCESS_SET_QUOTA: u32 = 0x0100;
pub const PROCESS_SET_INFORMATION: u32 = 0x0200;
pub const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
pub const PROCESS_SET_PORT: u32 = 0x0800;
pub const PROCESS_SUSPEND_RESUME: u32 = 0x0800;
pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
pub const PROCESS_SET_LIMITED_INFORMATION: u32 = 0x2000;

pub const THREAD_TERMINATE: u32 = 0x0001;
pub const THREAD_SUSPEND_RESUME: u32 = 0x0002;
pub const THREAD_ALERT: u32 = 0x0004;
pub const THREAD_GET_CONTEXT: u32 = 0x0008;
pub const THREAD_SET_CONTEXT: u32 = 0x0010;
pub const THREAD_SET_INFORMATION: u32 = 0x0020;
pub const THREAD_QUERY_INFORMATION: u32 = 0x0040;
pub const THREAD_SET_THREAD_TOKEN: u32 = 0x0080;
pub const THREAD_IMPERSONATE: u32 = 0x0100;
pub const THREAD_DIRECT_IMPERSONATION: u32 = 0x0200;
pub const THREAD_SET_LIMITED_INFORMATION: u32 = 0x0400;
pub const THREAD_QUERY_LIMITED_INFORMATION: u32 = 0x0800;
pub const THREAD_RESUME: u32 = 0x1000;

pub const JOB_OBJECT_ASSIGN_PROCESS: u32 = 0x0001;
pub const JOB_OBJECT_SET_ATTRIBUTES: u32 = 0x0002;
pub const JOB_OBJECT_QUERY: u32 = 0x0004;
pub const JOB_OBJECT_TERMINATE: u32 = 0x0008;
pub const JOB_OBJECT_SET_SECURITY_ATTRIBUTES: u32 = 0x0010;

pub type PROCESSINFOCLASS = ULONG;
pub type THREADINFOCLASS = ULONG;
pub type PPS_APC_ROUTINE = Option<unsafe extern "system" fn(ApcArgument1: PVOID, ApcArgument2: PVOID, ApcArgument3: PVOID)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INITIAL_TEB {
    pub OldStackBase: PVOID,
    pub OldStackLimit: PVOID,
    pub StackBase: PVOID,
    pub StackLimit: PVOID,
    pub StackAllocationBase: PVOID,
}
pub type PINITIAL_TEB = *mut INITIAL_TEB;

#[repr(C)]
pub struct PROCESS_BASIC_INFORMATION {
    pub ExitStatus: NTSTATUS,
    pub PebBaseAddress: PPEB,
    pub AffinityMask: KAFFINITY,
    pub BasePriority: KPRIORITY,
    pub UniqueProcessId: HANDLE,
    pub InheritedFromUniqueProcessId: HANDLE,
}

pub type PPROCESS_BASIC_INFORMATION = *mut PROCESS_BASIC_INFORMATION;

#[repr(C)]
pub struct PROCESS_EXTENDED_BASIC_INFORMATION {
    pub Size: SIZE_T,
    pub BasicInfo: PROCESS_BASIC_INFORMATION,
    pub Flags: PROCESS_EXTENDED_BASIC_INFORMATION_FLAGS,
}

#[repr(C)]
pub union PROCESS_EXTENDED_BASIC_INFORMATION_FLAGS {
    pub Flags: ULONG,
    pub bits: PROCESS_EXTENDED_BASIC_INFORMATION_FLAGS_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_EXTENDED_BASIC_INFORMATION_FLAGS_BITS {
    pub IsProtectedProcess: u32,
    pub IsWow64Process: u32,
    pub IsProcessDeleting: u32,
    pub IsCrossSessionCreate: u32,
    pub IsFrozen: u32,
    pub IsBackground: u32,
    pub IsStronglyNamed: u32,
    pub IsSecureProcess: u32,
    pub IsSubsystemProcess: u32,
    pub SpareBits: u32,
}

pub type PPROCESS_EXTENDED_BASIC_INFORMATION = *mut PROCESS_EXTENDED_BASIC_INFORMATION;

pub type KAFFINITY = ULONG_PTR;
pub type PKAFFINITY = *mut KAFFINITY;
pub type KPRIORITY = LONG;

#[repr(C)]
pub struct GROUP_AFFINITY {
    pub Mask: KAFFINITY,
    pub Group: WORD,
    pub Reserved: [WORD; 3],
}
pub type PGROUP_AFFINITY = *mut GROUP_AFFINITY;

#[repr(C)]
pub struct GROUP_AFFINITY32 {
    pub Mask: DWORD,
    pub Group: WORD,
    pub Reserved: [WORD; 3],
}
pub type PGROUP_AFFINITY32 = *mut GROUP_AFFINITY32;

#[repr(C)]
pub struct GROUP_AFFINITY64 {
    pub Mask: u64,
    pub Group: WORD,
    pub Reserved: [WORD; 3],
}
pub type PGROUP_AFFINITY64 = *mut GROUP_AFFINITY64;

#[repr(C)]
pub struct VM_COUNTERS {
    pub PeakVirtualSize: SIZE_T,
    pub VirtualSize: SIZE_T,
    pub PageFaultCount: ULONG,
    pub PeakWorkingSetSize: SIZE_T,
    pub WorkingSetSize: SIZE_T,
    pub QuotaPeakPagedPoolUsage: SIZE_T,
    pub QuotaPagedPoolUsage: SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: SIZE_T,
    pub QuotaNonPagedPoolUsage: SIZE_T,
    pub PagefileUsage: SIZE_T,
    pub PeakPagefileUsage: SIZE_T,
}

pub type PVM_COUNTERS = *mut VM_COUNTERS;

#[repr(C)]
pub struct VM_COUNTERS_EX {
    pub PeakVirtualSize: SIZE_T,
    pub VirtualSize: SIZE_T,
    pub PageFaultCount: ULONG,
    pub PeakWorkingSetSize: SIZE_T,
    pub WorkingSetSize: SIZE_T,
    pub QuotaPeakPagedPoolUsage: SIZE_T,
    pub QuotaPagedPoolUsage: SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: SIZE_T,
    pub QuotaNonPagedPoolUsage: SIZE_T,
    pub PagefileUsage: SIZE_T,
    pub PeakPagefileUsage: SIZE_T,
    pub PrivateUsage: SIZE_T,
}

pub type PVM_COUNTERS_EX = *mut VM_COUNTERS_EX;

#[repr(C)]
pub struct VM_COUNTERS_EX2 {
    pub CountersEx: VM_COUNTERS_EX,
    pub PrivateWorkingSetSize: SIZE_T,
    pub SharedCommitUsage: SIZE_T,
}

pub type PVM_COUNTERS_EX2 = *mut VM_COUNTERS_EX2;

#[repr(C)]
pub struct KERNEL_USER_TIMES {
    pub CreateTime: LARGE_INTEGER,
    pub ExitTime: LARGE_INTEGER,
    pub KernelTime: LARGE_INTEGER,
    pub UserTime: LARGE_INTEGER,
}

pub type PKERNEL_USER_TIMES = *mut KERNEL_USER_TIMES;

#[repr(C)]
pub struct THREAD_BASIC_INFORMATION {
    pub ExitStatus: NTSTATUS,
    pub TebBaseAddress: PTEB,
    pub ClientId: CLIENT_ID,
    pub AffinityMask: KAFFINITY,
    pub Priority: KPRIORITY,
    pub BasePriority: KPRIORITY,
}

pub type PTHREAD_BASIC_INFORMATION = *mut THREAD_BASIC_INFORMATION;

pub const NtCurrentProcess: HANDLE = -1isize as HANDLE;
pub const NtCurrentThread: HANDLE = -2isize as HANDLE;
pub const NtCurrentSession: HANDLE = -3isize as HANDLE;

pub const PS_ATTRIBUTE_NUMBER_MASK: ULONG_PTR = 0x0000ffff;
pub const PS_ATTRIBUTE_THREAD: ULONG_PTR = 0x00010000;
pub const PS_ATTRIBUTE_INPUT: ULONG_PTR = 0x00020000;
pub const PS_ATTRIBUTE_ADDITIVE: ULONG_PTR = 0x00040000;

#[inline]
pub const fn ps_attribute_value(number: ULONG_PTR, thread: bool, input: bool, additive: bool) -> ULONG_PTR {
    (number & PS_ATTRIBUTE_NUMBER_MASK) |
    (if thread { PS_ATTRIBUTE_THREAD } else { 0 }) |
    (if input { PS_ATTRIBUTE_INPUT } else { 0 }) |
    (if additive { PS_ATTRIBUTE_ADDITIVE } else { 0 })
}

pub const PS_ATTRIBUTE_PARENT_PROCESS: ULONG_PTR = ps_attribute_value(0, false, true, true);
pub const PS_ATTRIBUTE_DEBUG_OBJECT: ULONG_PTR = ps_attribute_value(1, false, true, true);
pub const PS_ATTRIBUTE_TOKEN: ULONG_PTR = ps_attribute_value(2, false, true, true);
pub const PS_ATTRIBUTE_CLIENT_ID: ULONG_PTR = ps_attribute_value(3, true, false, false);
pub const PS_ATTRIBUTE_TEB_ADDRESS: ULONG_PTR = ps_attribute_value(4, true, false, false);
pub const PS_ATTRIBUTE_IMAGE_NAME: ULONG_PTR = ps_attribute_value(5, false, true, false);
pub const PS_ATTRIBUTE_IMAGE_INFO: ULONG_PTR = ps_attribute_value(6, false, false, false);
pub const PS_ATTRIBUTE_MEMORY_RESERVE: ULONG_PTR = ps_attribute_value(7, false, true, false);
pub const PS_ATTRIBUTE_PRIORITY_CLASS: ULONG_PTR = ps_attribute_value(8, false, true, false);
pub const PS_ATTRIBUTE_ERROR_MODE: ULONG_PTR = ps_attribute_value(9, false, true, false);
pub const PS_ATTRIBUTE_STD_HANDLE_INFO: ULONG_PTR = ps_attribute_value(10, false, true, false);
pub const PS_ATTRIBUTE_HANDLE_LIST: ULONG_PTR = ps_attribute_value(11, false, true, false);
pub const PS_ATTRIBUTE_GROUP_AFFINITY: ULONG_PTR = ps_attribute_value(12, true, true, false);
pub const PS_ATTRIBUTE_PREFERRED_NODE: ULONG_PTR = ps_attribute_value(13, false, true, false);
pub const PS_ATTRIBUTE_IDEAL_PROCESSOR: ULONG_PTR = ps_attribute_value(14, true, true, false);
pub const PS_ATTRIBUTE_UMS_THREAD: ULONG_PTR = ps_attribute_value(15, true, true, false);
pub const PS_ATTRIBUTE_MITIGATION_OPTIONS: ULONG_PTR = ps_attribute_value(16, false, true, false);
pub const PS_ATTRIBUTE_PROTECTION_LEVEL: ULONG_PTR = ps_attribute_value(17, false, true, true);
pub const PS_ATTRIBUTE_SECURE_PROCESS: ULONG_PTR = ps_attribute_value(18, false, true, false);
pub const PS_ATTRIBUTE_JOB_LIST: ULONG_PTR = ps_attribute_value(19, false, true, false);
pub const PS_ATTRIBUTE_CHILD_PROCESS_POLICY: ULONG_PTR = ps_attribute_value(20, false, true, false);
pub const PS_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: ULONG_PTR = ps_attribute_value(21, false, true, false);
pub const PS_ATTRIBUTE_WIN32K_FILTER: ULONG_PTR = ps_attribute_value(22, false, true, false);
pub const PS_ATTRIBUTE_SAFE_OPEN_PROMPT_ORIGIN_CLAIM: ULONG_PTR = ps_attribute_value(23, false, true, false);
pub const PS_ATTRIBUTE_BNO_ISOLATION: ULONG_PTR = ps_attribute_value(24, false, true, false);
pub const PS_ATTRIBUTE_DESKTOP_APP_POLICY: ULONG_PTR = ps_attribute_value(25, false, true, false);
pub const PS_ATTRIBUTE_CHPE: ULONG_PTR = ps_attribute_value(26, false, true, true);
pub const PS_ATTRIBUTE_MITIGATION_AUDIT_OPTIONS: ULONG_PTR = ps_attribute_value(27, false, true, false);
pub const PS_ATTRIBUTE_MACHINE_TYPE: ULONG_PTR = ps_attribute_value(28, false, true, true);
pub const PS_ATTRIBUTE_COMPONENT_FILTER: ULONG_PTR = ps_attribute_value(29, false, true, false);
pub const PS_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: ULONG_PTR = ps_attribute_value(30, true, true, false);

#[repr(C)]
pub union PS_ATTRIBUTE_U {
    pub Value: ULONG_PTR,
    pub ValuePtr: PVOID,
}

#[repr(C)]
pub struct PS_ATTRIBUTE {
    pub Attribute: ULONG_PTR,
    pub Size: SIZE_T,
    pub u: PS_ATTRIBUTE_U,
    pub ReturnLength: PSIZE_T,
}

pub type PPS_ATTRIBUTE = *mut PS_ATTRIBUTE;

#[repr(C)]
pub struct PS_ATTRIBUTE_LIST {
    pub TotalLength: SIZE_T,
    pub Attributes: *mut PS_ATTRIBUTE,
}

pub type PPS_ATTRIBUTE_LIST = *mut PS_ATTRIBUTE_LIST;

#[repr(u32)]
pub enum ProcessInfoClass {
    ProcessBasicInformation = 0,
    ProcessQuotaLimits = 1,
    ProcessIoCounters = 2,
    ProcessVmCounters = 3,
    ProcessTimes = 4,
    ProcessBasePriority = 5,
    ProcessRaisePriority = 6,
    ProcessDebugPort = 7,
    ProcessExceptionPort = 8,
    ProcessAccessToken = 9,
    ProcessLdtInformation = 10,
    ProcessLdtSize = 11,
    ProcessDefaultHardErrorMode = 12,
    ProcessIoPortHandlers = 13,
    ProcessPooledUsageAndLimits = 14,
    ProcessWorkingSetWatch = 15,
    ProcessUserModeIOPL = 16,
    ProcessEnableAlignmentFaultFixup = 17,
    ProcessPriorityClass = 18,
    ProcessWx86Information = 19,
    ProcessHandleCount = 20,
    ProcessAffinityMask = 21,
    ProcessPriorityBoost = 22,
    ProcessDeviceMap = 23,
    ProcessSessionInformation = 24,
    ProcessForegroundInformation = 25,
    ProcessWow64Information = 26,
    ProcessImageFileName = 27,
    ProcessLUIDDeviceMapsEnabled = 28,
    ProcessBreakOnTermination = 29,
    ProcessDebugObjectHandle = 30,
    ProcessDebugFlags = 31,
    ProcessHandleTracing = 32,
    ProcessIoPriority = 33,
    ProcessExecuteFlags = 34,
    ProcessTlsInformation = 35,
    ProcessCookie = 36,
    ProcessImageInformation = 37,
    ProcessCycleTime = 38,
    ProcessPagePriority = 39,
    ProcessInstrumentationCallback = 40,
    ProcessThreadStackAllocation = 41,
    ProcessWorkingSetWatchEx = 42,
    ProcessImageFileNameWin32 = 43,
    ProcessImageFileMapping = 44,
    ProcessAffinityUpdateMode = 45,
    ProcessMemoryAllocationMode = 46,
    ProcessGroupInformation = 47,
    ProcessTokenVirtualizationEnabled = 48,
    ProcessConsoleHostProcess = 49,
    ProcessWindowInformation = 50,
    ProcessHandleInformation = 51,
    ProcessMitigationPolicy = 52,
    ProcessDynamicFunctionTableInformation = 53,
    ProcessHandleCheckingMode = 54,
    ProcessKeepAliveCount = 55,
    ProcessRevokeFileHandles = 56,
    ProcessWorkingSetControl = 57,
    ProcessHandleTable = 58,
    ProcessCheckStackExtentsMode = 59,
    ProcessCommandLineInformation = 60,
    ProcessProtectionInformation = 61,
    ProcessMemoryExhaustion = 62,
    ProcessFaultInformation = 63,
    ProcessTelemetryIdInformation = 64,
    ProcessCommitReleaseInformation = 65,
    ProcessDefaultCpuSetsInformation = 66,
    ProcessAllowedCpuSetsInformation = 67,
    ProcessSubsystemProcess = 68,
    ProcessJobMemoryInformation = 69,
    ProcessInPrivate = 70,
    ProcessRaiseUMExceptionOnInvalidHandleClose = 71,
    ProcessIumChallengeResponse = 72,
    ProcessChildProcessInformation = 73,
    ProcessHighGraphicsPriorityInformation = 74,
    ProcessSubsystemInformation = 75,
    ProcessEnergyValues = 76,
    ProcessPowerThrottlingState = 77,
    ProcessActivityThrottlePolicy = 78,
    ProcessWin32kSyscallFilterInformation = 79,
    ProcessDisableSystemAllowedCpuSets = 80,
    ProcessWakeInformation = 81,
    ProcessEnergyTrackingState = 82,
    ProcessManageWritesToExecutableMemory = 83,
    ProcessCaptureTrustletLiveDump = 84,
    ProcessTelemetryCoverage = 85,
    ProcessEnclaveInformation = 86,
    ProcessEnableReadWriteVmLogging = 87,
    ProcessUptimeInformation = 88,
    ProcessImageSection = 89,
    ProcessDebugAuthInformation = 90,
    ProcessSystemResourceManagement = 91,
    ProcessSequenceNumber = 92,
    ProcessLoaderDetour = 93,
    ProcessSecurityDomainInformation = 94,
    ProcessCombineSecurityDomainsInformation = 95,
    ProcessEnableLogging = 96,
    ProcessLeapSecondInformation = 97,
    ProcessFiberShadowStackAllocation = 98,
    ProcessFreeFiberShadowStackAllocation = 99,
    ProcessAltSystemCallInformation = 100,
}

#[repr(C)]
pub struct NTPSS_MEMORY_BULK_INFORMATION {
    pub QueryFlags: ULONG,
    pub NumberOfEntries: ULONG,
    pub NextValidAddress: PVOID,
}
pub type PNTPSS_MEMORY_BULK_INFORMATION = *mut NTPSS_MEMORY_BULK_INFORMATION;

#[repr(C)]
pub struct PROCESS_EXCEPTION_PORT {
    pub ExceptionPortHandle: HANDLE,
    pub StateFlags: ULONG,
}
pub type PPROCESS_EXCEPTION_PORT = *mut PROCESS_EXCEPTION_PORT;

#[repr(C)]
pub struct PROCESS_ACCESS_TOKEN {
    pub Token: HANDLE,
    pub Thread: HANDLE,
}
pub type PPROCESS_ACCESS_TOKEN = *mut PROCESS_ACCESS_TOKEN;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PROCESS_STATE_CHANGE_TYPE {
    ProcessStateChangeSuspend = 0,
    ProcessStateChangeResume = 1,
    ProcessStateChangeMax = 2,
}
pub type PPROCESS_STATE_CHANGE_TYPE = *mut PROCESS_STATE_CHANGE_TYPE;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum THREAD_STATE_CHANGE_TYPE {
    ThreadStateChangeSuspend = 0,
    ThreadStateChangeResume = 1,
    ThreadStateChangeMax = 2,
}
pub type PTHREAD_STATE_CHANGE_TYPE = *mut THREAD_STATE_CHANGE_TYPE;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PS_ALERT_THREAD_EXTENDED_PARAMETER_TYPE {
    PsAlertMultipleExtendedParameterAutoBoostContext = 0,
    PsAlertMultipleExtendedParameterMax = 1,
}
pub type PPS_ALERT_THREAD_EXTENDED_PARAMETER_TYPE = *mut PS_ALERT_THREAD_EXTENDED_PARAMETER_TYPE;

#[repr(C)]
pub struct PS_ALERT_THREAD_EXTENDED_PARAMETER {
    pub TypeAndReserved: ULONGLONG,
    pub Anonymous: PS_ALERT_THREAD_EXTENDED_PARAMETER_ANON,
}
pub type PPS_ALERT_THREAD_EXTENDED_PARAMETER = *mut PS_ALERT_THREAD_EXTENDED_PARAMETER;

#[repr(C)]
pub union PS_ALERT_THREAD_EXTENDED_PARAMETER_ANON {
    pub ULong64: ULONGLONG,
    pub Pointer: PVOID,
    pub Size: SIZE_T,
    pub Handle: HANDLE,
    pub ULong: ULONG,
    pub Boolean: UCHAR,
}

#[inline]
pub fn NtCurrentTeb() -> PTEB {
    unsafe {
        let teb: PTEB;
        core::arch::asm!(
            "mov {}, gs:[0x30]",
            out(reg) teb);
        teb
    }
}

#[inline]
pub unsafe fn NtCurrentPeb() -> PPEB {
    (*NtCurrentTeb()).ProcessEnvironmentBlock
}

#[inline]
pub unsafe fn NtCurrentProcessId() -> HANDLE {
    (*NtCurrentTeb()).ClientId.UniqueProcess
}

#[inline]
pub unsafe fn NtCurrentThreadId() -> HANDLE {
    (*NtCurrentTeb()).ClientId.UniqueThread
}

unsafe extern "system" {
    pub fn NtQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn NtSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtQueryInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn NtSetInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtOpenProcess(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;

    pub fn NtTerminateProcess(
        ProcessHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;

    pub fn NtCreateThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ThreadContext: PCONTEXT,
        InitialTeb: PINITIAL_TEB,
        CreateSuspended: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtOpenThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;

    pub fn NtTerminateThread(
        ThreadHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;

    pub fn NtSuspendThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;

    pub fn NtResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;

    pub fn NtGetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;

    pub fn NtSetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;

    pub fn NtQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;

    pub fn NtCreateJobObject(
        JobHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtAssignProcessToJobObject(
        JobHandle: HANDLE,
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn NtQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: ULONG,
        JobObjectInformation: PVOID,
        JobObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}

#[repr(C)]
pub struct INITIAL_TEB_OLD {
    pub OldStackBase: PVOID,
    pub OldStackLimit: PVOID,
}

#[repr(C)]
pub struct PS_STD_HANDLE_INFO {
    pub Flags: ULONG,
    pub StdHandleSubsystemType: ULONG,
}

#[repr(C)]
pub union PS_STD_HANDLE_INFO_U {
    pub Flags: ULONG,
    pub bits: PS_STD_HANDLE_INFO_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PS_STD_HANDLE_INFO_BITS {
    pub StdHandleState: ULONG,
    pub PseudoHandleMask: ULONG,
}

pub type PPS_STD_HANDLE_INFO = *mut PS_STD_HANDLE_INFO;

#[repr(C)]
pub struct PS_MEMORY_RESERVE {
    pub ReserveAddress: PVOID,
    pub ReserveSize: SIZE_T,
}

pub type PPS_MEMORY_RESERVE = *mut PS_MEMORY_RESERVE;

#[repr(C)]
pub struct PS_BNO_ISOLATION_PARAMETERS {
    pub IsolationPrefix: UNICODE_STRING,
    pub HandleCount: ULONG,
    pub Handles: *mut PVOID,
    pub IsolationEnabled: BOOLEAN,
}

pub type PPS_BNO_ISOLATION_PARAMETERS = *mut PS_BNO_ISOLATION_PARAMETERS;


#[repr(C)]
#[derive(Clone, Copy)]
pub struct PS_CREATE_INFO_U_INIT_STATE {
    pub InitFlags: ULONG,
    pub AdditionalFileAccess: ACCESS_MASK,
}

pub const PS_CREATE_INFO_INIT_FLAGS_WRITE_OUTPUT_ON_EXIT: ULONG = 0x00000001;
pub const PS_CREATE_INFO_INIT_FLAGS_DETECT_MANIFEST: ULONG = 0x00000002;
pub const PS_CREATE_INFO_INIT_FLAGS_IFEO_SKIP_DEBUGGER: ULONG = 0x00000004;
pub const PS_CREATE_INFO_INIT_FLAGS_IFEO_DO_NOT_PROPAGATE_KEY_STATE: ULONG = 0x00000008;
pub const PS_CREATE_INFO_INIT_FLAGS_SPARE_BITS1: ULONG = 0x000000F0;
pub const PS_CREATE_INFO_INIT_FLAGS_SPARE_BITS2: ULONG = 0x0000FF00;
pub const PS_CREATE_INFO_INIT_FLAGS_PROHIBITED_IMAGE_CHARACTERISTICS: ULONG = 0xFFFF0000;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PS_CREATE_INFO_U_SUCCESS_STATE {
    pub OutputFlags: ULONG,
    pub FileHandle: HANDLE,
    pub SectionHandle: HANDLE,
    pub UserProcessParametersNative: ULONGLONG,
    pub UserProcessParametersWow64: ULONG,
    pub CurrentParameterFlags: ULONG,
    pub PebAddressNative: ULONGLONG,
    pub PebAddressWow64: ULONG,
    pub ManifestAddress: ULONGLONG,
    pub ManifestSize: ULONG,
}

pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_PROTECTED_PROCESS: ULONG = 0x00000001;
pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_ADDRESS_SPACE_OVERRIDE: ULONG = 0x00000002;
pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_DEV_OVERRIDE_ENABLED: ULONG = 0x00000004;
pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_MANIFEST_DETECTED: ULONG = 0x00000008;
pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_PROTECTED_PROCESS_LIGHT: ULONG = 0x00000010;
pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_SPARE_BITS1: ULONG = 0x000000E0;
pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_SPARE_BITS2: ULONG = 0x0000FF00;
pub const PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_SPARE_BITS3: ULONG = 0xFFFF0000;

#[repr(C)]
pub union PS_CREATE_INFO_U {
    pub InitState: PS_CREATE_INFO_U_INIT_STATE,
    pub FileHandle: HANDLE,
    pub DllCharacteristics: USHORT,
    pub IFEOKey: HANDLE,
    pub SuccessState: PS_CREATE_INFO_U_SUCCESS_STATE,
}

#[repr(C)]
pub struct PS_CREATE_INFO {
    pub Size: SIZE_T,
    pub State: PS_CREATE_STATE,
    pub u: PS_CREATE_INFO_U,
}

pub type PPS_CREATE_INFO = *mut PS_CREATE_INFO;

#[repr(C)]
pub struct PS_CREATE_INFO_INIT_STATE {
    pub InitFlags: ULONG,
    pub AdditionalFileAccess: ACCESS_MASK,
}

#[repr(C)]
pub struct PS_CREATE_INFO_FAIL_SECTION {
    pub FileHandle: HANDLE,
}

#[repr(C)]
pub struct PS_CREATE_INFO_EXE_FORMAT {
    pub DllCharacteristics: USHORT,
}

#[repr(C)]
pub struct PS_CREATE_INFO_EXE_NAME {
    pub IFEOKey: HANDLE,
}

#[repr(C)]
pub struct PS_CREATE_INFO_SUCCESS_STATE {
    pub OutputFlags: ULONG,
    pub FileHandle: HANDLE,
    pub SectionHandle: HANDLE,
    pub UserProcessParametersNative: ULONGLONG,
    pub UserProcessParametersWow64: ULONG,
    pub CurrentParameterFlags: ULONG,
    pub PebAddressNative: ULONGLONG,
    pub PebAddressWow64: ULONG,
    pub ManifestAddress: ULONGLONG,
    pub ManifestSize: ULONG,
}

#[repr(C)]
pub union PS_CREATE_INFO_INIT_FLAGS {
    pub InitFlags: ULONG,
    pub bits: PS_CREATE_INFO_INIT_FLAGS_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PS_CREATE_INFO_INIT_FLAGS_BITS {
    pub WriteOutputOnExit: u32,
    pub DetectManifest: u32,
    pub IFEOSkipDebugger: u32,
    pub IFEODoNotPropagateKeyState: u32,
    pub SpareBits1: u32,
    pub SpareBits2: u32,
    pub ProhibitedImageCharacteristics: u32,
}

#[repr(C)]
pub union PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS {
    pub OutputFlags: ULONG,
    pub bits: PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PS_CREATE_INFO_SUCCESS_OUTPUT_FLAGS_BITS {
    pub ProtectedProcess: u32,
    pub AddressSpaceOverride: u32,
    pub DevOverrideEnabled: u32,
    pub ManifestDetected: u32,
    pub ProtectedProcessLight: u32,
    pub SpareBits1: u32,
    pub SpareBits2: u32,
    pub SpareBits3: u32,
}

pub type PS_CREATE_STATE = ULONG;

pub const PsCreateInitialState: PS_CREATE_STATE = 0;
pub const PsCreateFailOnFileOpen: PS_CREATE_STATE = 1;
pub const PsCreateFailOnSectionCreate: PS_CREATE_STATE = 2;
pub const PsCreateFailExeFormat: PS_CREATE_STATE = 3;
pub const PsCreateFailMachineMismatch: PS_CREATE_STATE = 4;
pub const PsCreateFailExeName: PS_CREATE_STATE = 5;
pub const PsCreateSuccess: PS_CREATE_STATE = 6;
pub const PsCreateMaximumStates: PS_CREATE_STATE = 7;

pub const PROCESS_CREATE_FLAGS_NONE: ULONG = 0x00000000;
pub const PROCESS_CREATE_FLAGS_BREAKAWAY: ULONG = 0x00000001;
pub const PROCESS_CREATE_FLAGS_NO_DEBUG_INHERIT: ULONG = 0x00000002;
pub const PROCESS_CREATE_FLAGS_INHERIT_HANDLES: ULONG = 0x00000004;
pub const PROCESS_CREATE_FLAGS_OVERRIDE_ADDRESS_SPACE: ULONG = 0x00000008;
pub const PROCESS_CREATE_FLAGS_LARGE_PAGES: ULONG = 0x00000010;
pub const PROCESS_CREATE_FLAGS_LARGE_PAGE_SYSTEM_DLL: ULONG = 0x00000020;
pub const PROCESS_CREATE_FLAGS_PROTECTED_PROCESS: ULONG = 0x00000040;
pub const PROCESS_CREATE_FLAGS_CREATE_SESSION: ULONG = 0x00000080;
pub const PROCESS_CREATE_FLAGS_INHERIT_FROM_PARENT: ULONG = 0x00000100;
pub const PROCESS_CREATE_FLAGS_CREATE_SUSPENDED: ULONG = 0x00000200;
pub const PROCESS_CREATE_FLAGS_FORCE_BREAKAWAY: ULONG = 0x00000400;
pub const PROCESS_CREATE_FLAGS_MINIMAL_PROCESS: ULONG = 0x00000800;
pub const PROCESS_CREATE_FLAGS_RELEASE_SECTION: ULONG = 0x00001000;
pub const PROCESS_CREATE_FLAGS_CLONE_MINIMAL: ULONG = 0x00002000;
pub const PROCESS_CREATE_FLAGS_CLONE_MINIMAL_REDUCED_COMMIT: ULONG = 0x00004000;
pub const PROCESS_CREATE_FLAGS_AUXILIARY_PROCESS: ULONG = 0x00008000;
pub const PROCESS_CREATE_FLAGS_CREATE_STORE: ULONG = 0x00020000;
pub const PROCESS_CREATE_FLAGS_USE_PROTECTED_ENVIRONMENT: ULONG = 0x00040000;
pub const PROCESS_CREATE_FLAGS_IMAGE_EXPANSION_MITIGATION_DISABLE: ULONG = 0x00080000;
pub const PROCESS_CREATE_FLAGS_PARTITION_CREATE_SLAB_IDENTITY: ULONG = 0x00400000;

pub const THREAD_CREATE_FLAGS_NONE: ULONG = 0x00000000;
pub const THREAD_CREATE_FLAGS_CREATE_SUSPENDED: ULONG = 0x00000001;
pub const THREAD_CREATE_FLAGS_SKIP_THREAD_ATTACH: ULONG = 0x00000002;
pub const THREAD_CREATE_FLAGS_HIDE_FROM_DEBUGGER: ULONG = 0x00000004;
pub const THREAD_CREATE_FLAGS_LOADER_WORKER: ULONG = 0x00000010;
pub const THREAD_CREATE_FLAGS_SKIP_LOADER_INIT: ULONG = 0x00000020;
pub const THREAD_CREATE_FLAGS_BYPASS_PROCESS_FREEZE: ULONG = 0x00000040;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MEMORY_RESERVE_TYPE {
    MemoryReserveUserApc = 0,
    MemoryReserveIoCompletion = 1,
    MemoryReserveTypeMax = 2,
}

unsafe extern "system" {
    pub fn NtCreateProcessEx(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        Flags: ULONG,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
        Reserved: ULONG,
    ) -> NTSTATUS;

    pub fn NtCreateUserProcess(
        ProcessHandle: PHANDLE,
        ThreadHandle: PHANDLE,
        ProcessDesiredAccess: ACCESS_MASK,
        ThreadDesiredAccess: ACCESS_MASK,
        ProcessObjectAttributes: PCOBJECT_ATTRIBUTES,
        ThreadObjectAttributes: PCOBJECT_ATTRIBUTES,
        ProcessFlags: ULONG,
        ThreadFlags: ULONG,
        ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
        CreateInfo: PPS_CREATE_INFO,
        AttributeList: PPS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;

    pub fn NtCreateThreadEx(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        StartRoutine: PUSER_THREAD_START_ROUTINE,
        Argument: PVOID,
        CreateFlags: ULONG,
        ZeroBits: SIZE_T,
        StackSize: SIZE_T,
        MaximumStackSize: SIZE_T,
        AttributeList: PPS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;

    pub fn NtGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewProcessHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn NtGetNextThread(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewThreadHandle: PHANDLE,
    ) -> NTSTATUS;
}