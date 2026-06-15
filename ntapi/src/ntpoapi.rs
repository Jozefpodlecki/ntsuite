use crate::ntdef::*;

#[repr(C)]
pub struct COUNTED_REASON_CONTEXT {
    pub Version: ULONG,
    pub Flags: ULONG,
    pub ResourceFileName: UNICODE_STRING,
    pub ResourceReasonId: USHORT,
    pub StringCount: ULONG,
    pub ReasonStrings: PUNICODE_STRING,
}

pub type PCOUNTED_REASON_CONTEXT = *mut COUNTED_REASON_CONTEXT;

pub type POWER_INFORMATION_LEVEL = ULONG;

pub const SystemPowerPolicyAc: ULONG = 0;
pub const SystemPowerPolicyDc: ULONG = 1;
pub const VerifySystemPolicyAc: ULONG = 2;
pub const VerifySystemPolicyDc: ULONG = 3;
pub const SystemPowerCapabilities: ULONG = 4;
pub const SystemBatteryState: ULONG = 5;
pub const SystemPowerStateHandler: ULONG = 6;
pub const ProcessorStateHandler: ULONG = 7;
pub const SystemPowerPolicyCurrent: ULONG = 8;
pub const AdministratorPowerPolicy: ULONG = 9;
pub const SystemReserveHiberFile: ULONG = 10;
pub const ProcessorInformation: ULONG = 11;
pub const SystemPowerInformation: ULONG = 12;
pub const ProcessorStateHandler2: ULONG = 13;
pub const LastWakeTime: ULONG = 14;
pub const LastSleepTime: ULONG = 15;
pub const SystemExecutionState: ULONG = 16;
pub const SystemPowerStateNotifyHandler: ULONG = 17;
pub const ProcessorPowerPolicyAc: ULONG = 18;
pub const ProcessorPowerPolicyDc: ULONG = 19;
pub const VerifyProcessorPowerPolicyAc: ULONG = 20;
pub const VerifyProcessorPowerPolicyDc: ULONG = 21;
pub const ProcessorPowerPolicyCurrent: ULONG = 22;
pub const SystemPowerStateLogging: ULONG = 23;
pub const SystemPowerLoggingEntry: ULONG = 24;
pub const SetPowerSettingValue: ULONG = 25;
pub const NotifyUserPowerSetting: ULONG = 26;
pub const PowerInformationLevelUnused0: ULONG = 27;
pub const SystemMonitorHiberBootPowerOff: ULONG = 28;
pub const SystemVideoState: ULONG = 29;
pub const TraceApplicationPowerMessage: ULONG = 30;
pub const TraceApplicationPowerMessageEnd: ULONG = 31;
pub const ProcessorPerfStates: ULONG = 32;
pub const ProcessorIdleStates: ULONG = 33;
pub const ProcessorCap: ULONG = 34;
pub const SystemWakeSource: ULONG = 35;
pub const SystemHiberFileInformation: ULONG = 36;
pub const TraceServicePowerMessage: ULONG = 37;
pub const ProcessorLoad: ULONG = 38;
pub const PowerShutdownNotification: ULONG = 39;
pub const MonitorCapabilities: ULONG = 40;
pub const SessionPowerInit: ULONG = 41;
pub const SessionDisplayState: ULONG = 42;
pub const PowerRequestCreate: ULONG = 43;
pub const PowerRequestAction: ULONG = 44;
pub const GetPowerRequestList: ULONG = 45;
pub const ProcessorInformationEx: ULONG = 46;
pub const NotifyUserModeLegacyPowerEvent: ULONG = 47;
pub const GroupPark: ULONG = 48;
pub const ProcessorIdleDomains: ULONG = 49;
pub const WakeTimerList: ULONG = 50;
pub const SystemHiberFileSize: ULONG = 51;
pub const ProcessorIdleStatesHv: ULONG = 52;
pub const ProcessorPerfStatesHv: ULONG = 53;
pub const ProcessorPerfCapHv: ULONG = 54;
pub const ProcessorSetIdle: ULONG = 55;
pub const LogicalProcessorIdling: ULONG = 56;
pub const UserPresence: ULONG = 57;
pub const PowerSettingNotificationName: ULONG = 58;
pub const GetPowerSettingValue: ULONG = 59;
pub const IdleResiliency: ULONG = 60;
pub const SessionRITState: ULONG = 61;
pub const SessionConnectNotification: ULONG = 62;
pub const SessionPowerCleanup: ULONG = 63;
pub const SessionLockState: ULONG = 64;
pub const SystemHiberbootState: ULONG = 65;
pub const PlatformInformation: ULONG = 66;
pub const PdcInvocation: ULONG = 67;
pub const MonitorInvocation: ULONG = 68;
pub const FirmwareTableInformationRegistered: ULONG = 69;
pub const SetShutdownSelectedTime: ULONG = 70;
pub const SuspendResumeInvocation: ULONG = 71;
pub const PlmPowerRequestCreate: ULONG = 72;
pub const ScreenOff: ULONG = 73;
pub const CsDeviceNotification: ULONG = 74;
pub const PlatformRole: ULONG = 75;
pub const LastResumePerformance: ULONG = 76;
pub const DisplayBurst: ULONG = 77;
pub const ExitLatencySamplingPercentage: ULONG = 78;
pub const RegisterSpmPowerSettings: ULONG = 79;
pub const PlatformIdleStates: ULONG = 80;
pub const ProcessorIdleVeto: ULONG = 81;
pub const PlatformIdleVeto: ULONG = 82;
pub const SystemBatteryStatePrecise: ULONG = 83;
pub const ThermalEvent: ULONG = 84;
pub const PowerRequestActionInternal: ULONG = 85;
pub const BatteryDeviceState: ULONG = 86;
pub const PowerInformationInternal: ULONG = 87;
pub const ThermalStandby: ULONG = 88;
pub const SystemHiberFileType: ULONG = 89;
pub const PhysicalPowerButtonPress: ULONG = 90;
pub const QueryPotentialDripsConstraint: ULONG = 91;
pub const EnergyTrackerCreate: ULONG = 92;
pub const EnergyTrackerQuery: ULONG = 93;
pub const UpdateBlackBoxRecorder: ULONG = 94;
pub const SessionAllowExternalDmaDevices: ULONG = 95;
pub const SendSuspendResumeNotification: ULONG = 96;
pub const BlackBoxRecorderDirectAccessBuffer: ULONG = 97;
pub const SystemPowerSourceState: ULONG = 98;
pub const PowerInformationLevelMaximum: ULONG = 99;

