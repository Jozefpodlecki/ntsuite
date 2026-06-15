use crate::ntdef::*;

#[repr(C)]
pub struct EVENT_TRACE_HEADER {
    pub Size: USHORT,
    pub DUMMYUNIONNAME: EVENT_TRACE_HEADER_UNION1,
    pub DUMMYUNIONNAME2: EVENT_TRACE_HEADER_UNION2,
    pub ThreadId: ULONG,
    pub ProcessId: ULONG,
    pub TimeStamp: LARGE_INTEGER,
    pub DUMMYUNIONNAME3: EVENT_TRACE_HEADER_UNION3,
    pub DUMMYUNIONNAME4: EVENT_TRACE_HEADER_UNION4,
}
pub type PEVENT_TRACE_HEADER = *mut EVENT_TRACE_HEADER;

#[repr(C)]
pub union EVENT_TRACE_HEADER_UNION1 {
    pub FieldTypeFlags: USHORT,
    pub DUMMYSTRUCTNAME: EVENT_TRACE_HEADER_STRUCT1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER_STRUCT1 {
    pub HeaderType: UCHAR,
    pub MarkerFlags: UCHAR,
}

#[repr(C)]
pub union EVENT_TRACE_HEADER_UNION2 {
    pub Version: ULONG,
    pub Class: EVENT_TRACE_HEADER_CLASS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER_CLASS {
    pub Type: UCHAR,
    pub Level: UCHAR,
    pub Version: USHORT,
}

#[repr(C)]
pub union EVENT_TRACE_HEADER_UNION3 {
    pub Guid: GUID,
    pub GuidPtr: ULONGLONG,
}

#[repr(C)]
pub union EVENT_TRACE_HEADER_UNION4 {
    pub DUMMYSTRUCTNAME: EVENT_TRACE_HEADER_STRUCT2,
    pub ProcessorTime: ULONG64,
    pub DUMMYSTRUCTNAME2: EVENT_TRACE_HEADER_STRUCT3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER_STRUCT2 {
    pub KernelTime: ULONG,
    pub UserTime: ULONG,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER_STRUCT3 {
    pub ClientContext: ULONG,
    pub Flags: ULONG,
}

#[repr(C)]
pub struct EVENT_INSTANCE_HEADER {
    pub Size: USHORT,
    pub DUMMYUNIONNAME: EVENT_INSTANCE_HEADER_UNION1,
    pub DUMMYUNIONNAME2: EVENT_INSTANCE_HEADER_UNION2,
    pub ThreadId: ULONG,
    pub ProcessId: ULONG,
    pub TimeStamp: LARGE_INTEGER,
    pub RegHandle: ULONGLONG,
    pub InstanceId: ULONG,
    pub ParentInstanceId: ULONG,
    pub DUMMYUNIONNAME3: EVENT_INSTANCE_HEADER_UNION3,
    pub ParentRegHandle: ULONGLONG,
}
pub type PEVENT_INSTANCE_HEADER = *mut EVENT_INSTANCE_HEADER;

#[repr(C)]
pub union EVENT_INSTANCE_HEADER_UNION1 {
    pub FieldTypeFlags: USHORT,
    pub DUMMYSTRUCTNAME: EVENT_INSTANCE_HEADER_STRUCT1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_INSTANCE_HEADER_STRUCT1 {
    pub HeaderType: UCHAR,
    pub MarkerFlags: UCHAR,
}

#[repr(C)]
pub union EVENT_INSTANCE_HEADER_UNION2 {
    pub Version: ULONG,
    pub Class: EVENT_INSTANCE_HEADER_CLASS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_INSTANCE_HEADER_CLASS {
    pub Type: UCHAR,
    pub Level: UCHAR,
    pub Version: USHORT,
}

#[repr(C)]
pub union EVENT_INSTANCE_HEADER_UNION3 {
    pub DUMMYSTRUCTNAME: EVENT_INSTANCE_HEADER_STRUCT2,
    pub ProcessorTime: ULONG64,
    pub DUMMYSTRUCTNAME2: EVENT_INSTANCE_HEADER_STRUCT3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_INSTANCE_HEADER_STRUCT2 {
    pub KernelTime: ULONG,
    pub UserTime: ULONG,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_INSTANCE_HEADER_STRUCT3 {
    pub EventId: ULONG,
    pub Flags: ULONG,
}