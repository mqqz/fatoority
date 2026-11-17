using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct CsidProduction
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidProduction_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsidProductionBindingError Create(ZatcaEnvironment environment, DiplomatOptionSliceU8 requestId, DiplomatSliceU8 token, DiplomatSliceU8 secret);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidProduction_env", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaEnvironmentBindingError Env(CsidProduction* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidProduction_request_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError RequestId(CsidProduction* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidProduction_binary_security_token", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError BinarySecurityToken(CsidProduction* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidProduction_secret", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Secret(CsidProduction* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsidProduction_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(CsidProduction* handle);
}