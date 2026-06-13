use core::ffi::c_void;

use crate::ntdef::*;

pub const LOW_PRIORITY: i32 = 0;
pub const LOW_REALTIME_PRIORITY: i32 = 16;
pub const HIGH_PRIORITY: i32 = 31;
pub const MAXIMUM_PRIORITY: i32 = 32;

#[repr(u32)]
pub enum KTHREAD_STATE {
    Initialized = 0,
    Ready = 1,
    Running = 2,
    Standby = 3,
    Terminated = 4,
    Waiting = 5,
    Transition = 6,
    DeferredReady = 7,
    GateWaitObsolete = 8,
    WaitingForProcessInSwap = 9,
    MaximumThreadState = 10,
}

#[repr(u32)]
pub enum KHETERO_CPU_POLICY {
    All = 0,
    Large = 1,
    LargeOrIdle = 2,
    Small = 3,
    SmallOrIdle = 4,
    DynamicOrStaticMax = 5,
    BiasedSmall = 6,
    BiasedLarge = 7,
    Default = 8,
    Max = 9,
}

#[repr(u32)]
pub enum KWAIT_REASON {
    Executive = 0,
    FreePage = 1,
    PageIn = 2,
    PoolAllocation = 3,
    DelayExecution = 4,
    Suspended = 5,
    UserRequest = 6,
    WrExecutive = 7,
    WrFreePage = 8,
    WrPageIn = 9,
    WrPoolAllocation = 10,
    WrDelayExecution = 11,
    WrSuspended = 12,
    WrUserRequest = 13,
    WrEventPair = 14,
    WrQueue = 15,
    WrLpcReceive = 16,
    WrLpcReply = 17,
    WrVirtualMemory = 18,
    WrPageOut = 19,
    WrRendezvous = 20,
    WrKeyedEvent = 21,
    WrTerminated = 22,
    WrProcessInSwap = 23,
    WrCpuRateControl = 24,
    WrCalloutStack = 25,
    WrKernel = 26,
    WrResource = 27,
    WrPushLock = 28,
    WrMutex = 29,
    WrQuantumEnd = 30,
    WrDispatchInt = 31,
    WrPreempted = 32,
    WrYieldExecution = 33,
    WrFastMutex = 34,
    WrGuardedMutex = 35,
    WrRundown = 36,
    WrAlertByThreadId = 37,
    WrDeferredPreempt = 38,
    WrPhysicalFault = 39,
    WrIoRing = 40,
    WrMdlCache = 41,
    WrRcu = 42,
    MaximumWaitReason = 43,
}

#[repr(u32)]
pub enum KPROFILE_SOURCE {
    Time = 0,
    AlignmentFixup = 1,
    TotalIssues = 2,
    PipelineDry = 3,
    LoadInstructions = 4,
    PipelineFrozen = 5,
    BranchInstructions = 6,
    TotalNonissues = 7,
    DcacheMisses = 8,
    IcacheMisses = 9,
    CacheMisses = 10,
    BranchMispredictions = 11,
    StoreInstructions = 12,
    FpInstructions = 13,
    IntegerInstructions = 14,
    Issue2 = 15,
    Issue3 = 16,
    Issue4 = 17,
    SpecialInstructions = 18,
    TotalCycles = 19,
    IcacheIssues = 20,
    DcacheAccesses = 21,
    MemoryBarrierCycles = 22,
    LoadLinkedIssues = 23,
    Maximum = 24,
}

pub type PKTHREAD_STATE = *mut KTHREAD_STATE;
pub type PKHETERO_CPU_POLICY = *mut KHETERO_CPU_POLICY;
pub type PKWAIT_REASON = *mut KWAIT_REASON;

unsafe extern "system" {
    pub fn NtCallbackReturn(
        OutputBuffer: *const c_void,
        OutputLength: ULONG,
        Status: NTSTATUS,
    ) -> NTSTATUS;

    pub fn NtQueryDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
    ) -> NTSTATUS;

    pub fn NtSetDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
        State: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtYieldExecution() -> NTSTATUS;
}