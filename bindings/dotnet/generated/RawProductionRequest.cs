using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ProductionRequest
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionRequest_method", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Method(ProductionRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionRequest_url", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Url(ProductionRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionRequest_headers_json", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError HeadersJson(ProductionRequest* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionRequest_body", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError Body(ProductionRequest* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionRequest_decode", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionCsidBindingError Decode(ProductionRequest* handle, ushort status, DiplomatSliceU8 body);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ProductionRequest_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ProductionRequest* handle);
}