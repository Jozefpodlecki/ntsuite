use crate::{ntdef::*, ntobapi::*, ntrtl::*};

pub type PCCH = *const i8;
pub type PCH = *mut i8;
pub type LPDEBUG_EVENT = *mut core::ffi::c_void;
pub type PREGHANDLE = *mut REGHANDLE;
pub type REGHANDLE = ULONGLONG;

pub const DBG_STATUS_CONTROL_C: u32 = 1;
pub const DBG_STATUS_SYSRQ: u32 = 2;
pub const DBG_STATUS_BUGCHECK_FIRST: u32 = 3;
pub const DBG_STATUS_BUGCHECK_SECOND: u32 = 4;
pub const DBG_STATUS_FATAL: u32 = 5;
pub const DBG_STATUS_DEBUG_CONTROL: u32 = 6;
pub const DBG_STATUS_WORKER: u32 = 7;

pub const DEBUG_READ_EVENT: u32 = 0x0001;
pub const DEBUG_PROCESS_ASSIGN: u32 = 0x0002;
pub const DEBUG_SET_INFORMATION: u32 = 0x0004;
pub const DEBUG_QUERY_INFORMATION: u32 = 0x0008;
pub const DEBUG_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | DEBUG_READ_EVENT | DEBUG_PROCESS_ASSIGN | DEBUG_SET_INFORMATION | DEBUG_QUERY_INFORMATION;
pub const DEBUG_KILL_ON_CLOSE: u32 = 0x1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGKM_EXCEPTION {
    pub ExceptionRecord: EXCEPTION_RECORD,
    pub FirstChance: ULONG,
}
pub type PDBGKM_EXCEPTION = *mut DBGKM_EXCEPTION;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGKM_CREATE_THREAD {
    pub SubSystemKey: ULONG,
    pub StartAddress: PVOID,
}
pub type PDBGKM_CREATE_THREAD = *mut DBGKM_CREATE_THREAD;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGKM_CREATE_PROCESS {
    pub SubSystemKey: ULONG,
    pub FileHandle: HANDLE,
    pub BaseOfImage: PVOID,
    pub DebugInfoFileOffset: ULONG,
    pub DebugInfoSize: ULONG,
    pub InitialThread: DBGKM_CREATE_THREAD,
}
pub type PDBGKM_CREATE_PROCESS = *mut DBGKM_CREATE_PROCESS;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGKM_EXIT_THREAD {
    pub ExitStatus: NTSTATUS,
}
pub type PDBGKM_EXIT_THREAD = *mut DBGKM_EXIT_THREAD;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGKM_EXIT_PROCESS {
    pub ExitStatus: NTSTATUS,
}
pub type PDBGKM_EXIT_PROCESS = *mut DBGKM_EXIT_PROCESS;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGKM_LOAD_DLL {
    pub FileHandle: HANDLE,
    pub BaseOfDll: PVOID,
    pub DebugInfoFileOffset: ULONG,
    pub DebugInfoSize: ULONG,
    pub NamePointer: PVOID,
}
pub type PDBGKM_LOAD_DLL = *mut DBGKM_LOAD_DLL;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGKM_UNLOAD_DLL {
    pub BaseAddress: PVOID,
}
pub type PDBGKM_UNLOAD_DLL = *mut DBGKM_UNLOAD_DLL;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DBG_STATE {
    DbgIdle = 0,
    DbgReplyPending = 1,
    DbgCreateThreadStateChange = 2,
    DbgCreateProcessStateChange = 3,
    DbgExitThreadStateChange = 4,
    DbgExitProcessStateChange = 5,
    DbgExceptionStateChange = 6,
    DbgBreakpointStateChange = 7,
    DbgSingleStepStateChange = 8,
    DbgLoadDllStateChange = 9,
    DbgUnloadDllStateChange = 10,
}
pub type PDBG_STATE = *mut DBG_STATE;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGUI_CREATE_THREAD {
    pub HandleToThread: HANDLE,
    pub NewThread: DBGKM_CREATE_THREAD,
}
pub type PDBGUI_CREATE_THREAD = *mut DBGUI_CREATE_THREAD;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DBGUI_CREATE_PROCESS {
    pub HandleToProcess: HANDLE,
    pub HandleToThread: HANDLE,
    pub NewProcess: DBGKM_CREATE_PROCESS,
}
pub type PDBGUI_CREATE_PROCESS = *mut DBGUI_CREATE_PROCESS;

#[repr(C)]
pub struct DBGUI_WAIT_STATE_CHANGE {
    pub NewState: DBG_STATE,
    pub AppClientId: CLIENT_ID,
    pub StateInfo: DBGUI_WAIT_STATE_CHANGE_STATE_INFO,
}
pub type PDBGUI_WAIT_STATE_CHANGE = *mut DBGUI_WAIT_STATE_CHANGE;

