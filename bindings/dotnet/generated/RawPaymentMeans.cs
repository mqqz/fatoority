using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct PaymentMeans
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_with_instruction_note", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultPaymentMeansBindingError WithInstructionNote(PaymentMeans* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_instruction_notes_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint InstructionNotesLen(PaymentMeans* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_instruction_note_at", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError InstructionNoteAt(PaymentMeans* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultPaymentMeansBindingError New(DiplomatSliceU8 code, DiplomatOptionSliceU8 instruction);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_with_account", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultPaymentMeansBindingError WithAccount(PaymentMeans* handle, DiplomatOptionSliceU8 id, DiplomatOptionSliceU8 terms);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Code(PaymentMeans* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_instruction_note", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError InstructionNote(PaymentMeans* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_has_account", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool HasAccount(PaymentMeans* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_account_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError AccountIdentifier(PaymentMeans* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_payment_terms", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError PaymentTerms(PaymentMeans* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_PaymentMeans_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(PaymentMeans* handle);
}