using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Bytes
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Bytes_to_owned_slice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOwnedSliceU8 AsSlice(Bytes* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Bytes_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Bytes* handle);
}