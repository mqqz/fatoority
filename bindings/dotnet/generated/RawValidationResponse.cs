using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ValidationResponse
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_http_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionUShort HttpStatus(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_outcome", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern InvoiceOutcome Outcome(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_ensure_accepted", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError EnsureAccepted(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_cleared_invoice_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ClearedInvoiceXml(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_validation_results", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationResultsBindingError ValidationResults(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_cleared_invoice_base64", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ClearedInvoiceBase64(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_reporting_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionReportingStatus ReportingStatus(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_reporting_status_raw", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ReportingStatusRaw(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_clearance_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionClearanceStatus ClearanceStatus(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_clearance_status_raw", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ClearanceStatusRaw(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_qr_seller_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError QrSellerStatus(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_qr_buyer_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError QrBuyerStatus(ValidationResponse* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResponse_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ValidationResponse* handle);
}