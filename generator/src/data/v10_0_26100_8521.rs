use std::collections::HashMap;
use std::sync::LazyLock;

use crate::data::FunctionSignature;

pub static V10_0_26100_8521_SIGNATURES: LazyLock<HashMap<&'static str, FunctionSignature>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    
    map.insert("NtQueryFullAttributesFile", FunctionSignature {
        name: "NtQueryFullAttributesFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PFILE_NETWORK_OPEN_INFORMATION", "FileInformation"),
        ],
    });
    
    map.insert("NtCreateFile", FunctionSignature {
        name: "NtCreateFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "AllocationSize"),
            ("ULONG", "FileAttributes"),
            ("ULONG", "ShareAccess"),
            ("ULONG", "CreateDisposition"),
            ("ULONG", "CreateOptions"),
            ("PVOID", "EaBuffer"),
            ("ULONG", "EaLength"),
        ],
    });
    
    map.insert("NtOpenProcess", FunctionSignature {
        name: "NtOpenProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PCLIENT_ID", "ClientId"),
        ],
    });
    
    map.insert("NtReadFile", FunctionSignature {
        name: "NtReadFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });
    
    map.insert("NtFreezeTransactions", FunctionSignature {
        name: "NtFreezeTransactions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });
    
    map.insert("NtLockFile", FunctionSignature {
        name: "NtLockFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PLARGE_INTEGER", "Length"),
            ("ULONG", "Key"),
            ("BOOLEAN", "FailImmediately"),
            ("BOOLEAN", "ExclusiveLock"),
        ],
    });
    
    map.insert("NtAssignProcessToJobObject", FunctionSignature {
        name: "NtAssignProcessToJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("HANDLE", "ProcessHandle"),
        ],
    });
    
    map.insert("NtManagePartition", FunctionSignature {
        name: "NtManagePartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PartitionHandle"),
            ("ULONG", "Operation"),
            ("PVOID", "Parameters"),
            ("ULONG", "ParametersSize"),
        ],
    });
    
    map.insert("NtCreateProcess", FunctionSignature {
        name: "NtCreateProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ParentProcess"),
            ("BOOLEAN", "InheritObjectTable"),
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "DebugPort"),
            ("HANDLE", "ExceptionPort"),
        ],
    });
    
    map.insert("NtCreateThreadEx", FunctionSignature {
        name: "NtCreateThreadEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "StartRoutine"),
            ("PVOID", "Argument"),
            ("ULONG", "CreateFlags"),
            ("SIZE_T", "ZeroBits"),
            ("SIZE_T", "StackSize"),
            ("SIZE_T", "MaximumStackSize"),
            ("PPS_ATTRIBUTE_LIST", "AttributeList"),
        ],
    });
    
    map.insert("NtOpenProcessToken", FunctionSignature {
        name: "NtOpenProcessToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("PHANDLE", "TokenHandle"),
        ],
    });
    
    map.insert("NtOpenKey", FunctionSignature {
        name: "NtOpenKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });
    
    map.insert("NtClose", FunctionSignature {
        name: "NtClose",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
        ],
    });
    
    map.insert("NtQueryInformationFile", FunctionSignature {
        name: "NtQueryInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
        ],
    });
    
    map.insert("NtSetInformationFile", FunctionSignature {
        name: "NtSetInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
        ],
    });
    
    map.insert("NtQuerySystemInformation", FunctionSignature {
        name: "NtQuerySystemInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SYSTEM_INFORMATION_CLASS", "SystemInformationClass"),
            ("PVOID", "SystemInformation"),
            ("ULONG", "SystemInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });
    
    map.insert("NtQueryVirtualMemory", FunctionSignature {
        name: "NtQueryVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("MEMORY_INFORMATION_CLASS", "MemoryInformationClass"),
            ("PVOID", "MemoryInformation"),
            ("SIZE_T", "MemoryInformationLength"),
            ("PSIZE_T", "ReturnLength"),
        ],
    });
    
    map.insert("NtAllocateVirtualMemory", FunctionSignature {
        name: "NtAllocateVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG_PTR", "ZeroBits"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Protect"),
        ],
    });
    
    map.insert("NtFreeVirtualMemory", FunctionSignature {
        name: "NtFreeVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "FreeType"),
        ],
    });
    
    map.insert("NtProtectVirtualMemory", FunctionSignature {
        name: "NtProtectVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "NewProtect"),
            ("PULONG", "OldProtect"),
        ],
    });
    
    map.insert("NtWriteFile", FunctionSignature {
        name: "NtWriteFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });
    
    map.insert("NtWaitForSingleObject", FunctionSignature {
        name: "NtWaitForSingleObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });
    
    map.insert("NtDelayExecution", FunctionSignature {
        name: "NtDelayExecution",
        return_type: "NTSTATUS",
        parameters: vec![
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "DelayInterval"),
        ],
    });
    
    map.insert("NtCreateEvent", FunctionSignature {
        name: "NtCreateEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("EVENT_TYPE", "EventType"),
            ("BOOLEAN", "InitialState"),
        ],
    });
    
    map.insert("NtSetEvent", FunctionSignature {
        name: "NtSetEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("PLONG", "PreviousState"),
        ],
    });
    
    map.insert("NtClearEvent", FunctionSignature {
        name: "NtClearEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
        ],
    });
    
    map.insert("NtCreateMutant", FunctionSignature {
        name: "NtCreateMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "MutantHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("BOOLEAN", "InitialOwner"),
        ],
    });
    
    map.insert("NtOpenMutant", FunctionSignature {
        name: "NtOpenMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "MutantHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });
    
    map.insert("NtReleaseMutant", FunctionSignature {
        name: "NtReleaseMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MutantHandle"),
            ("PLONG", "PreviousCount"),
        ],
    });
    
    map.insert("NtCreateSemaphore", FunctionSignature {
        name: "NtCreateSemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SemaphoreHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("LONG", "InitialCount"),
            ("LONG", "MaximumCount"),
        ],
    });
    
    map.insert("NtOpenSemaphore", FunctionSignature {
        name: "NtOpenSemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SemaphoreHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });
    
    map.insert("NtReleaseSemaphore", FunctionSignature {
        name: "NtReleaseSemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SemaphoreHandle"),
            ("LONG", "ReleaseCount"),
            ("PLONG", "PreviousCount"),
        ],
    });
    
    map.insert("NtCreateTimer", FunctionSignature {
        name: "NtCreateTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("TIMER_TYPE", "TimerType"),
        ],
    });
    
    map.insert("NtOpenTimer", FunctionSignature {
        name: "NtOpenTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });
    
    map.insert("NtSetTimer", FunctionSignature {
        name: "NtSetTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PTIMER_APC_ROUTINE", "TimerApcRoutine"),
            ("PVOID", "TimerContext"),
            ("BOOLEAN", "Resume"),
            ("LONG", "Period"),
            ("PBOOLEAN", "PreviousState"),
        ],
    });
    
    map.insert("NtCancelTimer", FunctionSignature {
        name: "NtCancelTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PBOOLEAN", "CurrentState"),
        ],
    });
    
    map.insert("NtQueryPerformanceCounter", FunctionSignature {
        name: "NtQueryPerformanceCounter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PLARGE_INTEGER", "PerformanceCounter"),
            ("PLARGE_INTEGER", "PerformanceFrequency"),
        ],
    });
    
    map.insert("NtQuerySystemTime", FunctionSignature {
        name: "NtQuerySystemTime",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PLARGE_INTEGER", "SystemTime"),
        ],
    });
    
    map.insert("NtSetSystemTime", FunctionSignature {
        name: "NtSetSystemTime",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PLARGE_INTEGER", "SystemTime"),
            ("PLARGE_INTEGER", "PreviousTime"),
        ],
    });
    
    map.insert("NtTerminateProcess", FunctionSignature {
        name: "NtTerminateProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("NTSTATUS", "ExitStatus"),
        ],
    });
    
    map.insert("NtTerminateThread", FunctionSignature {
        name: "NtTerminateThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("NTSTATUS", "ExitStatus"),
        ],
    });
    
    map.insert("NtSuspendThread", FunctionSignature {
        name: "NtSuspendThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });
    
    map.insert("NtResumeThread", FunctionSignature {
        name: "NtResumeThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });
    
    map.insert("NtSuspendProcess", FunctionSignature {
        name: "NtSuspendProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
        ],
    });
    
    map.insert("NtResumeProcess", FunctionSignature {
        name: "NtResumeProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
        ],
    });
    
    map.insert("NtShutdownSystem", FunctionSignature {
        name: "NtShutdownSystem",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SHUTDOWN_ACTION", "Action"),
        ],
    });
    
    map.insert("NtRaiseHardError", FunctionSignature {
        name: "NtRaiseHardError",
        return_type: "NTSTATUS",
        parameters: vec![
            ("NTSTATUS", "ErrorStatus"),
            ("ULONG", "NumberOfParameters"),
            ("ULONG", "UnicodeStringParameterMask"),
            ("PULONG_PTR", "Parameters"),
            ("ULONG", "ResponseOption"),
            ("PULONG", "Response"),
        ],
    });
    
    map.insert("NtRaiseException", FunctionSignature {
        name: "NtRaiseException",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PEXCEPTION_RECORD", "ExceptionRecord"),
            ("PCONTEXT", "ContextRecord"),
            ("BOOLEAN", "FirstChance"),
        ],
    });
    
    map.insert("NtContinue", FunctionSignature {
        name: "NtContinue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PCONTEXT", "ContextRecord"),
            ("BOOLEAN", "TestAlert"),
        ],
    });
    
    map.insert("NtYieldExecution", FunctionSignature {
        name: "NtYieldExecution",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtFreezeTransactions", FunctionSignature {
        name: "NtFreezeTransactions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtRecoverTransactionManager", FunctionSignature {
        name: "NtRecoverTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
        ],
    });

    map.insert("NtRollbackTransaction", FunctionSignature {
        name: "NtRollbackTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("BOOLEAN", "Wait"),
        ],
    });

    map.insert("NtCommitTransaction", FunctionSignature {
        name: "NtCommitTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("BOOLEAN", "Wait"),
        ],
    });

    map.insert("NtCreateTransaction", FunctionSignature {
        name: "NtCreateTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("LUID", "Uow"),
            ("HANDLE", "TmHandle"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "IsolationLevel"),
            ("ULONG", "IsolationFlags"),
            ("PLARGE_INTEGER", "Timeout"),
            ("PWSTR", "Description"),
        ],
    });

    map.insert("ZwCreateTransaction", FunctionSignature {
        name: "ZwCreateTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("LUID", "Uow"),
            ("HANDLE", "TmHandle"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "IsolationLevel"),
            ("ULONG", "IsolationFlags"),
            ("PLARGE_INTEGER", "Timeout"),
            ("PWSTR", "Description"),
        ],
    });

    map.insert("ZwCommitComplete", FunctionSignature {
        name: "ZwCommitComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwRollbackComplete", FunctionSignature {
        name: "ZwRollbackComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwPrepareComplete", FunctionSignature {
        name: "ZwPrepareComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwPrePrepareComplete", FunctionSignature {
        name: "ZwPrePrepareComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwRecoverEnlistment", FunctionSignature {
        name: "ZwRecoverEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "EnlistmentKey"),
        ],
    });

    map.insert("ZwRecoverResourceManager", FunctionSignature {
        name: "ZwRecoverResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
        ],
    });

    map.insert("NtRestoreKey", FunctionSignature {
        name: "NtRestoreKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtSaveKeyEx", FunctionSignature {
        name: "NtSaveKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "FileHandle"),
            ("ULONG", "Format"),
        ],
    });

    map.insert("ZwLoadKeyEx", FunctionSignature {
        name: "ZwLoadKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
            ("ULONG", "Flags"),
            ("HANDLE", "TrustClassKey"),
            ("HANDLE", "Event"),
            ("ULONG", "DesiredAccess"),
            ("PHANDLE", "RootHandle"),
        ],
    });

    map.insert("NtLoadKeyEx", FunctionSignature {
        name: "NtLoadKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
            ("ULONG", "Flags"),
            ("HANDLE", "TrustClassKey"),
            ("HANDLE", "Event"),
            ("ULONG", "DesiredAccess"),
            ("PHANDLE", "RootHandle"),
        ],
    });

    map.insert("NtNotifyChangeMultipleKeys", FunctionSignature {
        name: "NtNotifyChangeMultipleKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MasterKeyHandle"),
            ("ULONG", "Count"),
            ("POBJECT_ATTRIBUTES", "KeyObjects"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("PVOID", "ChangeBuffer"),
            ("ULONG", "BufferSize"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("ZwCreateRegistryTransaction", FunctionSignature {
        name: "ZwCreateRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtRollbackRegistryTransaction", FunctionSignature {
        name: "NtRollbackRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("NtCommitRegistryTransaction", FunctionSignature {
        name: "NtCommitRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("NtCreateThreadStateChange", FunctionSignature {
        name: "NtCreateThreadStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "StateChangeHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ThreadHandle"),
            ("HANDLE", "ProcessHandle"),
        ],
    });

    map.insert("NtSetInformationThread", FunctionSignature {
        name: "NtSetInformationThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("THREADINFOCLASS", "ThreadInformationClass"),
            ("PVOID", "ThreadInformation"),
            ("ULONG", "ThreadInformationLength"),
        ],
    });

    map.insert("NtQueryInformationThread", FunctionSignature {
        name: "NtQueryInformationThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("THREADINFOCLASS", "ThreadInformationClass"),
            ("PVOID", "ThreadInformation"),
            ("ULONG", "ThreadInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwSetInformationThread", FunctionSignature {
        name: "ZwSetInformationThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("THREADINFOCLASS", "ThreadInformationClass"),
            ("PVOID", "ThreadInformation"),
            ("ULONG", "ThreadInformationLength"),
        ],
    });

    map.insert("NtSuspendThread", FunctionSignature {
        name: "NtSuspendThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });

    map.insert("NtResumeThread", FunctionSignature {
        name: "NtResumeThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });

    map.insert("NtSuspendProcess", FunctionSignature {
        name: "NtSuspendProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
        ],
    });

    map.insert("NtResumeProcess", FunctionSignature {
        name: "NtResumeProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
        ],
    });

    map.insert("NtAlpcSetInformation", FunctionSignature {
        name: "NtAlpcSetInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "AlpcHandle"),
            ("ALPC_PORT_INFORMATION_CLASS", "AlpcInformationClass"),
            ("PVOID", "AlpcInformation"),
            ("ULONG", "AlpcInformationLength"),
        ],
    });

    map.insert("ZwAlpcCreateSectionView", FunctionSignature {
        name: "ZwAlpcCreateSectionView",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PALPC_DATA_VIEW_ATTR", "SectionView"),
        ],
    });

    map.insert("NtAlpcDeleteSecurityContext", FunctionSignature {
        name: "NtAlpcDeleteSecurityContext",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "Context"),
        ],
    });

    map.insert("ZwAlpcImpersonateClientContainerOfPort", FunctionSignature {
        name: "ZwAlpcImpersonateClientContainerOfPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwCreatePort", FunctionSignature {
        name: "ZwCreatePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "MaxConnectionInfoLength"),
            ("ULONG", "MaxMessageLength"),
            ("ULONG", "MaxPoolUsage"),
        ],
    });

    map.insert("NtSecureConnectPort", FunctionSignature {
        name: "NtSecureConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
            ("PVOID", "ClientView"),
            ("PSID", "RequiredServerSid"),
            ("PVOID", "ConnectData"),
            ("PULONG", "DataLength"),
        ],
    });

    map.insert("ZwDeleteWnfStateName", FunctionSignature {
        name: "ZwDeleteWnfStateName",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
        ],
    });

    map.insert("NtDeleteWnfStateData", FunctionSignature {
        name: "NtDeleteWnfStateData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQueryWnfStateData", FunctionSignature {
        name: "NtQueryWnfStateData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
            ("PWNF_CHANGE_STAMP", "ChangeStamp"),
        ],
    });

    map.insert("ZwQueryWnfStateData", FunctionSignature {
        name: "ZwQueryWnfStateData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
            ("PWNF_CHANGE_STAMP", "ChangeStamp"),
        ],
    });

    map.insert("NtInitiatePowerAction", FunctionSignature {
        name: "NtInitiatePowerAction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POWER_ACTION", "SystemAction"),
            ("SYSTEM_POWER_STATE", "LightestSystemState"),
            ("ULONG", "Flags"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("ZwSetSystemPowerState", FunctionSignature {
        name: "ZwSetSystemPowerState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POWER_ACTION", "SystemAction"),
            ("SYSTEM_POWER_STATE", "LightestSystemState"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtSetSystemInformation", FunctionSignature {
        name: "NtSetSystemInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SYSTEM_INFORMATION_CLASS", "SystemInformationClass"),
            ("PVOID", "SystemInformation"),
            ("ULONG", "SystemInformationLength"),
        ],
    });

    map.insert("ZwSetSystemTime", FunctionSignature {
        name: "ZwSetSystemTime",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PLARGE_INTEGER", "SystemTime"),
            ("PLARGE_INTEGER", "PreviousTime"),
        ],
    });

    map.insert("ZwShutdownSystem", FunctionSignature {
        name: "ZwShutdownSystem",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SHUTDOWN_ACTION", "Action"),
        ],
    });

    map.insert("NtAllocateVirtualMemoryEx", FunctionSignature {
        name: "NtAllocateVirtualMemoryEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Protect"),
            ("PULONG", "ExtendedParameters"),
            ("ULONG", "ParameterCount"),
        ],
    });

    map.insert("NtMapUserPhysicalPages", FunctionSignature {
        name: "NtMapUserPhysicalPages",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "VirtualAddress"),
            ("ULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("NtMapUserPhysicalPagesScatter", FunctionSignature {
        name: "NtMapUserPhysicalPagesScatter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "VirtualAddresses"),
            ("ULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("ZwAllocateUserPhysicalPagesEx", FunctionSignature {
        name: "ZwAllocateUserPhysicalPagesEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
            ("PULONG", "ExtendedParameters"),
            ("ULONG", "ParameterCount"),
        ],
    });

    map.insert("NtCreateTokenEx", FunctionSignature {
        name: "NtCreateTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("TOKEN_TYPE", "Type"),
            ("PLUID", "AuthenticationId"),
            ("PLARGE_INTEGER", "ExpirationTime"),
            ("PTOKEN_USER", "User"),
            ("PTOKEN_GROUPS", "Groups"),
            ("PTOKEN_PRIVILEGES", "Privileges"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "UserAttributes"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "DeviceAttributes"),
            ("PTOKEN_GROUPS", "DeviceGroups"),
            ("PTOKEN_MANDATORY_POLICY", "MandatoryPolicy"),
            ("PTOKEN_OWNER", "Owner"),
            ("PTOKEN_PRIMARY_GROUP", "PrimaryGroup"),
            ("PTOKEN_DEFAULT_DACL", "DefaultDacl"),
            ("PTOKEN_SOURCE", "Source"),
        ],
    });

    map.insert("NtFilterToken", FunctionSignature {
        name: "NtFilterToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ExistingTokenHandle"),
            ("ULONG", "Flags"),
            ("PTOKEN_GROUPS", "SidsToDisable"),
            ("PTOKEN_PRIVILEGES", "PrivilegesToDelete"),
            ("PTOKEN_GROUPS", "RestrictedSids"),
            ("PHANDLE", "NewTokenHandle"),
        ],
    });

    map.insert("NtAdjustPrivilegesToken", FunctionSignature {
        name: "NtAdjustPrivilegesToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("BOOLEAN", "DisableAllPrivileges"),
            ("PTOKEN_PRIVILEGES", "NewState"),
            ("ULONG", "BufferLength"),
            ("PTOKEN_PRIVILEGES", "PreviousState"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwOpenThreadTokenEx", FunctionSignature {
        name: "ZwOpenThreadTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("BOOLEAN", "OpenAsSelf"),
            ("ULONG", "HandleAttributes"),
            ("PHANDLE", "TokenHandle"),
        ],
    });

    map.insert("ZwFsControlFile", FunctionSignature {
        name: "ZwFsControlFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "FsControlCode"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
        ],
    });

    map.insert("NtQueryVolumeInformationFile", FunctionSignature {
        name: "NtQueryVolumeInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FsInformation"),
            ("ULONG", "Length"),
            ("FSINFOCLASS", "FsInformationClass"),
        ],
    });

    map.insert("NtFlushBuffersFile", FunctionSignature {
        name: "NtFlushBuffersFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("NtCancelIoFile", FunctionSignature {
        name: "NtCancelIoFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("NtCancelIoFileEx", FunctionSignature {
        name: "NtCancelIoFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "CancelContext"),
        ],
    });

    map.insert("NtInitializeEnclave", FunctionSignature {
        name: "NtInitializeEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "ZeroBits"),
            ("PVOID", "EnclaveInformation"),
            ("ULONG", "InformationLength"),
        ],
    });

    map.insert("NtLoadEnclaveData", FunctionSignature {
        name: "NtLoadEnclaveData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PVOID", "LoadInformation"),
            ("ULONG", "InformationLength"),
            ("PULONG", "EnclaveInformation"),
        ],
    });

    map.insert("NtWaitForDebugEvent", FunctionSignature {
        name: "NtWaitForDebugEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DebugObjectHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
            ("PDBGUI_WAIT_STATE_CHANGE", "WaitStateChange"),
        ],
    });

    map.insert("NtDebugContinue", FunctionSignature {
        name: "NtDebugContinue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DebugObjectHandle"),
            ("CLIENT_ID", "ClientId"),
            ("NTSTATUS", "ContinueStatus"),
        ],
    });

    map.insert("NtRemoveIoCompletionEx", FunctionSignature {
        name: "NtRemoveIoCompletionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("PVOID", "Buffer"),
            ("ULONG", "Count"),
            ("PULONG", "Removed"),
            ("PLARGE_INTEGER", "Timeout"),
            ("BOOLEAN", "Alertable"),
        ],
    });

    map.insert("NtQueryInstallUILanguage", FunctionSignature {
        name: "NtQueryInstallUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "InstallUILanguageId"),
        ],
    });

    map.insert("NtQueryOpenSubKeys", FunctionSignature {
        name: "NtQueryOpenSubKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PULONG", "SubKeyCount"),
        ],
    });

    map.insert("NtQueryLicenseValue", FunctionSignature {
        name: "NtQueryLicenseValue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "ValueName"),
            ("PULONG", "Type"),
            ("PVOID", "Data"),
            ("ULONG", "DataSize"),
            ("PULONG", "ResultDataSize"),
        ],
    });

    map.insert("ZwQueryInformationByName", FunctionSignature {
        name: "ZwQueryInformationByName",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
        ],
    });

    map.insert("ZwEnableLastKnownGood", FunctionSignature {
        name: "ZwEnableLastKnownGood",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwSetInformationKey", FunctionSignature {
        name: "ZwSetInformationKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("KEY_SET_INFORMATION_CLASS", "KeySetInformationClass"),
            ("PVOID", "KeySetInformation"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("NtAlpcRevokeSecurityContext", FunctionSignature {
        name: "NtAlpcRevokeSecurityContext",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "Context"),
        ],
    });

    map.insert("ZwReplaceKey", FunctionSignature {
        name: "ZwReplaceKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "FileHandle"),
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwUnloadKeyEx", FunctionSignature {
        name: "ZwUnloadKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwOpenKeyEx", FunctionSignature {
        name: "ZwOpenKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "OpenOptions"),
        ],
    });

    map.insert("ZwAlpcOpenSenderProcess", FunctionSignature {
        name: "ZwAlpcOpenSenderProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtGetCachedSigningLevel", FunctionSignature {
        name: "NtGetCachedSigningLevel",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PULONG", "Flags"),
            ("PULONG", "SigningLevel"),
            ("PUCHAR", "Thumbprint"),
            ("PULONG", "ThumbprintSize"),
            ("PULONG", "SectionFlags"),
        ],
    });

    map.insert("NtCallbackReturn", FunctionSignature {
        name: "NtCallbackReturn",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Result"),
            ("ULONG", "ResultLength"),
            ("NTSTATUS", "Status"),
        ],
    });

    map.insert("ZwFreeUserPhysicalPages", FunctionSignature {
        name: "ZwFreeUserPhysicalPages",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("ZwNotifyChangeSession", FunctionSignature {
        name: "ZwNotifyChangeSession",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SessionHandle"),
            ("HANDLE", "EventHandle"),
            ("PVOID", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("PVOID", "ChangeBuffer"),
            ("ULONG", "BufferSize"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("ZwSerializeBoot", FunctionSignature {
        name: "ZwSerializeBoot",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "SerializeBootValue"),
        ],
    });

    map.insert("ZwQueryInformationProcess", FunctionSignature {
        name: "ZwQueryInformationProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PROCESSINFOCLASS", "ProcessInformationClass"),
            ("PVOID", "ProcessInformation"),
            ("ULONG", "ProcessInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwStartProfile", FunctionSignature {
        name: "ZwStartProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProfileObject"),
            ("KPROFILE_SOURCE", "ProfileSource"),
        ],
    });

    map.insert("NtGetNotificationResourceManager", FunctionSignature {
        name: "NtGetNotificationResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("PVOID", "TransactionNotification"),
            ("ULONG", "NotificationLength"),
            ("PLARGE_INTEGER", "Timeout"),
            ("PULONG", "ReturnLength"),
            ("ULONG", "Asynchronous"),
            ("ULONG_PTR", "AsynchronousContext"),
        ],
    });

    map.insert("ZwQueryInformationPort", FunctionSignature {
        name: "ZwQueryInformationPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PORT_INFORMATION_CLASS", "PortInformationClass"),
            ("PVOID", "PortInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwQuerySecurityAttributesToken", FunctionSignature {
        name: "ZwQuerySecurityAttributesToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("PUNICODE_STRING", "AttributeName"),
            ("PVOID", "Information"),
            ("ULONG", "Length"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCreateJobSet", FunctionSignature {
        name: "NtCreateJobSet",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "NumJob"),
            ("PJOB_SET_ARRAY", "UserJobSet"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwLockProductActivationKeys", FunctionSignature {
        name: "ZwLockProductActivationKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Unknown"),
        ],
    });

    map.insert("ZwImpersonateAnonymousToken", FunctionSignature {
        name: "ZwImpersonateAnonymousToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
        ],
    });

    map.insert("NtCreateTimer2", FunctionSignature {
        name: "NtCreateTimer2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Attributes"),
        ],
    });

    map.insert("ZwIsSystemResumeAutomatic", FunctionSignature {
        name: "ZwIsSystemResumeAutomatic",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwContinue", FunctionSignature {
        name: "ZwContinue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PCONTEXT", "ContextRecord"),
            ("BOOLEAN", "TestAlert"),
        ],
    });

    map.insert("ZwModifyBootEntry", FunctionSignature {
        name: "ZwModifyBootEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PBOOT_ENTRY", "BootEntry"),
        ],
    });

    map.insert("NtSetBootOptions", FunctionSignature {
        name: "NtSetBootOptions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "BootOptions"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("NtFlushBuffersFileEx", FunctionSignature {
        name: "NtFlushBuffersFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "Parameters"),
            ("ULONG", "ParametersSize"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("NtQueryInformationByName", FunctionSignature {
        name: "NtQueryInformationByName",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
        ],
    });

    map.insert("ZwSinglePhaseReject", FunctionSignature {
        name: "ZwSinglePhaseReject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("NtNotifyChangeDirectoryFile", FunctionSignature {
        name: "NtNotifyChangeDirectoryFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
        ],
    });

    map.insert("ZwReplacePartitionUnit", FunctionSignature {
        name: "ZwReplacePartitionUnit",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PartitionHandle"),
            ("HANDLE", "TargetHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtStartProfile", FunctionSignature {
        name: "NtStartProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProfileObject"),
            ("KPROFILE_SOURCE", "ProfileSource"),
        ],
    });

    map.insert("NtSetInformationDebugObject", FunctionSignature {
        name: "NtSetInformationDebugObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DebugObjectHandle"),
            ("DEBUGOBJECTINFOCLASS", "DebugObjectInformationClass"),
            ("PVOID", "DebugObjectInformation"),
            ("ULONG", "DebugObjectInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtOpenThread", FunctionSignature {
        name: "NtOpenThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PCLIENT_ID", "ClientId"),
        ],
    });

    map.insert("ZwLockRegistryKey", FunctionSignature {
        name: "ZwLockRegistryKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("NtSetInformationSymbolicLink", FunctionSignature {
        name: "NtSetInformationSymbolicLink",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "LinkHandle"),
            ("SYMBOLIC_LINK_INFORMATION_CLASS", "LinkInformationClass"),
            ("PVOID", "LinkInformation"),
            ("ULONG", "LinkInformationLength"),
        ],
    });

    map.insert("NtSetSystemEnvironmentValue", FunctionSignature {
        name: "NtSetSystemEnvironmentValue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PUNICODE_STRING", "VariableValue"),
        ],
    });

    map.insert("NtAddDriverEntry", FunctionSignature {
        name: "NtAddDriverEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PEFI_DRIVER_ENTRY", "DriverEntry"),
            ("PULONG", "Id")
        ],
    });

    map.insert("ZwAllocateReserveObject", FunctionSignature {
        name: "ZwAllocateReserveObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ReserveObject"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Type"),
        ],
    });

    map.insert("ZwTerminateEnclave", FunctionSignature {
        name: "ZwTerminateEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
        ],
    });

    map.insert("NtQueryTimerResolution", FunctionSignature {
        name: "NtQueryTimerResolution",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "MinimumResolution"),
            ("PULONG", "MaximumResolution"),
            ("PULONG", "CurrentResolution"),
        ],
    });

    map.insert("NtWaitForKeyedEvent", FunctionSignature {
        name: "NtWaitForKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyedEventHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "Key"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwAlpcImpersonateClientOfPort", FunctionSignature {
        name: "ZwAlpcImpersonateClientOfPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwQueryDirectoryObject", FunctionSignature {
        name: "ZwQueryDirectoryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DirectoryHandle"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferLength"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("BOOLEAN", "RestartScan"),
            ("PULONG", "Context"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtReplaceKey", FunctionSignature {
        name: "NtReplaceKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "NewFile"),
            ("HANDLE", "KeyHandle"),
            ("POBJECT_ATTRIBUTES", "OldFile"),
        ],
    });

    map.insert("ZwRollbackEnlistment", FunctionSignature {
        name: "ZwRollbackEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwSetInformationProcess", FunctionSignature {
        name: "ZwSetInformationProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PROCESSINFOCLASS", "ProcessInformationClass"),
            ("PVOID", "ProcessInformation"),
            ("ULONG", "ProcessInformationLength"),
        ],
    });

    map.insert("NtQueryInformationWorkerFactory", FunctionSignature {
        name: "NtQueryInformationWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("WORKERFACTORYINFOCLASS", "WorkerFactoryInformationClass"),
            ("PVOID", "WorkerFactoryInformation"),
            ("ULONG", "WorkerFactoryInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtAlertThread", FunctionSignature {
        name: "NtAlertThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
        ],
    });

    map.insert("NtWriteRequestData", FunctionSignature {
        name: "NtWriteRequestData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "DataIndex"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "OutputBufferSize"),
        ],
    });

    map.insert("NtGetWriteWatch", FunctionSignature {
        name: "NtGetWriteWatch",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "RegionSize"),
            ("PVOID", "UserAddressArray"),
            ("PULONG", "EntriesCount"),
            ("PULONG", "Granularity"),
        ],
    });

    map.insert("ZwAlertThreadByThreadIdEx", FunctionSignature {
        name: "ZwAlertThreadByThreadIdEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadId"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtCreateRegistryTransaction", FunctionSignature {
        name: "NtCreateRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtRequestPort", FunctionSignature {
        name: "NtRequestPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("NtSetDriverEntryOrder", FunctionSignature {
        name: "NtSetDriverEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "LoadOrder"),
            ("ULONG", "Number"),
        ],
    });

    map.insert("NtTranslateFilePath", FunctionSignature {
        name: "NtTranslateFilePath",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Operation"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferLength"),
        ],
    });

    map.insert("ZwQueryDirectoryFile", FunctionSignature {
        name: "ZwQueryDirectoryFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PUNICODE_STRING", "FileName"),
            ("BOOLEAN", "RestartScan"),
        ],
    });

    map.insert("ZwSaveKey", FunctionSignature {
        name: "ZwSaveKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "FileHandle"),
        ],
    });

    map.insert("NtExtendSection", FunctionSignature {
        name: "NtExtendSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("PLARGE_INTEGER", "SectionSize"),
        ],
    });

    map.insert("NtDebugActiveProcess", FunctionSignature {
        name: "NtDebugActiveProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "DebugObjectHandle"),
        ],
    });

    map.insert("ZwSetWnfProcessNotificationEvent", FunctionSignature {
        name: "ZwSetWnfProcessNotificationEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
        ],
    });

    map.insert("ZwCreateProcessEx", FunctionSignature {
        name: "ZwCreateProcessEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ParentProcess"),
            ("ULONG", "Flags"),
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "DebugPort"),
            ("HANDLE", "ExceptionPort"),
            ("ULONG", "JobMemberLevel"),
        ],
    });

    map.insert("NtCompareTokens", FunctionSignature {
        name: "NtCompareTokens",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FirstTokenHandle"),
            ("HANDLE", "SecondTokenHandle"),
            ("PBOOLEAN", "Equal"),
        ],
    });

    map.insert("ZwQuerySemaphore", FunctionSignature {
        name: "ZwQuerySemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SemaphoreHandle"),
            ("SEMAPHORE_INFORMATION_CLASS", "SemaphoreInformationClass"),
            ("PVOID", "SemaphoreInformation"),
            ("ULONG", "SemaphoreInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtOpenResourceManager", FunctionSignature {
        name: "NtOpenResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ResourceManagerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "TmHandle"),
            ("LUID", "ResourceManagerId"),
        ],
    });

    map.insert("NtQueryIoRingCapabilities", FunctionSignature {
        name: "NtQueryIoRingCapabilities",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SIZE_T", "IoRingCapabilitiesLength"),
            ("PVOID", "IoRingCapabilities"),
        ],
    });

    map.insert("ZwSetSystemInformation", FunctionSignature {
        name: "ZwSetSystemInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SYSTEM_INFORMATION_CLASS", "SystemInformationClass"),
            ("PVOID", "SystemInformation"),
            ("ULONG", "SystemInformationLength"),
        ],
    });

    map.insert("ZwRollbackRegistryTransaction", FunctionSignature {
        name: "ZwRollbackRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwQueueApcThread", FunctionSignature {
        name: "ZwQueueApcThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcArgument1"),
            ("PVOID", "ApcArgument2"),
            ("PVOID", "ApcArgument3"),
        ],
    });

    map.insert("ZwQuerySecurityObject", FunctionSignature {
        name: "ZwQuerySecurityObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("SECURITY_INFORMATION", "SecurityInformation"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("ULONG", "Length"),
            ("PULONG", "LengthNeeded"),
        ],
    });

    map.insert("ZwUnlockFile", FunctionSignature {
        name: "ZwUnlockFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PLARGE_INTEGER", "Length"),
            ("ULONG", "Key"),
        ],
    });

    map.insert("ZwDisableLastKnownGood", FunctionSignature {
        name: "ZwDisableLastKnownGood",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwThawRegistry", FunctionSignature {
        name: "ZwThawRegistry",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtFlushVirtualMemory", FunctionSignature {
        name: "NtFlushVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("NtLockVirtualMemory", FunctionSignature {
        name: "NtLockVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "MapType"),
        ],
    });

    map.insert("ZwQueryOpenSubKeysEx", FunctionSignature {
        name: "ZwQueryOpenSubKeysEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Format"),
            ("PVOID", "Info"),
            ("ULONG", "InfoLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtDuplicateToken", FunctionSignature {
        name: "NtDuplicateToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ExistingTokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("BOOLEAN", "EffectiveOnly"),
            ("TOKEN_TYPE", "TokenType"),
            ("PHANDLE", "NewTokenHandle"),
        ],
    });

    map.insert("NtCompareSigningLevels", FunctionSignature {
        name: "NtCompareSigningLevels",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "LeftSigningLevel"),
            ("ULONG", "RightSigningLevel"),
        ],
    });

    map.insert("ZwOpenCpuPartition", FunctionSignature {
        name: "ZwOpenCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "CpuPartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwQueryDefaultLocale", FunctionSignature {
        name: "ZwQueryDefaultLocale",
        return_type: "NTSTATUS",
        parameters: vec![
            ("BOOLEAN", "UserProfile"),
            ("PLCID", "DefaultLocaleId"),
        ],
    });

    map.insert("NtSetIoCompletionEx", FunctionSignature {
        name: "NtSetIoCompletionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("HANDLE", "FileHandle"),
            ("PVOID", "KeyContext"),
            ("PVOID", "ApcContext"),
            ("NTSTATUS", "IoStatus"),
            ("ULONG_PTR", "Information"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwWriteFileGather", FunctionSignature {
        name: "ZwWriteFileGather",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PFILE_SEGMENT_ELEMENT", "SegmentArray"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });

    map.insert("NtUpdateWnfStateData", FunctionSignature {
        name: "NtUpdateWnfStateData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("PVOID", "ExplicitScope"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtDeleteFile", FunctionSignature {
        name: "NtDeleteFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwQueryDefaultUILanguage", FunctionSignature {
        name: "ZwQueryDefaultUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "DefaultUILanguageId"),
        ],
    });

    map.insert("ZwLoadEnclaveData", FunctionSignature {
        name: "ZwLoadEnclaveData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PVOID", "LoadInformation"),
            ("ULONG", "InformationLength"),
            ("PULONG", "EnclaveInformation"),
        ],
    });

    map.insert("NtPlugPlayControl", FunctionSignature {
        name: "NtPlugPlayControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwSetInformationCpuPartition", FunctionSignature {
        name: "ZwSetInformationCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "CpuPartitionHandle"),
            ("CPU_PARTITION_INFORMATION_CLASS", "CpuPartitionInformationClass"),
            ("PVOID", "CpuPartitionInformation"),
            ("ULONG", "CpuPartitionInformationLength"),
        ],
    });

    map.insert("ZwGetNextProcess", FunctionSignature {
        name: "ZwGetNextProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("ULONG", "Flags"),
            ("PHANDLE", "NextProcessHandle"),
        ],
    });

    map.insert("ZwSetIoCompletionEx", FunctionSignature {
        name: "ZwSetIoCompletionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("HANDLE", "FileHandle"),
            ("PVOID", "KeyContext"),
            ("PVOID", "ApcContext"),
            ("NTSTATUS", "IoStatus"),
            ("ULONG_PTR", "Information"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwOpenThreadToken", FunctionSignature {
        name: "ZwOpenThreadToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("BOOLEAN", "OpenAsSelf"),
            ("PHANDLE", "TokenHandle"),
        ],
    });
    
    map.insert("NtCreatePagingFile", FunctionSignature {
        name: "NtCreatePagingFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "PageFileName"),
            ("PLARGE_INTEGER", "MinimumSize"),
            ("PLARGE_INTEGER", "MaximumSize"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwFlushProcessWriteBuffers", FunctionSignature {
        name: "ZwFlushProcessWriteBuffers",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtQueueApcThreadEx", FunctionSignature {
        name: "NtQueueApcThreadEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("HANDLE", "Reserved"),
            ("PKNORMAL_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcArgument1"),
            ("PVOID", "ApcArgument2"),
            ("PVOID", "ApcArgument3"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtGetNextProcess", FunctionSignature {
        name: "NtGetNextProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("ULONG", "Flags"),
            ("PHANDLE", "NewProcessHandle"),
        ],
    });

    map.insert("ZwFlushBuffersFile", FunctionSignature {
        name: "ZwFlushBuffersFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("ZwAlpcCancelMessage", FunctionSignature {
        name: "ZwAlpcCancelMessage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("ZwOpenProcessTokenEx", FunctionSignature {
        name: "ZwOpenProcessTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("PHANDLE", "TokenHandle"),
        ],
    });

    map.insert("NtQueryInformationCpuPartition", FunctionSignature {
        name: "NtQueryInformationCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "CpuPartitionHandle"),
            ("u32", "CpuPartitionInformationClass"),
            ("PVOID", "CpuPartitionInformation"),
            ("ULONG", "CpuPartitionInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwAlpcOpenSenderThread", FunctionSignature {
        name: "ZwAlpcOpenSenderThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQueryKey", FunctionSignature {
        name: "NtQueryKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("KEY_INFORMATION_CLASS", "KeyInformationClass"),
            ("PVOID", "KeyInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("NtAlpcOpenSenderThread", FunctionSignature {
        name: "NtAlpcOpenSenderThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtCreateWorkerFactory", FunctionSignature {
        name: "NtCreateWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "WorkerFactoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "CompletionPortHandle"),
            ("HANDLE", "WorkerProcessHandle"),
            ("PVOID", "StartRoutine"),
            ("PVOID", "StartParameter"),
            ("ULONG", "MaxThreadCount"),
            ("SIZE_T", "StackReserve"),
            ("SIZE_T", "StackCommit"),
        ],
    });

    map.insert("ZwDebugActiveProcess", FunctionSignature {
        name: "ZwDebugActiveProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "DebugObjectHandle"),
        ],
    });

    map.insert("NtSetDebugFilterState", FunctionSignature {
        name: "NtSetDebugFilterState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ComponentId"),
            ("ULONG", "Level"),
            ("BOOLEAN", "State"),
        ],
    });

    map.insert("NtWaitForAlertByThreadId", FunctionSignature {
        name: "NtWaitForAlertByThreadId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadId"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwWriteFile", FunctionSignature {
        name: "ZwWriteFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });

    map.insert("NtSetCachedSigningLevel", FunctionSignature {
        name: "NtSetCachedSigningLevel",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "SigningLevel"),
            ("PUCHAR", "Thumbprint"),
            ("ULONG", "ThumbprintSize"),
            ("ULONG", "SectionFlags"),
        ],
    });

    map.insert("ZwProtectVirtualMemory", FunctionSignature {
        name: "ZwProtectVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "NewProtect"),
            ("PULONG", "OldProtect"),
        ],
    });

    map.insert("ZwReadOnlyEnlistment", FunctionSignature {
        name: "ZwReadOnlyEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwAlpcCreateResourceReserve", FunctionSignature {
        name: "ZwAlpcCreateResourceReserve",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "MessageSize"),
            ("PULONG", "ActualMessageSize"),
        ],
    });

    map.insert("NtQuerySymbolicLinkObject", FunctionSignature {
        name: "NtQuerySymbolicLinkObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "LinkHandle"),
            ("PUNICODE_STRING", "LinkTarget"),
            ("PULONG", "ReturnedLength"),
        ],
    });

    map.insert("ZwRestoreKey", FunctionSignature {
        name: "ZwRestoreKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwUpdateWnfStateData", FunctionSignature {
        name: "ZwUpdateWnfStateData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("PVOID", "ExplicitScope"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtEnableLastKnownGood", FunctionSignature {
        name: "NtEnableLastKnownGood",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtImpersonateAnonymousToken", FunctionSignature {
        name: "NtImpersonateAnonymousToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
        ],
    });

    map.insert("NtRemoveIoCompletion", FunctionSignature {
        name: "NtRemoveIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("PVOID", "KeyContext"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwOpenMutant", FunctionSignature {
        name: "ZwOpenMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "MutantHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtMakeTemporaryObject", FunctionSignature {
        name: "NtMakeTemporaryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
        ],
    });

    map.insert("NtRenameTransactionManager", FunctionSignature {
        name: "NtRenameTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "LogFileName"),
            ("PGUID", "TmIdentity"),
        ],
    });

    map.insert("ZwSetEventBoostPriority", FunctionSignature {
        name: "ZwSetEventBoostPriority",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
        ],
    });

    map.insert("ZwUnlockVirtualMemory", FunctionSignature {
        name: "ZwUnlockVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "MapType"),
        ],
    });

    map.insert("NtReleaseKeyedEvent", FunctionSignature {
        name: "NtReleaseKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyedEventHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "Key"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwSetSystemEnvironmentValue", FunctionSignature {
        name: "ZwSetSystemEnvironmentValue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PUNICODE_STRING", "VariableValue"),
        ],
    });

    map.insert("ZwSetSecurityObject", FunctionSignature {
        name: "ZwSetSecurityObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("SECURITY_INFORMATION", "SecurityInformation"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
        ],
    });

    map.insert("NtQuerySemaphore", FunctionSignature {
        name: "NtQuerySemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SemaphoreHandle"),
            ("u32", "SemaphoreInformationClass"),
            ("PVOID", "SemaphoreInformation"),
            ("ULONG", "SemaphoreInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtDeleteObjectAuditAlarm", FunctionSignature {
        name: "NtDeleteObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwInitializeNlsFiles", FunctionSignature {
        name: "ZwInitializeNlsFiles",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "BaseAddress"),
            ("PLCID", "DefaultLocaleId"),
            ("PLARGE_INTEGER", "DefaultCasingTableSize"),
        ],
    });

    map.insert("ZwPrePrepareEnlistment", FunctionSignature {
        name: "ZwPrePrepareEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("ZwAdjustPrivilegesToken", FunctionSignature {
        name: "ZwAdjustPrivilegesToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("BOOLEAN", "DisableAllPrivileges"),
            ("PTOKEN_PRIVILEGES", "NewState"),
            ("ULONG", "BufferLength"),
            ("PTOKEN_PRIVILEGES", "PreviousState"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCreateProfile", FunctionSignature {
        name: "NtCreateProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProfileHandle"),
            ("HANDLE", "Process"),
            ("PVOID", "RangeBase"),
            ("SIZE_T", "RangeSize"),
            ("ULONG", "BucketSize"),
            ("PULONG", "ProfileInfo"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwOpenProcess", FunctionSignature {
        name: "ZwOpenProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PCLIENT_ID", "ClientId"),
        ],
    });

    map.insert("ZwFreeVirtualMemory", FunctionSignature {
        name: "ZwFreeVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "FreeType"),
        ],
    });

    map.insert("ZwReadFileScatter", FunctionSignature {
        name: "ZwReadFileScatter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PFILE_SEGMENT_ELEMENT", "SegmentArray"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });

    map.insert("NtManageHotPatch", FunctionSignature {
        name: "NtManageHotPatch",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Operation"),
            ("PVOID", "Parameters"),
            ("ULONG", "ParametersSize"),
        ],
    });

    map.insert("NtCreateIoRing", FunctionSignature {
        name: "NtCreateIoRing",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "IoRingHandle"),
            ("ULONG", "Version"),
            ("ULONG", "SubmissionQueueSize"),
            ("ULONG", "CompletionQueueSize"),
            ("ULONG", "Flags"),
            ("PVOID", "SubmissionQueue"),
            ("PVOID", "CompletionQueue"),
        ],
    });

    map.insert("ZwPulseEvent", FunctionSignature {
        name: "ZwPulseEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("LPLONG", "PreviousState"),
        ],
    });

    map.insert("ZwFreezeTransactions", FunctionSignature {
        name: "ZwFreezeTransactions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwRemoveProcessDebug", FunctionSignature {
        name: "ZwRemoveProcessDebug",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "DebugObjectHandle"),
        ],
    });

    map.insert("ZwFlushVirtualMemory", FunctionSignature {
        name: "ZwFlushVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("NtAddBootEntry", FunctionSignature {
        name: "NtAddBootEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PBOOT_ENTRY", "BootEntry"),
        ],
    });

    map.insert("ZwResetWriteWatch", FunctionSignature {
        name: "ZwResetWriteWatch",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "RegionSize"),
        ],
    });

    map.insert("ZwSetTimer2", FunctionSignature {
        name: "ZwSetTimer2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PLARGE_INTEGER", "Period"),
            ("PTIMER_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtGetCurrentProcessorNumberEx", FunctionSignature {
        name: "NtGetCurrentProcessorNumberEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PPROCESSOR_NUMBER", "ProcNumber"),
        ],
    });

    map.insert("NtSubmitIoRing", FunctionSignature {
        name: "NtSubmitIoRing",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoRingHandle"),
            ("ULONG", "SubmissionCount"),
            ("ULONG", "CompletionCount"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwWaitForKeyedEvent", FunctionSignature {
        name: "ZwWaitForKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyedEventHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "Key"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwSetBootOptions", FunctionSignature {
        name: "ZwSetBootOptions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "BootOptions"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("ZwOpenIoCompletion", FunctionSignature {
        name: "ZwOpenIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "IoCompletionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwCompareSigningLevels", FunctionSignature {
        name: "ZwCompareSigningLevels",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "LeftSigningLevel"),
            ("ULONG", "RightSigningLevel"),
        ],
    });

    map.insert("ZwSetQuotaInformationFile", FunctionSignature {
        name: "ZwSetQuotaInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("ZwMakeTemporaryObject", FunctionSignature {
        name: "ZwMakeTemporaryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
        ],
    });

    map.insert("ZwCreateEnlistment", FunctionSignature {
        name: "ZwCreateEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EnlistmentHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ResourceManagerHandle"),
            ("HANDLE", "TransactionHandle"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "NotificationMask"),
            ("PVOID", "EnlistmentKey"),
        ],
    });

    map.insert("NtDeviceIoControlFile", FunctionSignature {
        name: "NtDeviceIoControlFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "IoControlCode"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
        ],
    });

    map.insert("NtPropagationFailed", FunctionSignature {
        name: "NtPropagationFailed",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("ULONG", "PropagationId"),
            ("PULONG", "PropagationCount"),
        ],
    });

    map.insert("NtQueryEaFile", FunctionSignature {
        name: "NtQueryEaFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PVOID", "EaList"),
            ("ULONG", "EaListLength"),
            ("PULONG", "EaIndex"),
            ("BOOLEAN", "RestartScan"),
        ],
    });

    map.insert("NtCreateJobObject", FunctionSignature {
        name: "NtCreateJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "JobHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtCallEnclave", FunctionSignature {
        name: "NtCallEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Routine"),
            ("PVOID", "Parameter"),
            ("BOOLEAN", "WaitForThread"),
            ("PVOID", "Result"),
        ],
    });

    map.insert("ZwAlertThreadByThreadId", FunctionSignature {
        name: "ZwAlertThreadByThreadId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadId"),
        ],
    });

    map.insert("NtRemoveProcessDebug", FunctionSignature {
        name: "NtRemoveProcessDebug",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "DebugObjectHandle"),
        ],
    });

    map.insert("NtWaitForWorkViaWorkerFactory", FunctionSignature {
        name: "NtWaitForWorkViaWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("PULONG", "WorkerCount"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "WorkerParameter"),
        ],
    });

    map.insert("ZwCreateEvent", FunctionSignature {
        name: "ZwCreateEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("EVENT_TYPE", "EventType"),
            ("BOOLEAN", "InitialState"),
        ],
    });

    map.insert("ZwImpersonateClientOfPort", FunctionSignature {
        name: "ZwImpersonateClientOfPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("NtOpenEventPair", FunctionSignature {
        name: "NtOpenEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventPairHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwUnmapViewOfSectionEx", FunctionSignature {
        name: "ZwUnmapViewOfSectionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwOpenPartition", FunctionSignature {
        name: "ZwOpenPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtCompleteConnectPort", FunctionSignature {
        name: "NtCompleteConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("NtOpenIoCompletion", FunctionSignature {
        name: "NtOpenIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "IoCompletionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwPrivilegeObjectAuditAlarm", FunctionSignature {
        name: "ZwPrivilegeObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("HANDLE", "TokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("BOOLEAN", "ObjectCreation"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("NtAlpcAcceptConnectPort", FunctionSignature {
        name: "NtAlpcAcceptConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("HANDLE", "ConnectionPortHandle"),
            ("ULONG", "Flags"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
            ("PVOID", "PortContext"),
            ("PVOID", "ConnectionContext"),
            ("PULONG", "ServerViewSize"),
            ("PVOID", "ConnectionView"),
        ],
    });

    map.insert("NtMapViewOfSectionEx", FunctionSignature {
        name: "NtMapViewOfSectionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG_PTR", "ZeroBits"),
            ("SIZE_T", "CommitSize"),
            ("PLARGE_INTEGER", "SectionOffset"),
            ("PSIZE_T", "ViewSize"),
            ("ULONG", "InheritDisposition"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Win32Protect"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtOpenDirectoryObject", FunctionSignature {
        name: "NtOpenDirectoryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DirectoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtSetInformationTransactionManager", FunctionSignature {
        name: "NtSetInformationTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
            ("u32", "TransactionManagerInformationClass"),
            ("PVOID", "TransactionManagerInformation"),
            ("ULONG", "TransactionManagerInformationLength"),
        ],
    });

    map.insert("NtSetTimer2", FunctionSignature {
        name: "NtSetTimer2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PLARGE_INTEGER", "Period"),
            ("PTIMER_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwQueryBootEntryOrder", FunctionSignature {
        name: "ZwQueryBootEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "BootEntries"),
            ("PULONG", "BootCount"),
        ],
    });

    map.insert("NtQueryInformationEnlistment", FunctionSignature {
        name: "NtQueryInformationEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("u32", "EnlistmentInformationClass"),
            ("PVOID", "EnlistmentInformation"),
            ("ULONG", "EnlistmentInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwFlushInstructionCache", FunctionSignature {
        name: "ZwFlushInstructionCache",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "RegionSize"),
        ],
    });

    map.insert("ZwGetWriteWatch", FunctionSignature {
        name: "ZwGetWriteWatch",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "RegionSize"),
            ("PVOID", "UserAddressArray"),
            ("PULONG", "EntriesCount"),
            ("PULONG", "Granularity"),
        ],
    });

    map.insert("ZwQueryPortInformationProcess", FunctionSignature {
        name: "ZwQueryPortInformationProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "PortFlags"),
        ],
    });

    map.insert("NtCommitComplete", FunctionSignature {
        name: "NtCommitComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwCreateSection", FunctionSignature {
        name: "ZwCreateSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SectionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PLARGE_INTEGER", "MaximumSize"),
            ("ULONG", "SectionPageProtection"),
            ("ULONG", "AllocationAttributes"),
            ("HANDLE", "FileHandle"),
        ],
    });

    map.insert("NtAllocateUserPhysicalPages", FunctionSignature {
        name: "NtAllocateUserPhysicalPages",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("ZwQueryBootOptions", FunctionSignature {
        name: "ZwQueryBootOptions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "BootOptionsId"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("ZwAssignProcessToJobObject", FunctionSignature {
        name: "ZwAssignProcessToJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("HANDLE", "ProcessHandle"),
        ],
    });

    map.insert("ZwCreateJobObject", FunctionSignature {
        name: "ZwCreateJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "JobHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtIsSystemResumeAutomatic", FunctionSignature {
        name: "NtIsSystemResumeAutomatic",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtFlushInstructionCache", FunctionSignature {
        name: "NtFlushInstructionCache",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "RegionSize"),
        ],
    });

    map.insert("ZwAllocateVirtualMemory", FunctionSignature {
        name: "ZwAllocateVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG_PTR", "ZeroBits"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Protect"),
        ],
    });

    map.insert("ZwSecureConnectPort", FunctionSignature {
        name: "ZwSecureConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
            ("PVOID", "ClientView"),
            ("PSID", "RequiredServerSid"),
            ("PVOID", "ConnectData"),
            ("PULONG", "DataLength"),
        ],
    });

    map.insert("ZwRollforwardTransactionManager", FunctionSignature {
        name: "ZwRollforwardTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("ZwGetContextThread", FunctionSignature {
        name: "ZwGetContextThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PCONTEXT", "Context"),
        ],
    });

    map.insert("ZwOpenDirectoryObject", FunctionSignature {
        name: "ZwOpenDirectoryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DirectoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtPrePrepareEnlistment", FunctionSignature {
        name: "NtPrePrepareEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("NtCreateIRTimer", FunctionSignature {
        name: "NtCreateIRTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwCreateTokenEx", FunctionSignature {
        name: "ZwCreateTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("TOKEN_TYPE", "Type"),
            ("PLUID", "AuthenticationId"),
            ("PLARGE_INTEGER", "ExpirationTime"),
            ("PTOKEN_USER", "User"),
            ("PTOKEN_GROUPS", "Groups"),
            ("PTOKEN_PRIVILEGES", "Privileges"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "UserAttributes"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "DeviceAttributes"),
            ("PTOKEN_GROUPS", "DeviceGroups"),
            ("PTOKEN_MANDATORY_POLICY", "MandatoryPolicy"),
            ("PTOKEN_OWNER", "Owner"),
            ("PTOKEN_PRIMARY_GROUP", "PrimaryGroup"),
            ("PTOKEN_DEFAULT_DACL", "DefaultDacl"),
            ("PTOKEN_SOURCE", "Source"),
        ],
    });

    map.insert("NtDeletePrivateNamespace", FunctionSignature {
        name: "NtDeletePrivateNamespace",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "NamespaceHandle"),
        ],
    });

    map.insert("NtFlushProcessWriteBuffers", FunctionSignature {
        name: "NtFlushProcessWriteBuffers",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtQuerySecurityObject", FunctionSignature {
        name: "NtQuerySecurityObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("SECURITY_INFORMATION", "SecurityInformation"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("ULONG", "Length"),
            ("PULONG", "LengthNeeded"),
        ],
    });

    map.insert("ZwAreMappedFilesTheSame", FunctionSignature {
        name: "ZwAreMappedFilesTheSame",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "File1MappedAsAnImage"),
            ("PVOID", "File2MappedAsFile"),
        ],
    });

    map.insert("ZwAlpcAcceptConnectPort", FunctionSignature {
        name: "ZwAlpcAcceptConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("HANDLE", "ConnectionPortHandle"),
            ("ULONG", "Flags"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
            ("PVOID", "PortContext"),
            ("PVOID", "ConnectionContext"),
            ("PULONG", "ServerViewSize"),
            ("PVOID", "ConnectionView"),
        ],
    });

    map.insert("ZwQuerySymbolicLinkObject", FunctionSignature {
        name: "ZwQuerySymbolicLinkObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "LinkHandle"),
            ("PUNICODE_STRING", "LinkTarget"),
            ("PULONG", "ReturnedLength"),
        ],
    });

    map.insert("NtAlertResumeThread", FunctionSignature {
        name: "NtAlertResumeThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });

    map.insert("ZwRaiseException", FunctionSignature {
        name: "ZwRaiseException",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PEXCEPTION_RECORD", "ExceptionRecord"),
            ("PCONTEXT", "ContextRecord"),
            ("BOOLEAN", "FirstChance"),
        ],
    });

    map.insert("NtRegisterProtocolAddressInformation", FunctionSignature {
        name: "NtRegisterProtocolAddressInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("PUNICODE_STRING", "ProtocolName"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "Persistence"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwAlpcQueryInformationMessage", FunctionSignature {
        name: "ZwAlpcQueryInformationMessage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ALPC_MESSAGE_INFORMATION_CLASS", "MessageInformationClass"),
            ("PVOID", "MessageInformation"),
            ("ULONG", "MessageInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtNotifyChangeDirectoryFileEx", FunctionSignature {
        name: "NtNotifyChangeDirectoryFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtInitializeNlsFiles", FunctionSignature {
        name: "NtInitializeNlsFiles",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "BaseAddress"),
            ("PLCID", "DefaultLocaleId"),
            ("PLARGE_INTEGER", "DefaultCasingTableSize"),
        ],
    });

    map.insert("ZwCreateJobSet", FunctionSignature {
        name: "ZwCreateJobSet",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "NumJob"),
            ("PJOB_SET_ARRAY", "UserJobSet"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQuerySecurityAttributesToken", FunctionSignature {
        name: "NtQuerySecurityAttributesToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("PUNICODE_STRING", "AttributeName"),
            ("PVOID", "Information"),
            ("ULONG", "Length"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwOpenKey", FunctionSignature {
        name: "ZwOpenKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwReplyPort", FunctionSignature {
        name: "ZwReplyPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("NtRecoverResourceManager", FunctionSignature {
        name: "NtRecoverResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
        ],
    });

    map.insert("NtWriteFileGather", FunctionSignature {
        name: "NtWriteFileGather",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PFILE_SEGMENT_ELEMENT", "SegmentArray"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });

    map.insert("NtModifyBootEntry", FunctionSignature {
        name: "NtModifyBootEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PBOOT_ENTRY", "BootEntry"),
        ],
    });

    map.insert("NtAssociateWaitCompletionPacket", FunctionSignature {
        name: "NtAssociateWaitCompletionPacket",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WaitCompletionPacketHandle"),
            ("HANDLE", "CompletionPacketHandle"),
            ("HANDLE", "WaitObject"),
            ("ULONG", "Flags"),
            ("ULONG_PTR", "KeyContext"),
            ("PVOID", "ApcContext"),
        ],
    });

    map.insert("ZwCreateNamedPipeFile", FunctionSignature {
        name: "ZwCreateNamedPipeFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "ShareAccess"),
            ("ULONG", "CreateDisposition"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "NamedPipeType"),
            ("ULONG", "ReadMode"),
            ("ULONG", "CompletionMode"),
            ("ULONG", "MaximumInstances"),
            ("ULONG", "InboundQuota"),
            ("ULONG", "OutboundQuota"),
            ("PLARGE_INTEGER", "DefaultTimeout"),
        ],
    });

    map.insert("ZwReleaseWorkerFactoryWorker", FunctionSignature {
        name: "ZwReleaseWorkerFactoryWorker",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
        ],
    });

    map.insert("NtSetLowEventPair", FunctionSignature {
        name: "NtSetLowEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("NtSetInformationKey", FunctionSignature {
        name: "NtSetInformationKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("KEY_SET_INFORMATION_CLASS", "KeySetInformationClass"),
            ("PVOID", "KeySetInformation"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("ZwGetDevicePowerState", FunctionSignature {
        name: "ZwGetDevicePowerState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DeviceHandle"),
            ("PULONG", "State"),
        ],
    });

    map.insert("NtReplyPort", FunctionSignature {
        name: "NtReplyPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("ZwVdmControl", FunctionSignature {
        name: "ZwVdmControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "ControlData"),
        ],
    });

    map.insert("NtCreateTransactionManager", FunctionSignature {
        name: "NtCreateTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "LogFileName"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "CommitStrength"),
        ],
    });

    map.insert("NtRollbackEnlistment", FunctionSignature {
        name: "NtRollbackEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwAdjustGroupsToken", FunctionSignature {
        name: "ZwAdjustGroupsToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("BOOLEAN", "ResetToDefault"),
            ("PTOKEN_GROUPS", "NewState"),
            ("ULONG", "BufferLength"),
            ("PTOKEN_GROUPS", "PreviousState"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtWaitLowEventPair", FunctionSignature {
        name: "NtWaitLowEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtSetSecurityObject", FunctionSignature {
        name: "NtSetSecurityObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("SECURITY_INFORMATION", "SecurityInformation"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
        ],
    });

    map.insert("ZwSetHighWaitLowEventPair", FunctionSignature {
        name: "ZwSetHighWaitLowEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("NtCreateEnclave", FunctionSignature {
        name: "NtCreateEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "ZeroBits"),
            ("SIZE_T", "Size"),
            ("ULONG", "EnclaveType"),
            ("PVOID", "EnclaveInformation"),
            ("ULONG", "EnclaveInformationLength"),
            ("PULONG", "EnclaveError"),
        ],
    });

    map.insert("NtPrivilegedServiceAuditAlarm", FunctionSignature {
        name: "NtPrivilegedServiceAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PUNICODE_STRING", "ServiceName"),
            ("HANDLE", "TokenHandle"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("NtFreeUserPhysicalPages", FunctionSignature {
        name: "NtFreeUserPhysicalPages",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("NtVdmControl", FunctionSignature {
        name: "NtVdmControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "ControlData"),
        ],
    });

    map.insert("NtCreateSection", FunctionSignature {
        name: "NtCreateSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SectionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PLARGE_INTEGER", "MaximumSize"),
            ("ULONG", "SectionPageProtection"),
            ("ULONG", "AllocationAttributes"),
            ("HANDLE", "FileHandle"),
        ],
    });

    map.insert("NtAccessCheckByTypeResultList", FunctionSignature {
        name: "NtAccessCheckByTypeResultList",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("PPRIVILEGE_SET", "PrivilegeSet"),
            ("PULONG", "PrivilegeSetLength"),
            ("PULONG", "GrantedAccessList"),
            ("PULONG", "AccessStatusList"),
        ],
    });

    map.insert("ZwFilterToken", FunctionSignature {
        name: "ZwFilterToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ExistingTokenHandle"),
            ("ULONG", "Flags"),
            ("PTOKEN_GROUPS", "SidsToDisable"),
            ("PTOKEN_PRIVILEGES", "PrivilegesToDelete"),
            ("PTOKEN_GROUPS", "RestrictedSids"),
            ("PHANDLE", "NewTokenHandle"),
        ],
    });

    map.insert("NtConnectPort", FunctionSignature {
        name: "NtConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
            ("PVOID", "ClientView"),
            ("PSID", "RequiredServerSid"),
            ("PVOID", "ConnectData"),
            ("PULONG", "DataLength"),
        ],
    });

    map.insert("NtQueryEvent", FunctionSignature {
        name: "NtQueryEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("EVENT_INFORMATION_CLASS", "EventInformationClass"),
            ("PVOID", "EventInformation"),
            ("ULONG", "EventInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwTestAlert", FunctionSignature {
        name: "ZwTestAlert",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtUnlockVirtualMemory", FunctionSignature {
        name: "NtUnlockVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "MapType"),
        ],
    });

    map.insert("ZwModifyDriverEntry", FunctionSignature {
        name: "ZwModifyDriverEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PEFI_DRIVER_ENTRY", "DriverEntry"),
        ],
    });

    map.insert("NtQueryAuxiliaryCounterFrequency", FunctionSignature {
        name: "NtQueryAuxiliaryCounterFrequency",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONGLONG", "Frequency"),
        ],
    });

    map.insert("NtResetWriteWatch", FunctionSignature {
        name: "NtResetWriteWatch",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "RegionSize"),
        ],
    });

    map.insert("NtFlushKey", FunctionSignature {
        name: "NtFlushKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("NtReplyWaitReceivePort", FunctionSignature {
        name: "NtReplyWaitReceivePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ReplyMessage"),
            ("PVOID", "ReceiveMessage"),
        ],
    });

    map.insert("NtSetInformationProcess", FunctionSignature {
        name: "NtSetInformationProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PROCESSINFOCLASS", "ProcessInformationClass"),
            ("PVOID", "ProcessInformation"),
            ("ULONG", "ProcessInformationLength"),
        ],
    });

    map.insert("ZwCreateWorkerFactory", FunctionSignature {
        name: "ZwCreateWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "WorkerFactoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "CompletionPortHandle"),
            ("HANDLE", "WorkerProcessHandle"),
            ("PVOID", "StartRoutine"),
            ("PVOID", "StartParameter"),
            ("ULONG", "MaxThreadCount"),
            ("SIZE_T", "StackReserve"),
            ("SIZE_T", "StackCommit"),
        ],
    });

    map.insert("ZwTranslateFilePath", FunctionSignature {
        name: "ZwTranslateFilePath",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Operation"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferLength"),
        ],
    });

    map.insert("NtLoadDriver", FunctionSignature {
        name: "NtLoadDriver",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "DriverServiceName"),
        ],
    });

    map.insert("NtQueryBootOptions", FunctionSignature {
        name: "NtQueryBootOptions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "BootOptionsId"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("ZwInitiatePowerAction", FunctionSignature {
        name: "ZwInitiatePowerAction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POWER_ACTION", "SystemAction"),
            ("SYSTEM_POWER_STATE", "LightestSystemState"),
            ("ULONG", "Flags"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("NtAlpcConnectPort", FunctionSignature {
        name: "NtAlpcConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
            ("PALPC_HANDLE", "ConnectionPort"),
            ("ULONG", "Flags"),
            ("PSID", "RequiredServerSid"),
            ("PALPC_MESSAGE_ATTRIBUTES", "MessageAttributes"),
            ("PVOID", "ConnectData"),
            ("PULONG", "ConnectDataLength"),
        ],
    });

    map.insert("NtContinueEx", FunctionSignature {
        name: "NtContinueEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PCONTEXT", "ContextRecord"),
            ("BOOLEAN", "TestAlert"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtSetLdtEntries", FunctionSignature {
        name: "NtSetLdtEntries",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Selector1"),
            ("LDT_ENTRY", "LdtEntry1"),
            ("ULONG", "Selector2"),
            ("LDT_ENTRY", "LdtEntry2"),
        ],
    });

    map.insert("ZwCreateKey", FunctionSignature {
        name: "ZwCreateKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "TitleIndex"),
            ("PUNICODE_STRING", "Class"),
            ("ULONG", "CreateOptions"),
            ("PULONG", "Disposition"),
        ],
    });

    map.insert("NtAlpcDeletePortSection", FunctionSignature {
        name: "NtAlpcDeletePortSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "SectionHandle"),
        ],
    });

    map.insert("NtFsControlFile", FunctionSignature {
        name: "NtFsControlFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "FsControlCode"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
        ],
    });

    map.insert("ZwQueryQuotaInformationFile", FunctionSignature {
        name: "ZwQueryQuotaInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PVOID", "SidList"),
            ("ULONG", "SidListLength"),
            ("PULONG", "StartSidIndex"),
            ("BOOLEAN", "RestartScan"),
        ],
    });

    map.insert("ZwCompleteConnectPort", FunctionSignature {
        name: "ZwCompleteConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("ZwYieldExecution", FunctionSignature {
        name: "ZwYieldExecution",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwSetInformationObject", FunctionSignature {
        name: "ZwSetInformationObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("OBJECT_INFORMATION_CLASS", "ObjectInformationClass"),
            ("PVOID", "ObjectInformation"),
            ("ULONG", "ObjectInformationLength"),
        ],
    });

    map.insert("ZwCreateLowBoxToken", FunctionSignature {
        name: "ZwCreateLowBoxToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TokenHandle"),
            ("HANDLE", "ExistingTokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PSID", "PackageSid"),
            ("ULONG", "CapabilityCount"),
            ("PSID_AND_ATTRIBUTES", "Capabilities"),
            ("ULONG", "HandleCount"),
            ("PHANDLE", "Handles"),
        ],
    });

    map.insert("ZwNotifyChangeMultipleKeys", FunctionSignature {
        name: "ZwNotifyChangeMultipleKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MasterKeyHandle"),
            ("ULONG", "Count"),
            ("POBJECT_ATTRIBUTES", "KeyObjects"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("PVOID", "ChangeBuffer"),
            ("ULONG", "BufferSize"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("NtOpenPrivateNamespace", FunctionSignature {
        name: "NtOpenPrivateNamespace",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "NamespaceHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtPropagationComplete", FunctionSignature {
        name: "NtPropagationComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("ULONG", "PropagationId"),
            ("NTSTATUS", "PropagationStatus"),
            ("PULONG", "PropagationCount"),
        ],
    });

    map.insert("ZwNotifyChangeDirectoryFileEx", FunctionSignature {
        name: "ZwNotifyChangeDirectoryFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwCancelIoFile", FunctionSignature {
        name: "ZwCancelIoFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("ZwQueryMultipleValueKey", FunctionSignature {
        name: "ZwQueryMultipleValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PKEY_VALUE_ENTRY", "ValueEntries"),
            ("ULONG", "ValueCount"),
            ("PULONG", "RequiredBufferSize"),
            ("PULONG", "ValueBufferSize"),
        ],
    });

    map.insert("ZwFlushInstallUILanguage", FunctionSignature {
        name: "ZwFlushInstallUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "InstallUILanguageId"),
        ],
    });

    map.insert("NtFilterBootOption", FunctionSignature {
        name: "NtFilterBootOption",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "BootOptionId"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("ZwAdjustTokenClaimsAndDeviceGroups", FunctionSignature {
        name: "ZwAdjustTokenClaimsAndDeviceGroups",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("BOOLEAN", "ResetToDefault"),
            ("PTOKEN_CLAIMS", "NewClaims"),
            ("PTOKEN_CLAIMS", "NewDeviceGroups"),
            ("ULONG", "BufferLength"),
            ("PTOKEN_CLAIMS", "PreviousClaims"),
            ("PTOKEN_CLAIMS", "PreviousDeviceGroups"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwSubmitIoRing", FunctionSignature {
        name: "ZwSubmitIoRing",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoRingHandle"),
            ("ULONG", "SubmissionCount"),
            ("ULONG", "CompletionCount"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtSetBootEntryOrder", FunctionSignature {
        name: "NtSetBootEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "BootEntries"),
            ("ULONG", "BootCount"),
        ],
    });

    map.insert("NtOpenTransaction", FunctionSignature {
        name: "NtOpenTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("LUID", "Uow"),
            ("HANDLE", "TmHandle"),
        ],
    });

    map.insert("ZwCreateKeyTransacted", FunctionSignature {
        name: "ZwCreateKeyTransacted",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "TitleIndex"),
            ("PUNICODE_STRING", "Class"),
            ("ULONG", "CreateOptions"),
            ("HANDLE", "TransactionHandle"),
            ("PULONG", "Disposition"),
        ],
    });

    map.insert("NtReplyWaitReplyPort", FunctionSignature {
        name: "NtReplyWaitReplyPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ReplyMessage"),
            ("PVOID", "ReceiveMessage"),
        ],
    });

    map.insert("NtChangeProcessState", FunctionSignature {
        name: "NtChangeProcessState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ULONG", "TargetState"),
        ],
    });

    map.insert("ZwRemoveIoCompletionEx", FunctionSignature {
        name: "ZwRemoveIoCompletionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("PVOID", "Buffer"),
            ("ULONG", "Count"),
            ("PULONG", "Removed"),
            ("PLARGE_INTEGER", "Timeout"),
            ("BOOLEAN", "Alertable"),
        ],
    });

    map.insert("NtAlpcDeleteResourceReserve", FunctionSignature {
        name: "NtAlpcDeleteResourceReserve",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "ResourceReserve"),
        ],
    });

    map.insert("NtAreMappedFilesTheSame", FunctionSignature {
        name: "NtAreMappedFilesTheSame",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "File1MappedAsAnImage"),
            ("PVOID", "File2MappedAsFile"),
        ],
    });

    map.insert("ZwReleaseSemaphore", FunctionSignature {
        name: "ZwReleaseSemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SemaphoreHandle"),
            ("LONG", "ReleaseCount"),
            ("LPLONG", "PreviousCount"),
        ],
    });

    map.insert("ZwCreateDebugObject", FunctionSignature {
        name: "ZwCreateDebugObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DebugObjectHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtCreateDirectoryObjectEx", FunctionSignature {
        name: "NtCreateDirectoryObjectEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DirectoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "ObjectName"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwOpenProcessToken", FunctionSignature {
        name: "ZwOpenProcessToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("PHANDLE", "TokenHandle"),
        ],
    });

    map.insert("NtOpenKeyTransactedEx", FunctionSignature {
        name: "NtOpenKeyTransactedEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "OpenOptions"),
            ("HANDLE", "TransactionHandle"),
        ],
    });

    map.insert("ZwCreatePrivateNamespace", FunctionSignature {
        name: "ZwCreatePrivateNamespace",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "NamespaceHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PVOID", "CreateParameters"),
        ],
    });

    map.insert("NtSetIntervalProfile", FunctionSignature {
        name: "NtSetIntervalProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Interval"),
            ("KPROFILE_SOURCE", "Source"),
        ],
    });

    map.insert("NtCancelTimer2", FunctionSignature {
        name: "NtCancelTimer2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PBOOLEAN", "CurrentState"),
        ],
    });

    map.insert("NtQueryInformationAtom", FunctionSignature {
        name: "NtQueryInformationAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "AtomHandle"),
            ("ATOM_INFORMATION_CLASS", "AtomInformationClass"),
            ("PVOID", "AtomInformation"),
            ("ULONG", "AtomInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtQueryAttributesFile", FunctionSignature {
        name: "NtQueryAttributesFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PFILE_BASIC_INFORMATION", "FileInformation"),
        ],
    });

    map.insert("ZwRegisterThreadTerminatePort", FunctionSignature {
        name: "ZwRegisterThreadTerminatePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("ZwQueryEaFile", FunctionSignature {
        name: "ZwQueryEaFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PVOID", "EaList"),
            ("ULONG", "EaListLength"),
            ("PULONG", "EaIndex"),
            ("BOOLEAN", "RestartScan"),
        ],
    });

    map.insert("NtWaitForMultipleObjects32", FunctionSignature {
        name: "NtWaitForMultipleObjects32",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Count"),
            ("PHANDLE", "Handles"),
            ("WAIT_TYPE", "WaitType"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwIsProcessInJob", FunctionSignature {
        name: "ZwIsProcessInJob",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "JobHandle"),
            ("PBOOLEAN", "Result"),
        ],
    });

    map.insert("ZwSetIRTimer", FunctionSignature {
        name: "ZwSetIRTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PLARGE_INTEGER", "Period"),
            ("PTIMER_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwRemoveIoCompletion", FunctionSignature {
        name: "ZwRemoveIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("PVOID", "KeyContext"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtQueryInformationJobObject", FunctionSignature {
        name: "NtQueryInformationJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("JOBOBJECTINFOCLASS", "JobObjectInformationClass"),
            ("PVOID", "JobObjectInformation"),
            ("ULONG", "JobObjectInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwOpenTransaction", FunctionSignature {
        name: "ZwOpenTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("LUID", "Uow"),
            ("HANDLE", "TmHandle"),
        ],
    });

    map.insert("ZwEnumerateTransactionObject", FunctionSignature {
        name: "ZwEnumerateTransactionObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnumerateHandle"),
            ("PULONG", "TransactionObject"),
            ("ULONG", "ObjectType"),
            ("ULONG", "Count"),
            ("PULONG", "ReturnCount"),
        ],
    });

    map.insert("NtSignalAndWaitForSingleObject", FunctionSignature {
        name: "NtSignalAndWaitForSingleObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SignalHandle"),
            ("HANDLE", "WaitHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtQueryOpenSubKeysEx", FunctionSignature {
        name: "NtQueryOpenSubKeysEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Format"),
            ("PVOID", "Info"),
            ("ULONG", "InfoLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtWaitForMultipleObjects", FunctionSignature {
        name: "NtWaitForMultipleObjects",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Count"),
            ("PHANDLE", "Handles"),
            ("WAIT_TYPE", "WaitType"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtPrepareEnlistment", FunctionSignature {
        name: "NtPrepareEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("NtAlpcCancelMessage", FunctionSignature {
        name: "NtAlpcCancelMessage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("NtWriteVirtualMemory", FunctionSignature {
        name: "NtWriteVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PSIZE_T", "NumberOfBytesWritten"),
        ],
    });

    map.insert("ZwLoadKey2", FunctionSignature {
        name: "ZwLoadKey2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQuerySection", FunctionSignature {
        name: "NtQuerySection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("SECTION_INFORMATION_CLASS", "SectionInformationClass"),
            ("PVOID", "SectionInformation"),
            ("SIZE_T", "SectionInformationLength"),
            ("PSIZE_T", "ReturnLength"),
        ],
    });

    map.insert("NtPrivilegeObjectAuditAlarm", FunctionSignature {
        name: "NtPrivilegeObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("HANDLE", "TokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("BOOLEAN", "ObjectCreation"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("NtCreateLowBoxToken", FunctionSignature {
        name: "NtCreateLowBoxToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TokenHandle"),
            ("HANDLE", "ExistingTokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PSID", "PackageSid"),
            ("ULONG", "CapabilityCount"),
            ("PSID_AND_ATTRIBUTES", "Capabilities"),
            ("ULONG", "HandleCount"),
            ("PHANDLE", "Handles"),
        ],
    });

    map.insert("NtIsProcessInJob", FunctionSignature {
        name: "NtIsProcessInJob",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "JobHandle"),
            ("PBOOLEAN", "Result"),
        ],
    });

    map.insert("ZwLoadKey", FunctionSignature {
        name: "ZwLoadKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
        ],
    });

    map.insert("NtAlpcQueryInformation", FunctionSignature {
        name: "NtAlpcQueryInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ALPC_INFORMATION_CLASS", "AlpcInformationClass"),
            ("PVOID", "AlpcInformation"),
            ("ULONG", "AlpcInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtModifyDriverEntry", FunctionSignature {
        name: "NtModifyDriverEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PEFI_DRIVER_ENTRY", "DriverEntry"),
        ],
    });

    map.insert("ZwManageHotPatch", FunctionSignature {
        name: "ZwManageHotPatch",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Operation"),
            ("PVOID", "Parameters"),
            ("ULONG", "ParametersSize"),
        ],
    });

    map.insert("ZwCreateProfileEx", FunctionSignature {
        name: "ZwCreateProfileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProfileHandle"),
            ("HANDLE", "Process"),
            ("PVOID", "RangeBase"),
            ("SIZE_T", "RangeSize"),
            ("ULONG", "BucketSize"),
            ("PULONG", "ProfileInfo"),
            ("ULONG", "Flags"),
            ("PULONG", "Buffer"),
            ("ULONG", "BufferSize"),
        ],
    });

    map.insert("ZwChangeProcessState", FunctionSignature {
        name: "ZwChangeProcessState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ULONG", "TargetState"),
        ],
    });

    map.insert("ZwCreateMutant", FunctionSignature {
        name: "ZwCreateMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "MutantHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("BOOLEAN", "InitialOwner"),
        ],
    });

    map.insert("NtTraceControl", FunctionSignature {
        name: "NtTraceControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Operation"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferSize"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferSize"),
            ("PULONG", "OutputBufferUsed"),
        ],
    });

    map.insert("ZwEnumerateSystemEnvironmentValuesEx", FunctionSignature {
        name: "ZwEnumerateSystemEnvironmentValuesEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "InformationClass"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferLength"),
        ],
    });

    map.insert("ZwQueryAuxiliaryCounterFrequency", FunctionSignature {
        name: "ZwQueryAuxiliaryCounterFrequency",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONGLONG", "Frequency"),
        ],
    });

    map.insert("ZwDebugContinue", FunctionSignature {
        name: "ZwDebugContinue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DebugObjectHandle"),
            ("CLIENT_ID", "ClientId"),
            ("NTSTATUS", "ContinueStatus"),
        ],
    });

    map.insert("ZwConvertBetweenAuxiliaryCounterAndPerformanceCounter", FunctionSignature {
        name: "ZwConvertBetweenAuxiliaryCounterAndPerformanceCounter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONGLONG", "AuxiliaryCounterValue"),
            ("PULONGLONG", "PerformanceCounterValue"),
            ("PULONGLONG", "ConversionConstant"),
        ],
    });

    map.insert("NtGetMUIRegistryInfo", FunctionSignature {
        name: "NtGetMUIRegistryInfo",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Flags"),
            ("PVOID", "Data"),
            ("PULONG", "DataSize"),
        ],
    });

    map.insert("NtAlpcDeleteSectionView", FunctionSignature {
        name: "NtAlpcDeleteSectionView",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "SectionView"),
        ],
    });

    map.insert("ZwResumeProcess", FunctionSignature {
        name: "ZwResumeProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
        ],
    });

    map.insert("ZwSetInformationEnlistment", FunctionSignature {
        name: "ZwSetInformationEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("ENLISTMENT_INFORMATION_CLASS", "EnlistmentInformationClass"),
            ("PVOID", "EnlistmentInformation"),
            ("ULONG", "EnlistmentInformationLength"),
        ],
    });

    map.insert("ZwListenPort", FunctionSignature {
        name: "ZwListenPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ConnectionRequest"),
        ],
    });

    map.insert("NtSetDefaultUILanguage", FunctionSignature {
        name: "NtSetDefaultUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DefaultUILanguageId"),
        ],
    });

    map.insert("ZwOpenKeyTransacted", FunctionSignature {
        name: "ZwOpenKeyTransacted",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "TransactionHandle"),
        ],
    });

    map.insert("ZwCreateFile", FunctionSignature {
        name: "ZwCreateFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "AllocationSize"),
            ("ULONG", "FileAttributes"),
            ("ULONG", "ShareAccess"),
            ("ULONG", "CreateDisposition"),
            ("ULONG", "CreateOptions"),
            ("PVOID", "EaBuffer"),
            ("ULONG", "EaLength"),
        ],
    });

    map.insert("ZwSetDefaultLocale", FunctionSignature {
        name: "ZwSetDefaultLocale",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DefaultLocaleId"),
            ("BOOLEAN", "UserProfile"),
        ],
    });

    map.insert("ZwCreateWaitablePort", FunctionSignature {
        name: "ZwCreateWaitablePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtAlertMultipleThreadByThreadId", FunctionSignature {
        name: "NtAlertMultipleThreadByThreadId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "ThreadIds"),
            ("ULONG", "Count"),
        ],
    });

    map.insert("ZwCancelSynchronousIoFile", FunctionSignature {
        name: "ZwCancelSynchronousIoFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("ZwQueryInformationCpuPartition", FunctionSignature {
        name: "ZwQueryInformationCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "CpuPartitionHandle"),
            ("CPU_PARTITION_INFORMATION_CLASS", "CpuPartitionInformationClass"),
            ("PVOID", "CpuPartitionInformation"),
            ("ULONG", "CpuPartitionInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtOpenPartition", FunctionSignature {
        name: "NtOpenPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwUnmapViewOfSection", FunctionSignature {
        name: "ZwUnmapViewOfSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
        ],
    });

    map.insert("NtCommitEnlistment", FunctionSignature {
        name: "NtCommitEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("ZwLockVirtualMemory", FunctionSignature {
        name: "ZwLockVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "MapType"),
        ],
    });

    map.insert("ZwQueryIoCompletion", FunctionSignature {
        name: "ZwQueryIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("IO_COMPLETION_INFORMATION_CLASS", "IoCompletionInformationClass"),
            ("PVOID", "IoCompletionInformation"),
            ("ULONG", "IoCompletionInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtWaitHighEventPair", FunctionSignature {
        name: "NtWaitHighEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtCancelWaitCompletionPacket", FunctionSignature {
        name: "NtCancelWaitCompletionPacket",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WaitCompletionPacketHandle"),
        ],
    });

    map.insert("ZwGetNextThread", FunctionSignature {
        name: "ZwGetNextThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("ULONG", "Flags"),
            ("PHANDLE", "NextThreadHandle"),
        ],
    });

    map.insert("ZwSetDefaultHardErrorPort", FunctionSignature {
        name: "ZwSetDefaultHardErrorPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("ZwCreateKeyedEvent", FunctionSignature {
        name: "ZwCreateKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyedEventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwQueryInformationResourceManager", FunctionSignature {
        name: "ZwQueryInformationResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("RESOURCEMANAGER_INFORMATION_CLASS", "ResourceManagerInformationClass"),
            ("PVOID", "ResourceManagerInformation"),
            ("ULONG", "ResourceManagerInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtAdjustGroupsToken", FunctionSignature {
        name: "NtAdjustGroupsToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("BOOLEAN", "ResetToDefault"),
            ("PTOKEN_GROUPS", "NewState"),
            ("ULONG", "BufferLength"),
            ("PTOKEN_GROUPS", "PreviousState"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtSetEaFile", FunctionSignature {
        name: "NtSetEaFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("ZwSetTimer", FunctionSignature {
        name: "ZwSetTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PTIMER_APC_ROUTINE", "TimerApcRoutine"),
            ("PVOID", "TimerContext"),
            ("BOOLEAN", "Resume"),
            ("LONG", "Period"),
            ("PBOOLEAN", "PreviousState"),
        ],
    });

    map.insert("NtAlertThreadByThreadIdEx", FunctionSignature {
        name: "NtAlertThreadByThreadIdEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadId"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwQueryWnfStateNameInformation", FunctionSignature {
        name: "ZwQueryWnfStateNameInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("WNF_STATE_NAME_INFORMATION_CLASS", "InformationClass"),
            ("PVOID", "Information"),
            ("ULONG", "InformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwQueryObject", FunctionSignature {
        name: "ZwQueryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("OBJECT_INFORMATION_CLASS", "ObjectInformationClass"),
            ("PVOID", "ObjectInformation"),
            ("ULONG", "ObjectInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwMapCMFModule", FunctionSignature {
        name: "ZwMapCMFModule",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "CMFModule"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("NtNotifyChangeSession", FunctionSignature {
        name: "NtNotifyChangeSession",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SessionHandle"),
            ("HANDLE", "EventHandle"),
            ("PVOID", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("PVOID", "ChangeBuffer"),
            ("ULONG", "BufferSize"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("ZwCreateEventPair", FunctionSignature {
        name: "ZwCreateEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventPairHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwCompactKeys", FunctionSignature {
        name: "ZwCompactKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Count"),
            ("PHANDLE", "KeyHandles"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwOpenEnlistment", FunctionSignature {
        name: "ZwOpenEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EnlistmentHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ResourceManagerHandle"),
            ("LUID", "EnlistmentId"),
        ],
    });

    map.insert("ZwCompareObjects", FunctionSignature {
        name: "ZwCompareObjects",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FirstObjectHandle"),
            ("HANDLE", "SecondObjectHandle"),
        ],
    });

    map.insert("ZwCommitTransaction", FunctionSignature {
        name: "ZwCommitTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("BOOLEAN", "Wait"),
        ],
    });

    map.insert("ZwQueryAttributesFile", FunctionSignature {
        name: "ZwQueryAttributesFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PFILE_BASIC_INFORMATION", "FileInformation"),
        ],
    });

    map.insert("ZwOpenSection", FunctionSignature {
        name: "ZwOpenSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SectionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwSetBootEntryOrder", FunctionSignature {
        name: "ZwSetBootEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "BootEntries"),
            ("ULONG", "BootCount"),
        ],
    });

    map.insert("ZwAccessCheckByTypeResultList", FunctionSignature {
        name: "ZwAccessCheckByTypeResultList",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("PPRIVILEGE_SET", "PrivilegeSet"),
            ("PULONG", "PrivilegeSetLength"),
            ("PULONG", "GrantedAccessList"),
            ("PULONG", "AccessStatusList"),
        ],
    });

    map.insert("ZwQueryDriverEntryOrder", FunctionSignature {
        name: "ZwQueryDriverEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "LoadOrder"),
            ("PULONG", "Number"),
        ],
    });

    map.insert("ZwWriteVirtualMemory", FunctionSignature {
        name: "ZwWriteVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PSIZE_T", "NumberOfBytesWritten"),
        ],
    });

    map.insert("ZwQueryInformationTransactionManager", FunctionSignature {
        name: "ZwQueryInformationTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
            ("TRANSACTIONMANAGER_INFORMATION_CLASS", "TransactionManagerInformationClass"),
            ("PVOID", "TransactionManagerInformation"),
            ("ULONG", "TransactionManagerInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtEnumerateSystemEnvironmentValuesEx", FunctionSignature {
        name: "NtEnumerateSystemEnvironmentValuesEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "InformationClass"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferLength"),
        ],
    });

    map.insert("ZwQueryTimerResolution", FunctionSignature {
        name: "ZwQueryTimerResolution",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "MinimumResolution"),
            ("PULONG", "MaximumResolution"),
            ("PULONG", "CurrentResolution"),
        ],
    });

    map.insert("ZwSetInformationResourceManager", FunctionSignature {
        name: "ZwSetInformationResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("RESOURCEMANAGER_INFORMATION_CLASS", "ResourceManagerInformationClass"),
            ("PVOID", "ResourceManagerInformation"),
            ("ULONG", "ResourceManagerInformationLength"),
        ],
    });

    map.insert("NtUnloadKeyEx", FunctionSignature {
        name: "NtUnloadKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtOpenProcessTokenEx", FunctionSignature {
        name: "NtOpenProcessTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("PHANDLE", "TokenHandle"),
        ],
    });

    map.insert("ZwRenameTransactionManager", FunctionSignature {
        name: "ZwRenameTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "LogFileName"),
            ("PGUID", "TmIdentity"),
        ],
    });

    map.insert("NtAlertThreadByThreadId", FunctionSignature {
        name: "NtAlertThreadByThreadId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadId"),
        ],
    });

    map.insert("NtSetValueKey", FunctionSignature {
        name: "NtSetValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "ValueName"),
            ("ULONG", "TitleIndex"),
            ("ULONG", "Type"),
            ("PVOID", "Data"),
            ("ULONG", "DataSize"),
        ],
    });

    map.insert("ZwFilterTokenEx", FunctionSignature {
        name: "ZwFilterTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ExistingTokenHandle"),
            ("ULONG", "Flags"),
            ("PTOKEN_GROUPS", "SidsToDisable"),
            ("PTOKEN_PRIVILEGES", "PrivilegesToDelete"),
            ("PTOKEN_GROUPS", "RestrictedSids"),
            ("ULONG", "DisableUserClaimsCount"),
            ("PCUNICODE_STRING", "UserClaimsToDisable"),
            ("ULONG", "DisableDeviceClaimsCount"),
            ("PCUNICODE_STRING", "DeviceClaimsToDisable"),
            ("PTOKEN_GROUPS", "DeviceGroupsToDisable"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "RestrictedUserAttributes"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "RestrictedDeviceAttributes"),
            ("PTOKEN_GROUPS", "RestrictedDeviceGroups"),
            ("PHANDLE", "NewTokenHandle"),
        ],
    });

    map.insert("ZwAlpcDeleteSectionView", FunctionSignature {
        name: "ZwAlpcDeleteSectionView",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "SectionView"),
        ],
    });

    map.insert("ZwOpenSymbolicLinkObject", FunctionSignature {
        name: "ZwOpenSymbolicLinkObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "LinkHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwDeleteDriverEntry", FunctionSignature {
        name: "ZwDeleteDriverEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DriverEntry"),
        ],
    });

    map.insert("ZwSetInformationJobObject", FunctionSignature {
        name: "ZwSetInformationJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("JOBOBJECTINFOCLASS", "JobObjectInformationClass"),
            ("PVOID", "JobObjectInformation"),
            ("ULONG", "JobObjectInformationLength"),
        ],
    });

    map.insert("NtSerializeBoot", FunctionSignature {
        name: "NtSerializeBoot",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "SerializeBootValue"),
        ],
    });

    map.insert("NtSetThreadExecutionState", FunctionSignature {
        name: "NtSetThreadExecutionState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ExecutionState"),
            ("PULONG", "PreviousExecutionState"),
        ],
    });

    map.insert("NtOpenSection", FunctionSignature {
        name: "NtOpenSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SectionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtMakePermanentObject", FunctionSignature {
        name: "NtMakePermanentObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
        ],
    });

    map.insert("NtSaveKey", FunctionSignature {
        name: "NtSaveKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "FileHandle"),
        ],
    });

    map.insert("NtCreateIoCompletion", FunctionSignature {
        name: "NtCreateIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "IoCompletionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Count"),
        ],
    });

    map.insert("NtSetContextThread", FunctionSignature {
        name: "NtSetContextThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PCONTEXT", "Context"),
        ],
    });

    map.insert("ZwWriteRequestData", FunctionSignature {
        name: "ZwWriteRequestData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "DataIndex"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "OutputBufferSize"),
        ],
    });

    map.insert("NtGetContextThread", FunctionSignature {
        name: "NtGetContextThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PCONTEXT", "Context"),
        ],
    });

    map.insert("ZwQueryValueKey", FunctionSignature {
        name: "ZwQueryValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "ValueName"),
            ("KEY_VALUE_INFORMATION_CLASS", "KeyValueInformationClass"),
            ("PVOID", "KeyValueInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("NtAlpcQueryInformationMessage", FunctionSignature {
        name: "NtAlpcQueryInformationMessage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ALPC_MESSAGE_INFORMATION_CLASS", "MessageInformationClass"),
            ("PVOID", "MessageInformation"),
            ("ULONG", "MessageInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtReplacePartitionUnit", FunctionSignature {
        name: "NtReplacePartitionUnit",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PartitionHandle"),
            ("HANDLE", "TargetHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtApphelpCacheControl", FunctionSignature {
        name: "NtApphelpCacheControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "Data"),
        ],
    });

    map.insert("ZwQueueApcThreadEx", FunctionSignature {
        name: "ZwQueueApcThreadEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("HANDLE", "Reserved"),
            ("PKNORMAL_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcArgument1"),
            ("PVOID", "ApcArgument2"),
            ("PVOID", "ApcArgument3"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtCreatePrivateNamespace", FunctionSignature {
        name: "NtCreatePrivateNamespace",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "NamespaceHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PVOID", "CreateParameters"),
        ],
    });

    map.insert("NtSetInformationVirtualMemory", FunctionSignature {
        name: "NtSetInformationVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("VIRTUAL_MEMORY_INFORMATION_CLASS", "VirtualMemoryInformationClass"),
            ("PVOID", "VirtualMemoryInformation"),
            ("ULONG", "VirtualMemoryInformationLength"),
        ],
    });

    map.insert("ZwQueryTimer", FunctionSignature {
        name: "ZwQueryTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("TIMER_INFORMATION_CLASS", "TimerInformationClass"),
            ("PVOID", "TimerInformation"),
            ("ULONG", "TimerInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwAlpcDeletePortSection", FunctionSignature {
        name: "ZwAlpcDeletePortSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "SectionHandle"),
        ],
    });

    map.insert("ZwDeleteObjectAuditAlarm", FunctionSignature {
        name: "ZwDeleteObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwAddDriverEntry", FunctionSignature {
        name: "ZwAddDriverEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PEFI_DRIVER_ENTRY", "DriverEntry"),
            ("PULONG", "Id"),
        ],
    });

    map.insert("NtSetHighEventPair", FunctionSignature {
        name: "NtSetHighEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("ZwOpenRegistryTransaction", FunctionSignature {
        name: "ZwOpenRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwAccessCheckAndAuditAlarm", FunctionSignature {
        name: "ZwAccessCheckAndAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PBOOLEAN", "AccessStatus"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwUmsThreadYield", FunctionSignature {
        name: "ZwUmsThreadYield",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Reason"),
        ],
    });

    map.insert("ZwContinueEx", FunctionSignature {
        name: "ZwContinueEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PCONTEXT", "ContextRecord"),
            ("BOOLEAN", "TestAlert"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtAddAtom", FunctionSignature {
        name: "NtAddAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "AtomName"),
            ("ULONG", "Length"),
            ("PUSHORT", "Atom"),
        ],
    });

    map.insert("NtOpenKeyedEvent", FunctionSignature {
        name: "NtOpenKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyedEventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwQuerySystemEnvironmentValue", FunctionSignature {
        name: "ZwQuerySystemEnvironmentValue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PWSTR", "VariableValue"),
            ("USHORT", "VariableValueLength"),
            ("PUSHORT", "ReturnLength"),
        ],
    });

    map.insert("NtDeleteBootEntry", FunctionSignature {
        name: "NtDeleteBootEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "BootEntryId"),
        ],
    });

    map.insert("ZwCreateMailslotFile", FunctionSignature {
        name: "ZwCreateMailslotFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "MailslotQuota"),
            ("ULONG", "MaximumMessageSize"),
            ("PLARGE_INTEGER", "ReadTimeout"),
        ],
    });

    map.insert("NtTerminateJobObject", FunctionSignature {
        name: "NtTerminateJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("NTSTATUS", "ExitStatus"),
        ],
    });

    map.insert("ZwReleaseKeyedEvent", FunctionSignature {
        name: "ZwReleaseKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyedEventHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "Key"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwCreateCrossVmEvent", FunctionSignature {
        name: "ZwCreateCrossVmEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "VmHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwFlushBuffersFileEx", FunctionSignature {
        name: "ZwFlushBuffersFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "Parameters"),
            ("ULONG", "ParametersSize"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("NtQueryDirectoryFileEx", FunctionSignature {
        name: "NtQueryDirectoryFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PUNICODE_STRING", "FileName"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwSetIntervalProfile", FunctionSignature {
        name: "ZwSetIntervalProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Interval"),
            ("KPROFILE_SOURCE", "Source"),
        ],
    });

    map.insert("ZwImpersonateThread", FunctionSignature {
        name: "ZwImpersonateThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ServerThreadHandle"),
            ("HANDLE", "ClientThreadHandle"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
        ],
    });

    map.insert("ZwQueryPerformanceCounter", FunctionSignature {
        name: "ZwQueryPerformanceCounter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PLARGE_INTEGER", "PerformanceCounter"),
            ("PLARGE_INTEGER", "PerformanceFrequency"),
        ],
    });

    map.insert("NtDisplayString", FunctionSignature {
        name: "NtDisplayString",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "String"),
        ],
    });

    map.insert("NtGetCurrentProcessorNumber", FunctionSignature {
        name: "NtGetCurrentProcessorNumber",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwAccessCheckByType", FunctionSignature {
        name: "ZwAccessCheckByType",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("PPRIVILEGE_SET", "PrivilegeSet"),
            ("PULONG", "PrivilegeSetLength"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PULONG", "AccessStatus"),
        ],
    });

    map.insert("NtQueryValueKey", FunctionSignature {
        name: "NtQueryValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "ValueName"),
            ("KEY_VALUE_INFORMATION_CLASS", "KeyValueInformationClass"),
            ("PVOID", "KeyValueInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("NtMapCMFModule", FunctionSignature {
        name: "NtMapCMFModule",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "CMFModule"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("ZwRollbackTransaction", FunctionSignature {
        name: "ZwRollbackTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("BOOLEAN", "Wait"),
        ],
    });

    map.insert("ZwMapUserPhysicalPages", FunctionSignature {
        name: "ZwMapUserPhysicalPages",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "VirtualAddress"),
            ("ULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("NtQueryInformationTransactionManager", FunctionSignature {
        name: "NtQueryInformationTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
            ("TRANSACTIONMANAGER_INFORMATION_CLASS", "TransactionManagerInformationClass"),
            ("PVOID", "TransactionManagerInformation"),
            ("ULONG", "TransactionManagerInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtRollforwardTransactionManager", FunctionSignature {
        name: "NtRollforwardTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("ZwAlpcConnectPort", FunctionSignature {
        name: "ZwAlpcConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
            ("PALPC_HANDLE", "ConnectionPort"),
            ("ULONG", "Flags"),
            ("PSID", "RequiredServerSid"),
            ("PALPC_MESSAGE_ATTRIBUTES", "MessageAttributes"),
            ("PVOID", "ConnectData"),
            ("PULONG", "ConnectDataLength"),
        ],
    });

    map.insert("ZwWaitForSingleObject", FunctionSignature {
        name: "ZwWaitForSingleObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtQuerySecurityPolicy", FunctionSignature {
        name: "NtQuerySecurityPolicy",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "SecurityPolicyId"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwSetInformationWorkerFactory", FunctionSignature {
        name: "ZwSetInformationWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("WORKERFACTORYINFOCLASS", "WorkerFactoryInformationClass"),
            ("PVOID", "WorkerFactoryInformation"),
            ("ULONG", "WorkerFactoryInformationLength"),
        ],
    });

    map.insert("NtSetSystemPowerState", FunctionSignature {
        name: "NtSetSystemPowerState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POWER_ACTION", "SystemAction"),
            ("SYSTEM_POWER_STATE", "LightestSystemState"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwCreateTimer2", FunctionSignature {
        name: "ZwCreateTimer2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Attributes"),
        ],
    });

    map.insert("ZwAddAtomEx", FunctionSignature {
        name: "ZwAddAtomEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "AtomName"),
            ("ULONG", "Length"),
            ("PUSHORT", "Atom"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwSetInformationSymbolicLink", FunctionSignature {
        name: "ZwSetInformationSymbolicLink",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "LinkHandle"),
            ("SYMBOLIC_LINK_INFORMATION_CLASS", "LinkInformationClass"),
            ("PVOID", "LinkInformation"),
            ("ULONG", "LinkInformationLength"),
        ],
    });

    map.insert("NtAlpcDisconnectPort", FunctionSignature {
        name: "NtAlpcDisconnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("NtSetIRTimer", FunctionSignature {
        name: "NtSetIRTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PLARGE_INTEGER", "Period"),
            ("PTIMER_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwRaiseHardError", FunctionSignature {
        name: "ZwRaiseHardError",
        return_type: "NTSTATUS",
        parameters: vec![
            ("NTSTATUS", "ErrorStatus"),
            ("ULONG", "NumberOfParameters"),
            ("ULONG", "UnicodeStringParameterMask"),
            ("PULONG_PTR", "Parameters"),
            ("ULONG", "ResponseOption"),
            ("PULONG", "Response"),
        ],
    });

    map.insert("ZwCreateTimer", FunctionSignature {
        name: "ZwCreateTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("TIMER_TYPE", "TimerType"),
        ],
    });

    map.insert("NtAlpcCreateSecurityContext", FunctionSignature {
        name: "NtAlpcCreateSecurityContext",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PALPC_SECURITY_ATTR", "SecurityAttribute"),
        ],
    });

    map.insert("ZwOpenObjectAuditAlarm", FunctionSignature {
        name: "ZwOpenObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ACCESS_MASK", "GrantedAccess"),
            ("PPRIVILEGE_SET", "Privileges"),
            ("BOOLEAN", "ObjectCreation"),
            ("BOOLEAN", "AccessGranted"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwAlertMultipleThreadByThreadId", FunctionSignature {
        name: "ZwAlertMultipleThreadByThreadId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "ThreadIds"),
            ("ULONG", "Count"),
        ],
    });

    map.insert("NtUnmapViewOfSection", FunctionSignature {
        name: "NtUnmapViewOfSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
        ],
    });

    map.insert("NtFreezeRegistry", FunctionSignature {
        name: "NtFreezeRegistry",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwPrepareEnlistment", FunctionSignature {
        name: "ZwPrepareEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("ZwCreateTransactionManager", FunctionSignature {
        name: "ZwCreateTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "LogFileName"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "CommitStrength"),
        ],
    });

    map.insert("NtOpenCpuPartition", FunctionSignature {
        name: "NtOpenCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "CpuPartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwCancelTimer2", FunctionSignature {
        name: "ZwCancelTimer2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PBOOLEAN", "CurrentState"),
        ],
    });

    map.insert("ZwOpenTimer", FunctionSignature {
        name: "ZwOpenTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwCommitRegistryTransaction", FunctionSignature {
        name: "ZwCommitRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwQueryInformationToken", FunctionSignature {
        name: "ZwQueryInformationToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("TOKEN_INFORMATION_CLASS", "TokenInformationClass"),
            ("PVOID", "TokenInformation"),
            ("ULONG", "TokenInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtAdjustTokenClaimsAndDeviceGroups", FunctionSignature {
        name: "NtAdjustTokenClaimsAndDeviceGroups",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("BOOLEAN", "ResetToDefault"),
            ("PTOKEN_CLAIMS", "NewClaims"),
            ("PTOKEN_CLAIMS", "NewDeviceGroups"),
            ("ULONG", "BufferLength"),
            ("PTOKEN_CLAIMS", "PreviousClaims"),
            ("PTOKEN_CLAIMS", "PreviousDeviceGroups"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtReleaseWorkerFactoryWorker", FunctionSignature {
        name: "NtReleaseWorkerFactoryWorker",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
        ],
    });

    map.insert("NtOpenSession", FunctionSignature {
        name: "NtOpenSession",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SessionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtSetInformationToken", FunctionSignature {
        name: "NtSetInformationToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("TOKEN_INFORMATION_CLASS", "TokenInformationClass"),
            ("PVOID", "TokenInformation"),
            ("ULONG", "TokenInformationLength"),
        ],
    });

    map.insert("NtDeleteValueKey", FunctionSignature {
        name: "NtDeleteValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "ValueName"),
        ],
    });

    map.insert("ZwOpenJobObject", FunctionSignature {
        name: "ZwOpenJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "JobHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtOpenEnlistment", FunctionSignature {
        name: "NtOpenEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EnlistmentHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ResourceManagerHandle"),
            ("LUID", "EnlistmentId"),
        ],
    });

    map.insert("ZwRecoverTransactionManager", FunctionSignature {
        name: "ZwRecoverTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
        ],
    });

    map.insert("ZwCreateProcessStateChange", FunctionSignature {
        name: "ZwCreateProcessStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "StateChangeHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "ThreadHandle"),
        ],
    });

    map.insert("ZwSetIoCompletion", FunctionSignature {
        name: "ZwSetIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("ULONG_PTR", "KeyContext"),
            ("PVOID", "ApcContext"),
            ("NTSTATUS", "IoStatus"),
            ("ULONG_PTR", "Information"),
        ],
    });

    map.insert("NtCreateKeyedEvent", FunctionSignature {
        name: "NtCreateKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyedEventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtAllocateUserPhysicalPagesEx", FunctionSignature {
        name: "NtAllocateUserPhysicalPagesEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
            ("PULONG", "ExtendedParameters"),
            ("ULONG", "ParameterCount"),
        ],
    });

    map.insert("ZwQueryIntervalProfile", FunctionSignature {
        name: "ZwQueryIntervalProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("KPROFILE_SOURCE", "Source"),
            ("PULONG", "Interval"),
        ],
    });

    map.insert("ZwLoadDriver", FunctionSignature {
        name: "ZwLoadDriver",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "DriverServiceName"),
        ],
    });

    map.insert("NtSetInformationObject", FunctionSignature {
        name: "NtSetInformationObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("OBJECT_INFORMATION_CLASS", "ObjectInformationClass"),
            ("PVOID", "ObjectInformation"),
            ("ULONG", "ObjectInformationLength"),
        ],
    });

    map.insert("ZwCreateSectionEx", FunctionSignature {
        name: "ZwCreateSectionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SectionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PLARGE_INTEGER", "MaximumSize"),
            ("ULONG", "SectionPageProtection"),
            ("ULONG", "AllocationAttributes"),
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtOpenTransactionManager", FunctionSignature {
        name: "NtOpenTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "LogFileName"),
            ("LUID", "TmIdentity"),
            ("ULONG", "OpenOptions"),
        ],
    });

    map.insert("NtStopProfile", FunctionSignature {
        name: "NtStopProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProfileObject"),
            ("KPROFILE_SOURCE", "ProfileSource"),
        ],
    });

    map.insert("NtCreateUserProcess", FunctionSignature {
        name: "NtCreateUserProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "ProcessDesiredAccess"),
            ("ACCESS_MASK", "ThreadDesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ProcessObjectAttributes"),
            ("POBJECT_ATTRIBUTES", "ThreadObjectAttributes"),
            ("ULONG", "ProcessFlags"),
            ("ULONG", "ThreadFlags"),
            ("PRTL_USER_PROCESS_PARAMETERS", "ProcessParameters"),
            ("PVOID", "CreateInfo"),
            ("PVOID", "AttributeList"),
        ],
    });

    map.insert("NtSetSystemEnvironmentValueEx", FunctionSignature {
        name: "NtSetSystemEnvironmentValueEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PWSTR", "VariableValue"),
            ("USHORT", "VariableValueLength"),
            ("ULONG", "Attributes"),
        ],
    });

    map.insert("NtDuplicateObject", FunctionSignature {
        name: "NtDuplicateObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SourceProcessHandle"),
            ("HANDLE", "SourceHandle"),
            ("HANDLE", "TargetProcessHandle"),
            ("PHANDLE", "TargetHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("ULONG", "Options"),
        ],
    });

    map.insert("NtNotifyChangeKey", FunctionSignature {
        name: "NtNotifyChangeKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("PVOID", "ChangeBuffer"),
            ("ULONG", "BufferSize"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("ZwAllocateLocallyUniqueId", FunctionSignature {
        name: "ZwAllocateLocallyUniqueId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PLUID", "LuId"),
        ],
    });

    map.insert("NtCreateResourceManager", FunctionSignature {
        name: "NtCreateResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ResourceManagerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "TmHandle"),
            ("PGUID", "ResourceManagerGuid"),
            ("ULONG", "CreateOptions"),
            ("PUNICODE_STRING", "Description"),
        ],
    });

    map.insert("ZwQueryMutant", FunctionSignature {
        name: "ZwQueryMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MutantHandle"),
            ("MUTANT_INFORMATION_CLASS", "MutantInformationClass"),
            ("PVOID", "MutantInformation"),
            ("ULONG", "MutantInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwSetVolumeInformationFile", FunctionSignature {
        name: "ZwSetVolumeInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FsInformation"),
            ("ULONG", "Length"),
            ("FSINFOCLASS", "FsInformationClass"),
        ],
    });

    map.insert("ZwConnectPort", FunctionSignature {
        name: "ZwConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
            ("PVOID", "ClientView"),
            ("PSID", "RequiredServerSid"),
            ("PVOID", "ConnectData"),
            ("PULONG", "DataLength"),
        ],
    });

    map.insert("ZwWaitLowEventPair", FunctionSignature {
        name: "ZwWaitLowEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtResetEvent", FunctionSignature {
        name: "NtResetEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("PLONG", "PreviousState"),
        ],
    });

    map.insert("NtQueryDefaultUILanguage", FunctionSignature {
        name: "NtQueryDefaultUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "DefaultUILanguageId"),
        ],
    });

    map.insert("ZwCreateSymbolicLinkObject", FunctionSignature {
        name: "ZwCreateSymbolicLinkObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "LinkHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "LinkTarget"),
        ],
    });

    map.insert("NtSetCachedSigningLevel2", FunctionSignature {
        name: "NtSetCachedSigningLevel2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "SigningLevel"),
            ("PUCHAR", "Thumbprint"),
            ("ULONG", "ThumbprintSize"),
            ("ULONG", "SectionFlags"),
        ],
    });

    map.insert("ZwCreatePagingFile", FunctionSignature {
        name: "ZwCreatePagingFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "PageFileName"),
            ("PLARGE_INTEGER", "MinimumSize"),
            ("PLARGE_INTEGER", "MaximumSize"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtGetDevicePowerState", FunctionSignature {
        name: "NtGetDevicePowerState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DeviceHandle"),
            ("PULONG", "State"),
        ],
    });

    map.insert("ZwUnloadKey", FunctionSignature {
        name: "ZwUnloadKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
        ],
    });

    map.insert("NtCreateProfileEx", FunctionSignature {
        name: "NtCreateProfileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProfileHandle"),
            ("HANDLE", "Process"),
            ("PVOID", "RangeBase"),
            ("SIZE_T", "RangeSize"),
            ("ULONG", "BucketSize"),
            ("PULONG", "ProfileInfo"),
            ("ULONG", "Flags"),
            ("PULONG", "Buffer"),
            ("ULONG", "BufferSize"),
        ],
    });

    map.insert("ZwAlpcCreatePort", FunctionSignature {
        name: "ZwAlpcCreatePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
        ],
    });

    map.insert("NtSetEventEx", FunctionSignature {
        name: "NtSetEventEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("LONG", "EventNumber"),
            ("PLONG", "PreviousState"),
        ],
    });

    map.insert("NtSetIoCompletion", FunctionSignature {
        name: "NtSetIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("ULONG_PTR", "KeyContext"),
            ("PVOID", "ApcContext"),
            ("NTSTATUS", "IoStatus"),
            ("ULONG_PTR", "Information"),
        ],
    });

    map.insert("ZwCreateDirectoryObjectEx", FunctionSignature {
        name: "ZwCreateDirectoryObjectEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DirectoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "ObjectName"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQueryDebugFilterState", FunctionSignature {
        name: "NtQueryDebugFilterState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ComponentId"),
            ("ULONG", "Level"),
        ],
    });

    map.insert("ZwPlugPlayControl", FunctionSignature {
        name: "ZwPlugPlayControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtQueryDirectoryFile", FunctionSignature {
        name: "NtQueryDirectoryFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PUNICODE_STRING", "FileName"),
            ("BOOLEAN", "RestartScan"),
        ],
    });

    map.insert("NtQueryDefaultLocale", FunctionSignature {
        name: "NtQueryDefaultLocale",
        return_type: "NTSTATUS",
        parameters: vec![
            ("BOOLEAN", "UserProfile"),
            ("PLCID", "DefaultLocaleId"),
        ],
    });

    map.insert("ZwTerminateJobObject", FunctionSignature {
        name: "ZwTerminateJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("NTSTATUS", "ExitStatus"),
        ],
    });

    map.insert("ZwQueryDebugFilterState", FunctionSignature {
        name: "ZwQueryDebugFilterState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ComponentId"),
            ("ULONG", "Level"),
        ],
    });

    map.insert("NtQueryBootEntryOrder", FunctionSignature {
        name: "NtQueryBootEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "BootEntries"),
            ("PULONG", "BootCount"),
        ],
    });

    map.insert("NtCreateWnfStateName", FunctionSignature {
        name: "NtCreateWnfStateName",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("ULONG", "NameLifetime"),
            ("ULONG", "DataScope"),
            ("BOOLEAN", "PermanentData"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("ULONG", "MaximumStateSize"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
        ],
    });

    map.insert("NtQueryDirectoryObject", FunctionSignature {
        name: "NtQueryDirectoryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DirectoryHandle"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferLength"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("BOOLEAN", "RestartScan"),
            ("PULONG", "Context"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtAllocateUuids", FunctionSignature {
        name: "NtAllocateUuids",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "Time"),
            ("PULONG", "TimeHi"),
            ("PULONG", "Seq"),
            ("PUCHAR", "NodeId"),
        ],
    });

    map.insert("ZwQueryKey", FunctionSignature {
        name: "ZwQueryKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("KEY_INFORMATION_CLASS", "KeyInformationClass"),
            ("PVOID", "KeyInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("ZwAddAtom", FunctionSignature {
        name: "ZwAddAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "AtomName"),
            ("ULONG", "Length"),
            ("PUSHORT", "Atom"),
        ],
    });

    map.insert("ZwNotifyChangeDirectoryFile", FunctionSignature {
        name: "ZwNotifyChangeDirectoryFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
        ],
    });

    map.insert("NtQueryInformationResourceManager", FunctionSignature {
        name: "NtQueryInformationResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("RESOURCEMANAGER_INFORMATION_CLASS", "ResourceManagerInformationClass"),
            ("PVOID", "ResourceManagerInformation"),
            ("ULONG", "ResourceManagerInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCreateKey", FunctionSignature {
        name: "NtCreateKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "TitleIndex"),
            ("PUNICODE_STRING", "Class"),
            ("ULONG", "CreateOptions"),
            ("PULONG", "Disposition"),
        ],
    });

    map.insert("NtOpenFile", FunctionSignature {
        name: "NtOpenFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "ShareAccess"),
            ("ULONG", "OpenOptions"),
        ],
    });

    map.insert("ZwAccessCheck", FunctionSignature {
        name: "ZwAccessCheck",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("PPRIVILEGE_SET", "PrivilegeSet"),
            ("PULONG", "PrivilegeSetLength"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PBOOLEAN", "AccessStatus"),
        ],
    });

    map.insert("NtSetQuotaInformationFile", FunctionSignature {
        name: "NtSetQuotaInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("ZwSetInformationFile", FunctionSignature {
        name: "ZwSetInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
        ],
    });

    map.insert("ZwWaitForMultipleObjects", FunctionSignature {
        name: "ZwWaitForMultipleObjects",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Count"),
            ("PHANDLE", "Handles"),
            ("WAIT_TYPE", "WaitType"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtUnloadKey2", FunctionSignature {
        name: "NtUnloadKey2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwGetCurrentProcessorNumberEx", FunctionSignature {
        name: "ZwGetCurrentProcessorNumberEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PPROCESSOR_NUMBER", "ProcNumber"),
        ],
    });

    map.insert("NtRequestWaitReplyPort", FunctionSignature {
        name: "NtRequestWaitReplyPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "RequestMessage"),
            ("PVOID", "ReplyMessage"),
        ],
    });

    map.insert("ZwOpenSemaphore", FunctionSignature {
        name: "ZwOpenSemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SemaphoreHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwCreateEnclave", FunctionSignature {
        name: "ZwCreateEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("*mut PVOID", "BaseAddress"),
            ("ULONG_PTR", "ZeroBits"),
            ("SIZE_T", "Size"),
            ("SIZE_T", "InitialCommitment"),
            ("ULONG", "EnclaveType"),
            ("PVOID", "EnclaveInformation"),
            ("ULONG", "EnclaveInformationLength"),
            ("PULONG", "EnclaveError"),
        ],
    });

    map.insert("ZwAccessCheckByTypeAndAuditAlarm", FunctionSignature {
        name: "ZwAccessCheckByTypeAndAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PBOOLEAN", "AccessStatus"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwAlpcRevokeSecurityContext", FunctionSignature {
        name: "ZwAlpcRevokeSecurityContext",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "Context"),
        ],
    });

    map.insert("ZwCompressKey", FunctionSignature {
        name: "ZwCompressKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("NtOpenEvent", FunctionSignature {
        name: "NtOpenEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwQueryVolumeInformationFile", FunctionSignature {
        name: "ZwQueryVolumeInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FsInformation"),
            ("ULONG", "Length"),
            ("FSINFOCLASS", "FsInformationClass"),
        ],
    });

    map.insert("NtOpenRegistryTransaction", FunctionSignature {
        name: "NtOpenRegistryTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TransactionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwPowerInformation", FunctionSignature {
        name: "ZwPowerInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POWER_INFORMATION_LEVEL", "InformationLevel"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
        ],
    });

    map.insert("NtLoadKey2", FunctionSignature {
        name: "NtLoadKey2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQueueApcThreadEx2", FunctionSignature {
        name: "NtQueueApcThreadEx2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("HANDLE", "Reserved"),
            ("PKNORMAL_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcArgument1"),
            ("PVOID", "ApcArgument2"),
            ("PVOID", "ApcArgument3"),
            ("ULONG", "Flags"),
            ("ULONG", "Reserved2"),
        ],
    });

    map.insert("ZwPrivilegeCheck", FunctionSignature {
        name: "ZwPrivilegeCheck",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ClientToken"),
            ("PPRIVILEGE_SET", "RequiredPrivileges"),
            ("PBOOLEAN", "SubjectContextLocked"),
            ("PBOOLEAN", "Result"),
        ],
    });

    map.insert("ZwResetEvent", FunctionSignature {
        name: "ZwResetEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("PLONG", "PreviousState"),
        ],
    });

    map.insert("ZwMapViewOfSectionEx", FunctionSignature {
        name: "ZwMapViewOfSectionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG_PTR", "ZeroBits"),
            ("SIZE_T", "CommitSize"),
            ("PLARGE_INTEGER", "SectionOffset"),
            ("PSIZE_T", "ViewSize"),
            ("ULONG", "InheritDisposition"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Win32Protect"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwReplyWaitReplyPort", FunctionSignature {
        name: "ZwReplyWaitReplyPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ReplyMessage"),
            ("PVOID", "ReceiveMessage"),
        ],
    });

    map.insert("ZwAlpcCreateSecurityContext", FunctionSignature {
        name: "ZwAlpcCreateSecurityContext",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PALPC_SECURITY_ATTR", "SecurityAttribute"),
        ],
    });

    map.insert("ZwGetMUIRegistryInfo", FunctionSignature {
        name: "ZwGetMUIRegistryInfo",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Flags"),
            ("PVOID", "Data"),
            ("PULONG", "DataSize"),
        ],
    });

    map.insert("ZwCreateSemaphore", FunctionSignature {
        name: "ZwCreateSemaphore",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SemaphoreHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("LONG", "InitialCount"),
            ("LONG", "MaximumCount"),
        ],
    });

    map.insert("ZwRegisterProtocolAddressInformation", FunctionSignature {
        name: "ZwRegisterProtocolAddressInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("PUNICODE_STRING", "ProtocolName"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "Persistence"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQuerySystemInformationEx", FunctionSignature {
        name: "NtQuerySystemInformationEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SYSTEM_INFORMATION_CLASS", "SystemInformationClass"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "SystemInformation"),
            ("ULONG", "SystemInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtUnsubscribeWnfStateChange", FunctionSignature {
        name: "NtUnsubscribeWnfStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_CHANGE_STAMP", "ChangeStamp"),
        ],
    });

    map.insert("NtCreatePartition", FunctionSignature {
        name: "NtCreatePartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwLockFile", FunctionSignature {
        name: "ZwLockFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PLARGE_INTEGER", "Length"),
            ("ULONG", "Key"),
            ("BOOLEAN", "FailImmediately"),
            ("BOOLEAN", "ExclusiveLock"),
        ],
    });

    map.insert("NtAllocateLocallyUniqueId", FunctionSignature {
        name: "NtAllocateLocallyUniqueId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PLUID", "LuId"),
        ],
    });

    map.insert("ZwAlpcSetInformation", FunctionSignature {
        name: "ZwAlpcSetInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ALPC_INFORMATION_CLASS", "AlpcInformationClass"),
            ("PVOID", "AlpcInformation"),
            ("ULONG", "AlpcInformationLength"),
        ],
    });

    map.insert("NtThawRegistry", FunctionSignature {
        name: "NtThawRegistry",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwSetDriverEntryOrder", FunctionSignature {
        name: "ZwSetDriverEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "LoadOrder"),
            ("ULONG", "Number"),
        ],
    });

    map.insert("NtDeleteKey", FunctionSignature {
        name: "NtDeleteKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("ZwCreateResourceManager", FunctionSignature {
        name: "ZwCreateResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ResourceManagerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "TmHandle"),
            ("PGUID", "ResourceManagerGuid"),
            ("ULONG", "CreateOptions"),
            ("PUNICODE_STRING", "Description"),
        ],
    });

    map.insert("NtAlpcCreatePortSection", FunctionSignature {
        name: "NtAlpcCreatePortSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("HANDLE", "SectionHandle"),
            ("SIZE_T", "SectionSize"),
            ("PALPC_HANDLE", "AlpcSectionHandle"),
            ("PVOID", "SectionView"),
        ],
    });

    map.insert("NtFlushInstallUILanguage", FunctionSignature {
        name: "NtFlushInstallUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "InstallUILanguageId"),
        ],
    });

    map.insert("ZwSaveMergedKeys", FunctionSignature {
        name: "ZwSaveMergedKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "HighPreparedKeyHandle"),
            ("HANDLE", "LowPreparedKeyHandle"),
            ("HANDLE", "FileHandle"),
        ],
    });

    map.insert("ZwOpenPrivateNamespace", FunctionSignature {
        name: "ZwOpenPrivateNamespace",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "NamespaceHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwSetHighEventPair", FunctionSignature {
        name: "ZwSetHighEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("NtLoadKey", FunctionSignature {
        name: "NtLoadKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
        ],
    });

    map.insert("ZwQueryInformationJobObject", FunctionSignature {
        name: "ZwQueryInformationJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("JOBOBJECTINFOCLASS", "JobObjectInformationClass"),
            ("PVOID", "JobObjectInformation"),
            ("ULONG", "JobObjectInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtWorkerFactoryWorkerReady", FunctionSignature {
        name: "NtWorkerFactoryWorkerReady",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
        ],
    });

    map.insert("ZwSetSystemEnvironmentValueEx", FunctionSignature {
        name: "ZwSetSystemEnvironmentValueEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PWSTR", "VariableValue"),
            ("USHORT", "VariableValueLength"),
            ("ULONG", "Attributes"),
        ],
    });

    map.insert("NtDeleteWnfStateName", FunctionSignature {
        name: "NtDeleteWnfStateName",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
        ],
    });

    map.insert("NtShutdownWorkerFactory", FunctionSignature {
        name: "NtShutdownWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwReplyWaitReceivePort", FunctionSignature {
        name: "ZwReplyWaitReceivePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ReplyMessage"),
            ("PVOID", "ReceiveMessage"),
        ],
    });

    map.insert("NtOpenThreadToken", FunctionSignature {
        name: "NtOpenThreadToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("BOOLEAN", "OpenAsSelf"),
            ("PHANDLE", "TokenHandle"),
        ],
    });

    map.insert("ZwWaitForDebugEvent", FunctionSignature {
        name: "ZwWaitForDebugEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DebugObjectHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
            ("PDBGUI_WAIT_STATE_CHANGE", "WaitStateChange"),
        ],
    });

    map.insert("NtRenameKey", FunctionSignature {
        name: "NtRenameKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "NewName"),
        ],
    });

    map.insert("NtCreateCrossVmEvent", FunctionSignature {
        name: "NtCreateCrossVmEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "VmHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwQueryInformationAtom", FunctionSignature {
        name: "ZwQueryInformationAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "AtomHandle"),
            ("ATOM_INFORMATION_CLASS", "AtomInformationClass"),
            ("PVOID", "AtomInformation"),
            ("ULONG", "AtomInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwAllocateUuids", FunctionSignature {
        name: "ZwAllocateUuids",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "Time"),
            ("PULONG", "TimeHi"),
            ("PULONG", "Seq"),
            ("PUCHAR", "NodeId"),
        ],
    });

    map.insert("NtAlpcCreateSectionView", FunctionSignature {
        name: "NtAlpcCreateSectionView",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PALPC_DATA_VIEW_ATTR", "SectionView"),
        ],
    });

    map.insert("ZwSetDefaultUILanguage", FunctionSignature {
        name: "ZwSetDefaultUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DefaultUILanguageId"),
        ],
    });

    map.insert("ZwOpenFile", FunctionSignature {
        name: "ZwOpenFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "ShareAccess"),
            ("ULONG", "OpenOptions"),
        ],
    });

    map.insert("NtSystemDebugControl", FunctionSignature {
        name: "NtSystemDebugControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCloseObjectAuditAlarm", FunctionSignature {
        name: "NtCloseObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwDeleteFile", FunctionSignature {
        name: "ZwDeleteFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwAllocateUserPhysicalPages", FunctionSignature {
        name: "ZwAllocateUserPhysicalPages",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("ZwAccessCheckByTypeResultListAndAuditAlarmByHandle", FunctionSignature {
        name: "ZwAccessCheckByTypeResultListAndAuditAlarmByHandle",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("HANDLE", "ClientToken"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PULONG", "GrantedAccessList"),
            ("PULONG", "AccessStatusList"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("NtSetInformationIoRing", FunctionSignature {
        name: "NtSetInformationIoRing",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoRingHandle"),
            ("ULONG", "InformationClass"),
            ("PVOID", "Information"),
            ("ULONG", "InformationLength"),
        ],
    });

    map.insert("ZwStopProfile", FunctionSignature {
        name: "ZwStopProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProfileObject"),
            ("KPROFILE_SOURCE", "ProfileSource"),
        ],
    });

    map.insert("NtLoadKey3", FunctionSignature {
        name: "NtLoadKey3",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
            ("ULONG", "Flags"),
            ("ULONG", "Reserved"),
        ],
    });

    map.insert("ZwSetUuidSeed", FunctionSignature {
        name: "ZwSetUuidSeed",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "Seed"),
        ],
    });

    map.insert("NtCopyFileChunk", FunctionSignature {
        name: "NtCopyFileChunk",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SourceFileHandle"),
            ("HANDLE", "TargetFileHandle"),
            ("PLARGE_INTEGER", "SourceOffset"),
            ("PLARGE_INTEGER", "TargetOffset"),
            ("SIZE_T", "Length"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwReadRequestData", FunctionSignature {
        name: "ZwReadRequestData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "DataIndex"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "OutputBufferSize"),
        ],
    });

    map.insert("ZwQuerySection", FunctionSignature {
        name: "ZwQuerySection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("SECTION_INFORMATION_CLASS", "SectionInformationClass"),
            ("PVOID", "SectionInformation"),
            ("SIZE_T", "SectionInformationLength"),
            ("PSIZE_T", "ReturnLength"),
        ],
    });

    map.insert("ZwChangeThreadState", FunctionSignature {
        name: "ZwChangeThreadState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ULONG", "TargetState"),
        ],
    });

    map.insert("NtReadRequestData", FunctionSignature {
        name: "NtReadRequestData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "DataIndex"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "OutputBufferSize"),
        ],
    });

    map.insert("ZwDeleteValueKey", FunctionSignature {
        name: "ZwDeleteValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "ValueName"),
        ],
    });

    map.insert("NtAccessCheckByTypeAndAuditAlarm", FunctionSignature {
        name: "NtAccessCheckByTypeAndAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PBOOLEAN", "AccessStatus"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwReplyWaitReceivePortEx", FunctionSignature {
        name: "ZwReplyWaitReceivePortEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ReplyMessage"),
            ("PVOID", "ReceiveMessage"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtAlpcImpersonateClientOfPort", FunctionSignature {
        name: "NtAlpcImpersonateClientOfPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQuerySystemEnvironmentValueEx", FunctionSignature {
        name: "NtQuerySystemEnvironmentValueEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PWSTR", "VariableValue"),
            ("PUSHORT", "VariableValueLength"),
            ("PULONG", "Attributes"),
        ],
    });

    map.insert("ZwCancelWaitCompletionPacket", FunctionSignature {
        name: "ZwCancelWaitCompletionPacket",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WaitCompletionPacketHandle"),
        ],
    });

    map.insert("NtAcquireCrossVmMutant", FunctionSignature {
        name: "NtAcquireCrossVmMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MutantHandle"),
            ("HANDLE", "VmHandle"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtUmsThreadYield", FunctionSignature {
        name: "NtUmsThreadYield",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Reason"),
        ],
    });

    map.insert("ZwSetInformationIoRing", FunctionSignature {
        name: "ZwSetInformationIoRing",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoRingHandle"),
            ("ULONG", "InformationClass"),
            ("PVOID", "Information"),
            ("ULONG", "InformationLength"),
        ],
    });

    map.insert("NtGetCompleteWnfStateSubscription", FunctionSignature {
        name: "NtGetCompleteWnfStateSubscription",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PWNF_CHANGE_STAMP", "ChangeStamp"),
            ("PWNF_USER_SUBSCRIPTION", "Subscription"),
        ],
    });

    map.insert("ZwAcquireCrossVmMutant", FunctionSignature {
        name: "ZwAcquireCrossVmMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MutantHandle"),
            ("HANDLE", "VmHandle"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwShutdownWorkerFactory", FunctionSignature {
        name: "ZwShutdownWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwSystemDebugControl", FunctionSignature {
        name: "ZwSystemDebugControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtAccessCheck", FunctionSignature {
        name: "NtAccessCheck",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("PPRIVILEGE_SET", "PrivilegeSet"),
            ("PULONG", "PrivilegeSetLength"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PBOOLEAN", "AccessStatus"),
        ],
    });

    map.insert("ZwGetNlsSectionPtr", FunctionSignature {
        name: "ZwGetNlsSectionPtr",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "SectionType"),
            ("ULONG", "SectionId"),
            ("PVOID", "SectionData"),
            ("PULONG", "SectionSize"),
        ],
    });

    map.insert("ZwTraceEvent", FunctionSignature {
        name: "ZwTraceEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TraceHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "FieldSize"),
            ("PVOID", "Fields"),
        ],
    });

    map.insert("ZwAlpcSendWaitReceivePort", FunctionSignature {
        name: "ZwAlpcSendWaitReceivePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "SendMessage"),
            ("PALPC_MESSAGE_ATTRIBUTES", "SendAttributes"),
            ("PVOID", "ReceiveMessage"),
            ("PULONG", "ReceiveMessageLength"),
            ("PALPC_MESSAGE_ATTRIBUTES", "ReceiveAttributes"),
        ],
    });

    map.insert("NtMapViewOfSection", FunctionSignature {
        name: "NtMapViewOfSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG_PTR", "ZeroBits"),
            ("SIZE_T", "CommitSize"),
            ("PLARGE_INTEGER", "SectionOffset"),
            ("PSIZE_T", "ViewSize"),
            ("ULONG", "InheritDisposition"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Win32Protect"),
        ],
    });

    map.insert("NtReplyWaitReceivePortEx", FunctionSignature {
        name: "NtReplyWaitReceivePortEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ReplyMessage"),
            ("PVOID", "ReceiveMessage"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtFilterTokenEx", FunctionSignature {
        name: "NtFilterTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ExistingTokenHandle"),
            ("ULONG", "Flags"),
            ("PTOKEN_GROUPS", "SidsToDisable"),
            ("PTOKEN_PRIVILEGES", "PrivilegesToDelete"),
            ("PTOKEN_GROUPS", "RestrictedSids"),
            ("ULONG", "DisableUserClaimsCount"),
            ("PCUNICODE_STRING", "UserClaimsToDisable"),
            ("ULONG", "DisableDeviceClaimsCount"),
            ("PCUNICODE_STRING", "DeviceClaimsToDisable"),
            ("PTOKEN_GROUPS", "DeviceGroupsToDisable"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "RestrictedUserAttributes"),
            ("PTOKEN_SECURITY_ATTRIBUTES_INFORMATION", "RestrictedDeviceAttributes"),
            ("PTOKEN_GROUPS", "RestrictedDeviceGroups"),
            ("PHANDLE", "NewTokenHandle"),
        ],
    });

    map.insert("ZwOpenEvent", FunctionSignature {
        name: "ZwOpenEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtReadOnlyEnlistment", FunctionSignature {
        name: "NtReadOnlyEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwAlpcDisconnectPort", FunctionSignature {
        name: "ZwAlpcDisconnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("ZwFlushWriteBuffer", FunctionSignature {
        name: "ZwFlushWriteBuffer",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtSetEventBoostPriority", FunctionSignature {
        name: "NtSetEventBoostPriority",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
        ],
    });

    map.insert("NtRecoverEnlistment", FunctionSignature {
        name: "NtRecoverEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "EnlistmentKey"),
        ],
    });

    map.insert("ZwUnloadKey2", FunctionSignature {
        name: "ZwUnloadKey2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtCreateProcessStateChange", FunctionSignature {
        name: "NtCreateProcessStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "StateChangeHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ProcessHandle"),
            ("HANDLE", "ThreadHandle"),
        ],
    });

    map.insert("NtQueryObject", FunctionSignature {
        name: "NtQueryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
            ("OBJECT_INFORMATION_CLASS", "ObjectInformationClass"),
            ("PVOID", "ObjectInformation"),
            ("ULONG", "ObjectInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwCallbackReturn", FunctionSignature {
        name: "ZwCallbackReturn",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Result"),
            ("ULONG", "ResultLength"),
            ("NTSTATUS", "Status"),
        ],
    });

    map.insert("ZwTerminateThread", FunctionSignature {
        name: "ZwTerminateThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("NTSTATUS", "ExitStatus"),
        ],
    });

    map.insert("ZwAlertThread", FunctionSignature {
        name: "ZwAlertThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
        ],
    });

    map.insert("ZwClose", FunctionSignature {
        name: "ZwClose",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
        ],
    });

    map.insert("ZwUnsubscribeWnfStateChange", FunctionSignature {
        name: "ZwUnsubscribeWnfStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_CHANGE_STAMP", "ChangeStamp"),
        ],
    });

    map.insert("ZwIsUILanguageComitted", FunctionSignature {
        name: "ZwIsUILanguageComitted",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwLoadKey3", FunctionSignature {
        name: "ZwLoadKey3",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
            ("POBJECT_ATTRIBUTES", "SourceFile"),
            ("ULONG", "Flags"),
            ("ULONG", "Reserved"),
        ],
    });

    map.insert("ZwCreateThread", FunctionSignature {
        name: "ZwCreateThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ProcessHandle"),
            ("PCLIENT_ID", "ClientId"),
            ("PCONTEXT", "ThreadContext"),
            ("PINITIAL_TEB", "InitialTeb"),
            ("BOOLEAN", "CreateSuspended"),
        ],
    });

    map.insert("NtQueryMutant", FunctionSignature {
        name: "NtQueryMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MutantHandle"),
            ("MUTANT_INFORMATION_CLASS", "MutantInformationClass"),
            ("PVOID", "MutantInformation"),
            ("ULONG", "MutantInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtEnumerateTransactionObject", FunctionSignature {
        name: "NtEnumerateTransactionObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnumerateHandle"),
            ("PULONG", "TransactionObject"),
            ("ULONG", "ObjectType"),
            ("ULONG", "Count"),
            ("PULONG", "ReturnCount"),
        ],
    });

    map.insert("NtEnumerateBootEntries", FunctionSignature {
        name: "NtEnumerateBootEntries",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Buffer"),
            ("PULONG", "BufferLength"),
        ],
    });

    map.insert("ZwCreateUserProcess", FunctionSignature {
        name: "ZwCreateUserProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "ProcessDesiredAccess"),
            ("ACCESS_MASK", "ThreadDesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ProcessObjectAttributes"),
            ("POBJECT_ATTRIBUTES", "ThreadObjectAttributes"),
            ("ULONG", "ProcessFlags"),
            ("ULONG", "ThreadFlags"),
            ("PRTL_USER_PROCESS_PARAMETERS", "ProcessParameters"),
            ("PVOID", "CreateInfo"),
            ("PVOID", "AttributeList"),
        ],
    });

    map.insert("NtGetNlsSectionPtr", FunctionSignature {
        name: "NtGetNlsSectionPtr",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "SectionType"),
            ("ULONG", "SectionId"),
            ("PVOID", "SectionData"),
            ("PULONG", "SectionSize"),
        ],
    });

    map.insert("NtQueryMultipleValueKey", FunctionSignature {
        name: "NtQueryMultipleValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PKEY_VALUE_ENTRY", "ValueEntries"),
            ("ULONG", "ValueCount"),
            ("PULONG", "RequiredBufferSize"),
            ("PULONG", "ValueBufferSize"),
        ],
    });

    map.insert("ZwDeviceIoControlFile", FunctionSignature {
        name: "ZwDeviceIoControlFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "IoControlCode"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
        ],
    });

    map.insert("NtQuerySystemEnvironmentValue", FunctionSignature {
        name: "NtQuerySystemEnvironmentValue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PWSTR", "VariableValue"),
            ("USHORT", "VariableValueLength"),
            ("PUSHORT", "ReturnLength"),
        ],
    });

    map.insert("ZwCommitEnlistment", FunctionSignature {
        name: "ZwCommitEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PLARGE_INTEGER", "TmVirtualClock"),
        ],
    });

    map.insert("ZwCloseObjectAuditAlarm", FunctionSignature {
        name: "ZwCloseObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwSetInformationDebugObject", FunctionSignature {
        name: "ZwSetInformationDebugObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "DebugObjectHandle"),
            ("DEBUGOBJECTINFOCLASS", "DebugObjectInformationClass"),
            ("PVOID", "DebugObjectInformation"),
            ("ULONG", "DebugObjectInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwEnumerateDriverEntries", FunctionSignature {
        name: "ZwEnumerateDriverEntries",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "LoadOrder"),
            ("PULONG", "Number"),
        ],
    });

    map.insert("ZwCancelTimer", FunctionSignature {
        name: "ZwCancelTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PBOOLEAN", "CurrentState"),
        ],
    });

    map.insert("ZwFreezeRegistry", FunctionSignature {
        name: "ZwFreezeRegistry",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwDirectGraphicsCall", FunctionSignature {
        name: "ZwDirectGraphicsCall",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Command"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferSize"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferSize"),
        ],
    });

    map.insert("NtReadVirtualMemoryEx", FunctionSignature {
        name: "NtReadVirtualMemoryEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PSIZE_T", "NumberOfBytesRead"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtOpenSymbolicLinkObject", FunctionSignature {
        name: "NtOpenSymbolicLinkObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "LinkHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtQueryInformationTransaction", FunctionSignature {
        name: "NtQueryInformationTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("TRANSACTION_INFORMATION_CLASS", "TransactionInformationClass"),
            ("PVOID", "TransactionInformation"),
            ("ULONG", "TransactionInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwSetEvent", FunctionSignature {
        name: "ZwSetEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("PLONG", "PreviousState"),
        ],
    });

    map.insert("NtDrawText", FunctionSignature {
        name: "NtDrawText",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WindowHandle"),
            ("PUNICODE_STRING", "Text"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtSetTimerResolution", FunctionSignature {
        name: "NtSetTimerResolution",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DesiredResolution"),
            ("BOOLEAN", "SetResolution"),
            ("PULONG", "ActualResolution"),
        ],
    });

    map.insert("ZwRequestPort", FunctionSignature {
        name: "ZwRequestPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("NtCompressKey", FunctionSignature {
        name: "NtCompressKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("ZwQueryInformationWorkerFactory", FunctionSignature {
        name: "ZwQueryInformationWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("WORKERFACTORYINFOCLASS", "WorkerFactoryInformationClass"),
            ("PVOID", "WorkerFactoryInformation"),
            ("ULONG", "WorkerFactoryInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCreateMailslotFile", FunctionSignature {
        name: "NtCreateMailslotFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "MailslotQuota"),
            ("ULONG", "MaximumMessageSize"),
            ("PLARGE_INTEGER", "ReadTimeout"),
        ],
    });

    map.insert("ZwReadFile", FunctionSignature {
        name: "ZwReadFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });

    map.insert("NtCreateCrossVmMutant", FunctionSignature {
        name: "NtCreateCrossVmMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "MutantHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "VmHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtCreateProcessEx", FunctionSignature {
        name: "NtCreateProcessEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ParentProcess"),
            ("ULONG", "Flags"),
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "DebugPort"),
            ("HANDLE", "ExceptionPort"),
            ("ULONG", "JobMemberLevel"),
        ],
    });

    map.insert("ZwGetCurrentProcessorNumber", FunctionSignature {
        name: "ZwGetCurrentProcessorNumber",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtPowerInformation", FunctionSignature {
        name: "NtPowerInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POWER_INFORMATION_LEVEL", "InformationLevel"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferLength"),
        ],
    });

    map.insert("NtAlpcOpenSenderProcess", FunctionSignature {
        name: "NtAlpcOpenSenderProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwAlpcConnectPortEx", FunctionSignature {
        name: "ZwAlpcConnectPortEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
            ("PALPC_HANDLE", "ConnectionPort"),
            ("ULONG", "Flags"),
            ("PSID", "RequiredServerSid"),
            ("PALPC_MESSAGE_ATTRIBUTES", "MessageAttributes"),
            ("PVOID", "ConnectData"),
            ("PULONG", "ConnectDataLength"),
            ("PVOID", "ReceiveMessage"),
            ("PULONG", "ReceiveMessageLength"),
        ],
    });

    map.insert("NtSetHighWaitLowEventPair", FunctionSignature {
        name: "NtSetHighWaitLowEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("ZwSetInformationVirtualMemory", FunctionSignature {
        name: "ZwSetInformationVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("VIRTUAL_MEMORY_INFORMATION_CLASS", "VirtualMemoryInformationClass"),
            ("PVOID", "VirtualMemoryInformation"),
            ("ULONG", "VirtualMemoryInformationLength"),
        ],
    });

    map.insert("NtCompactKeys", FunctionSignature {
        name: "NtCompactKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Count"),
            ("PHANDLE", "KeyHandles"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtListenPort", FunctionSignature {
        name: "NtListenPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "ConnectionRequest"),
        ],
    });

    map.insert("NtPulseEvent", FunctionSignature {
        name: "NtPulseEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("PLONG", "PreviousState"),
        ],
    });

    map.insert("ZwSetLowEventPair", FunctionSignature {
        name: "ZwSetLowEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("NtSaveMergedKeys", FunctionSignature {
        name: "NtSaveMergedKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "HighPreparedKeyHandle"),
            ("HANDLE", "LowPreparedKeyHandle"),
            ("HANDLE", "FileHandle"),
        ],
    });

    map.insert("ZwSetTimerResolution", FunctionSignature {
        name: "ZwSetTimerResolution",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DesiredResolution"),
            ("BOOLEAN", "SetResolution"),
            ("PULONG", "ActualResolution"),
        ],
    });

    map.insert("NtSetLowWaitHighEventPair", FunctionSignature {
        name: "NtSetLowWaitHighEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("NtTestAlert", FunctionSignature {
        name: "NtTestAlert",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwSuspendProcess", FunctionSignature {
        name: "ZwSuspendProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
        ],
    });

    map.insert("NtOpenThreadTokenEx", FunctionSignature {
        name: "NtOpenThreadTokenEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("BOOLEAN", "OpenAsSelf"),
            ("ULONG", "HandleAttributes"),
            ("PHANDLE", "TokenHandle"),
        ],
    });

    map.insert("ZwCompareTokens", FunctionSignature {
        name: "ZwCompareTokens",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FirstTokenHandle"),
            ("HANDLE", "SecondTokenHandle"),
            ("PBOOLEAN", "Equal"),
        ],
    });

    map.insert("NtEnumerateKey", FunctionSignature {
        name: "NtEnumerateKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Index"),
            ("KEY_INFORMATION_CLASS", "KeyInformationClass"),
            ("PVOID", "KeyInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("NtSetInformationEnlistment", FunctionSignature {
        name: "NtSetInformationEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("ENLISTMENT_INFORMATION_CLASS", "EnlistmentInformationClass"),
            ("PVOID", "EnlistmentInformation"),
            ("ULONG", "EnlistmentInformationLength"),
        ],
    });

    map.insert("ZwQueryIoRingCapabilities", FunctionSignature {
        name: "ZwQueryIoRingCapabilities",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SIZE_T", "IoRingCapabilitiesLength"),
            ("PVOID", "IoRingCapabilities"),
        ],
    });

    map.insert("ZwSetDebugFilterState", FunctionSignature {
        name: "ZwSetDebugFilterState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ComponentId"),
            ("ULONG", "Level"),
            ("BOOLEAN", "State"),
        ],
    });

    map.insert("NtAlpcCreatePort", FunctionSignature {
        name: "NtAlpcCreatePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
        ],
    });

    map.insert("NtQueueApcThread", FunctionSignature {
        name: "NtQueueApcThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcArgument1"),
            ("PVOID", "ApcArgument2"),
            ("PVOID", "ApcArgument3"),
        ],
    });

    map.insert("NtQueryQuotaInformationFile", FunctionSignature {
        name: "NtQueryQuotaInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PVOID", "SidList"),
            ("ULONG", "SidListLength"),
            ("PULONG", "StartSidIndex"),
            ("BOOLEAN", "RestartScan"),
        ],
    });

    map.insert("ZwCopyFileChunk", FunctionSignature {
        name: "ZwCopyFileChunk",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SourceFileHandle"),
            ("HANDLE", "TargetFileHandle"),
            ("PLARGE_INTEGER", "SourceOffset"),
            ("PLARGE_INTEGER", "TargetOffset"),
            ("SIZE_T", "Length"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwManagePartition", FunctionSignature {
        name: "ZwManagePartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PartitionHandle"),
            ("ULONG", "Operation"),
            ("PVOID", "Parameters"),
            ("ULONG", "ParametersSize"),
        ],
    });

    map.insert("ZwExtendSection", FunctionSignature {
        name: "ZwExtendSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("PLARGE_INTEGER", "SectionSize"),
        ],
    });

    map.insert("NtOpenKeyEx", FunctionSignature {
        name: "NtOpenKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "OpenOptions"),
        ],
    });

    map.insert("NtTraceEvent", FunctionSignature {
        name: "NtTraceEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TraceHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "FieldSize"),
            ("PVOID", "Fields"),
        ],
    });

    map.insert("NtAddAtomEx", FunctionSignature {
        name: "NtAddAtomEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "AtomName"),
            ("ULONG", "Length"),
            ("PUSHORT", "Atom"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwPropagationFailed", FunctionSignature {
        name: "ZwPropagationFailed",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("ULONG", "PropagationId"),
            ("NTSTATUS", "PropagationStatus"),
            ("PULONG", "PropagationCount"),
        ],
    });

    map.insert("ZwGetCompleteWnfStateSubscription", FunctionSignature {
        name: "ZwGetCompleteWnfStateSubscription",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PWNF_CHANGE_STAMP", "ChangeStamp"),
            ("PWNF_USER_SUBSCRIPTION", "Subscription"),
        ],
    });

    map.insert("NtSetInformationCpuPartition", FunctionSignature {
        name: "NtSetInformationCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "CpuPartitionHandle"),
            ("CPU_PARTITION_INFORMATION_CLASS", "CpuPartitionInformationClass"),
            ("PVOID", "CpuPartitionInformation"),
            ("ULONG", "CpuPartitionInformationLength"),
        ],
    });

    map.insert("NtAllocateReserveObject", FunctionSignature {
        name: "NtAllocateReserveObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ReserveObject"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Type"),
        ],
    });

    map.insert("NtUnlockFile", FunctionSignature {
        name: "NtUnlockFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PLARGE_INTEGER", "Length"),
            ("ULONG", "Key"),
        ],
    });

    map.insert("NtSetTimerEx", FunctionSignature {
        name: "NtSetTimerEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PLARGE_INTEGER", "Period"),
            ("PTIMER_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtCreateWaitCompletionPacket", FunctionSignature {
        name: "NtCreateWaitCompletionPacket",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "WaitCompletionPacketHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtAlpcCreateResourceReserve", FunctionSignature {
        name: "NtAlpcCreateResourceReserve",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "MessageSize"),
            ("PULONG", "ActualMessageSize"),
        ],
    });

    map.insert("NtCreateCpuPartition", FunctionSignature {
        name: "NtCreateCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "CpuPartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtAlpcImpersonateClientContainerOfPort", FunctionSignature {
        name: "NtAlpcImpersonateClientContainerOfPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtReadFileScatter", FunctionSignature {
        name: "NtReadFileScatter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PFILE_SEGMENT_ELEMENT", "SegmentArray"),
            ("ULONG", "Length"),
            ("PLARGE_INTEGER", "ByteOffset"),
            ("PULONG", "Key"),
        ],
    });

    map.insert("NtQueryInformationProcess", FunctionSignature {
        name: "NtQueryInformationProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PROCESSINFOCLASS", "ProcessInformationClass"),
            ("PVOID", "ProcessInformation"),
            ("ULONG", "ProcessInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtThawTransactions", FunctionSignature {
        name: "NtThawTransactions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
        ],
    });

    map.insert("ZwSetTimerEx", FunctionSignature {
        name: "ZwSetTimerEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("PLARGE_INTEGER", "DueTime"),
            ("PLARGE_INTEGER", "Period"),
            ("PTIMER_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtQueryInformationPort", FunctionSignature {
        name: "NtQueryInformationPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PORT_INFORMATION_CLASS", "PortInformationClass"),
            ("PVOID", "PortInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCreateNamedPipeFile", FunctionSignature {
        name: "NtCreateNamedPipeFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "FileHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "ShareAccess"),
            ("ULONG", "CreateDisposition"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "NamedPipeType"),
            ("ULONG", "ReadMode"),
            ("ULONG", "CompletionMode"),
            ("ULONG", "MaximumInstances"),
            ("ULONG", "InboundQuota"),
            ("ULONG", "OutboundQuota"),
            ("PLARGE_INTEGER", "DefaultTimeout"),
        ],
    });

    map.insert("ZwRenameKey", FunctionSignature {
        name: "ZwRenameKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "NewName"),
        ],
    });

    map.insert("ZwAlpcQueryInformation", FunctionSignature {
        name: "ZwAlpcQueryInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ALPC_INFORMATION_CLASS", "AlpcInformationClass"),
            ("PVOID", "AlpcInformation"),
            ("ULONG", "AlpcInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCreateToken", FunctionSignature {
        name: "NtCreateToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("TOKEN_TYPE", "Type"),
            ("PLUID", "AuthenticationId"),
            ("PLARGE_INTEGER", "ExpirationTime"),
            ("PTOKEN_USER", "User"),
            ("PTOKEN_GROUPS", "Groups"),
            ("PTOKEN_PRIVILEGES", "Privileges"),
            ("PTOKEN_OWNER", "Owner"),
            ("PTOKEN_PRIMARY_GROUP", "PrimaryGroup"),
            ("PTOKEN_DEFAULT_DACL", "DefaultDacl"),
            ("PTOKEN_SOURCE", "Source"),
        ],
    });

    map.insert("NtLockRegistryKey", FunctionSignature {
        name: "NtLockRegistryKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("NtImpersonateThread", FunctionSignature {
        name: "NtImpersonateThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ServerThreadHandle"),
            ("HANDLE", "ClientThreadHandle"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
        ],
    });

    map.insert("ZwWorkerFactoryWorkerReady", FunctionSignature {
        name: "ZwWorkerFactoryWorkerReady",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
        ],
    });

    map.insert("NtConvertBetweenAuxiliaryCounterAndPerformanceCounter", FunctionSignature {
        name: "NtConvertBetweenAuxiliaryCounterAndPerformanceCounter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONGLONG", "AuxiliaryCounterValue"),
            ("PULONGLONG", "PerformanceCounterValue"),
            ("PULONGLONG", "ConversionConstant"),
        ],
    });

    map.insert("ZwSetInformationToken", FunctionSignature {
        name: "ZwSetInformationToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("TOKEN_INFORMATION_CLASS", "TokenInformationClass"),
            ("PVOID", "TokenInformation"),
            ("ULONG", "TokenInformationLength"),
        ],
    });

    map.insert("NtCancelSynchronousIoFile", FunctionSignature {
        name: "NtCancelSynchronousIoFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
        ],
    });

    map.insert("ZwCallEnclave", FunctionSignature {
        name: "ZwCallEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Routine"),
            ("PVOID", "Parameter"),
            ("BOOLEAN", "WaitForThread"),
            ("PVOID", "Result"),
        ],
    });

    map.insert("ZwEnumerateBootEntries", FunctionSignature {
        name: "ZwEnumerateBootEntries",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "Buffer"),
            ("PULONG", "BufferLength"),
        ],
    });

    map.insert("NtQueryInformationToken", FunctionSignature {
        name: "NtQueryInformationToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TokenHandle"),
            ("TOKEN_INFORMATION_CLASS", "TokenInformationClass"),
            ("PVOID", "TokenInformation"),
            ("ULONG", "TokenInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwDelayExecution", FunctionSignature {
        name: "ZwDelayExecution",
        return_type: "NTSTATUS",
        parameters: vec![
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "DelayInterval"),
        ],
    });

    map.insert("ZwQueryDirectoryFileEx", FunctionSignature {
        name: "ZwQueryDirectoryFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
            ("BOOLEAN", "ReturnSingleEntry"),
            ("PUNICODE_STRING", "FileName"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwSetEaFile", FunctionSignature {
        name: "ZwSetEaFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "Buffer"),
            ("ULONG", "Length"),
        ],
    });

    map.insert("NtCreateDebugObject", FunctionSignature {
        name: "NtCreateDebugObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DebugObjectHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtSinglePhaseReject", FunctionSignature {
        name: "NtSinglePhaseReject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("NtDeleteAtom", FunctionSignature {
        name: "NtDeleteAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("USHORT", "Atom"),
        ],
    });

    map.insert("NtRevertContainerImpersonation", FunctionSignature {
        name: "NtRevertContainerImpersonation",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtCreateKeyTransacted", FunctionSignature {
        name: "NtCreateKeyTransacted",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "TitleIndex"),
            ("PUNICODE_STRING", "Class"),
            ("ULONG", "CreateOptions"),
            ("HANDLE", "TransactionHandle"),
            ("PULONG", "Disposition"),
        ],
    });

    map.insert("ZwCreateCrossVmMutant", FunctionSignature {
        name: "ZwCreateCrossVmMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "MutantHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "VmHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwDuplicateObject", FunctionSignature {
        name: "ZwDuplicateObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SourceProcessHandle"),
            ("HANDLE", "SourceHandle"),
            ("HANDLE", "TargetProcessHandle"),
            ("PHANDLE", "TargetHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("ULONG", "Options"),
        ],
    });

    map.insert("NtRegisterThreadTerminatePort", FunctionSignature {
        name: "NtRegisterThreadTerminatePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("ZwAlertResumeThread", FunctionSignature {
        name: "ZwAlertResumeThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });

    map.insert("NtImpersonateClientOfPort", FunctionSignature {
        name: "NtImpersonateClientOfPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "PortMessage"),
        ],
    });

    map.insert("NtPrePrepareComplete", FunctionSignature {
        name: "NtPrePrepareComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("ZwQueueApcThreadEx2", FunctionSignature {
        name: "ZwQueueApcThreadEx2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("HANDLE", "Reserved"),
            ("PKNORMAL_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcArgument1"),
            ("PVOID", "ApcArgument2"),
            ("PVOID", "ApcArgument3"),
            ("ULONG", "Flags"),
            ("ULONG", "Reserved2"),
        ],
    });

    map.insert("ZwSetInformationTransaction", FunctionSignature {
        name: "ZwSetInformationTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("TRANSACTION_INFORMATION_CLASS", "TransactionInformationClass"),
            ("PVOID", "TransactionInformation"),
            ("ULONG", "TransactionInformationLength"),
        ],
    });

    map.insert("ZwWaitHighEventPair", FunctionSignature {
        name: "ZwWaitHighEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtAlpcConnectPortEx", FunctionSignature {
        name: "NtAlpcConnectPortEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PUNICODE_STRING", "PortName"),
            ("PALPC_PORT_ATTRIBUTES", "PortAttributes"),
            ("PALPC_HANDLE", "ConnectionPort"),
            ("ULONG", "Flags"),
            ("PSID", "RequiredServerSid"),
            ("PALPC_MESSAGE_ATTRIBUTES", "MessageAttributes"),
            ("PVOID", "ConnectData"),
            ("PULONG", "ConnectDataLength"),
            ("PVOID", "ReceiveMessage"),
            ("PULONG", "ReceiveMessageLength"),
        ],
    });

    map.insert("ZwGetCachedSigningLevel", FunctionSignature {
        name: "ZwGetCachedSigningLevel",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PULONG", "Flags"),
            ("PULONG", "SigningLevel"),
            ("PUCHAR", "Thumbprint"),
            ("PULONG", "ThumbprintSize"),
            ("PULONG", "SectionFlags"),
        ],
    });

    map.insert("NtAcceptConnectPort", FunctionSignature {
        name: "NtAcceptConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PVOID", "PortContext"),
            ("HANDLE", "ConnectionPortHandle"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
            ("PVOID", "ServerView"),
            ("PULONG", "ServerViewSize"),
        ],
    });

    map.insert("ZwOpenTransactionManager", FunctionSignature {
        name: "ZwOpenTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "LogFileName"),
            ("LUID", "TmIdentity"),
            ("ULONG", "OpenOptions"),
        ],
    });

    map.insert("NtSetDefaultHardErrorPort", FunctionSignature {
        name: "NtSetDefaultHardErrorPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
        ],
    });

    map.insert("NtQueryWnfStateNameInformation", FunctionSignature {
        name: "NtQueryWnfStateNameInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("WNF_STATE_NAME_INFORMATION_CLASS", "InformationClass"),
            ("PVOID", "Information"),
            ("ULONG", "InformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwQueryInstallUILanguage", FunctionSignature {
        name: "ZwQueryInstallUILanguage",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "InstallUILanguageId"),
        ],
    });

    map.insert("ZwSetThreadExecutionState", FunctionSignature {
        name: "ZwSetThreadExecutionState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ExecutionState"),
            ("PULONG", "PreviousExecutionState"),
        ],
    });

    map.insert("ZwSetLdtEntries", FunctionSignature {
        name: "ZwSetLdtEntries",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Selector1"),
            ("LDT_ENTRY", "LdtEntry1"),
            ("ULONG", "Selector2"),
            ("LDT_ENTRY", "LdtEntry2"),
        ],
    });

    map.insert("ZwInitializeEnclave", FunctionSignature {
        name: "ZwInitializeEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("SIZE_T", "ZeroBits"),
            ("PVOID", "EnclaveInformation"),
            ("ULONG", "InformationLength"),
        ],
    });

    map.insert("ZwDeletePrivateNamespace", FunctionSignature {
        name: "ZwDeletePrivateNamespace",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "NamespaceHandle"),
        ],
    });

    map.insert("NtSetDefaultLocale", FunctionSignature {
        name: "NtSetDefaultLocale",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DefaultLocaleId"),
            ("BOOLEAN", "UserProfile"),
        ],
    });

    map.insert("ZwDeleteAtom", FunctionSignature {
        name: "ZwDeleteAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("USHORT", "Atom"),
        ],
    });

    map.insert("NtSetInformationJobObject", FunctionSignature {
        name: "NtSetInformationJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "JobHandle"),
            ("JOBOBJECTINFOCLASS", "JobObjectInformationClass"),
            ("PVOID", "JobObjectInformation"),
            ("ULONG", "JobObjectInformationLength"),
        ],
    });

    map.insert("NtCreateDirectoryObject", FunctionSignature {
        name: "NtCreateDirectoryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DirectoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwSubscribeWnfStateChange", FunctionSignature {
        name: "ZwSubscribeWnfStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("WNF_CHANGE_STAMP", "ChangeStamp"),
            ("ULONG", "Flags"),
            ("HANDLE", "EventHandle"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PWNF_USER_SUBSCRIPTION", "Subscription"),
        ],
    });

    map.insert("ZwRequestWaitReplyPort", FunctionSignature {
        name: "ZwRequestWaitReplyPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("PVOID", "RequestMessage"),
            ("PVOID", "ReplyMessage"),
        ],
    });

    map.insert("ZwSignalAndWaitForSingleObject", FunctionSignature {
        name: "ZwSignalAndWaitForSingleObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SignalHandle"),
            ("HANDLE", "WaitHandle"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwQueryInformationEnlistment", FunctionSignature {
        name: "ZwQueryInformationEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("ENLISTMENT_INFORMATION_CLASS", "EnlistmentInformationClass"),
            ("PVOID", "EnlistmentInformation"),
            ("ULONG", "EnlistmentInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwFindAtom", FunctionSignature {
        name: "ZwFindAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "AtomName"),
            ("ULONG", "Length"),
            ("PUSHORT", "Atom"),
        ],
    });

    map.insert("ZwEnumerateKey", FunctionSignature {
        name: "ZwEnumerateKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Index"),
            ("KEY_INFORMATION_CLASS", "KeyInformationClass"),
            ("PVOID", "KeyInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("ZwMapUserPhysicalPagesScatter", FunctionSignature {
        name: "ZwMapUserPhysicalPagesScatter",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PVOID", "VirtualAddresses"),
            ("ULONG_PTR", "NumberOfPages"),
            ("PULONG_PTR", "UserPfnArray"),
        ],
    });

    map.insert("NtDisableLastKnownGood", FunctionSignature {
        name: "NtDisableLastKnownGood",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("NtAccessCheckByTypeResultListAndAuditAlarm", FunctionSignature {
        name: "NtAccessCheckByTypeResultListAndAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PULONG", "GrantedAccessList"),
            ("PULONG", "AccessStatusList"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwAcquireProcessActivityReference", FunctionSignature {
        name: "ZwAcquireProcessActivityReference",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ActivityReferenceHandle"),
            ("HANDLE", "ParentProcessHandle"),
            ("ULONG", "ProcessActivityType"),
        ],
    });

    map.insert("ZwWaitForAlertByThreadId", FunctionSignature {
        name: "ZwWaitForAlertByThreadId",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadId"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("ZwSetValueKey", FunctionSignature {
        name: "ZwSetValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PUNICODE_STRING", "ValueName"),
            ("ULONG", "TitleIndex"),
            ("ULONG", "Type"),
            ("PVOID", "Data"),
            ("ULONG", "DataSize"),
        ],
    });

    map.insert("ZwCreateIoCompletion", FunctionSignature {
        name: "ZwCreateIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "IoCompletionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "Count"),
        ],
    });

    map.insert("ZwAssociateWaitCompletionPacket", FunctionSignature {
        name: "ZwAssociateWaitCompletionPacket",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WaitCompletionPacketHandle"),
            ("HANDLE", "CompletionPacketHandle"),
            ("HANDLE", "WaitObject"),
            ("ULONG", "Flags"),
            ("ULONG_PTR", "KeyContext"),
            ("PVOID", "ApcContext"),
        ],
    });

    map.insert("NtCreateEnlistment", FunctionSignature {
        name: "NtCreateEnlistment",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EnlistmentHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ResourceManagerHandle"),
            ("HANDLE", "TransactionHandle"),
            ("ULONG", "CreateOptions"),
            ("ULONG", "NotificationMask"),
            ("PVOID", "EnlistmentKey"),
        ],
    });

    map.insert("NtOpenJobObject", FunctionSignature {
        name: "NtOpenJobObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "JobHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwGetNotificationResourceManager", FunctionSignature {
        name: "ZwGetNotificationResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("PTRANSACTION_NOTIFICATION", "TransactionNotification"),
            ("ULONG", "NotificationLength"),
            ("PLARGE_INTEGER", "Timeout"),
            ("PULONG", "ReturnLength"),
            ("ULONG", "Asynchronous"),
            ("ULONG_PTR", "AsynchronousContext"),
        ],
    });

    map.insert("ZwCreateProcess", FunctionSignature {
        name: "ZwCreateProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProcessHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ParentProcess"),
            ("BOOLEAN", "InheritObjectTable"),
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "DebugPort"),
            ("HANDLE", "ExceptionPort"),
        ],
    });

    map.insert("NtSetVolumeInformationFile", FunctionSignature {
        name: "NtSetVolumeInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FsInformation"),
            ("ULONG", "Length"),
            ("FSINFOCLASS", "FsInformationClass"),
        ],
    });

    map.insert("ZwDeleteWnfStateData", FunctionSignature {
        name: "ZwDeleteWnfStateData",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwCreateToken", FunctionSignature {
        name: "ZwCreateToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("TOKEN_TYPE", "Type"),
            ("PLUID", "AuthenticationId"),
            ("PLARGE_INTEGER", "ExpirationTime"),
            ("PTOKEN_USER", "User"),
            ("PTOKEN_GROUPS", "Groups"),
            ("PTOKEN_PRIVILEGES", "Privileges"),
            ("PTOKEN_OWNER", "Owner"),
            ("PTOKEN_PRIMARY_GROUP", "PrimaryGroup"),
            ("PTOKEN_DEFAULT_DACL", "DefaultDacl"),
            ("PTOKEN_SOURCE", "Source"),
        ],
    });

    map.insert("ZwDeleteKey", FunctionSignature {
        name: "ZwDeleteKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("ZwFilterBootOption", FunctionSignature {
        name: "ZwFilterBootOption",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "BootOptionId"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("NtOpenKeyTransacted", FunctionSignature {
        name: "NtOpenKeyTransacted",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "TransactionHandle"),
        ],
    });

    map.insert("ZwCreateWnfStateName", FunctionSignature {
        name: "ZwCreateWnfStateName",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("ULONG", "NameLifetime"),
            ("ULONG", "DataScope"),
            ("BOOLEAN", "PermanentData"),
            ("PWNF_TYPE_ID", "TypeId"),
            ("ULONG", "MaximumStateSize"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
        ],
    });

    map.insert("NtCreateSymbolicLinkObject", FunctionSignature {
        name: "NtCreateSymbolicLinkObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "LinkHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PUNICODE_STRING", "LinkTarget"),
        ],
    });

    map.insert("ZwQueryEvent", FunctionSignature {
        name: "ZwQueryEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("EVENT_INFORMATION_CLASS", "EventInformationClass"),
            ("PVOID", "EventInformation"),
            ("ULONG", "EventInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwCreateThreadEx", FunctionSignature {
        name: "ZwCreateThreadEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "StartRoutine"),
            ("PVOID", "Argument"),
            ("ULONG", "CreateFlags"),
            ("SIZE_T", "ZeroBits"),
            ("SIZE_T", "StackSize"),
            ("SIZE_T", "MaximumStackSize"),
            ("PPS_ATTRIBUTE_LIST", "AttributeList"),
        ],
    });

    map.insert("NtChangeThreadState", FunctionSignature {
        name: "NtChangeThreadState",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ULONG", "TargetState"),
        ],
    });

    map.insert("ZwDisplayString", FunctionSignature {
        name: "ZwDisplayString",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "String"),
        ],
    });

    map.insert("ZwReleaseMutant", FunctionSignature {
        name: "ZwReleaseMutant",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "MutantHandle"),
            ("PLONG", "PreviousCount"),
        ],
    });

    map.insert("NtFindAtom", FunctionSignature {
        name: "NtFindAtom",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "AtomName"),
            ("ULONG", "Length"),
            ("PUSHORT", "Atom"),
        ],
    });

    map.insert("ZwApphelpCacheControl", FunctionSignature {
        name: "ZwApphelpCacheControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "ControlCode"),
            ("PVOID", "Data"),
        ],
    });

    map.insert("ZwPssCaptureVaSpaceBulk", FunctionSignature {
        name: "ZwPssCaptureVaSpaceBulk",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SnapshotHandle"),
            ("PULONG", "PageCount"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("ZwCreateIRTimer", FunctionSignature {
        name: "ZwCreateIRTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TimerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwQueryInformationTransaction", FunctionSignature {
        name: "ZwQueryInformationTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("TRANSACTION_INFORMATION_CLASS", "TransactionInformationClass"),
            ("PVOID", "TransactionInformation"),
            ("ULONG", "TransactionInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwCreateThreadStateChange", FunctionSignature {
        name: "ZwCreateThreadStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "StateChangeHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ThreadHandle"),
            ("HANDLE", "ProcessHandle"),
        ],
    });

    map.insert("NtUnloadKey", FunctionSignature {
        name: "NtUnloadKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "TargetKey"),
        ],
    });

    map.insert("ZwOpenResourceManager", FunctionSignature {
        name: "ZwOpenResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ResourceManagerHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "TmHandle"),
            ("LUID", "ResourceManagerId"),
        ],
    });

    map.insert("ZwOpenKeyTransactedEx", FunctionSignature {
        name: "ZwOpenKeyTransactedEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "OpenOptions"),
            ("HANDLE", "TransactionHandle"),
        ],
    });

    map.insert("NtFlushWriteBuffer", FunctionSignature {
        name: "NtFlushWriteBuffer",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwSetCachedSigningLevel2", FunctionSignature {
        name: "ZwSetCachedSigningLevel2",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "SigningLevel"),
            ("PUCHAR", "Thumbprint"),
            ("ULONG", "ThumbprintSize"),
            ("ULONG", "SectionFlags"),
        ],
    });

    map.insert("ZwSetEventEx", FunctionSignature {
        name: "ZwSetEventEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
            ("LONG", "EventNumber"),
            ("PLONG", "PreviousState"),
        ],
    });

    map.insert("ZwSaveKeyEx", FunctionSignature {
        name: "ZwSaveKeyEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "FileHandle"),
            ("ULONG", "Format"),
        ],
    });

    map.insert("NtDeleteDriverEntry", FunctionSignature {
        name: "NtDeleteDriverEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "DriverEntry"),
        ],
    });

    map.insert("NtRollbackComplete", FunctionSignature {
        name: "NtRollbackComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("NtLockProductActivationKeys", FunctionSignature {
        name: "NtLockProductActivationKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Unknown"),
        ],
    });

    map.insert("ZwCreateProfile", FunctionSignature {
        name: "ZwCreateProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ProfileHandle"),
            ("HANDLE", "Process"),
            ("PVOID", "RangeBase"),
            ("SIZE_T", "RangeSize"),
            ("ULONG", "BucketSize"),
            ("PULONG", "ProfileInfo"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwQuerySystemInformation", FunctionSignature {
        name: "ZwQuerySystemInformation",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SYSTEM_INFORMATION_CLASS", "SystemInformationClass"),
            ("PVOID", "SystemInformation"),
            ("ULONG", "SystemInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtQueryDriverEntryOrder", FunctionSignature {
        name: "NtQueryDriverEntryOrder",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "LoadOrder"),
            ("PULONG", "Number"),
        ],
    });

    map.insert("NtGetNextThread", FunctionSignature {
        name: "NtGetNextThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ULONG", "HandleAttributes"),
            ("ULONG", "Flags"),
            ("PHANDLE", "NextThreadHandle"),
        ],
    });

    map.insert("ZwNotifyChangeKey", FunctionSignature {
        name: "ZwNotifyChangeKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("HANDLE", "Event"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("ULONG", "CompletionFilter"),
            ("BOOLEAN", "WatchTree"),
            ("PVOID", "ChangeBuffer"),
            ("ULONG", "BufferSize"),
            ("BOOLEAN", "Asynchronous"),
        ],
    });

    map.insert("ZwQueryFullAttributesFile", FunctionSignature {
        name: "ZwQueryFullAttributesFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PFILE_NETWORK_OPEN_INFORMATION", "FileInformation"),
        ],
    });

    map.insert("NtQueryPortInformationProcess", FunctionSignature {
        name: "NtQueryPortInformationProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "PortFlags"),
        ],
    });

    map.insert("ZwReadVirtualMemoryEx", FunctionSignature {
        name: "ZwReadVirtualMemoryEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PSIZE_T", "NumberOfBytesRead"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwQueryVirtualMemory", FunctionSignature {
        name: "ZwQueryVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("MEMORY_INFORMATION_CLASS", "MemoryInformationClass"),
            ("PVOID", "MemoryInformation"),
            ("SIZE_T", "MemoryInformationLength"),
            ("PSIZE_T", "ReturnLength"),
        ],
    });

    map.insert("NtQueryIntervalProfile", FunctionSignature {
        name: "NtQueryIntervalProfile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("KPROFILE_SOURCE", "Source"),
            ("PULONG", "Interval"),
        ],
    });

    map.insert("NtCreateEventPair", FunctionSignature {
        name: "NtCreateEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventPairHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwMakePermanentObject", FunctionSignature {
        name: "ZwMakePermanentObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "Handle"),
        ],
    });

    map.insert("ZwAlpcCreatePortSection", FunctionSignature {
        name: "ZwAlpcCreatePortSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("HANDLE", "SectionHandle"),
            ("SIZE_T", "SectionSize"),
            ("PALPC_HANDLE", "AlpcSectionHandle"),
            ("PVOID", "SectionView"),
        ],
    });

    map.insert("NtCreateThread", FunctionSignature {
        name: "NtCreateThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("HANDLE", "ProcessHandle"),
            ("PCLIENT_ID", "ClientId"),
            ("PCONTEXT", "ThreadContext"),
            ("PINITIAL_TEB", "InitialTeb"),
            ("BOOLEAN", "CreateSuspended"),
        ],
    });

    map.insert("NtSubscribeWnfStateChange", FunctionSignature {
        name: "NtSubscribeWnfStateChange",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWNF_STATE_NAME", "StateName"),
            ("WNF_CHANGE_STAMP", "ChangeStamp"),
            ("ULONG", "Flags"),
            ("HANDLE", "EventHandle"),
            ("PIO_APC_ROUTINE", "ApcRoutine"),
            ("PVOID", "ApcContext"),
            ("PWNF_USER_SUBSCRIPTION", "Subscription"),
        ],
    });

    map.insert("NtPssCaptureVaSpaceBulk", FunctionSignature {
        name: "NtPssCaptureVaSpaceBulk",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SnapshotHandle"),
            ("PULONG", "PageCount"),
            ("PVOID", "Buffer"),
            ("PULONG", "BufferSize"),
        ],
    });

    map.insert("NtTerminateEnclave", FunctionSignature {
        name: "NtTerminateEnclave",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
        ],
    });

    map.insert("NtQueryTimer", FunctionSignature {
        name: "NtQueryTimer",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TimerHandle"),
            ("TIMER_INFORMATION_CLASS", "TimerInformationClass"),
            ("PVOID", "TimerInformation"),
            ("ULONG", "TimerInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwUnloadDriver", FunctionSignature {
        name: "ZwUnloadDriver",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "DriverServiceName"),
        ],
    });

    map.insert("NtSetWnfProcessNotificationEvent", FunctionSignature {
        name: "NtSetWnfProcessNotificationEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
        ],
    });

    map.insert("NtAccessCheckByType", FunctionSignature {
        name: "NtAccessCheckByType",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("PPRIVILEGE_SET", "PrivilegeSet"),
            ("PULONG", "PrivilegeSetLength"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PULONG", "AccessStatus"),
        ],
    });

    map.insert("ZwAcceptConnectPort", FunctionSignature {
        name: "ZwAcceptConnectPort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("PVOID", "PortContext"),
            ("HANDLE", "ConnectionPortHandle"),
            ("PSECURITY_QUALITY_OF_SERVICE", "SecurityQos"),
            ("PVOID", "ServerView"),
            ("PULONG", "ServerViewSize"),
        ],
    });

    map.insert("NtPrivilegeCheck", FunctionSignature {
        name: "NtPrivilegeCheck",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ClientToken"),
            ("PPRIVILEGE_SET", "RequiredPrivileges"),
            ("PBOOLEAN", "SubjectContextLocked"),
            ("PBOOLEAN", "Result"),
        ],
    });

    map.insert("ZwAddBootEntry", FunctionSignature {
        name: "ZwAddBootEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PBOOT_ENTRY", "BootEntry"),
        ],
    });

    map.insert("NtCreateSectionEx", FunctionSignature {
        name: "NtCreateSectionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SectionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PLARGE_INTEGER", "MaximumSize"),
            ("ULONG", "SectionPageProtection"),
            ("ULONG", "AllocationAttributes"),
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwTerminateProcess", FunctionSignature {
        name: "ZwTerminateProcess",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("NTSTATUS", "ExitStatus"),
        ],
    });

    map.insert("ZwCreateWaitCompletionPacket", FunctionSignature {
        name: "ZwCreateWaitCompletionPacket",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "WaitCompletionPacketHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwCreateIoRing", FunctionSignature {
        name: "ZwCreateIoRing",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "IoRingHandle"),
            ("ULONG", "Version"),
            ("ULONG", "SubmissionQueueSize"),
            ("ULONG", "CompletionQueueSize"),
            ("ULONG", "Flags"),
            ("PVOID", "SubmissionQueue"),
            ("PVOID", "CompletionQueue"),
        ],
    });

    map.insert("NtDirectGraphicsCall", FunctionSignature {
        name: "NtDirectGraphicsCall",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Command"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferSize"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferSize"),
        ],
    });

    map.insert("ZwAlpcDeleteResourceReserve", FunctionSignature {
        name: "ZwAlpcDeleteResourceReserve",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "ResourceReserve"),
        ],
    });

    map.insert("ZwTraceControl", FunctionSignature {
        name: "ZwTraceControl",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Operation"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferSize"),
            ("PVOID", "OutputBuffer"),
            ("ULONG", "OutputBufferSize"),
            ("PULONG", "OutputBufferUsed"),
        ],
    });

    map.insert("NtAlpcSendWaitReceivePort", FunctionSignature {
        name: "NtAlpcSendWaitReceivePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "SendMessage"),
            ("PALPC_MESSAGE_ATTRIBUTES", "SendAttributes"),
            ("PVOID", "ReceiveMessage"),
            ("PULONG", "ReceiveMessageLength"),
            ("PALPC_MESSAGE_ATTRIBUTES", "ReceiveAttributes"),
        ],
    });

    map.insert("ZwPropagationComplete", FunctionSignature {
        name: "ZwPropagationComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("ULONG", "PropagationId"),
            ("NTSTATUS", "PropagationStatus"),
            ("PULONG", "PropagationCount"),
        ],
    });

    map.insert("ZwSetCachedSigningLevel", FunctionSignature {
        name: "ZwSetCachedSigningLevel",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("ULONG", "Flags"),
            ("ULONG", "SigningLevel"),
            ("PUCHAR", "Thumbprint"),
            ("ULONG", "ThumbprintSize"),
            ("ULONG", "SectionFlags"),
        ],
    });

    map.insert("ZwQueryInformationThread", FunctionSignature {
        name: "ZwQueryInformationThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("THREADINFOCLASS", "ThreadInformationClass"),
            ("PVOID", "ThreadInformation"),
            ("ULONG", "ThreadInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwDrawText", FunctionSignature {
        name: "ZwDrawText",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WindowHandle"),
            ("PUNICODE_STRING", "Text"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwCreateDirectoryObject", FunctionSignature {
        name: "ZwCreateDirectoryObject",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "DirectoryHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwRevertContainerImpersonation", FunctionSignature {
        name: "ZwRevertContainerImpersonation",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwDeleteBootEntry", FunctionSignature {
        name: "ZwDeleteBootEntry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "BootEntryId"),
        ],
    });

    map.insert("ZwSetContextThread", FunctionSignature {
        name: "ZwSetContextThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PCONTEXT", "Context"),
        ],
    });

    map.insert("ZwAllocateVirtualMemoryEx", FunctionSignature {
        name: "ZwAllocateVirtualMemoryEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PSIZE_T", "RegionSize"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Protect"),
            ("PULONG", "ExtendedParameters"),
            ("ULONG", "ParameterCount"),
        ],
    });

    map.insert("ZwOpenKeyedEvent", FunctionSignature {
        name: "ZwOpenKeyedEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "KeyedEventHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwSetLowWaitHighEventPair", FunctionSignature {
        name: "ZwSetLowWaitHighEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventPairHandle"),
        ],
    });

    map.insert("ZwPrivilegedServiceAuditAlarm", FunctionSignature {
        name: "ZwPrivilegedServiceAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PUNICODE_STRING", "ServiceName"),
            ("HANDLE", "TokenHandle"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("BOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("NtCreateWaitablePort", FunctionSignature {
        name: "NtCreateWaitablePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwQuerySystemEnvironmentValueEx", FunctionSignature {
        name: "ZwQuerySystemEnvironmentValueEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "VariableName"),
            ("PWSTR", "VariableValue"),
            ("PUSHORT", "VariableValueLength"),
            ("PULONG", "Attributes"),
        ],
    });

    map.insert("ZwFlushKey", FunctionSignature {
        name: "ZwFlushKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
        ],
    });

    map.insert("ZwQueryInformationFile", FunctionSignature {
        name: "ZwQueryInformationFile",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "FileInformation"),
            ("ULONG", "Length"),
            ("FILE_INFORMATION_CLASS", "FileInformationClass"),
        ],
    });

    map.insert("NtOpenObjectAuditAlarm", FunctionSignature {
        name: "NtOpenObjectAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("ACCESS_MASK", "GrantedAccess"),
            ("PPRIVILEGE_SET", "Privileges"),
            ("BOOLEAN", "ObjectCreation"),
            ("BOOLEAN", "AccessGranted"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("NtEnumerateValueKey", FunctionSignature {
        name: "NtEnumerateValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Index"),
            ("KEY_VALUE_INFORMATION_CLASS", "KeyValueInformationClass"),
            ("PVOID", "KeyValueInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("ZwCancelIoFileEx", FunctionSignature {
        name: "ZwCancelIoFileEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FileHandle"),
            ("PIO_STATUS_BLOCK", "IoStatusBlock"),
            ("PVOID", "CancelContext"),
        ],
    });

    map.insert("ZwWaitForWorkViaWorkerFactory", FunctionSignature {
        name: "ZwWaitForWorkViaWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("PULONG", "WorkerCount"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "WorkerParameter"),
        ],
    });

    map.insert("ZwOpenSession", FunctionSignature {
        name: "ZwOpenSession",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "SessionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwClearEvent", FunctionSignature {
        name: "ZwClearEvent",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EventHandle"),
        ],
    });

    map.insert("NtAccessCheckAndAuditAlarm", FunctionSignature {
        name: "NtAccessCheckAndAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PACCESS_MASK", "GrantedAccess"),
            ("PBOOLEAN", "AccessStatus"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("NtInitializeRegistry", FunctionSignature {
        name: "NtInitializeRegistry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwMapViewOfSection", FunctionSignature {
        name: "ZwMapViewOfSection",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "SectionHandle"),
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG_PTR", "ZeroBits"),
            ("SIZE_T", "CommitSize"),
            ("PLARGE_INTEGER", "SectionOffset"),
            ("PSIZE_T", "ViewSize"),
            ("ULONG", "InheritDisposition"),
            ("ULONG", "AllocationType"),
            ("ULONG", "Win32Protect"),
        ],
    });

    map.insert("NtSetInformationWorkerFactory", FunctionSignature {
        name: "NtSetInformationWorkerFactory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "WorkerFactoryHandle"),
            ("WORKERFACTORYINFOCLASS", "WorkerFactoryInformationClass"),
            ("PVOID", "WorkerFactoryInformation"),
            ("ULONG", "WorkerFactoryInformationLength"),
        ],
    });

    map.insert("NtIsUILanguageComitted", FunctionSignature {
        name: "NtIsUILanguageComitted",
        return_type: "NTSTATUS",
        parameters: vec![],
    });

    map.insert("ZwReadVirtualMemory", FunctionSignature {
        name: "ZwReadVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PSIZE_T", "NumberOfBytesRead"),
        ],
    });

    map.insert("ZwWaitForMultipleObjects32", FunctionSignature {
        name: "ZwWaitForMultipleObjects32",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Count"),
            ("PHANDLE", "Handles"),
            ("WAIT_TYPE", "WaitType"),
            ("BOOLEAN", "Alertable"),
            ("PLARGE_INTEGER", "Timeout"),
        ],
    });

    map.insert("NtCreatePort", FunctionSignature {
        name: "NtCreatePort",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PortHandle"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("ULONG", "MaxConnectionInfoLength"),
            ("ULONG", "MaxMessageLength"),
            ("ULONG", "MaxPoolUsage"),
        ],
    });

    map.insert("ZwQueryLicenseValue", FunctionSignature {
        name: "ZwQueryLicenseValue",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PWSTR", "ValueName"),
            ("PULONG", "Type"),
            ("PVOID", "Data"),
            ("ULONG", "DataSize"),
            ("PULONG", "ResultDataSize"),
        ],
    });

    map.insert("ZwInitializeRegistry", FunctionSignature {
        name: "ZwInitializeRegistry",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "Flags"),
        ],
    });

    map.insert("NtUnloadDriver", FunctionSignature {
        name: "NtUnloadDriver",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "DriverServiceName"),
        ],
    });

    map.insert("ZwOpenEventPair", FunctionSignature {
        name: "ZwOpenEventPair",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "EventPairHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwThawTransactions", FunctionSignature {
        name: "ZwThawTransactions",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "TmHandle"),
        ],
    });

    map.insert("ZwSuspendThread", FunctionSignature {
        name: "ZwSuspendThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });

    map.insert("ZwCreatePartition", FunctionSignature {
        name: "ZwCreatePartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "PartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("ZwQuerySystemInformationEx", FunctionSignature {
        name: "ZwQuerySystemInformationEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("SYSTEM_INFORMATION_CLASS", "SystemInformationClass"),
            ("PVOID", "InputBuffer"),
            ("ULONG", "InputBufferLength"),
            ("PVOID", "SystemInformation"),
            ("ULONG", "SystemInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("ZwResumeThread", FunctionSignature {
        name: "ZwResumeThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ThreadHandle"),
            ("PULONG", "PreviousSuspendCount"),
        ],
    });

    map.insert("ZwQuerySecurityPolicy", FunctionSignature {
        name: "ZwQuerySecurityPolicy",
        return_type: "NTSTATUS",
        parameters: vec![
            ("ULONG", "SecurityPolicyId"),
            ("PVOID", "Buffer"),
            ("ULONG", "BufferSize"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtCompareObjects", FunctionSignature {
        name: "NtCompareObjects",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "FirstObjectHandle"),
            ("HANDLE", "SecondObjectHandle"),
        ],
    });

    map.insert("NtSetUuidSeed", FunctionSignature {
        name: "NtSetUuidSeed",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "Seed"),
        ],
    });

    map.insert("NtPrepareComplete", FunctionSignature {
        name: "NtPrepareComplete",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "EnlistmentHandle"),
            ("PVOID", "TmVirtualClock"),
        ],
    });

    map.insert("NtSetInformationTransaction", FunctionSignature {
        name: "NtSetInformationTransaction",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionHandle"),
            ("TRANSACTION_INFORMATION_CLASS", "TransactionInformationClass"),
            ("PVOID", "TransactionInformation"),
            ("ULONG", "TransactionInformationLength"),
        ],
    });

    map.insert("ZwDuplicateToken", FunctionSignature {
        name: "ZwDuplicateToken",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ExistingTokenHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("BOOLEAN", "EffectiveOnly"),
            ("TOKEN_TYPE", "TokenType"),
            ("PHANDLE", "NewTokenHandle"),
        ],
    });

    map.insert("NtEnumerateDriverEntries", FunctionSignature {
        name: "NtEnumerateDriverEntries",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PULONG", "LoadOrder"),
            ("PULONG", "Number"),
        ],
    });

    map.insert("ZwCreateCpuPartition", FunctionSignature {
        name: "ZwCreateCpuPartition",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "CpuPartitionHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
        ],
    });

    map.insert("NtAcquireProcessActivityReference", FunctionSignature {
        name: "NtAcquireProcessActivityReference",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ActivityReferenceHandle"),
            ("HANDLE", "ParentProcessHandle"),
            ("ULONG", "ProcessActivityType"),
        ],
    });

    map.insert("NtSetInformationResourceManager", FunctionSignature {
        name: "NtSetInformationResourceManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ResourceManagerHandle"),
            ("RESOURCEMANAGER_INFORMATION_CLASS", "ResourceManagerInformationClass"),
            ("PVOID", "ResourceManagerInformation"),
            ("ULONG", "ResourceManagerInformationLength"),
        ],
    });

    map.insert("NtUnmapViewOfSectionEx", FunctionSignature {
        name: "NtUnmapViewOfSectionEx",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("ULONG", "Flags"),
        ],
    });

    map.insert("ZwEnumerateValueKey", FunctionSignature {
        name: "ZwEnumerateValueKey",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("ULONG", "Index"),
            ("KEY_VALUE_INFORMATION_CLASS", "KeyValueInformationClass"),
            ("PVOID", "KeyValueInformation"),
            ("ULONG", "Length"),
            ("PULONG", "ResultLength"),
        ],
    });

    map.insert("ZwSetInformationTransactionManager", FunctionSignature {
        name: "ZwSetInformationTransactionManager",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "TransactionManagerHandle"),
            ("TRANSACTIONMANAGER_INFORMATION_CLASS", "TransactionManagerInformationClass"),
            ("PVOID", "TransactionManagerInformation"),
            ("ULONG", "TransactionManagerInformationLength"),
        ],
    });

    map.insert("NtReadVirtualMemory", FunctionSignature {
        name: "NtReadVirtualMemory",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "ProcessHandle"),
            ("PVOID", "BaseAddress"),
            ("PVOID", "Buffer"),
            ("SIZE_T", "BufferSize"),
            ("PSIZE_T", "NumberOfBytesRead"),
        ],
    });

    map.insert("NtQueryIoCompletion", FunctionSignature {
        name: "NtQueryIoCompletion",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "IoCompletionHandle"),
            ("IO_COMPLETION_INFORMATION_CLASS", "IoCompletionInformationClass"),
            ("PVOID", "IoCompletionInformation"),
            ("ULONG", "IoCompletionInformationLength"),
            ("PULONG", "ReturnLength"),
        ],
    });

    map.insert("NtAccessCheckByTypeResultListAndAuditAlarmByHandle", FunctionSignature {
        name: "NtAccessCheckByTypeResultListAndAuditAlarmByHandle",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("HANDLE", "ClientToken"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PULONG", "GrantedAccessList"),
            ("PULONG", "AccessStatusList"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map.insert("ZwOpenThread", FunctionSignature {
        name: "ZwOpenThread",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PHANDLE", "ThreadHandle"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_ATTRIBUTES", "ObjectAttributes"),
            ("PCLIENT_ID", "ClientId"),
        ],
    });

    map.insert("ZwAlpcDeleteSecurityContext", FunctionSignature {
        name: "ZwAlpcDeleteSecurityContext",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "PortHandle"),
            ("ULONG", "Flags"),
            ("PVOID", "Context"),
        ],
    });

    map.insert("ZwQueryOpenSubKeys", FunctionSignature {
        name: "ZwQueryOpenSubKeys",
        return_type: "NTSTATUS",
        parameters: vec![
            ("HANDLE", "KeyHandle"),
            ("PULONG", "SubKeyCount"),
        ],
    });

    map.insert("ZwAccessCheckByTypeResultListAndAuditAlarm", FunctionSignature {
        name: "ZwAccessCheckByTypeResultListAndAuditAlarm",
        return_type: "NTSTATUS",
        parameters: vec![
            ("PUNICODE_STRING", "SubsystemName"),
            ("PVOID", "HandleId"),
            ("PUNICODE_STRING", "ObjectTypeName"),
            ("PUNICODE_STRING", "ObjectName"),
            ("PSECURITY_DESCRIPTOR", "SecurityDescriptor"),
            ("HANDLE", "ClientToken"),
            ("ACCESS_MASK", "DesiredAccess"),
            ("POBJECT_TYPE_LIST", "ObjectTypeList"),
            ("ULONG", "ObjectTypeListLength"),
            ("PGENERIC_MAPPING", "GenericMapping"),
            ("BOOLEAN", "ObjectCreation"),
            ("PULONG", "GrantedAccessList"),
            ("PULONG", "AccessStatusList"),
            ("PBOOLEAN", "GenerateOnClose"),
        ],
    });

    map
});