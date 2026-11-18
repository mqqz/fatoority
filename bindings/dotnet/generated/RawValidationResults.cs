using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ValidationResults
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_status", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionValidationStatus Status(ValidationResults* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_status_raw", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid StatusRaw(ValidationResults* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_info_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint InfoLen(ValidationResults* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_info_message", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationMessageBindingError InfoMessage(ValidationResults* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_warning_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint WarningLen(ValidationResults* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_warning_message", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationMessageBindingError WarningMessage(ValidationResults* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_error_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint ErrorLen(ValidationResults* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_error_message", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultValidationMessageBindingError ErrorMessage(ValidationResults* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_ValidationResults_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ValidationResults* handle);
}