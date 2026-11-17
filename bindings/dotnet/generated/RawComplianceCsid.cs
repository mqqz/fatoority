using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ComplianceCsid
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceCsid_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultComplianceCsidBindingError Create(ZatcaEnvironment environment, DiplomatOptionSliceU8 requestId, DiplomatSliceU8 token, DiplomatSliceU8 secret);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceCsid_env", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaEnvironmentBindingError Env(ComplianceCsid* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceCsid_request_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError RequestId(ComplianceCsid* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceCsid_binary_security_token", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError BinarySecurityToken(ComplianceCsid* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceCsid_secret", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Secret(ComplianceCsid* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceCsid_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ComplianceCsid* handle);
}