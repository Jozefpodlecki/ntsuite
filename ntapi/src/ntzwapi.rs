use crate::{ntdbg::*, ntdef::*, ntexapi::*, ntioapi::*, ntkeapi::*, ntlpcapi::*, ntmisc::*, ntmmapi::*, ntobapi::*, ntpebteb::*, ntpnpapi::*, ntpoapi::*, ntpsapi::*, ntregapi::*, ntrtl::*, ntseapi::*, nttmapi::*, ntwmi::*};

unsafe extern "system" {
    pub fn ZwAcceptConnectPort(
        PortHandle: PHANDLE,
        PortContext: PVOID,
        ConnectionRequest: PPORT_MESSAGE,
        AcceptConnection: BOOLEAN,
        ServerView: PPORT_VIEW,
        ClientView: PREMOTE_PORT_VIEW,
    ) -> NTSTATUS;

    pub fn ZwAccessCheck(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: PULONG,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwAccessCheckAndAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PCUNICODE_STRING,
        ObjectName: PCUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DesiredAccess: ACCESS_MASK,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwAccessCheckByType(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: PULONG,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwAccessCheckByTypeAndAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PCUNICODE_STRING,
        ObjectName: PCUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: ULONG,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwAccessCheckByTypeResultList(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: PULONG,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwAccessCheckByTypeResultListAndAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PCUNICODE_STRING,
        ObjectName: PCUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: ULONG,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        ClientToken: HANDLE,
        ObjectTypeName: PCUNICODE_STRING,
        ObjectName: PCUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: ULONG,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwAcquireCMFViewOwnership(
        TimeStamp: PULONGLONG,
        tokenTaken: PBOOLEAN,
        replaceExisting: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwAcquireCrossVmMutant(
        CrossVmMutant: HANDLE,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwAcquireProcessActivityReference(
        ActivityReferenceHandle: PHANDLE,
        ParentProcessHandle: HANDLE,
        ProcessActivityType: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAddAtom(
        AtomName: PCWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;

    pub fn ZwAddAtomEx(
        AtomName: PCWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAddBootEntry(
        BootEntry: PBOOT_ENTRY,
        Id: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAddDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
        Id: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAdjustGroupsToken(
        TokenHandle: HANDLE,
        ResetToDefault: BOOLEAN,
        NewState: PTOKEN_GROUPS,
        BufferLength: ULONG,
        PreviousState: PTOKEN_GROUPS,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAdjustPrivilegesToken(
        TokenHandle: HANDLE,
        DisableAllPrivileges: BOOLEAN,
        NewState: PTOKEN_PRIVILEGES,
        BufferLength: ULONG,
        PreviousState: PTOKEN_PRIVILEGES,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAdjustTokenClaimsAndDeviceGroups(
        TokenHandle: HANDLE,
        UserResetToDefault: BOOLEAN,
        DeviceResetToDefault: BOOLEAN,
        DeviceGroupsResetToDefault: BOOLEAN,
        NewUserState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceGroupsState: PTOKEN_GROUPS,
        UserBufferLength: ULONG,
        PreviousUserState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceBufferLength: ULONG,
        PreviousDeviceState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroupsBufferLength: ULONG,
        PreviousDeviceGroups: PTOKEN_GROUPS,
        UserReturnLength: PULONG,
        DeviceReturnLength: PULONG,
        DeviceGroupsReturnBufferLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAlertMultipleThreadByThreadId(
        MultipleThreadId: PHANDLE,
        Count: ULONG,
        ExtendedParameters: PPS_ALERT_THREAD_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAlertResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAlertThread(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwAlertThreadByThreadId(
        ThreadId: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwAlertThreadByThreadIdEx(
        ThreadId: HANDLE,
        Lock: PRTL_SRWLOCK,
    ) -> NTSTATUS;

    pub fn ZwAllocateLocallyUniqueId(
        Luid: PLUID,
    ) -> NTSTATUS;

    pub fn ZwAllocateReserveObject(
        MemoryReserveHandle: PHANDLE,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        Type: MEMORY_RESERVE_TYPE,
    ) -> NTSTATUS;

    pub fn ZwAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: PSIZE_T,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;

    pub fn ZwAllocateUserPhysicalPagesEx(
        ProcessHandle: HANDLE,
        NumberOfPages: PULONG_PTR,
        UserPfnArray: PULONG_PTR,
        ExtendedParameters: PMEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAllocateUuids(
        Time: PULARGE_INTEGER,
        Range: PULONG,
        Sequence: PULONG,
        Seed: PCHAR,
    ) -> NTSTATUS;

    pub fn ZwAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        RegionSize: PSIZE_T,
        AllocationType: ULONG,
        PageProtection: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAllocateVirtualMemoryEx(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        AllocationType: ULONG,
        PageProtection: ULONG,
        ExtendedParameters: PMEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAlpcAcceptConnectPort(
        PortHandle: PHANDLE,
        ConnectionPortHandle: HANDLE,
        Flags: ULONG,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        PortContext: PVOID,
        ConnectionRequest: PPORT_MESSAGE,
        ConnectionMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        AcceptConnection: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: ULONG,
        MessageContext: PALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;

    pub fn ZwAlpcConnectPort(
        PortHandle: PHANDLE,
        PortName: PCUNICODE_STRING,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        Flags: ULONG,
        RequiredServerSid: PSID,
        ConnectionMessage: PPORT_MESSAGE,
        BufferLength: PSIZE_T,
        OutMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwAlpcConnectPortEx(
        PortHandle: PHANDLE,
        ConnectionPortObjectAttributes: POBJECT_ATTRIBUTES,
        ClientPortObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        Flags: ULONG,
        ServerSecurityRequirements: PSECURITY_DESCRIPTOR,
        ConnectionMessage: PPORT_MESSAGE,
        BufferLength: PSIZE_T,
        OutMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwAlpcCreatePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwAlpcCreatePortSection(
        PortHandle: HANDLE,
        Flags: ULONG,
        SectionHandle: HANDLE,
        SectionSize: SIZE_T,
        AlpcSectionHandle: PALPC_HANDLE,
        ActualSectionSize: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: ULONG,
        MessageSize: SIZE_T,
        ResourceId: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: ULONG,
        ViewAttributes: PALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;

    pub fn ZwAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        SecurityAttribute: PALPC_SECURITY_ATTR,
    ) -> NTSTATUS;

    pub fn ZwAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: ULONG,
        SectionHandle: ALPC_HANDLE,
    ) -> NTSTATUS;

    pub fn ZwAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: ULONG,
        ResourceId: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: ULONG,
        ViewBase: PVOID,
    ) -> NTSTATUS;

    pub fn ZwAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;

    pub fn ZwAlpcDisconnectPort(
        PortHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: PVOID,
    ) -> NTSTATUS;

    pub fn ZwAlpcOpenSenderProcess(
        ProcessHandle: PHANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: ULONG,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwAlpcOpenSenderThread(
        ThreadHandle: PHANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: ULONG,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAlpcQueryInformationMessage(
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS,
        MessageInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;

    pub fn ZwAlpcSendWaitReceivePort(
        PortHandle: HANDLE,
        Flags: ULONG,
        SendMessage: PPORT_MESSAGE,
        SendMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        ReceiveMessage: PPORT_MESSAGE,
        BufferLength: PSIZE_T,
        ReceiveMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;

    pub fn ZwApphelpCacheControl(
        ServiceClass: AHC_SERVICE_CLASS,
        ServiceContext: PVOID,
    ) -> NTSTATUS;

    pub fn ZwAreMappedFilesTheSame(
        File1MappedAsAnImage: PVOID,
        File2MappedAsFile: PVOID,
    ) -> NTSTATUS;

    pub fn ZwAssignProcessToJobObject(
        JobHandle: HANDLE,
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwAssociateWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        IoCompletionHandle: HANDLE,
        TargetObjectHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
        AlreadySignaled: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCallbackReturn(
        OutputBuffer: PVOID,
        OutputLength: ULONG,
        Status: NTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwCallEnclave(
        Routine: PENCLAVE_ROUTINE,
        Reserved: PVOID,
        Flags: ULONG,
        RoutineParamReturn: *mut PVOID,
    ) -> NTSTATUS;

    pub fn ZwCancelIoFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn ZwCancelIoFileEx(
        FileHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn ZwCancelSynchronousIoFile(
        ThreadHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn ZwCancelTimer(
        TimerHandle: HANDLE,
        CurrentState: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCancelTimer2(
        TimerHandle: HANDLE,
        Parameters: PT2_CANCEL_PARAMETERS,
    ) -> NTSTATUS;

    pub fn ZwCancelWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        RemoveSignaledPacket: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwChangeProcessState(
        ProcessStateChangeHandle: HANDLE,
        ProcessHandle: HANDLE,
        StateChangeType: PROCESS_STATE_CHANGE_TYPE,
        ExtendedInformation: PVOID,
        ExtendedInformationLength: SIZE_T,
        Reserved: ULONG,
    ) -> NTSTATUS;

    pub fn ZwChangeThreadState(
        ThreadStateChangeHandle: HANDLE,
        ThreadHandle: HANDLE,
        StateChangeType: THREAD_STATE_CHANGE_TYPE,
        ExtendedInformation: PVOID,
        ExtendedInformationLength: SIZE_T,
        Reserved: ULONG,
    ) -> NTSTATUS;

    pub fn ZwClearEvent(
        EventHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwClose(
        Handle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwCloseObjectAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCommitComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwCommitEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwCommitRegistryTransaction(
        RegistryTransactionHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCommitTransaction(
        TransactionHandle: HANDLE,
        Wait: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCompactKeys(
        Count: ULONG,
        KeyArray: *const HANDLE,
    ) -> NTSTATUS;

    pub fn ZwCompareObjects(
        FirstObjectHandle: HANDLE,
        SecondObjectHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwCompareSigningLevels(
        FirstSigningLevel: SE_SIGNING_LEVEL,
        SecondSigningLevel: SE_SIGNING_LEVEL,
    ) -> NTSTATUS;

    pub fn ZwCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCompleteConnectPort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwCompressKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwConnectPort(
        PortHandle: PHANDLE,
        PortName: PCUNICODE_STRING,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: PULONG,
        ConnectionInformation: PVOID,
        ConnectionInformationLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwContinue(
        ContextRecord: PCONTEXT,
        TestAlert: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwContinueEx(
        ContextRecord: PCONTEXT,
        ContinueArgument: PVOID,
    ) -> NTSTATUS;

    pub fn ZwConvertBetweenAuxiliaryCounterAndPerformanceCounter(
        ConvertAuxiliaryToPerformanceCounter: BOOLEAN,
        PerformanceOrAuxiliaryCounterValue: PULONG64,
        ConvertedValue: PULONG64,
        ConversionError: PULONG64,
    ) -> NTSTATUS;

    pub fn ZwCopyFileChunk(
        SourceHandle: HANDLE,
        DestinationHandle: HANDLE,
        EventHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Length: ULONG,
        SourceOffset: PLARGE_INTEGER,
        DestOffset: PLARGE_INTEGER,
        SourceKey: PULONG,
        DestKey: PULONG,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateCpuPartition(
        CpuPartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwCreateCrossVmEvent(
        CrossVmEvent: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CrossVmEventFlags: ULONG,
        VMID: LPCGUID,
        ServiceID: LPCGUID,
    ) -> NTSTATUS;

    pub fn ZwCreateCrossVmMutant(
        EventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CrossVmEventFlags: ULONG,
        VMID: LPCGUID,
        ServiceID: LPCGUID,
    ) -> NTSTATUS;

    pub fn ZwCreateDebugObject(
        DebugObjectHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateDirectoryObject(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwCreateDirectoryObjectEx(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        Size: SIZE_T,
        InitialCommitment: SIZE_T,
        EnclaveType: ULONG,
        EnclaveInformation: PVOID,
        EnclaveInformationLength: ULONG,
        EnclaveError: PULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateEnlistment(
        EnlistmentHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ResourceManagerHandle: HANDLE,
        TransactionHandle: HANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CreateOptions: ULONG,
        NotificationMask: NOTIFICATION_MASK,
        EnlistmentKey: PVOID,
    ) -> NTSTATUS;

    pub fn ZwCreateEvent(
        EventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        EventType: EVENT_TYPE,
        InitialState: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCreateEventPair(
        EventPairHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwCreateFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        AllocationSize: PLARGE_INTEGER,
        FileAttributes: ULONG,
        ShareAccess: ULONG,
        CreateDisposition: ULONG,
        CreateOptions: ULONG,
        EaBuffer: PVOID,
        EaLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateIoCompletion(
        IoCompletionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        NumberOfConcurrentThreads: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateIoRing(
        IoRingHandle: PHANDLE,
        CreateParametersLength: ULONG,
        CreateParameters: PVOID,
        OutputParametersLength: ULONG,
        OutputParameters: PVOID,
    ) -> NTSTATUS;

    pub fn ZwCreateIRTimer(
        TimerHandle: PHANDLE,
        TimerId: PULONG,
        DesiredAccess: ACCESS_MASK,
    ) -> NTSTATUS;

    pub fn ZwCreateJobObject(
        JobHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwCreateJobSet(
        NumJob: ULONG,
        UserJobSet: PJOB_SET_ARRAY,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateKey(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TitleIndex: ULONG,
        Class: PCUNICODE_STRING,
        CreateOptions: ULONG,
        Disposition: PULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateKeyedEvent(
        KeyedEventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateKeyTransacted(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TitleIndex: ULONG,
        Class: PCUNICODE_STRING,
        CreateOptions: ULONG,
        TransactionHandle: HANDLE,
        Disposition: PULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateLowBoxToken(
        TokenHandle: PHANDLE,
        ExistingTokenHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PackageSid: PSID,
        CapabilityCount: ULONG,
        Capabilities: PSID_AND_ATTRIBUTES,
        HandleCount: ULONG,
        Handles: *const HANDLE,
    ) -> NTSTATUS;

    pub fn ZwCreateMailslotFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CreateOptions: ULONG,
        MailslotQuota: ULONG,
        MaximumMessageSize: ULONG,
        ReadTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwCreateMutant(
        MutantHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        InitialOwner: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCreateNamedPipeFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: ULONG,
        CreateDisposition: ULONG,
        CreateOptions: ULONG,
        NamedPipeType: ULONG,
        ReadMode: ULONG,
        CompletionMode: ULONG,
        MaximumInstances: ULONG,
        InboundQuota: ULONG,
        OutboundQuota: ULONG,
        DefaultTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwCreatePagingFile(
        PageFileName: PCUNICODE_STRING,
        MinimumSize: PLARGE_INTEGER,
        MaximumSize: PLARGE_INTEGER,
        Priority: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreatePartition(
        ParentPartitionHandle: HANDLE,
        PartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwCreatePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: ULONG,
        MaxMessageLength: ULONG,
        MaxPoolUsage: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreatePrivateNamespace(
        NamespaceHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        BoundaryDescriptor: POBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;

    pub fn ZwCreateProcess(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        InheritObjectTable: BOOLEAN,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        TokenHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwCreateProcessEx(
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

    pub fn ZwCreateProcessStateChange(
        ProcessStateChangeHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        Reserved: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateProfile(
        ProfileHandle: PHANDLE,
        Process: HANDLE,
        ProfileBase: PVOID,
        ProfileSize: SIZE_T,
        BucketSize: ULONG,
        Buffer: PULONG,
        BufferSize: ULONG,
        ProfileSource: KPROFILE_SOURCE,
        Affinity: KAFFINITY,
    ) -> NTSTATUS;

    pub fn ZwCreateProfileEx(
        ProfileHandle: PHANDLE,
        Process: HANDLE,
        ProfileBase: PVOID,
        ProfileSize: SIZE_T,
        BucketSize: ULONG,
        Buffer: PULONG,
        BufferSize: ULONG,
        ProfileSource: KPROFILE_SOURCE,
        GroupCount: USHORT,
        GroupAffinity: PGROUP_AFFINITY,
    ) -> NTSTATUS;

    pub fn ZwCreateRegistryTransaction(
        RegistryTransactionHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjAttributes: POBJECT_ATTRIBUTES,
        CreateOptions: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateResourceManager(
        ResourceManagerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        TmHandle: HANDLE,
        RmGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CreateOptions: ULONG,
        Description: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwCreateSection(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        MaximumSize: PLARGE_INTEGER,
        SectionPageProtection: ULONG,
        AllocationAttributes: ULONG,
        FileHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwCreateSectionEx(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        MaximumSize: PLARGE_INTEGER,
        SectionPageProtection: ULONG,
        AllocationAttributes: ULONG,
        FileHandle: HANDLE,
        ExtendedParameters: PMEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateSemaphore(
        SemaphoreHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        InitialCount: LONG,
        MaximumCount: LONG,
    ) -> NTSTATUS;

    pub fn ZwCreateSymbolicLinkObject(
        LinkHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LinkTarget: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwCreateThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ThreadContext: PCONTEXT,
        InitialTeb: PINITIAL_TEB,
        CreateSuspended: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwCreateThreadEx(
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

    pub fn ZwCreateThreadStateChange(
        ThreadStateChangeHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ThreadHandle: HANDLE,
        Reserved: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        TimerType: TIMER_TYPE,
    ) -> NTSTATUS;

    pub fn ZwCreateTimer2(
        TimerHandle: PHANDLE,
        TimerId: PULONG,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        Attributes: ULONG,
        DesiredAccess: ACCESS_MASK,
    ) -> NTSTATUS;

    pub fn ZwCreateToken(
        TokenHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Type: TOKEN_TYPE,
        AuthenticationId: PLUID,
        ExpirationTime: PLARGE_INTEGER,
        User: PTOKEN_USER,
        Groups: PTOKEN_GROUPS,
        Privileges: PTOKEN_PRIVILEGES,
        Owner: PTOKEN_OWNER,
        PrimaryGroup: PTOKEN_PRIMARY_GROUP,
        DefaultDacl: PTOKEN_DEFAULT_DACL,
        Source: PTOKEN_SOURCE,
    ) -> NTSTATUS;

    pub fn ZwCreateTokenEx(
        TokenHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Type: TOKEN_TYPE,
        AuthenticationId: PLUID,
        ExpirationTime: PLARGE_INTEGER,
        User: PTOKEN_USER,
        Groups: PTOKEN_GROUPS,
        Privileges: PTOKEN_PRIVILEGES,
        UserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroups: PTOKEN_GROUPS,
        MandatoryPolicy: PTOKEN_MANDATORY_POLICY,
        Owner: PTOKEN_OWNER,
        PrimaryGroup: PTOKEN_PRIMARY_GROUP,
        DefaultDacl: PTOKEN_DEFAULT_DACL,
        Source: PTOKEN_SOURCE,
    ) -> NTSTATUS;

    pub fn ZwCreateTransaction(
        TransactionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Uow: LPGUID,
        TmHandle: HANDLE,
        CreateOptions: ULONG,
        IsolationLevel: ULONG,
        IsolationFlags: ULONG,
        Timeout: PLARGE_INTEGER,
        Description: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwCreateTransactionManager(
        TmHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LogFileName: PCUNICODE_STRING,
        CreateOptions: ULONG,
        CommitStrength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateUserProcess(
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

    pub fn ZwCreateWaitablePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: ULONG,
        MaxMessageLength: ULONG,
        MaxPoolUsage: ULONG,
    ) -> NTSTATUS;

    pub fn ZwCreateWaitCompletionPacket(
        WaitCompletionPacketHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwCreateWnfStateName(
        StateName: PWNF_STATE_NAME,
        NameLifetime: WNF_STATE_NAME_LIFETIME,
        DataScope: WNF_DATA_SCOPE,
        PersistData: BOOLEAN,
        TypeId: PCWNF_TYPE_ID,
        MaximumStateSize: ULONG,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;

    pub fn ZwCreateWorkerFactory(
        WorkerFactoryHandleReturn: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        CompletionPortHandle: HANDLE,
        WorkerProcessHandle: HANDLE,
        StartRoutine: PVOID,
        StartParameter: PVOID,
        MaxThreadCount: ULONG,
        StackReserve: SIZE_T,
        StackCommit: SIZE_T,
    ) -> NTSTATUS;

    pub fn ZwDebugActiveProcess(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwDelayExecution(
        Alertable: BOOLEAN,
        DelayInterval: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwDeleteAtom(
        Atom: RTL_ATOM,
    ) -> NTSTATUS;

    pub fn ZwDeleteBootEntry(
        Id: ULONG,
    ) -> NTSTATUS;

    pub fn ZwDeleteDriverEntry(
        Id: ULONG,
    ) -> NTSTATUS;

    pub fn ZwDeleteFile(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwDeleteKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwDeleteObjectAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwDeletePrivateNamespace(
        NamespaceHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwDeleteValueKey(
        KeyHandle: HANDLE,
        ValueName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwDeleteWnfStateData(
        StateName: PCWNF_STATE_NAME,
        ExplicitScope: PCSID,
    ) -> NTSTATUS;

    pub fn ZwDeleteWnfStateName(
        StateName: PCWNF_STATE_NAME,
    ) -> NTSTATUS;

    pub fn ZwDeviceIoControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        IoControlCode: ULONG,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwDirectGraphicsCall(
        InputBufferLength: ULONG,
        InputBuffer: PVOID,
        OutputBufferLength: ULONG,
        OutputBuffer: PVOID,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwDisableLastKnownGood() -> NTSTATUS;

    pub fn ZwDisplayString(
        String: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwDrawText(
        Text: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Options: ULONG,
    ) -> NTSTATUS;

    pub fn ZwDuplicateToken(
        ExistingTokenHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        EffectiveOnly: BOOLEAN,
        Type: TOKEN_TYPE,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwEnableLastKnownGood() -> NTSTATUS;

    pub fn ZwEnumerateBootEntries(
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwEnumerateDriverEntries(
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwEnumerateKey(
        KeyHandle: HANDLE,
        Index: ULONG,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwEnumerateSystemEnvironmentValuesEx(
        InformationClass: ULONG,
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwEnumerateTransactionObject(
        RootObjectHandle: HANDLE,
        QueryType: KTMOBJECT_TYPE,
        ObjectCursor: PKTMOBJECT_CURSOR,
        ObjectCursorLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: ULONG,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwExtendSection(
        SectionHandle: HANDLE,
        NewSectionSize: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwFilterBootOption(
        FilterOperation: FILTER_BOOT_OPTION_OPERATION,
        ObjectType: ULONG,
        ElementType: ULONG,
        Data: PVOID,
        DataSize: ULONG,
    ) -> NTSTATUS;

    pub fn ZwFilterToken(
        ExistingTokenHandle: HANDLE,
        Flags: ULONG,
        SidsToDisable: PTOKEN_GROUPS,
        PrivilegesToDelete: PTOKEN_PRIVILEGES,
        RestrictedSids: PTOKEN_GROUPS,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwFilterTokenEx(
        ExistingTokenHandle: HANDLE,
        Flags: ULONG,
        SidsToDisable: PTOKEN_GROUPS,
        PrivilegesToDelete: PTOKEN_PRIVILEGES,
        RestrictedSids: PTOKEN_GROUPS,
        DisableUserClaimsCount: ULONG,
        UserClaimsToDisable: PCUNICODE_STRING,
        DisableDeviceClaimsCount: ULONG,
        DeviceClaimsToDisable: PCUNICODE_STRING,
        DeviceGroupsToDisable: PTOKEN_GROUPS,
        RestrictedUserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceGroups: PTOKEN_GROUPS,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwFindAtom(
        AtomName: PCWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;

    pub fn ZwFlushBuffersFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn ZwFlushBuffersFileEx(
        FileHandle: HANDLE,
        Flags: ULONG,
        Parameters: PVOID,
        ParametersSize: ULONG,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn ZwFlushInstallUILanguage(
        InstallUILanguage: LANGID,
        SetComittedFlag: ULONG,
    ) -> NTSTATUS;

    pub fn ZwFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
    ) -> NTSTATUS;

    pub fn ZwFlushKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwFlushProcessWriteBuffers() -> NTSTATUS;

    pub fn ZwFlushVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        IoStatus: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn ZwFlushWriteBuffer() -> NTSTATUS;

    pub fn ZwFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: PULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;

    pub fn ZwFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        FreeType: ULONG,
    ) -> NTSTATUS;

    pub fn ZwFreezeRegistry(
        TimeOutInSeconds: ULONG,
    ) -> NTSTATUS;

    pub fn ZwFreezeTransactions(
        FreezeTimeout: PLARGE_INTEGER,
        ThawTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwFsControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsControlCode: ULONG,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwGetCachedSigningLevel(
        File: HANDLE,
        Flags: PULONG,
        SigningLevel: PSE_SIGNING_LEVEL,
        Thumbprint: PUCHAR,
        ThumbprintSize: PULONG,
        ThumbprintAlgorithm: PULONG,
    ) -> NTSTATUS;

    pub fn ZwGetCompleteWnfStateSubscription(
        OldDescriptorStateName: PWNF_STATE_NAME,
        OldSubscriptionId: *mut u64,
        OldDescriptorEventMask: ULONG,
        OldDescriptorStatus: ULONG,
        NewDeliveryDescriptor: PWNF_DELIVERY_DESCRIPTOR,
        DescriptorSize: ULONG,
    ) -> NTSTATUS;

    pub fn ZwGetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;

    pub fn ZwGetCurrentProcessorNumber() -> ULONG;

    pub fn ZwGetCurrentProcessorNumberEx(
        ProcessorNumber: PPROCESSOR_NUMBER,
    ) -> ULONG;

    pub fn ZwGetDevicePowerState(
        Device: HANDLE,
        State: PDEVICE_POWER_STATE,
    ) -> NTSTATUS;

    pub fn ZwGetMUIRegistryInfo(
        Flags: ULONG,
        DataSize: PULONG,
        Data: PVOID,
    ) -> NTSTATUS;

    pub fn ZwGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewProcessHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwGetNextThread(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewThreadHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwGetNlsSectionPtr(
        SectionType: ULONG,
        SectionData: ULONG,
        ContextData: PVOID,
        SectionPointer: *mut PVOID,
        SectionSize: PULONG,
    ) -> NTSTATUS;

    pub fn ZwGetNotificationResourceManager(
        ResourceManagerHandle: HANDLE,
        TransactionNotification: PTRANSACTION_NOTIFICATION,
        NotificationLength: ULONG,
        Timeout: PLARGE_INTEGER,
        ReturnLength: PULONG,
        Asynchronous: ULONG,
        AsynchronousContext: ULONG_PTR,
    ) -> NTSTATUS;

    pub fn ZwGetPlugPlayEvent(
        EventHandle: HANDLE,
        Context: PVOID,
        EventBlock: PPLUGPLAY_EVENT_BLOCK,
        EventBufferSize: ULONG,
    ) -> NTSTATUS;

    pub fn ZwGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: ULONG,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
        UserAddressArray: *mut PVOID,
        EntriesInUserAddressArray: PULONG_PTR,
        Granularity: PULONG,
    ) -> NTSTATUS;

    pub fn ZwImpersonateAnonymousToken(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
    ) -> NTSTATUS;

    pub fn ZwImpersonateThread(
        ServerThreadHandle: HANDLE,
        ClientThreadHandle: HANDLE,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
    ) -> NTSTATUS;

    pub fn ZwInitializeEnclave(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        EnclaveInformation: PVOID,
        EnclaveInformationLength: ULONG,
        EnclaveError: PULONG,
    ) -> NTSTATUS;

    pub fn ZwInitializeNlsFiles(
        BaseAddress: *mut PVOID,
        DefaultLocaleId: PLCID,
        DefaultCasingTableSize: PLARGE_INTEGER,
        CurrentNLSVersion: PULONG,
    ) -> NTSTATUS;

    pub fn ZwInitializeRegistry(
        BootCondition: USHORT,
    ) -> NTSTATUS;

    pub fn ZwInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: ULONG,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwIsProcessInJob(
        ProcessHandle: HANDLE,
        JobHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwIsSystemResumeAutomatic() -> BOOLEAN;

    pub fn ZwIsUILanguageComitted() -> NTSTATUS;

    pub fn ZwListenPort(
        PortHandle: HANDLE,
        ConnectionRequest: PPORT_MESSAGE,
    ) -> NTSTATUS;

    pub fn ZwLoadDriver(
        DriverServiceName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwLoadEnclaveData(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        Protect: ULONG,
        PageInformation: PVOID,
        PageInformationLength: ULONG,
        NumberOfBytesWritten: PSIZE_T,
        EnclaveError: PULONG,
    ) -> NTSTATUS;

    pub fn ZwLoadKey(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwLoadKey2(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwLoadKey3(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
        Flags: ULONG,
        ExtendedParameters: PCM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
        DesiredAccess: ACCESS_MASK,
        RootHandle: PHANDLE,
        Reserved: PVOID,
    ) -> NTSTATUS;

    pub fn ZwLoadKeyEx(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
        Flags: ULONG,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: ACCESS_MASK,
        RootHandle: PHANDLE,
        Reserved: PVOID,
    ) -> NTSTATUS;

    pub fn ZwLockFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: PLARGE_INTEGER,
        Length: PLARGE_INTEGER,
        Key: ULONG,
        FailImmediately: BOOLEAN,
        ExclusiveLock: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwLockProductActivationKeys(
        pPrivateVer: *mut ULONG,
        pSafeMode: PULONG,
    ) -> NTSTATUS;

    pub fn ZwLockRegistryKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;

    pub fn ZwMakePermanentObject(
        Handle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwMakeTemporaryObject(
        Handle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwManageHotPatch(
        HotPatchInformationClass: HOT_PATCH_INFORMATION_CLASS,
        HotPatchInformation: PVOID,
        HotPatchInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwManagePartition(
        TargetHandle: HANDLE,
        SourceHandle: HANDLE,
        PartitionInformationClass: PARTITION_INFORMATION_CLASS,
        PartitionInformation: PVOID,
        PartitionInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwMapCMFModule(
        What: ULONG,
        Index: ULONG,
        CacheIndexOut: PULONG,
        CacheFlagsOut: PULONG,
        ViewSizeOut: PULONG,
        BaseAddress: *mut PVOID,
    ) -> NTSTATUS;

    pub fn ZwMapUserPhysicalPages(
        VirtualAddress: PVOID,
        NumberOfPages: SIZE_T,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;

    pub fn ZwMapUserPhysicalPagesScatter(
        VirtualAddresses: *const PVOID,
        NumberOfPages: SIZE_T,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;

    pub fn ZwMapViewOfSection(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        CommitSize: SIZE_T,
        SectionOffset: PLARGE_INTEGER,
        ViewSize: PSIZE_T,
        InheritDisposition: SECTION_INHERIT,
        AllocationType: ULONG,
        PageProtection: ULONG,
    ) -> NTSTATUS;

    pub fn ZwMapViewOfSectionEx(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        SectionOffset: PLARGE_INTEGER,
        ViewSize: PSIZE_T,
        AllocationType: ULONG,
        PageProtection: ULONG,
        ExtendedParameters: PMEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;

    pub fn ZwModifyBootEntry(
        BootEntry: PBOOT_ENTRY,
    ) -> NTSTATUS;

    pub fn ZwModifyDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
    ) -> NTSTATUS;

    pub fn ZwNotifyChangeDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwNotifyChangeDirectoryFileEx(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
        DirectoryNotifyInformationClass: DIRECTORY_NOTIFY_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn ZwNotifyChangeKey(
        KeyHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
        Buffer: PVOID,
        BufferSize: ULONG,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwNotifyChangeMultipleKeys(
        MasterKeyHandle: HANDLE,
        Count: ULONG,
        SubordinateObjects: *const OBJECT_ATTRIBUTES,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
        Buffer: PVOID,
        BufferSize: ULONG,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwNotifyChangeSession(
        SessionHandle: HANDLE,
        ChangeSequenceNumber: ULONG,
        ChangeTimeStamp: PLARGE_INTEGER,
        Event: IO_SESSION_EVENT,
        NewState: IO_SESSION_STATE,
        PreviousState: IO_SESSION_STATE,
        Payload: PVOID,
        PayloadSize: ULONG,
    ) -> NTSTATUS;

    pub fn ZwOpenCpuPartition(
        CpuPartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenDirectoryObject(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenEnlistment(
        EnlistmentHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ResourceManagerHandle: HANDLE,
        EnlistmentGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenEvent(
        EventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenEventPair(
        EventPairHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: ULONG,
        OpenOptions: ULONG,
    ) -> NTSTATUS;

    pub fn ZwOpenIoCompletion(
        IoCompletionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenJobObject(
        JobHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenKey(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenKeyedEvent(
        KeyedEventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenKeyEx(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        OpenOptions: ULONG,
    ) -> NTSTATUS;

    pub fn ZwOpenKeyTransacted(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwOpenKeyTransactedEx(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        OpenOptions: ULONG,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwOpenMutant(
        MutantHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenObjectAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PCUNICODE_STRING,
        ObjectName: PCUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        GrantedAccess: ACCESS_MASK,
        Privileges: PPRIVILEGE_SET,
        ObjectCreation: BOOLEAN,
        AccessGranted: BOOLEAN,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwOpenPartition(
        PartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenPrivateNamespace(
        NamespaceHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        BoundaryDescriptor: POBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;

    pub fn ZwOpenProcess(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;

    pub fn ZwOpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwOpenProcessTokenEx(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwOpenRegistryTransaction(
        RegistryTransactionHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenResourceManager(
        ResourceManagerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        TmHandle: HANDLE,
        ResourceManagerGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenSection(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenSemaphore(
        SemaphoreHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenSession(
        SessionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenSymbolicLinkObject(
        LinkHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;

    pub fn ZwOpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        OpenAsSelf: BOOLEAN,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwOpenThreadTokenEx(
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        OpenAsSelf: BOOLEAN,
        HandleAttributes: ULONG,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;

    pub fn ZwOpenTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwOpenTransaction(
        TransactionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Uow: LPGUID,
        TmHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwOpenTransactionManager(
        TmHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LogFileName: PCUNICODE_STRING,
        TmIdentity: LPGUID,
        OpenOptions: ULONG,
    ) -> NTSTATUS;

    pub fn ZwPlugPlayControl(
        PnPControlClass: PLUGPLAY_CONTROL_CLASS,
        PnPControlData: PVOID,
        PnPControlDataLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwPowerInformation(
        InformationLevel: POWER_INFORMATION_LEVEL,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwPrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwPrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwPrePrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwPrePrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwPrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: PPRIVILEGE_SET,
        Result: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwPrivilegedServiceAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        ServiceName: PCUNICODE_STRING,
        ClientToken: HANDLE,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwPrivilegeObjectAuditAlarm(
        SubsystemName: PCUNICODE_STRING,
        HandleId: PVOID,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: ULONG,
        BufferLength: ULONG,
        Buffer: PVOID,
    ) -> NTSTATUS;

    pub fn ZwPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: ULONG,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        NewProtection: ULONG,
        OldProtection: PULONG,
    ) -> NTSTATUS;

    pub fn ZwPssCaptureVaSpaceBulk(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        BulkInformation: PNTPSS_MEMORY_BULK_INFORMATION,
        BulkInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwPulseEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;

    pub fn ZwQueryAttributesFile(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        FileInformation: PFILE_BASIC_INFORMATION,
    ) -> NTSTATUS;

    pub fn ZwQueryAuxiliaryCounterFrequency(
        AuxiliaryCounterFrequency: PULONG64,
    ) -> NTSTATUS;

    pub fn ZwQueryBootEntryOrder(
        Ids: PULONG,
        Count: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryBootOptions(
        BootOptions: PBOOT_OPTIONS,
        BootOptionsLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryDefaultLocale(
        UserProfile: BOOLEAN,
        DefaultLocaleId: PLCID,
    ) -> NTSTATUS;

    pub fn ZwQueryDefaultUILanguage(
        DefaultUILanguageId: *mut LANGID,
    ) -> NTSTATUS;

    pub fn ZwQueryDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
        ReturnSingleEntry: BOOLEAN,
        FileName: PCUNICODE_STRING,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwQueryDirectoryFileEx(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
        QueryFlags: ULONG,
        FileName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        RestartScan: BOOLEAN,
        Context: PULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryDriverEntryOrder(
        Ids: PULONG,
        Count: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        EaList: PVOID,
        EaListLength: ULONG,
        EaIndex: PULONG,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwQueryEvent(
        EventHandle: HANDLE,
        EventInformationClass: EVENT_INFORMATION_CLASS,
        EventInformation: PVOID,
        EventInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryFullAttributesFile(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        FileInformation: PFILE_NETWORK_OPEN_INFORMATION,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationAtom(
        Atom: RTL_ATOM,
        AtomInformationClass: ATOM_INFORMATION_CLASS,
        AtomInformation: PVOID,
        AtomInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationByName(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationCpuPartition(
        CpuPartitionHandle: HANDLE,
        CpuPartitionInformationClass: ULONG,
        CpuPartitionInformation: PVOID,
        CpuPartitionInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: PVOID,
        EnlistmentInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: PVOID,
        JobObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: PVOID,
        ResourceManagerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: PVOID,
        TokenInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: PVOID,
        TransactionInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationTransactionManager(
        TransactionManagerHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: PVOID,
        TransactionManagerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: PVOID,
        WorkerFactoryInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryInstallUILanguage(
        InstallUILanguageId: *mut LANGID,
    ) -> NTSTATUS;

    pub fn ZwQueryIntervalProfile(
        ProfileSource: KPROFILE_SOURCE,
        Interval: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryIoCompletion(
        IoCompletionHandle: HANDLE,
        IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS,
        IoCompletionInformation: PVOID,
        IoCompletionInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryIoRingCapabilities(
        IoRingCapabilitiesLength: SIZE_T,
        IoRingCapabilities: PVOID,
    ) -> NTSTATUS;

    pub fn ZwQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryLicenseValue(
        ValueName: PCUNICODE_STRING,
        Type: PULONG,
        Data: PVOID,
        DataSize: ULONG,
        ResultDataSize: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: PKEY_VALUE_ENTRY,
        EntryCount: ULONG,
        ValueBuffer: PVOID,
        BufferLength: PULONG,
        RequiredBufferLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryMutant(
        MutantHandle: HANDLE,
        MutantInformationClass: MUTANT_INFORMATION_CLASS,
        MutantInformation: PVOID,
        MutantInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: PVOID,
        ObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryOpenSubKeys(
        TargetKey: POBJECT_ATTRIBUTES,
        HandleCount: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryOpenSubKeysEx(
        TargetKey: POBJECT_ATTRIBUTES,
        BufferLength: ULONG,
        Buffer: PVOID,
        RequiredSize: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryPerformanceCounter(
        PerformanceCounter: PLARGE_INTEGER,
        PerformanceFrequency: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwQueryPortInformationProcess() -> LOGICAL;

    pub fn ZwQueryQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        SidList: PVOID,
        SidListLength: ULONG,
        StartSid: PSID,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: PVOID,
        SectionInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwQuerySecurityAttributesToken(
        TokenHandle: HANDLE,
        Attributes: PCUNICODE_STRING,
        NumberOfAttributes: ULONG,
        Buffer: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Length: ULONG,
        LengthNeeded: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySecurityPolicy(
        Policy: PCUNICODE_STRING,
        KeyName: PCUNICODE_STRING,
        ValueName: PCUNICODE_STRING,
        ValueType: SECURE_SETTING_VALUE_TYPE,
        Value: PVOID,
        ValueSize: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySemaphore(
        SemaphoreHandle: HANDLE,
        SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS,
        SemaphoreInformation: PVOID,
        SemaphoreInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: PUNICODE_STRING,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySystemEnvironmentValue(
        VariableName: PCUNICODE_STRING,
        VariableValue: PWSTR,
        ValueLength: USHORT,
        ReturnLength: PUSHORT,
    ) -> NTSTATUS;

    pub fn ZwQuerySystemEnvironmentValueEx(
        VariableName: PCUNICODE_STRING,
        VendorGuid: PCGUID,
        Buffer: PVOID,
        BufferLength: PULONG,
        Attributes: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySystemInformationEx(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQuerySystemTime(
        SystemTime: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwQueryTimer(
        TimerHandle: HANDLE,
        TimerInformationClass: TIMER_INFORMATION_CLASS,
        TimerInformation: PVOID,
        TimerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryTimerResolution(
        MaximumTime: PULONG,
        MinimumTime: PULONG,
        CurrentTime: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: PCUNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: PVOID,
        MemoryInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwQueryVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: PVOID,
        Length: ULONG,
        FsInformationClass: FSINFOCLASS,
    ) -> NTSTATUS;

    pub fn ZwQueryWnfStateData(
        StateName: PCWNF_STATE_NAME,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: PCSID,
        ChangeStamp: PWNF_CHANGE_STAMP,
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwQueryWnfStateNameInformation(
        StateName: PCWNF_STATE_NAME,
        NameInfoClass: WNF_STATE_NAME_INFORMATION,
        ExplicitScope: PCSID,
        Buffer: PVOID,
        BufferLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;

    pub fn ZwQueueApcThreadEx(
        ThreadHandle: HANDLE,
        ReserveHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;

    pub fn ZwQueueApcThreadEx2(
        ThreadHandle: HANDLE,
        ReserveHandle: HANDLE,
        ApcFlags: ULONG,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;

    pub fn ZwRaiseException(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
        FirstChance: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: ULONG,
        UnicodeStringParameterMask: ULONG,
        Parameters: PULONG_PTR,
        ValidResponseOptions: ULONG,
        Response: PULONG,
    ) -> NTSTATUS;

    pub fn ZwReadFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn ZwReadFileScatter(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: PFILE_SEGMENT_ELEMENT,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn ZwReadOnlyEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwReadRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: ULONG,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesRead: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        NumberOfBytesToRead: SIZE_T,
        NumberOfBytesRead: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwReadVirtualMemoryEx(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        NumberOfBytesToRead: SIZE_T,
        NumberOfBytesRead: PSIZE_T,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwRecoverEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentKey: PVOID,
    ) -> NTSTATUS;

    pub fn ZwRecoverResourceManager(
        ResourceManagerHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwRecoverTransactionManager(
        TransactionManagerHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: PCRM_PROTOCOL_ID,
        ProtocolInformationSize: ULONG,
        ProtocolInformation: PVOID,
        CreateOptions: ULONG,
    ) -> NTSTATUS;

    pub fn ZwRegisterThreadTerminatePort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwReleaseCMFViewOwnership() -> NTSTATUS;

    pub fn ZwReleaseKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: PVOID,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwReleaseMutant(
        MutantHandle: HANDLE,
        PreviousCount: PLONG,
    ) -> NTSTATUS;

    pub fn ZwReleaseSemaphore(
        SemaphoreHandle: HANDLE,
        ReleaseCount: LONG,
        PreviousCount: PLONG,
    ) -> NTSTATUS;

    pub fn ZwReleaseWorkerFactoryWorker(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwRemoveIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut PVOID,
        ApcContext: *mut PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwRemoveIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionInformation: PFILE_IO_COMPLETION_INFORMATION,
        Count: ULONG,
        NumEntriesRemoved: PULONG,
        Timeout: PLARGE_INTEGER,
        Alertable: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwRemoveProcessDebug(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwRenameKey(
        KeyHandle: HANDLE,
        NewName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwRenameTransactionManager(
        LogFileName: PCUNICODE_STRING,
        ExistingTransactionManagerGuid: LPGUID,
    ) -> NTSTATUS;

    pub fn ZwReplaceKey(
        NewFile: POBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwReplacePartitionUnit(
        TargetInstancePath: PCUNICODE_STRING,
        SpareInstancePath: PCUNICODE_STRING,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;

    pub fn ZwReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut PVOID,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;

    pub fn ZwReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut PVOID,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwReplyWaitReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;

    pub fn ZwRequestPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;

    pub fn ZwRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;

    pub fn ZwRequestWakeupLatency(
        latency: LATENCY_TIME,
    ) -> NTSTATUS;

    pub fn ZwResetEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;

    pub fn ZwResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
    ) -> NTSTATUS;

    pub fn ZwRestoreKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwResumeProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;

    pub fn ZwRevertContainerImpersonation() -> NTSTATUS;

    pub fn ZwRollbackComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwRollbackEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwRollbackRegistryTransaction(
        RegistryTransactionHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwRollbackTransaction(
        TransactionHandle: HANDLE,
        Wait: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwRollforwardTransactionManager(
        TransactionManagerHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwSaveKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSaveKeyEx(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Format: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSecureConnectPort(
        PortHandle: PHANDLE,
        PortName: PCUNICODE_STRING,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        RequiredServerSid: PSID,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: PULONG,
        ConnectionInformation: PVOID,
        ConnectionInformationLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwSerializeBoot() -> NTSTATUS;

    pub fn ZwSetBootEntryOrder(
        Ids: PULONG,
        Count: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetBootOptions(
        BootOptions: PBOOT_OPTIONS,
        FieldsToChange: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetCachedSigningLevel(
        Flags: ULONG,
        InputSigningLevel: SE_SIGNING_LEVEL,
        SourceFiles: PHANDLE,
        SourceFileCount: ULONG,
        TargetFile: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSetCachedSigningLevel2(
        Flags: ULONG,
        InputSigningLevel: SE_SIGNING_LEVEL,
        SourceFiles: PHANDLE,
        SourceFileCount: ULONG,
        TargetFile: HANDLE,
        CacheInformation: *mut SE_SET_FILE_CACHE_INFORMATION,
    ) -> NTSTATUS;

    pub fn ZwSetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;

    pub fn ZwSetDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
        State: BOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwSetDefaultHardErrorPort(
        DefaultHardErrorPort: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSetDefaultLocale(
        UserProfile: BOOLEAN,
        DefaultLocaleId: LCID,
    ) -> NTSTATUS;

    pub fn ZwSetDefaultUILanguage(
        DefaultUILanguageId: LANGID,
    ) -> NTSTATUS;

    pub fn ZwSetDriverEntryOrder(
        Ids: PULONG,
        Count: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;

    pub fn ZwSetEventBoostPriority(
        EventHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSetEventEx(
        ThreadId: HANDLE,
        Lock: PRTL_SRWLOCK,
    ) -> NTSTATUS;

    pub fn ZwSetHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSetHighWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSetInformationCpuPartition(
        CpuPartitionHandle: HANDLE,
        CpuPartitionInformationClass: ULONG,
        CpuPartitionInformation: PVOID,
        CpuPartitionInformationLength: ULONG,
        Reserved1: PVOID,
        Reserved2: ULONG,
        Reserved3: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: PVOID,
        DebugInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: PVOID,
        EnlistmentInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn ZwSetInformationIoRing(
        IoRingHandle: HANDLE,
        IoRingInformationClass: ULONG,
        IoRingInformationLength: ULONG,
        IoRingInformation: PVOID,
    ) -> NTSTATUS;

    pub fn ZwSetInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: PVOID,
        JobObjectInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationKey(
        KeyHandle: HANDLE,
        KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
        KeySetInformation: PVOID,
        KeySetInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: PVOID,
        ObjectInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: PVOID,
        ResourceManagerInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationSymbolicLink(
        LinkHandle: HANDLE,
        SymbolicLinkInformationClass: SYMBOLIC_LINK_INFO_CLASS,
        SymbolicLinkInformation: PVOID,
        SymbolicLinkInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: PVOID,
        TokenInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: PVOID,
        TransactionInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationTransactionManager(
        TmHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: PVOID,
        TransactionManagerInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationVirtualMemory(
        ProcessHandle: HANDLE,
        VmInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS,
        NumberOfEntries: SIZE_T,
        VirtualAddresses: PMEMORY_RANGE_ENTRY,
        VmInformation: PVOID,
        VmInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: PVOID,
        WorkerFactoryInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetIntervalProfile(
        Interval: ULONG,
        Source: KPROFILE_SOURCE,
    ) -> NTSTATUS;

    pub fn ZwSetIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
    ) -> NTSTATUS;

    pub fn ZwSetIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionPacketHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
    ) -> NTSTATUS;

    pub fn ZwSetIRTimer(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwSetLdtEntries(
        Selector0: ULONG,
        Entry0Low: ULONG,
        Entry0Hi: ULONG,
        Selector1: ULONG,
        Entry1Low: ULONG,
        Entry1Hi: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSetLowWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSetQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetSecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;

    pub fn ZwSetSystemEnvironmentValue(
        VariableName: PCUNICODE_STRING,
        VariableValue: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwSetSystemEnvironmentValueEx(
        VariableName: PCUNICODE_STRING,
        VendorGuid: PCGUID,
        Buffer: PVOID,
        BufferLength: ULONG,
        Attributes: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetSystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetSystemTime(
        SystemTime: PLARGE_INTEGER,
        PreviousTime: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: *mut EXECUTION_STATE,
    ) -> NTSTATUS;

    pub fn ZwSetTimer(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
        TimerApcRoutine: PTIMER_APC_ROUTINE,
        TimerContext: PVOID,
        ResumeTimer: BOOLEAN,
        Period: LONG,
        PreviousState: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn ZwSetTimer2(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
        Period: PLARGE_INTEGER,
        Parameters: PT2_SET_PARAMETERS,
    ) -> NTSTATUS;

    pub fn ZwSetTimerEx(
        TimerHandle: HANDLE,
        TimerSetInformationClass: TIMER_SET_INFORMATION_CLASS,
        TimerSetInformation: PVOID,
        TimerSetInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetTimerResolution(
        DesiredTime: ULONG,
        SetResolution: BOOLEAN,
        ActualTime: PULONG,
    ) -> NTSTATUS;

    pub fn ZwSetUuidSeed(
        Seed: PCHAR,
    ) -> NTSTATUS;

    pub fn ZwSetValueKey(
        KeyHandle: HANDLE,
        ValueName: PCUNICODE_STRING,
        TitleIndex: ULONG,
        Type: ULONG,
        Data: PVOID,
        DataSize: ULONG,
    ) -> NTSTATUS;

    pub fn ZwSetVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: PVOID,
        Length: ULONG,
        FsInformationClass: FSINFOCLASS,
    ) -> NTSTATUS;

    pub fn ZwSetWnfProcessNotificationEvent(
        NotificationEvent: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwShutdownSystem(
        Action: SHUTDOWN_ACTION,
    ) -> NTSTATUS;

    pub fn ZwShutdownWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        PendingWorkerCount: *mut LONG,
    ) -> NTSTATUS;

    pub fn ZwSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwSinglePhaseReject(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwStartProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwStopProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSubmitIoRing(
        IoRingHandle: HANDLE,
        Flags: ULONG,
        WaitOperations: ULONG,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwSubscribeWnfStateChange(
        StateName: PCWNF_STATE_NAME,
        ChangeStamp: WNF_CHANGE_STAMP,
        EventMask: ULONG,
        SubscriptionId: PULONG64,
    ) -> NTSTATUS;

    pub fn ZwSuspendProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwSuspendThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;

    pub fn ZwSystemDebugControl(
        Command: SYSDBG_COMMAND,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwTerminateEnclave(
        BaseAddress: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwTerminateJobObject(
        JobHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwTerminateProcess(
        ProcessHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwTerminateThread(
        ThreadHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;

    pub fn ZwTestAlert() -> NTSTATUS;

    pub fn ZwThawRegistry() -> NTSTATUS;

    pub fn ZwThawTransactions() -> NTSTATUS;

    pub fn ZwTraceControl(
        FunctionCode: ETWTRACECONTROLCODE,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwTraceEvent(
        TraceHandle: HANDLE,
        Flags: ULONG,
        FieldSize: ULONG,
        Fields: PVOID,
    ) -> NTSTATUS;

    pub fn ZwTranslateFilePath(
        InputFilePath: PFILE_PATH,
        OutputType: ULONG,
        OutputFilePath: PFILE_PATH,
        OutputFilePathLength: PULONG,
    ) -> NTSTATUS;

    pub fn ZwUmsThreadYield(
        SchedulerParam: PVOID,
    ) -> NTSTATUS;

    pub fn ZwUnloadDriver(
        DriverServiceName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn ZwUnloadKey(
        TargetKey: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn ZwUnloadKey2(
        TargetKey: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwUnloadKeyEx(
        TargetKey: POBJECT_ATTRIBUTES,
        Event: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwUnlockFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: PLARGE_INTEGER,
        Length: PLARGE_INTEGER,
        Key: ULONG,
    ) -> NTSTATUS;

    pub fn ZwUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;

    pub fn ZwUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
    ) -> NTSTATUS;

    pub fn ZwUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn ZwUnsubscribeWnfStateChange(
        StateName: PCWNF_STATE_NAME,
    ) -> NTSTATUS;

    pub fn ZwUpdateWnfStateData(
        StateName: PCWNF_STATE_NAME,
        Buffer: *const core::ffi::c_void,
        Length: ULONG,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: PCSID,
        MatchingChangeStamp: WNF_CHANGE_STAMP,
        CheckStamp: LOGICAL,
    ) -> NTSTATUS;

    pub fn ZwVdmControl(
        Service: VDMSERVICECLASS,
        ServiceData: PVOID,
    ) -> NTSTATUS;

    pub fn ZwWaitForAlertByThreadId(
        Address: PVOID,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
        WaitStateChange: PDBGUI_WAIT_STATE_CHANGE,
    ) -> NTSTATUS;

    pub fn ZwWaitForKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: PVOID,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwWaitForMultipleObjects(
        Count: ULONG,
        Handles: *const HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwWaitForMultipleObjects32(
        Count: ULONG,
        Handles: *const LONG,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwWaitForSingleObject(
        Handle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn ZwWaitForWorkViaWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        MiniPacket: PFILE_IO_COMPLETION_INFORMATION,
    ) -> NTSTATUS;

    pub fn ZwWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwWorkerFactoryWorkerReady(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn ZwWriteFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn ZwWriteFileGather(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: PFILE_SEGMENT_ELEMENT,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn ZwWriteRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: ULONG,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesWritten: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        NumberOfBytesToWrite: SIZE_T,
        NumberOfBytesWritten: PSIZE_T,
    ) -> NTSTATUS;

    pub fn ZwYieldExecution() -> NTSTATUS;
}