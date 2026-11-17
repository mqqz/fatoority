using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ZatcaStageReport
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_stage", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ZatcaStage Stage(ZatcaStageReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ZatcaStageStatus Status(ZatcaStageReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_provenance", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaRuleSourceBindingError Provenance(ZatcaStageReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_evaluated_assertions_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint EvaluatedAssertionsLen(ZatcaStageReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_evaluated_assertion", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError EvaluatedAssertion(ZatcaStageReport* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_findings_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint FindingsLen(ZatcaStageReport* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_finding", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaFindingBindingError Finding(ZatcaStageReport* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaStageReport_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ZatcaStageReport* handle);
}