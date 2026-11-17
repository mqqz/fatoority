using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ComplianceRequest
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceRequest_method", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Method(ComplianceRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceRequest_url", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Url(ComplianceRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceRequest_headers_json", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError HeadersJson(ComplianceRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceRequest_body", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError Body(ComplianceRequest* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceRequest_decode", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultComplianceCsidBindingError Decode(ComplianceRequest* handle, ushort status, DiplomatSliceU8 body);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ComplianceRequest_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ComplianceRequest* handle);
}