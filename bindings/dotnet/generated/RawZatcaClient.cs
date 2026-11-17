using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ZatcaClient
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaClientBindingError Create(Config* config);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_post_csr_for_ccsid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsidComplianceBindingError PostCsrForCcsid(ZatcaClient* handle, Csr* csr, DiplomatSliceU8 otp);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_post_ccsid_for_pcsid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsidProductionBindingError PostCcsidForPcsid(ZatcaClient* handle, CsidCompliance* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_renew_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsidProductionBindingError RenewCsid(ZatcaClient* handle, CsidProduction* credentials, Csr* csr, DiplomatSliceU8 otp, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_check_invoice_compliance", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResponseBindingError CheckInvoiceCompliance(ZatcaClient* handle, SignedInvoice* invoice, CsidCompliance* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_report_simplified_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResponseBindingError ReportSimplifiedInvoice(ZatcaClient* handle, SignedInvoice* invoice, CsidProduction* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_clear_standard_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResponseBindingError ClearStandardInvoice(ZatcaClient* handle, SignedInvoice* invoice, CsidProduction* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_post_csr_for_ccsid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultComplianceRequestBindingError PreparePostCsrForCcsid(ZatcaClient* handle, Csr* csr, DiplomatSliceU8 otp);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_post_ccsid_for_pcsid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionRequestBindingError PreparePostCcsidForPcsid(ZatcaClient* handle, CsidCompliance* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_renew_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionRequestBindingError PrepareRenewCsid(ZatcaClient* handle, CsidProduction* credentials, Csr* csr, DiplomatSliceU8 otp, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_check_invoice_compliance", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceRequestBindingError PrepareCheckInvoiceCompliance(ZatcaClient* handle, SignedInvoice* invoice, CsidCompliance* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_report_simplified_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceRequestBindingError PrepareReportSimplifiedInvoice(ZatcaClient* handle, SignedInvoice* invoice, CsidProduction* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_clear_standard_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceRequestBindingError PrepareClearStandardInvoice(ZatcaClient* handle, SignedInvoice* invoice, CsidProduction* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ZatcaClient* handle);
}