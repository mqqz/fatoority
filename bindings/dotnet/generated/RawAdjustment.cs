using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Adjustment
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAdjustmentBindingError Amount(DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_percentage", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAdjustmentBindingError Percentage(DiplomatSliceU8 @base, DiplomatSliceU8 percent);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_with_supplied_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAdjustmentBindingError WithSuppliedAmount(Adjustment* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_with_reason", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAdjustmentBindingError WithReason(Adjustment* handle, DiplomatOptionSliceU8 reason, DiplomatOptionSliceU8 code);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_value", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Value(Adjustment* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_base_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError BaseAmount(Adjustment* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_percent", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Percent(Adjustment* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_reason", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Reason(Adjustment* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_reason_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ReasonCode(Adjustment* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Adjustment_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Adjustment* handle);
}