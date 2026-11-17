using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ValidationMessage
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_message_type", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionMessageType MessageType(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_message_type_raw", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError MessageTypeRaw(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Code(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Category(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_message", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Message(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionValidationStatus Status(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_status_raw", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError StatusRaw(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ValidationMessage* handle);
}