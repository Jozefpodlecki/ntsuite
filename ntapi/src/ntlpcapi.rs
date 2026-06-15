use crate::{ntdef::*, ntrtl::CLIENT_ID};

pub type ALPC_HANDLE = HANDLE;
pub type PALPC_HANDLE = *mut ALPC_HANDLE;

#[repr(C)]
pub struct PORT_MESSAGE {
    pub u1: PORT_MESSAGE_U1,
    pub u2: PORT_MESSAGE_U2,
    pub ClientId: CLIENT_ID,
    pub MessageId: ULONG,
    pub ClientViewSize: SIZE_T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union PORT_MESSAGE_U1 {
    pub s1: PORT_MESSAGE_S1,
    pub Length: ULONG,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_MESSAGE_S1 {
    pub DataLength: CSHORT,
    pub TotalLength: CSHORT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union PORT_MESSAGE_U2 {
    pub s2: PORT_MESSAGE_S2,
    pub ZeroInit: ULONG,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PORT_MESSAGE_S2 {
    pub Type: CSHORT,
    pub DataInfoOffset: CSHORT,
}

pub type PPORT_MESSAGE = *mut PORT_MESSAGE;

#[repr(C)]
pub struct LPC_CLIENT_DIED_MSG {
    pub PortMsg: PORT_MESSAGE,
    pub CreateTime: LARGE_INTEGER,
}
pub type PLPC_CLIENT_DIED_MSG = *mut LPC_CLIENT_DIED_MSG;

#[repr(C)]
pub struct PORT_VIEW {
    pub Length: ULONG,
    pub SectionHandle: HANDLE,
    pub SectionOffset: ULONG,
    pub ViewSize: SIZE_T,
    pub ViewBase: PVOID,
    pub ViewRemoteBase: PVOID,
}
pub type PPORT_VIEW = *mut PORT_VIEW;

#[repr(C)]
pub struct REMOTE_PORT_VIEW {
    pub Length: ULONG,
    pub ViewSize: SIZE_T,
    pub ViewBase: PVOID,
}
pub type PREMOTE_PORT_VIEW = *mut REMOTE_PORT_VIEW;

#[repr(C)]
pub struct PORT_MESSAGE64 {
    pub u1: PORT_MESSAGE64_U1,
    pub u2: PORT_MESSAGE64_U2,
    pub ClientId: CLIENT_ID64,
    pub MessageId: ULONG,
    pub Anonymous: PORT_MESSAGE64_ANON,
}

#[repr(C)]
pub union PORT_MESSAGE64_U1 {
    pub s1: PORT_MESSAGE64_S1,
    pub Length: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PORT_MESSAGE64_S1 {
    pub DataLength: CSHORT,
    pub TotalLength: CSHORT,
}

#[repr(C)]
pub union PORT_MESSAGE64_U2 {
    pub s2: PORT_MESSAGE64_S2,
    pub ZeroInit: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PORT_MESSAGE64_S2 {
    pub Type: CSHORT,
    pub DataInfoOffset: CSHORT,
}

#[repr(C)]
pub union PORT_MESSAGE64_ANON {
    pub ClientViewSize: ULONGLONG,
    pub CallbackId: ULONG,
}

#[repr(C)]
pub struct CLIENT_ID64 {
    pub UniqueProcess: ULONGLONG,
    pub UniqueThread: ULONGLONG,
}

#[repr(C)]
pub struct LPC_CLIENT_DIED_MSG64 {
    pub PortMsg: PORT_MESSAGE64,
    pub CreateTime: LARGE_INTEGER,
}
pub type PLPC_CLIENT_DIED_MSG64 = *mut LPC_CLIENT_DIED_MSG64;

#[repr(C)]
pub struct PORT_VIEW64 {
    pub Length: ULONG,
    pub SectionHandle: ULONGLONG,
    pub SectionOffset: ULONG,
    pub ViewSize: ULONGLONG,
    pub ViewBase: ULONGLONG,
    pub ViewRemoteBase: ULONGLONG,
}
pub type PPORT_VIEW64 = *mut PORT_VIEW64;

#[repr(C)]
pub struct REMOTE_PORT_VIEW64 {
    pub Length: ULONG,
    pub ViewSize: ULONGLONG,
    pub ViewBase: ULONGLONG,
}

pub type PREMOTE_PORT_VIEW64 = *mut REMOTE_PORT_VIEW64;

#[repr(C)]
pub struct ALPC_PORT_ATTRIBUTES {
    pub Flags: ULONG,
    pub SecurityQos: SECURITY_QUALITY_OF_SERVICE,
    pub MaxMessageLength: SIZE_T,
    pub MemoryBandwidth: SIZE_T,
    pub MaxPoolUsage: SIZE_T,
    pub MaxSectionSize: SIZE_T,
    pub MaxViewSize: SIZE_T,
    pub MaxTotalSectionSize: SIZE_T,
    pub DupObjectTypes: ULONG,
    #[cfg(target_pointer_width = "64")]
    pub Reserved: ULONG,
}

pub type PALPC_PORT_ATTRIBUTES = *mut ALPC_PORT_ATTRIBUTES;

#[repr(C)]
pub struct ALPC_MESSAGE_ATTRIBUTES {
    pub AllocatedAttributes: ULONG,
    pub ValidAttributes: ULONG,
}
pub type PALPC_MESSAGE_ATTRIBUTES = *mut ALPC_MESSAGE_ATTRIBUTES;

#[repr(C)]
pub struct ALPC_PORT_MESSAGE_ZONE_INFORMATION {
    pub Buffer: PVOID,
    pub Size: ULONG,
}
pub type PALPC_PORT_MESSAGE_ZONE_INFORMATION = *mut ALPC_PORT_MESSAGE_ZONE_INFORMATION;

#[repr(C)]
pub struct ALPC_PORT_COMPLETION_LIST_INFORMATION {
    pub Buffer: PVOID,
    pub Size: ULONG,
    pub ConcurrencyCount: ULONG,
    pub AttributeFlags: ULONG,
}
pub type PALPC_PORT_COMPLETION_LIST_INFORMATION = *mut ALPC_PORT_COMPLETION_LIST_INFORMATION;

#[repr(C)]
pub struct ALPC_REGISTER_CALLBACK {
    pub CallbackObject: PVOID,
    pub CallbackContext: PVOID,
}
pub type PALPC_REGISTER_CALLBACK = *mut ALPC_REGISTER_CALLBACK;

#[repr(C)]
pub struct ALPC_SERVER_SESSION_INFORMATION {
    pub SessionId: ULONG,
    pub ProcessId: ULONG,
}
pub type PALPC_SERVER_SESSION_INFORMATION = *mut ALPC_SERVER_SESSION_INFORMATION;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ALPC_MESSAGE_INFORMATION_CLASS {
    AlpcMessageSidInformation = 0,
    AlpcMessageTokenModifiedIdInformation = 1,
    AlpcMessageDirectStatusInformation = 2,
    AlpcMessageHandleInformation = 3,
    MaxAlpcMessageInfoClass = 4,
}
pub type PALPC_MESSAGE_INFORMATION_CLASS = *mut ALPC_MESSAGE_INFORMATION_CLASS;

#[repr(C, align(128))]
pub struct ALPC_COMPLETION_LIST_HEADER {
    pub StartMagic: ULONG64,
    pub TotalSize: ULONG,
    pub ListOffset: ULONG,
    pub ListSize: ULONG,
    pub BitmapOffset: ULONG,
    pub BitmapSize: ULONG,
    pub DataOffset: ULONG,
    pub DataSize: ULONG,
    pub AttributeFlags: ULONG,
    pub AttributeSize: ULONG,
    pub State: ALPC_COMPLETION_LIST_STATE,
    pub LastMessageId: ULONG,
    pub LastCallbackId: ULONG,
    pub PostCount: ULONG,
    pub ReturnCount: ULONG,
    pub LogSequenceNumber: ULONG,
    pub UserLock: RTL_SRWLOCK,
    pub EndMagic: ULONG64,
}
pub type PALPC_COMPLETION_LIST_HEADER = *mut ALPC_COMPLETION_LIST_HEADER;

#[repr(C)]
pub struct ALPC_CONTEXT_ATTR {
    pub PortContext: PVOID,
    pub MessageContext: PVOID,
    pub Sequence: ULONG,
    pub MessageId: ULONG,
    pub CallbackId: ULONG,
}
pub type PALPC_CONTEXT_ATTR = *mut ALPC_CONTEXT_ATTR;

pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ACCESS: u32 = 0x10000;
pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ATTRIBUTES: u32 = 0x20000;
pub const ALPC_HANDLEFLG_DUPLICATE_INHERIT: u32 = 0x80000;

#[repr(C)]
pub struct ALPC_HANDLE_ATTR32 {
    pub Flags: ULONG,
    pub Reserved0: ULONG,
    pub SameAccess: ULONG,
    pub SameAttributes: ULONG,
    pub Indirect: ULONG,
    pub Inherit: ULONG,
    pub Reserved1: ULONG,
    pub Handle: ULONG,
    pub ObjectType: ULONG,
    pub DesiredAccess: ACCESS_MASK,
    pub GrantedAccess: ACCESS_MASK,
}
pub type PALPC_HANDLE_ATTR32 = *mut ALPC_HANDLE_ATTR32;

#[repr(C)]
pub struct ALPC_HANDLE_ATTR {
    pub Flags: ULONG,
    pub Reserved0: ULONG,
    pub SameAccess: ULONG,
    pub SameAttributes: ULONG,
    pub Indirect: ULONG,
    pub Inherit: ULONG,
    pub Reserved1: ULONG,
    pub Handle: HANDLE,
    pub HandleAttrArray: PALPC_HANDLE_ATTR32,
    pub ObjectType: ULONG,
    pub HandleCount: ULONG,
    pub DesiredAccess: ACCESS_MASK,
    pub GrantedAccess: ACCESS_MASK,
}
pub type PALPC_HANDLE_ATTR = *mut ALPC_HANDLE_ATTR;

pub const ALPC_SECFLG_CREATE_HANDLE: u32 = 0x20000;
pub const ALPC_SECFLG_NOSECTIONHANDLE: u32 = 0x40000;

#[repr(C)]
pub struct ALPC_SECURITY_ATTR {
    pub Flags: ULONG,
    pub QoS: PSECURITY_QUALITY_OF_SERVICE,
    pub ContextHandle: ALPC_HANDLE,
}
pub type PALPC_SECURITY_ATTR = *mut ALPC_SECURITY_ATTR;

pub const ALPC_VIEWFLG_UNMAP_EXISTING: u32 = 0x10000;
pub const ALPC_VIEWFLG_AUTO_RELEASE: u32 = 0x20000;
pub const ALPC_VIEWFLG_NOT_SECURE: u32 = 0x40000;

#[repr(C)]
pub struct ALPC_DATA_VIEW_ATTR {
    pub Flags: ULONG,
    pub SectionHandle: ALPC_HANDLE,
    pub ViewBase: PVOID,
    pub ViewSize: SIZE_T,
}
pub type PALPC_DATA_VIEW_ATTR = *mut ALPC_DATA_VIEW_ATTR;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ALPC_PORT_INFORMATION_CLASS {
    AlpcBasicInformation = 0,
    AlpcPortInformation = 1,
    AlpcAssociateCompletionPortInformation = 2,
    AlpcConnectedSIDInformation = 3,
    AlpcServerInformation = 4,
    AlpcMessageZoneInformation = 5,
    AlpcRegisterCompletionListInformation = 6,
    AlpcUnregisterCompletionListInformation = 7,
    AlpcAdjustCompletionListConcurrencyCountInformation = 8,
    AlpcRegisterCallbackInformation = 9,
    AlpcCompletionListRundownInformation = 10,
    AlpcWaitForPortReferences = 11,
    AlpcServerSessionInformation = 12,
}

#[repr(C)]
pub struct ALPC_COMPLETION_LIST_STATE {
    pub u1: ALPC_COMPLETION_LIST_STATE_U1,
}
pub type PALPC_COMPLETION_LIST_STATE = *mut ALPC_COMPLETION_LIST_STATE;

#[repr(C)]
pub union ALPC_COMPLETION_LIST_STATE_U1 {
    pub s1: ALPC_COMPLETION_LIST_STATE_S1,
    pub Value: ULONG64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALPC_COMPLETION_LIST_STATE_S1 {
    pub Head: u64,
    pub Tail: u64,
    pub ActiveThreadCount: u64,
}

pub const ALPC_COMPLETION_LIST_BUFFER_GRANULARITY_MASK: u64 = 0x3f;