#[repr(C)]
pub struct PROCESSOR_POWER_INFORMATION {
    pub Number: ULONG,
    pub MaxMhz: ULONG,
    pub CurrentMhz: ULONG,
    pub MhzLimit: ULONG,
    pub MaxIdleState: ULONG,
    pub CurrentIdleState: ULONG,
}
pub type PPROCESSOR_POWER_INFORMATION = *mut PROCESSOR_POWER_INFORMATION;

pub const PO_TZ_ACTIVE: u8 = 0;
pub const PO_TZ_PASSIVE: u8 = 1;
pub const PO_TZ_INVALID_MODE: u8 = 2;

#[repr(C)]
pub struct SYSTEM_POWER_INFORMATION {
    pub MaxIdlenessAllowed: ULONG,
    pub Idleness: ULONG,
    pub TimeRemaining: ULONG,
    pub CoolingMode: UCHAR,
}
pub type PSYSTEM_POWER_INFORMATION = *mut SYSTEM_POWER_INFORMATION;

#[repr(C)]
pub struct SYSTEM_HIBERFILE_INFORMATION {
    pub NumberOfMcbPairs: ULONG,
    pub Mcb: [LARGE_INTEGER; 1],
}
pub type PSYSTEM_HIBERFILE_INFORMATION = *mut SYSTEM_HIBERFILE_INFORMATION;

#[repr(C)]
pub struct SYSTEM_SERVICE_POWER_MESSAGE {
    pub MessageId: ULONG,
    pub SessionId: ULONG,
    pub Flags: ULONG,
}
pub type PSYSTEM_SERVICE_POWER_MESSAGE = *mut SYSTEM_SERVICE_POWER_MESSAGE;