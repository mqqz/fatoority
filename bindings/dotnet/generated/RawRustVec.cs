using System.Runtime.InteropServices;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

internal static unsafe class RustVec
{
    [DllImport(DiplomatNativeLib.Name, EntryPoint = "diplomat_owned_slice_u8_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void Destroy(byte* ptr, nuint len);
}