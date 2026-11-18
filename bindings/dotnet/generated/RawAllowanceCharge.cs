using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct AllowanceCharge
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_has_vat_rate", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool HasVatRate(AllowanceCharge* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_omit_vat_rate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAllowanceChargeBindingError OmitVatRate(AllowanceCharge* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_with_percentage", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAllowanceChargeBindingError WithPercentage(AllowanceCharge* handle, DiplomatSliceU8 @base, DiplomatSliceU8 percent);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_base_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid BaseAmount(AllowanceCharge* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_percentage", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Percentage(AllowanceCharge* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAllowanceChargeBindingError New(AdjustmentKind kind, DiplomatSliceU8 amount, DiplomatOptionSliceU8 reason, DiplomatOptionSliceU8 reasonCode, VatCategory category, DiplomatSliceU8 rate, TaxExemption* exemption);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_kind", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern AdjustmentKind Kind(AllowanceCharge* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Amount(AllowanceCharge* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_rate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Rate(AllowanceCharge* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern VatCategory Category(AllowanceCharge* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_with_tax_category_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAllowanceChargeBindingError WithTaxCategoryCount(AllowanceCharge* handle, nuint count);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_tax_category_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint TaxCategoryCount(AllowanceCharge* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_reason", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Reason(AllowanceCharge* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_reason_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid ReasonCode(AllowanceCharge* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_exemption", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTaxExemptionBindingError Exemption(AllowanceCharge* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_AllowanceCharge_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(AllowanceCharge* handle);
}