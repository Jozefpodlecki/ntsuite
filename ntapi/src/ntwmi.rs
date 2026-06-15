use crate::{evntrace::*, ntdef::*};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMI_TRACE_PACKET {
    pub Size: USHORT,
    pub DUMMYUNIONNAME: WMI_TRACE_PACKET_UNION,
}
pub type PWMI_TRACE_PACKET = *mut WMI_TRACE_PACKET;

#[repr(C)]
#[derive(Clone, Copy)]
pub union WMI_TRACE_PACKET_UNION {
    pub HookId: USHORT,
    pub DUMMYSTRUCTNAME: WMI_TRACE_PACKET_STRUCT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMI_TRACE_PACKET_STRUCT {
    pub Type: UCHAR,
    pub Group: UCHAR,
}

#[repr(C)]
pub struct PERFINFO_TRACE_HEADER {
    pub DUMMYUNIONNAME: PERFINFO_TRACE_HEADER_UNION1,
    pub DUMMYUNIONNAME2: PERFINFO_TRACE_HEADER_UNION2,
    pub DUMMYUNIONNAME3: PERFINFO_TRACE_HEADER_UNION3,
    pub Data: [UCHAR; 1],
}
pub type PPERFINFO_TRACE_HEADER = *mut PERFINFO_TRACE_HEADER;

#[repr(C)]
pub union PERFINFO_TRACE_HEADER_UNION1 {
    pub Marker: ULONG,
    pub DUMMYSTRUCTNAME: PERFINFO_TRACE_HEADER_STRUCT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PERFINFO_TRACE_HEADER_STRUCT {
    pub Version: USHORT,
    pub HeaderType: UCHAR,
    pub Flags: UCHAR,
}

#[repr(C)]
pub union PERFINFO_TRACE_HEADER_UNION2 {
    pub Header: ULONG,
    pub Packet: WMI_TRACE_PACKET,
}

pub type PERFINFO_TIMESTAMP = ULONGLONG;

#[repr(C)]
pub union PERFINFO_TRACE_HEADER_UNION3 {
    pub TS: PERFINFO_TIMESTAMP,
    pub SystemTime: LARGE_INTEGER,
}

#[repr(C)]
pub struct SYSTEM_TRACE_HEADER {
    pub DUMMYUNIONNAME: SYSTEM_TRACE_HEADER_UNION1,
    pub DUMMYUNIONNAME2: SYSTEM_TRACE_HEADER_UNION2,
    pub ThreadId: ULONG,
    pub ProcessId: ULONG,
    pub SystemTime: LARGE_INTEGER,
    pub KernelTime: ULONG,
    pub UserTime: ULONG,
}
pub type PSYSTEM_TRACE_HEADER = *mut SYSTEM_TRACE_HEADER;

#[repr(C)]
pub union SYSTEM_TRACE_HEADER_UNION1 {
    pub Marker: ULONG,
    pub DUMMYSTRUCTNAME: SYSTEM_TRACE_HEADER_STRUCT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_TRACE_HEADER_STRUCT {
    pub Version: USHORT,
    pub HeaderType: UCHAR,
    pub Flags: UCHAR,
}

#[repr(C)]
pub union SYSTEM_TRACE_HEADER_UNION2 {
    pub Header: ULONG,
    pub Packet: WMI_TRACE_PACKET,
}

#[repr(C)]
pub struct HEAP_SUBSEGMENT_INIT {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
    pub SubSegment: PVOID,
    pub BlockSize: SIZE_T,
    pub BlockCount: SIZE_T,
    pub AffinityIndex: ULONG,
}
pub type PHEAP_SUBSEGMENT_INIT = *mut HEAP_SUBSEGMENT_INIT;

#[repr(C)]
pub struct HEAP_AFFINITY_MANAGER_ENABLE {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
    pub BucketIndex: ULONG,
}
pub type PHEAP_AFFINITY_MANAGER_ENABLE = *mut HEAP_AFFINITY_MANAGER_ENABLE;

#[repr(C)]
pub struct HEAP_AFFINITY_SLOT_ASSIGN {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
    pub SubSegment: PVOID,
    pub SlotIndex: ULONG,
}
pub type PHEAP_AFFINITY_SLOT_ASSIGN = *mut HEAP_AFFINITY_SLOT_ASSIGN;

#[repr(C)]
pub struct HEAP_REUSE_THRESHOLD_ACTIVATED {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
    pub SubSegment: PVOID,
    pub BucketIndex: ULONG,
}
pub type PHEAP_REUSE_THRESHOLD_ACTIVATED = *mut HEAP_REUSE_THRESHOLD_ACTIVATED;

#[repr(C)]
pub struct HEAP_SUBSEGMENT_ACTIVATED {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
    pub SubSegment: PVOID,
}
pub type PHEAP_SUBSEGMENT_ACTIVATED = *mut HEAP_SUBSEGMENT_ACTIVATED;
#[repr(C)]
pub struct ETW_HEAP_EVENT_SNAPSHOT {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
    pub FreeSpace: SIZE_T,
    pub CommittedSpace: SIZE_T,
    pub ReservedSpace: SIZE_T,
    pub Flags: ULONG,
    pub ProcessId: ULONG,
    pub LargeUCRSpace: SIZE_T,
    pub FreeListLength: ULONG,
    pub UCRLength: ULONG,
    pub AllocatedSpace: SIZE_T,
}
pub type PETW_HEAP_EVENT_SNAPSHOT = *mut ETW_HEAP_EVENT_SNAPSHOT;

#[repr(C)]
pub struct ETW_HEAP_EVENT_RUNDOWN_RANGE {
    pub Address: PVOID,
    pub Size: SIZE_T,
}
pub type PETW_HEAP_EVENT_RUNDOWN_RANGE = *mut ETW_HEAP_EVENT_RUNDOWN_RANGE;

#[repr(C)]
pub struct ETW_HEAP_EVENT_RUNDOWN {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
    pub Flags: ULONG,
    pub ProcessId: ULONG,
    pub RangeCount: ULONG,
    pub Reserved: ULONG,
    pub Ranges: [ETW_HEAP_EVENT_RUNDOWN_RANGE; 1],
}
pub type PETW_HEAP_EVENT_RUNDOWN = *mut ETW_HEAP_EVENT_RUNDOWN;

#[repr(C)]
pub struct HEAP_EVENT_RANGE_CREATE {
    pub HeapHandle: PVOID,
    pub FirstRangeSize: SIZE_T,
    pub Flags: ULONG,
}
pub type PHEAP_EVENT_RANGE_CREATE = *mut HEAP_EVENT_RANGE_CREATE;

#[repr(C)]
pub struct HEAP_EVENT_RANGE {
    pub HeapHandle: PVOID,
    pub Address: PVOID,
    pub Size: SIZE_T,
}
pub type PHEAP_EVENT_RANGE = *mut HEAP_EVENT_RANGE;

#[repr(C)]
pub struct HEAP_RANGE_CREATE {
    pub Header: SYSTEM_TRACE_HEADER,
    pub Event: HEAP_EVENT_RANGE_CREATE,
}
pub type PHEAP_RANGE_CREATE = *mut HEAP_RANGE_CREATE;

#[repr(C)]
pub struct HEAP_RANGE_DESTROY {
    pub Header: SYSTEM_TRACE_HEADER,
    pub HeapHandle: PVOID,
}
pub type PHEAP_RANGE_DESTROY = *mut HEAP_RANGE_DESTROY;

#[repr(C)]
pub struct HEAP_RANGE_LOG {
    pub Header: SYSTEM_TRACE_HEADER,
    pub Range: HEAP_EVENT_RANGE,
}
pub type PHEAP_RANGE_LOG = *mut HEAP_RANGE_LOG;

#[repr(C)]
pub struct ETW_CRITSEC_EVENT_COLLISION {
    pub Header: SYSTEM_TRACE_HEADER,
    pub LockCount: ULONG,
    pub SpinCount: ULONG,
    pub OwningThread: PVOID,
    pub Address: PVOID,
}
pub type PETW_CRITSEC_EVENT_COLLISION = *mut ETW_CRITSEC_EVENT_COLLISION;

#[repr(C)]
pub struct ETW_CRITSEC_EVENT_INIT {
    pub Header: SYSTEM_TRACE_HEADER,
    pub SpinCount: PVOID,
    pub Address: PVOID,
}
pub type PETW_CRITSEC_EVENT_INIT = *mut ETW_CRITSEC_EVENT_INIT;

#[repr(C)]
pub struct STACK_WALK_EVENT_DATA {
    pub TimeStamp: ULONGLONG,
    pub ProcessId: ULONG,
    pub ThreadId: ULONG,
    pub Addresses: [PVOID; 1],
}
pub type PSTACK_WALK_EVENT_DATA = *mut STACK_WALK_EVENT_DATA;

#[repr(C)]
pub struct LOAD_DLL_EVENT_DATA {
    pub ImageName: [u16; 1],
}
pub type PLOAD_DLL_EVENT_DATA = *mut LOAD_DLL_EVENT_DATA;

#[repr(C)]
pub struct CM_PERF_COUNTERS {
    pub OpenedKeys: ULONGLONG,
    pub DelayCloseKCBs: ULONGLONG,
    pub PrivateAllocPages: ULONGLONG,
    pub PrivateAllocFree: ULONGLONG,
    pub PrivateAllocUsed: ULONGLONG,
    pub LookupCacheHit: ULONGLONG,
    pub LookupCacheMissFound: ULONGLONG,
    pub LookupCacheMissNotFound: ULONGLONG,
    pub ViewMap: ULONGLONG,
    pub ViewUnMap: ULONGLONG,
    pub HiveShrink: ULONGLONG,
}
pub type PCM_PERF_COUNTERS = *mut CM_PERF_COUNTERS;

#[repr(C)]
pub struct CI_LOG_SCHEDULER_EVENT {
    pub Header: EVENT_TRACE_HEADER,
    pub ProcessId: ULONG,
    pub ThreadId: ULONG,
    pub Priority: ULONG,
    pub TaskIndex: ULONG,
}
pub type PCI_LOG_SCHEDULER_EVENT = *mut CI_LOG_SCHEDULER_EVENT;

#[repr(C)]
pub struct CI_LOG_SCHEDULER_WAKEUP {
    pub Header: EVENT_TRACE_HEADER,
    pub Reason: ULONG,
}
pub type PCI_LOG_SCHEDULER_WAKEUP = *mut CI_LOG_SCHEDULER_WAKEUP;

#[repr(C)]
pub struct CI_LOG_SCHEDULER_SLEEP {
    pub Header: EVENT_TRACE_HEADER,
}
pub type PCI_LOG_SCHEDULER_SLEEP = *mut CI_LOG_SCHEDULER_SLEEP;

#[repr(C)]
pub struct CI_LOG_SCHEDULER_SLEEP_RESPONSE {
    pub Header: EVENT_TRACE_HEADER,
}
pub type PCI_LOG_SCHEDULER_SLEEP_RESPONSE = *mut CI_LOG_SCHEDULER_SLEEP_RESPONSE;

#[repr(C)]
pub struct CI_LOG_MMCSS_START {
    pub Header: EVENT_TRACE_HEADER,
}
pub type PCI_LOG_MMCSS_START = *mut CI_LOG_MMCSS_START;

#[repr(C)]
pub struct CI_LOG_MMCSS_STOP {
    pub Header: EVENT_TRACE_HEADER,
}
pub type PCI_LOG_MMCSS_STOP = *mut CI_LOG_MMCSS_STOP;

pub const UMS_ETW_DIRECTED_SWITCH_START_VOLATILE: u32 = 0x1;

#[repr(C)]
pub struct ETW_UMS_EVENT_DIRECTED_SWITCH_START {
    pub ProcessId: ULONG,
    pub ScheduledThreadId: ULONG,
    pub PrimaryThreadId: ULONG,
    pub SwitchFlags: ULONG,
}
pub type PETW_UMS_EVENT_DIRECTED_SWITCH_START = *mut ETW_UMS_EVENT_DIRECTED_SWITCH_START;

pub const UMS_ETW_DIRECTED_SWITCH_END_FAST: u32 = 0x1;

#[repr(C)]
pub struct ETW_UMS_EVENT_DIRECTED_SWITCH_END {
    pub ProcessId: ULONG,
    pub ScheduledThreadId: ULONG,
    pub PrimaryThreadId: ULONG,
    pub SwitchFlags: ULONG,
}
pub type PETW_UMS_EVENT_DIRECTED_SWITCH_END = *mut ETW_UMS_EVENT_DIRECTED_SWITCH_END;

pub const UMS_ETW_PARK_VOLATILE: u32 = 0x1;
pub const UMS_ETW_PARK_PRIMARY_PRESENT: u32 = 0x2;
pub const UMS_ETW_PARK_PRIMARY_DELIVERED_CONTEXT: u32 = 0x4;

#[repr(C)]
pub struct ETW_UMS_EVENT_PARK {
    pub ProcessId: ULONG,
    pub ScheduledThreadId: ULONG,
    pub ParkFlags: ULONG,
}
pub type PETW_UMS_EVENT_PARK = *mut ETW_UMS_EVENT_PARK;

#[repr(C)]
pub struct ETW_UMS_EVENT_DISASSOCIATE {
    pub ProcessId: ULONG,
    pub ScheduledThreadId: ULONG,
    pub PrimaryThreadId: ULONG,
    pub UmsApcControlFlags: ULONG,
    pub Status: NTSTATUS,
}
pub type PETW_UMS_EVENT_DISASSOCIATE = *mut ETW_UMS_EVENT_DISASSOCIATE;

#[repr(C)]
pub struct ETW_UMS_EVENT_CONTEXT_SWITCH {
    pub Header: SYSTEM_TRACE_HEADER,
    pub ScheduledThreadId: ULONG,
    pub SwitchCount: ULONG,
    pub KernelYieldCount: ULONG,
    pub MixedYieldCount: ULONG,
    pub YieldCount: ULONG,
}
pub type PETW_UMS_EVENT_CONTEXT_SWITCH = *mut ETW_UMS_EVENT_CONTEXT_SWITCH;

#[repr(C)]
pub struct ETW_SET_TIMER_EVENT {
    pub ExpectedDueTime: ULONG64,
    pub TimerAddress: ULONG_PTR,
    pub TargetProcessorGroup: USHORT,
    pub TargetProcessorIndex: UCHAR,
    pub Flags: UCHAR,
    pub Period: ULONG,
    pub EncodedDelay: UCHAR,
    pub Reserved0: UCHAR,
    pub Reserved1: USHORT,
}
pub type PETW_SET_TIMER_EVENT = *mut ETW_SET_TIMER_EVENT;

#[repr(C)]
pub struct ETW_CANCEL_TIMER_EVENT {
    pub TimerAddress: ULONG_PTR,
}
pub type PETW_CANCEL_TIMER_EVENT = *mut ETW_CANCEL_TIMER_EVENT;

#[repr(C)]
pub struct ETW_TIMER_EXPIRATION_EVENT {
    pub ExpectedDueTime: ULONG64,
    pub TimerAddress: ULONG_PTR,
    pub DeferredRoutine: ULONG_PTR,
    pub EncodedDelay: UCHAR,
}
pub type PETW_TIMER_EXPIRATION_EVENT = *mut ETW_TIMER_EXPIRATION_EVENT;

#[repr(C)]
pub struct ETW_TIMER_EXPIRATION_START_EVENT {
    pub InterruptTime: ULONG64,
}
pub type PETW_TIMER_EXPIRATION_START_EVENT = *mut ETW_TIMER_EXPIRATION_START_EVENT;

pub const ETW_KTIMER2_HAS_CALLBACK: u8 = 0x01;
pub const ETW_KTIMER2_PERIODIC: u8 = 0x02;
pub const ETW_KTIMER2_IDLE_RESILIENT: u8 = 0x04;
pub const ETW_KTIMER2_HIGH_RESOLUTION: u8 = 0x08;
pub const ETW_KTIMER2_NO_WAKE: u8 = 0x10;
pub const ETW_KTIMER2_NO_WAKE_FINITE: u8 = 0x20;

pub const ETW_TIMER_COALESCABLE: u8 = 0x01;
pub const ETW_TIMER_DPC: u8 = 0x02;
pub const ETW_TIMER_IDLE_RESILIENT: u8 = 0x04;
pub const ETW_TIMER_HIGH_RESOLUTION: u8 = 0x08;
pub const ETW_TIMER_NO_WAKE: u8 = 0x10;

#[repr(C)]
pub struct ETW_SET_KTIMER2_EVENT {
    pub DueTime: ULONG64,
    pub MaximumDueTime: ULONG64,
    pub Period: ULONG64,
    pub TimerKey: ULONG_PTR,
    pub Callback: ULONG_PTR,
    pub CallbackContextKey: ULONG_PTR,
    pub Flags: UCHAR,
}
pub type PETW_SET_KTIMER2_EVENT = *mut ETW_SET_KTIMER2_EVENT;

pub type ETW_KTIMER2_EXPIRATION_EVENT = ETW_SET_KTIMER2_EVENT;
pub type PETW_KTIMER2_EXPIRATION_EVENT = *mut ETW_KTIMER2_EXPIRATION_EVENT;

#[repr(C)]
pub struct ETW_CANCEL_KTIMER2_EVENT {
    pub TimerKey: ULONG_PTR,
}
pub type PETW_CANCEL_KTIMER2_EVENT = *mut ETW_CANCEL_KTIMER2_EVENT;

pub const ETW_DISABLE_KTIMER2_CANCEL: u8 = 0x1;
pub const ETW_DISABLE_KTIMER2_WAIT: u8 = 0x2;
pub const ETW_DISABLE_KTIMER2_DELAYED: u8 = 0x4;
pub const ETW_DISABLE_KTIMER2_HAS_DISABLE_CALLBACK: u8 = 0x8;

#[repr(C)]
pub struct ETW_DISABLE_KTIMER2_EVENT {
    pub TimerKey: ULONG_PTR,
    pub DisableCallback: ULONG_PTR,
    pub DisableContextKey: ULONG_PTR,
    pub Flags: UCHAR,
}
pub type PETW_DISABLE_KTIMER2_EVENT = *mut ETW_DISABLE_KTIMER2_EVENT;

#[repr(C)]
pub struct ETW_FINALIZE_KTIMER2_EVENT {
    pub TimerKey: ULONG_PTR,
    pub DisableCallback: ULONG_PTR,
    pub DisableContextKey: ULONG_PTR,
}
pub type PETW_FINALIZE_KTIMER2_EVENT = *mut ETW_FINALIZE_KTIMER2_EVENT;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PERFINFO_DYNAMIC_TICK_VETO_REASON {
    DynamicTickVetoNone = 0,
    DynamicTickVetoProcBusy = 1,
    DynamicTickVetoSoftwareTimer = 2,
    DynamicTickVetoClockConstraint = 3,
    DynamicTickVetoClockOutOfSync = 4,
    DynamicTickVetoClockUpdateFailed = 5,
    DynamicTickVetoMax = 6,
}
pub type PPERFINFO_DYNAMIC_TICK_VETO_REASON = *mut PERFINFO_DYNAMIC_TICK_VETO_REASON;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PERFINFO_DYNAMIC_TICK_DISABLE_REASON {
    DynamicTickDisableReasonNone = 0,
    DynamicTickDisableReasonBcdOverride = 1,
    DynamicTickDisableReasonNoHwSupport = 2,
    DynamicTickDisableReasonEmOverride = 3,
    DynamicTickDisableReasonMax = 4,
}
pub type PPERFINFO_DYNAMIC_TICK_DISABLE_REASON = *mut PERFINFO_DYNAMIC_TICK_DISABLE_REASON;

#[repr(C)]
pub struct ETW_CLOCK_CONFIGURATION_EVENT {
    pub KnownType: ULONG,
    pub Capabilities: ULONG,
    pub DisableReason: PERFINFO_DYNAMIC_TICK_DISABLE_REASON,
}
pub type PETW_CLOCK_CONFIGURATION_EVENT = *mut ETW_CLOCK_CONFIGURATION_EVENT;

#[repr(C)]
pub struct ETW_CLOCK_TIME_UPDATE {
    pub InterruptTime: ULONG64,
    pub ClockOwner: ULONG,
}
pub type PETW_CLOCK_TIME_UPDATE = *mut ETW_CLOCK_TIME_UPDATE;

#[repr(C)]
pub struct ETW_CLOCK_STATE_CHANGE_EVENT {
    pub NewState: UCHAR,
    pub PrevState: UCHAR,
    pub Reserved: [UCHAR; 6],
    pub Anonymous: ETW_CLOCK_STATE_CHANGE_EVENT_ANON,
}
pub type PETW_CLOCK_STATE_CHANGE_EVENT = *mut ETW_CLOCK_STATE_CHANGE_EVENT;

#[repr(C)]
pub union ETW_CLOCK_STATE_CHANGE_EVENT_ANON {
    pub DeliveredIncrement: ULONG64,
    pub RequestedIncrement: ULONG64,
    pub NextClockUpdateTime: ULONG64,
}

#[repr(C)]
pub struct ETW_PER_SESSION_QUOTA {
    pub SessionId: ULONG,
    pub CpuShareWeight: ULONG,
    pub CapturedWeightData: LONGLONG,
    pub CyclesAccumulated: ULONG64,
}
pub type PETW_PER_SESSION_QUOTA = *mut ETW_PER_SESSION_QUOTA;

#[repr(C)]
pub struct ETW_DFSS_START_NEW_INTERVAL {
    pub CurrentGeneration: ULONG,
    pub SessionCount: ULONG,
    pub TotalCycleCredit: ULONG64,
    pub TotalCyclesAccumulated: ULONG64,
    pub SessionQuota: [ETW_PER_SESSION_QUOTA; 1],
}
pub type PETW_DFSS_START_NEW_INTERVAL = *mut ETW_DFSS_START_NEW_INTERVAL;

#[repr(C)]
pub struct ETW_DFSS_RELEASE_THREAD_ON_IDLE {
    pub CurrentGeneration: ULONG,
    pub SessionSelectedToRun: ULONG,
    pub CycleBaseAllowance: ULONG64,
    pub CyclesRemaining: LONGLONG,
}
pub type PETW_DFSS_RELEASE_THREAD_ON_IDLE = *mut ETW_DFSS_RELEASE_THREAD_ON_IDLE;

#[repr(C)]
pub struct ETW_CPU_CACHE_FLUSH_EVENT {
    pub Address: PVOID,
    pub Bytes: SIZE_T,
    pub Clean: BOOLEAN,
    pub FullFlush: BOOLEAN,
    pub Rectangle: BOOLEAN,
    pub Reserved0: BOOLEAN,
    pub Reserved1: ULONG,
}
pub type PETW_CPU_CACHE_FLUSH_EVENT = *mut ETW_CPU_CACHE_FLUSH_EVENT;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ETWTRACECONTROLCODE {
    EtwStartLoggerCode = 1,
    EtwStopLoggerCode = 2,
    EtwQueryLoggerCode = 3,
    EtwUpdateLoggerCode = 4,
    EtwFlushLoggerCode = 5,
    EtwIncrementLoggerFile = 6,
    EtwRealtimeTransition = 7,
    EtwRealtimeConnectCode = 11,
    EtwActivityIdCreate = 12,
    EtwWdiScenarioCode = 13,
    EtwRealtimeDisconnectCode = 14,
    EtwRegisterGuidsCode = 15,
    EtwReceiveNotification = 16,
    EtwSendDataBlock = 17,
    EtwSendReplyDataBlock = 18,
    EtwReceiveReplyDataBlock = 19,
    EtwWdiSemUpdate = 20,
    EtwEnumTraceGuidList = 21,
    EtwGetTraceGuidInfo = 22,
    EtwEnumerateTraceGuids = 23,
    EtwRegisterSecurityProv = 24,
    EtwReferenceTimeCode = 25,
    EtwTrackBinaryCode = 26,
    EtwAddNotificationEvent = 27,
    EtwUpdateDisallowList = 28,
    EtwSetEnableAllKeywordsCode = 29,
    EtwSetProviderTraitsCode = 30,
    EtwUseDescriptorTypeCode = 31,
    EtwEnumTraceGroupList = 32,
    EtwGetTraceGroupInfo = 33,
    EtwGetDisallowList = 34,
    EtwSetCompressionSettings = 35,
    EtwGetCompressionSettings = 36,
    EtwUpdatePeriodicCaptureState = 37,
    EtwGetPrivateSessionTraceHandle = 38,
    EtwRegisterPrivateSession = 39,
    EtwQuerySessionDemuxObject = 40,
    EtwSetProviderBinaryTracking = 41,
    EtwMaxLoggers = 42,
    EtwMaxPmcCounter = 43,
    EtwQueryUsedProcessorCount = 44,
    EtwGetPmcOwnership = 45,
    EtwGetPmcSessions = 46,
    EtwTraceControlMax = 47,
}