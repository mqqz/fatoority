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
    internal static unsafe extern DiplomatOptionVoid MessageTypeRaw(ValidationMessage* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Code(ValidationMessage* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_category", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Category(ValidationMessage* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_message", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Message(ValidationMessage* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionValidationStatus Status(ValidationMessage* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_status_raw", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid StatusRaw(ValidationMessage* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationMessage_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ValidationMessage* handle);
}