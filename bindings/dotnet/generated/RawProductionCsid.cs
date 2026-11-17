using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ProductionCsid
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionCsid_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionCsidBindingError New(ZatcaEnvironment environment, DiplomatOptionSliceU8 requestId, DiplomatSliceU8 token, DiplomatSliceU8 secret);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionCsid_env", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaEnvironmentBindingError Env(ProductionCsid* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionCsid_request_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError RequestId(ProductionCsid* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionCsid_binary_security_token", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError BinarySecurityToken(ProductionCsid* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionCsid_secret", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Secret(ProductionCsid* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionCsid_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ProductionCsid* handle);
}