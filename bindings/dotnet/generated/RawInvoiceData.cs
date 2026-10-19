using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct InvoiceData
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_issue_time", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError IssueTime(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_issue_date", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError IssueDate(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_payment_means_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint PaymentMeansLen(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_payment_means", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultPaymentMeansBindingError PaymentMeans(InvoiceData* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_purchase_order_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError PurchaseOrderId(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_contract_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ContractId(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_payment_instruction", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError PaymentInstruction(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_actual_delivery_date", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ActualDeliveryDate(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_latest_delivery_date", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError LatestDeliveryDate(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_tax_currency", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError TaxCurrency(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_tax_currency_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError TaxCurrencyAmount(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_adjustments_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint AdjustmentsLen(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_adjustment", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAllowanceChargeBindingError Adjustment(InvoiceData* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Id(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_uuid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Uuid(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_previous_invoice_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError PreviousInvoiceHash(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_payment_means_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError PaymentMeansCode(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_currency", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Currency(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_issue_datetime", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError IssueDatetime(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_invoice_level_charge", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError InvoiceLevelCharge(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_invoice_level_discount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError InvoiceLevelDiscount(InvoiceData* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_allowance_reason", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError AllowanceReason(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_invoice_counter", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong InvoiceCounter(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_vat_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte VatCategory(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_flags_raw", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte FlagsRaw(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_invoice_type_kind", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte InvoiceTypeKind(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_invoice_sub_type", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte InvoiceSubType(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_seller", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultPartyBindingError Seller(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_buyer", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultPartyBindingError Buyer(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_note", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceNoteBindingError Note(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_line_items_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint LineItemsLen(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_line_item", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceLineItemBindingError LineItem(InvoiceData* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_original_invoice_ref", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOriginalInvoiceRefBindingError OriginalInvoiceRef(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_original_invoice_reason", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError OriginalInvoiceReason(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_notes_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint NotesLen(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_note_at", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceNoteBindingError NoteAt(InvoiceData* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_billing_references_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint BillingReferencesLen(InvoiceData* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_billing_reference", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOriginalInvoiceRefBindingError BillingReference(InvoiceData* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceData_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(InvoiceData* handle);
}