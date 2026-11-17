using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Xml
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Xml_validate_zatca", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaValidationReportBindingError ValidateZatca(DiplomatSliceU8 xml, DiplomatOptionSliceU8 optionsJson);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Xml_validate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBoolBindingError Validate(DiplomatSliceU8 xml);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Xml_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Hash(DiplomatSliceU8 xml, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Xml_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Xml* handle);
}