using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct InvoiceSigner
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceSigner_from_pem", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceSignerBindingError FromPem(DiplomatSliceU8 certPem, DiplomatSliceU8 keyPem);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceSigner_from_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultInvoiceSignerBindingError FromDer(DiplomatSliceU8 certDer, DiplomatSliceU8 keyDer);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceSigner_certificate_der", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultBytesBindingError CertificateDer(InvoiceSigner* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceSigner_certificate_pem", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError CertificatePem(InvoiceSigner* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceSigner_sign", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultSignedInvoiceBindingError Sign(InvoiceSigner* handle, FinalizedInvoice* invoice);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceSigner_sign_xml", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError SignXml(InvoiceSigner* handle, DiplomatSliceU8 xml, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_InvoiceSigner_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(InvoiceSigner* handle);
}