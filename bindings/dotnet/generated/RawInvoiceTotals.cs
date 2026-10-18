using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct InvoiceTotals
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_vat_breakdown_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint VatBreakdownLen(InvoiceTotals* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_vat_breakdown", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVatBreakdownBindingError VatBreakdown(InvoiceTotals* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_tax_inclusive", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError TaxInclusive(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_tax_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError TaxAmount(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_line_extension", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError LineExtension(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_allowance_total", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AllowanceTotal(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_charge_total", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ChargeTotal(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_taxable_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError TaxableAmount(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_prepaid_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError PrepaidAmount(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_payable_rounding_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError PayableRoundingAmount(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_payable_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError PayableAmount(InvoiceTotals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceTotals_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(InvoiceTotals* handle);
}