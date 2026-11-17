using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct InvoiceBuilder
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_seller_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddSellerIdentifier(InvoiceBuilder* handle, DiplomatSliceU8 value, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_buyer_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddBuyerIdentifier(InvoiceBuilder* handle, DiplomatSliceU8 value, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_vat_breakdown", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddVatBreakdown(InvoiceBuilder* handle, VatBreakdown* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_payment_means", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddPaymentMeans(InvoiceBuilder* handle, PaymentMeans* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_note", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddNote(InvoiceBuilder* handle, DiplomatOptionSliceU8 language, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_allowance", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddAllowance(InvoiceBuilder* handle, Adjustment* value, VatCategory category, DiplomatSliceU8 rate, TaxExemption* exemption);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_charge", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddCharge(InvoiceBuilder* handle, Adjustment* value, VatCategory category, DiplomatSliceU8 rate, TaxExemption* exemption);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_buyer_without_address", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetBuyerWithoutAddress(InvoiceBuilder* handle, DiplomatSliceU8 name, DiplomatOptionSliceU8 vatId, DiplomatOptionSliceU8 otherId, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_new_with_reference", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceBuilderBindingError NewWithReference(InvoiceTypeKind kind, InvoiceSubType subtype, OriginalInvoiceRef* reference, DiplomatSliceU8 reason);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_payment_instruction", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetPaymentInstruction(InvoiceBuilder* handle, DiplomatSliceU8 instruction);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_prepaid_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetPrepaidAmount(InvoiceBuilder* handle, DiplomatSliceU8 amount);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_payable_rounding_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetPayableRoundingAmount(InvoiceBuilder* handle, DiplomatSliceU8 amount);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_delivery", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetDelivery(InvoiceBuilder* handle, DiplomatOptionSliceU8 actual, DiplomatOptionSliceU8 latest);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_tax_currency", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetTaxCurrency(InvoiceBuilder* handle, DiplomatSliceU8 currency, DiplomatSliceU8 amount);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_tax_exemption", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddTaxExemption(InvoiceBuilder* handle, VatCategory category, DiplomatSliceU8 rate, TaxExemption* exemption);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_adjustment", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddAdjustment(InvoiceBuilder* handle, AllowanceCharge* adjustment);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_item", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddItem(InvoiceBuilder* handle, InvoiceLineItem* item);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceBuilderBindingError New(InvoiceTypeKind kind, InvoiceSubType subtype, DiplomatOptionSliceU8 originalId, DiplomatOptionSliceU8 originalUuid, DiplomatOptionSliceU8 originalDate, DiplomatOptionSliceU8 reason);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_billing_reference", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddBillingReference(InvoiceBuilder* handle, OriginalInvoiceRef* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_purchase_order_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetPurchaseOrderId(InvoiceBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_contract_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetContractId(InvoiceBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetId(InvoiceBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_uuid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetUuid(InvoiceBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_issue_datetime", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetIssueDatetime(InvoiceBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_currency", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetCurrency(InvoiceBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_previous_invoice_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetPreviousInvoiceHash(InvoiceBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_invoice_counter", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetInvoiceCounter(InvoiceBuilder* handle, ulong value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_vat_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetVatCategory(InvoiceBuilder* handle, VatCategory value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_flags", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetFlags(InvoiceBuilder* handle, byte value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_seller", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetSeller(InvoiceBuilder* handle, DiplomatSliceU8 name, Address* address, DiplomatSliceU8 vatId, DiplomatOptionSliceU8 otherId, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_set_buyer", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetBuyer(InvoiceBuilder* handle, DiplomatSliceU8 name, Address* address, DiplomatOptionSliceU8 vatId, DiplomatOptionSliceU8 otherId, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_add_line_item", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddLineItem(InvoiceBuilder* handle, DiplomatSliceU8 description, DiplomatSliceU8 quantity, DiplomatSliceU8 unitCode, DiplomatSliceU8 unitPrice, DiplomatSliceU8 vatRate, VatCategory category);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_build", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultFinalizedInvoiceBindingError Build(InvoiceBuilder* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceBuilder_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(InvoiceBuilder* handle);
}