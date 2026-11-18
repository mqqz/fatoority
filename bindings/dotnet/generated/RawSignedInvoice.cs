using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct SignedInvoice
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_from_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSignedInvoiceBindingError FromXml(DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_from_file", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSignedInvoiceBindingError FromFile(DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_data", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceDataBindingError Data(SignedInvoice* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_totals", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceTotalsBindingError Totals(SignedInvoice* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Hash(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Xml(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_qr_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError QrCode(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_signature", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Signature(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_public_key", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError PublicKey(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_invoice_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError InvoiceHash(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_to_xml_base64", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ToXmlBase64(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_issuer", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Issuer(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_serial", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Serial(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_cert_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError CertHash(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_signed_props_hash", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SignedPropsHash(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_signing_time", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SigningTime(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_zatca_key_signature_present", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBoolBindingError ZatcaKeySignaturePresent(SignedInvoice* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_zatca_key_signature", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError ZatcaKeySignature(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_into_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError IntoXml(SignedInvoice* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_SignedInvoice_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(SignedInvoice* handle);
}