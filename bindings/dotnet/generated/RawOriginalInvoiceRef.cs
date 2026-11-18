using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OriginalInvoiceRef
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOriginalInvoiceRefBindingError New(DiplomatSliceU8 id, DiplomatOptionSliceU8 uuid, DiplomatOptionSliceU8 date, DiplomatOptionSliceU8 time, DiplomatOptionSliceU8 typeCode);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_issue_time", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid IssueTime(OriginalInvoiceRef* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_document_type_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid DocumentTypeCode(OriginalInvoiceRef* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Id(OriginalInvoiceRef* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_uuid", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Uuid(OriginalInvoiceRef* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_issue_date", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid IssueDate(OriginalInvoiceRef* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OriginalInvoiceRef_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OriginalInvoiceRef* handle);
}