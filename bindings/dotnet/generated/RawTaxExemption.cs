using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct TaxExemption
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_TaxExemption_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTaxExemptionBindingError New(DiplomatOptionSliceU8 code, DiplomatOptionSliceU8 reason);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_TaxExemption_reason_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid ReasonCode(TaxExemption* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_TaxExemption_reason", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Reason(TaxExemption* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_TaxExemption_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(TaxExemption* handle);
}