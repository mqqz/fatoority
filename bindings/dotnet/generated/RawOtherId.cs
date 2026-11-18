using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OtherId
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OtherId_value", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Value(OtherId* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OtherId_scheme", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionVoid Scheme(OtherId* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_OtherId_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OtherId* handle);
}