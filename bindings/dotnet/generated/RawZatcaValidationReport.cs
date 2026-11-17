using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ZatcaValidationReport
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_schema_version", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern uint SchemaVersion(ZatcaValidationReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_profile", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Profile(ZatcaValidationReport* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_evaluated_at", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError EvaluatedAt(ZatcaValidationReport* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_is_complete", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool IsComplete(ZatcaValidationReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_has_errors", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool HasErrors(ZatcaValidationReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_is_valid", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool IsValid(ZatcaValidationReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_stages_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint StagesLen(ZatcaValidationReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_stage", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaStageReportBindingError Stage(ZatcaValidationReport* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_to_json", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ToJson(ZatcaValidationReport* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaValidationReport_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ZatcaValidationReport* handle);
}