using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct LineItemBuilder
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_tax_total_present", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetTaxTotalPresent(LineItemBuilder* handle, [MarshalAs(UnmanagedType.U1)] bool present);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_gross_amount_present", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetGrossAmountPresent(LineItemBuilder* handle, [MarshalAs(UnmanagedType.U1)] bool present);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern LineItemBuilder* New();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_build", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceLineItemBindingError Build(LineItemBuilder* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_name", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemName(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_invoiced_quantity_unit_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetInvoicedQuantityUnitCode(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemPriceBaseQuantityUnitCode(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetId(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_invoiced_quantity", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetInvoicedQuantity(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_net_price", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemNetPrice(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_gross_price", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemGrossPrice(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_price_discount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemPriceDiscount(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_price_base_quantity", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemPriceBaseQuantity(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_line_net_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetLineNetAmount(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_line_vat_amount", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetLineVatAmount(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_line_amount_with_vat", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetLineAmountWithVat(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_vat_rate", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetVatRate(LineItemBuilder* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_vat_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetVatCategory(LineItemBuilder* handle, byte value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_exemption", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetExemption(LineItemBuilder* handle, TaxExemption* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_add_allowance", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddAllowance(LineItemBuilder* handle, Adjustment* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_add_charge", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddCharge(LineItemBuilder* handle, Adjustment* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_add_document_reference", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddDocumentReference(LineItemBuilder* handle, OriginalInvoiceRef* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_add_tax_subtotal", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError AddTaxSubtotal(LineItemBuilder* handle, VatBreakdown* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_seller_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemSellerIdentifier(LineItemBuilder* handle, DiplomatSliceU8 value, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_buyer_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemBuyerIdentifier(LineItemBuilder* handle, DiplomatSliceU8 value, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_set_item_standard_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SetItemStandardIdentifier(LineItemBuilder* handle, DiplomatSliceU8 value, DiplomatOptionSliceU8 scheme);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_LineItemBuilder_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(LineItemBuilder* handle);
}