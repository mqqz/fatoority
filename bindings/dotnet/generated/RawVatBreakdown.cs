using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct VatBreakdown
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatBreakdown_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVatBreakdownBindingError New(byte category, DiplomatSliceU8 rate, DiplomatSliceU8 taxableAmount, DiplomatSliceU8 taxAmount, TaxExemption* exemption);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatBreakdown_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte Category(VatBreakdown* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatBreakdown_rate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Rate(VatBreakdown* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatBreakdown_taxable_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError TaxableAmount(VatBreakdown* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatBreakdown_tax_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError TaxAmount(VatBreakdown* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatBreakdown_exemption", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTaxExemptionBindingError Exemption(VatBreakdown* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_VatBreakdown_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(VatBreakdown* handle);
}