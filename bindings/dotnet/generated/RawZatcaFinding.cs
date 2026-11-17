using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ZatcaFinding
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_assertion_site", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError AssertionSite(ZatcaFinding* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_layer", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ValidationLayer Layer(ZatcaFinding* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Code(ZatcaFinding* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_severity", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Severity Severity(ZatcaFinding* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_message", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Message(ZatcaFinding* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_location_kind", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionValidationLocationKind LocationKind(ZatcaFinding* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_location_path", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError LocationPath(ZatcaFinding* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_location_line", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionUInt LocationLine(ZatcaFinding* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_location_column", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionUInt LocationColumn(ZatcaFinding* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaFinding_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ZatcaFinding* handle);
}