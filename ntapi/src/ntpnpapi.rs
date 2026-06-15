use crate::ntdef::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PLUGPLAY_EVENT_CATEGORY {
    HardwareProfileChangeEvent = 0,
    TargetDeviceChangeEvent = 1,
    DeviceClassChangeEvent = 2,
    CustomDeviceEvent = 3,
    DeviceInstallEvent = 4,
    DeviceArrivalEvent = 5,
    PowerEvent = 6,
    VetoEvent = 7,
    BlockedDriverEvent = 8,
    InvalidIDEvent = 9,
    MaxPlugEventCategory = 10,
}
pub type PPLUGPLAY_EVENT_CATEGORY = *mut PLUGPLAY_EVENT_CATEGORY;

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK {
    pub EventGuid: GUID,
    pub EventCategory: PLUGPLAY_EVENT_CATEGORY,
    pub Result: PULONG,
    pub Flags: ULONG,
    pub TotalSize: ULONG,
    pub DeviceObject: PVOID,
    pub u: PLUGPLAY_EVENT_BLOCK_U,
}
pub type PPLUGPLAY_EVENT_BLOCK = *mut PLUGPLAY_EVENT_BLOCK;

#[repr(C)]
pub union PLUGPLAY_EVENT_BLOCK_U {
    pub DeviceClass: PLUGPLAY_EVENT_BLOCK_DEVICE_CLASS,
    pub TargetDevice: PLUGPLAY_EVENT_BLOCK_TARGET_DEVICE,
    pub InstallDevice: PLUGPLAY_EVENT_BLOCK_INSTALL_DEVICE,
    pub CustomNotification: PLUGPLAY_EVENT_BLOCK_CUSTOM_NOTIFICATION,
    pub ProfileNotification: PLUGPLAY_EVENT_BLOCK_PROFILE_NOTIFICATION,
    pub PowerNotification: PLUGPLAY_EVENT_BLOCK_POWER_NOTIFICATION,
    pub VetoNotification: PLUGPLAY_EVENT_BLOCK_VETO_NOTIFICATION,
    pub BlockedDriverNotification: PLUGPLAY_EVENT_BLOCK_BLOCKED_DRIVER_NOTIFICATION,
    pub InvalidIDNotification: PLUGPLAY_EVENT_BLOCK_INVALID_ID_NOTIFICATION,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_DEVICE_CLASS {
    pub ClassGuid: GUID,
    pub SymbolicLinkName: [u16; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_TARGET_DEVICE {
    pub DeviceIds: [u16; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_INSTALL_DEVICE {
    pub DeviceId: [u16; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_CUSTOM_NOTIFICATION {
    pub NotificationStructure: PVOID,
    pub DeviceIds: [u16; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_PROFILE_NOTIFICATION {
    pub Notification: PVOID,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_POWER_NOTIFICATION {
    pub NotificationCode: ULONG,
    pub NotificationData: ULONG,
}

pub type PNP_VETO_TYPE = i32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_VETO_NOTIFICATION {
    pub VetoType: PNP_VETO_TYPE,
    pub DeviceIdVetoNameBuffer: [u16; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_BLOCKED_DRIVER_NOTIFICATION {
    pub BlockedDriverGuid: GUID,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_EVENT_BLOCK_INVALID_ID_NOTIFICATION {
    pub ParentId: [u16; 1],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PLUGPLAY_CONTROL_CLASS {
    PlugPlayControlEnumerateDevice = 0,
    PlugPlayControlRegisterNewDevice = 1,
    PlugPlayControlDeregisterDevice = 2,
    PlugPlayControlInitializeDevice = 3,
    PlugPlayControlStartDevice = 4,
    PlugPlayControlUnlockDevice = 5,
    PlugPlayControlQueryAndRemoveDevice = 6,
    PlugPlayControlUserResponse = 7,
    PlugPlayControlGenerateLegacyDevice = 8,
    PlugPlayControlGetInterfaceDeviceList = 9,
    PlugPlayControlProperty = 10,
    PlugPlayControlDeviceClassAssociation = 11,
    PlugPlayControlGetRelatedDevice = 12,
    PlugPlayControlGetInterfaceDeviceAlias = 13,
    PlugPlayControlDeviceStatus = 14,
    PlugPlayControlGetDeviceDepth = 15,
    PlugPlayControlQueryDeviceRelations = 16,
    PlugPlayControlTargetDeviceRelation = 17,
    PlugPlayControlQueryConflictList = 18,
    PlugPlayControlRetrieveDock = 19,
    PlugPlayControlResetDevice = 20,
    PlugPlayControlHaltDevice = 21,
    PlugPlayControlGetBlockedDriverList = 22,
    PlugPlayControlGetDeviceInterfaceEnabled = 23,
    MaxPlugPlayControl = 24,
}
pub type PPLUGPLAY_CONTROL_CLASS = *mut PLUGPLAY_CONTROL_CLASS;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DEVICE_RELATION_TYPE {
    BusRelations = 0,
    EjectionRelations = 1,
    PowerRelations = 2,
    RemovalRelations = 3,
    TargetDeviceRelation = 4,
    SingleBusRelations = 5,
    TransportRelations = 6,
}
pub type PDEVICE_RELATION_TYPE = *mut DEVICE_RELATION_TYPE;