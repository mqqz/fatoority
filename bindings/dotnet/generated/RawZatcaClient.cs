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

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_create_with_timeout", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaClientBindingError CreateWithTimeout(Config* config, uint timeoutMs);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_request_compliance_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultComplianceCsidBindingError RequestComplianceCsid(ZatcaClient* handle, Csr* csr, DiplomatSliceU8 otp);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_request_production_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionCsidBindingError RequestProductionCsid(ZatcaClient* handle, ComplianceCsid* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_renew_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionCsidBindingError RenewCsid(ZatcaClient* handle, ProductionCsid* credentials, Csr* csr, DiplomatSliceU8 otp, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_check_invoice_compliance", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResponseBindingError CheckInvoiceCompliance(ZatcaClient* handle, SignedInvoice* invoice, ComplianceCsid* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_report_simplified_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResponseBindingError ReportSimplifiedInvoice(ZatcaClient* handle, SignedInvoice* invoice, ProductionCsid* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_clear_standard_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResponseBindingError ClearStandardInvoice(ZatcaClient* handle, SignedInvoice* invoice, ProductionCsid* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_request_compliance_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultComplianceRequestBindingError PrepareRequestComplianceCsid(ZatcaClient* handle, Csr* csr, DiplomatSliceU8 otp);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_request_production_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionRequestBindingError PrepareRequestProductionCsid(ZatcaClient* handle, ComplianceCsid* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_renew_csid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultProductionRequestBindingError PrepareRenewCsid(ZatcaClient* handle, ProductionCsid* credentials, Csr* csr, DiplomatSliceU8 otp, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_check_invoice_compliance", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceRequestBindingError PrepareCheckInvoiceCompliance(ZatcaClient* handle, SignedInvoice* invoice, ComplianceCsid* credentials);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_report_simplified_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceRequestBindingError PrepareReportSimplifiedInvoice(ZatcaClient* handle, SignedInvoice* invoice, ProductionCsid* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_prepare_clear_standard_invoice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceRequestBindingError PrepareClearStandardInvoice(ZatcaClient* handle, SignedInvoice* invoice, ProductionCsid* credentials, ClearanceMode clearanceMode, DiplomatOptionSliceU8 acceptLanguage);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ZatcaClient_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ZatcaClient* handle);
}