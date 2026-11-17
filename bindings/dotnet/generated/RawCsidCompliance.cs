using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct CsidCompliance
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidCompliance_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsidComplianceBindingError Create(ZatcaEnvironment environment, DiplomatOptionSliceU8 requestId, DiplomatSliceU8 token, DiplomatSliceU8 secret);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidCompliance_env", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaEnvironmentBindingError Env(CsidCompliance* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidCompliance_request_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError RequestId(CsidCompliance* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidCompliance_binary_security_token", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError BinarySecurityToken(CsidCompliance* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidCompliance_secret", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Secret(CsidCompliance* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidCompliance_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(CsidCompliance* handle);
}