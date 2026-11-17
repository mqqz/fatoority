using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct FinalizedInvoice
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_FinalizedInvoice_from_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultFinalizedInvoiceBindingError FromXml(DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_FinalizedInvoice_from_file", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultFinalizedInvoiceBindingError FromFile(DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_FinalizedInvoice_data", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceDataBindingError Data(FinalizedInvoice* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_FinalizedInvoice_totals", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceTotalsBindingError Totals(FinalizedInvoice* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_FinalizedInvoice_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Hash(FinalizedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_FinalizedInvoice_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Xml(FinalizedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_FinalizedInvoice_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(FinalizedInvoice* handle);
}