using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct InvoiceLineItem
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_item_gross_price", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ItemGrossPrice(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_item_price_discount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ItemPriceDiscount(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_item_price_base_quantity", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ItemPriceBaseQuantity(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_item_price_base_quantity_unit_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError ItemPriceBaseQuantityUnitCode(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_adjustments_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint AdjustmentsLen(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_adjustment", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAdjustmentBindingError Adjustment(InvoiceLineItem* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_adjustment_is_charge", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBoolBindingError AdjustmentIsCharge(InvoiceLineItem* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_item_seller_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOtherIdBindingError ItemSellerIdentifier(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_item_buyer_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOtherIdBindingError ItemBuyerIdentifier(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_item_standard_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOtherIdBindingError ItemStandardIdentifier(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_with_document_reference", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceLineItemBindingError WithDocumentReference(InvoiceLineItem* handle, OriginalInvoiceRef* reference);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_with_tax_subtotal", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceLineItemBindingError WithTaxSubtotal(InvoiceLineItem* handle, VatBreakdown* subtotal);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_document_references_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint DocumentReferencesLen(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_document_reference", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOriginalInvoiceRefBindingError DocumentReference(InvoiceLineItem* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_tax_subtotals_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint TaxSubtotalsLen(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_tax_subtotal", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVatBreakdownBindingError TaxSubtotal(InvoiceLineItem* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceLineItemBindingError New(DiplomatSliceU8 description, DiplomatSliceU8 quantity, DiplomatSliceU8 unitCode, DiplomatSliceU8 unitPrice, DiplomatSliceU8 rate, byte category);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_with_metadata", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceLineItemBindingError WithMetadata(InvoiceLineItem* handle, DiplomatOptionSliceU8 id, TaxExemption* exemption);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Id(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_exemption", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTaxExemptionBindingError Exemption(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_description", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Description(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_unit_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError UnitCode(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_quantity", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Quantity(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_unit_price", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError UnitPrice(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_total_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError TotalAmount(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_vat_rate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError VatRate(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_vat_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError VatAmount(InvoiceLineItem* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_has_vat_rate", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool HasVatRate(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_has_tax_total", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool HasTaxTotal(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_has_gross_amount", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool HasGrossAmount(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_vat_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte VatCategory(InvoiceLineItem* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceLineItem_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(InvoiceLineItem* handle);
}