#[repr(C)]
pub union DBGUI_WAIT_STATE_CHANGE_STATE_INFO {
    pub Exception: DBGKM_EXCEPTION,
    pub CreateThread: DBGUI_CREATE_THREAD,
    pub CreateProcessInfo: DBGUI_CREATE_PROCESS,
    pub ExitThread: DBGKM_EXIT_THREAD,
    pub ExitProcess: DBGKM_EXIT_PROCESS,
    pub LoadDll: DBGKM_LOAD_DLL,
    pub UnloadDll: DBGKM_UNLOAD_DLL,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DEBUGOBJECTINFOCLASS {
    DebugObjectUnusedInformation = 0,
    DebugObjectKillProcessOnExitInformation = 1,
    MaxDebugObjectInfoClass = 2,
}
pub type PDEBUGOBJECTINFOCLASS = *mut DEBUGOBJECTINFOCLASS;

pub type PEVENT_FILTER_DESCRIPTOR = *mut core::ffi::c_void;

pub type ENABLECALLBACK = Option<unsafe extern "system" fn(
    SourceId: LPCGUID,
    IsEnabled: ULONG,
    Level: UCHAR,
    MatchAnyKeyword: ULONGLONG,
    MatchAllKeyword: ULONGLONG,
    FilterData: PEVENT_FILTER_DESCRIPTOR,
    CallbackContext: PVOID,
)>;
pub type PENABLECALLBACK = *mut ENABLECALLBACK;

unsafe extern "system" {
    pub fn DbgUserBreakPoint();
    pub fn DbgBreakPoint();
    pub fn DbgBreakPointWithStatus(Status: ULONG);
    pub fn DbgPrint(Format: PCCH) -> ULONG;
    pub fn DbgPrintEx(ComponentId: ULONG, Level: ULONG, Format: PCCH) -> ULONG;
    pub fn vDbgPrintEx(ComponentId: ULONG, Level: ULONG, Format: PCCH, arglist: *mut core::ffi::c_void) -> ULONG;
    pub fn vDbgPrintExWithPrefix(Prefix: PCCH, ComponentId: ULONG, Level: ULONG, Format: PCCH, arglist: *mut core::ffi::c_void) -> ULONG;
    pub fn DbgPrintReturnControlC(Format: PCCH) -> ULONG;
    pub fn DbgQueryDebugFilterState(ComponentId: ULONG, Level: ULONG) -> NTSTATUS;
    pub fn DbgSetDebugFilterState(ComponentId: ULONG, Level: ULONG, State: BOOLEAN) -> NTSTATUS;
    pub fn DbgPrompt(Prompt: PCCH, Response: PCH, Length: ULONG) -> ULONG;
    
    pub fn NtCreateDebugObject(
        DebugObjectHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    
    pub fn NtDebugActiveProcess(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    
    pub fn NtDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
    
    pub fn NtRemoveProcessDebug(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    
    pub fn NtSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: PVOID,
        DebugInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    
    pub fn NtWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
        WaitStateChange: PDBGUI_WAIT_STATE_CHANGE,
    ) -> NTSTATUS;
    
    pub fn DbgUiConnectToDbg() -> NTSTATUS;
    pub fn DbgUiGetThreadDebugObject() -> HANDLE;
    pub fn DbgUiSetThreadDebugObject(DebugObject: HANDLE);
    pub fn DbgUiWaitStateChange(StateChange: PDBGUI_WAIT_STATE_CHANGE, Timeout: PLARGE_INTEGER) -> NTSTATUS;
    pub fn DbgUiContinue(AppClientId: PCLIENT_ID, ContinueStatus: NTSTATUS) -> NTSTATUS;
    pub fn DbgUiStopDebugging(Process: HANDLE) -> NTSTATUS;
    pub fn DbgUiDebugActiveProcess(Process: HANDLE) -> NTSTATUS;
    pub fn DbgUiRemoteBreakin(Context: PVOID);
    pub fn DbgUiIssueRemoteBreakin(Process: HANDLE) -> NTSTATUS;
    pub fn DbgUiConvertStateChangeStructure(StateChange: PDBGUI_WAIT_STATE_CHANGE, DebugEvent: LPDEBUG_EVENT) -> NTSTATUS;
    pub fn DbgUiConvertStateChangeStructureEx(StateChange: PDBGUI_WAIT_STATE_CHANGE, DebugEvent: LPDEBUG_EVENT) -> NTSTATUS;
    
    pub fn EtwEventRegister(
        ProviderId: LPCGUID,
        EnableCallback: PENABLECALLBACK,
        CallbackContext: PVOID,
        RegHandle: PREGHANDLE,
    ) -> NTSTATUS;
}