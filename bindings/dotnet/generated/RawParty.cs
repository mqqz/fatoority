using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Party
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Party_identifiers_len", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint IdentifiersLen(Party* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Party_identifier", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOtherIdBindingError Identifier(Party* handle, nuint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Party_address", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAddressBindingError Address(Party* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Party_vat_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVatIdBindingError VatId(Party* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Party_other_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOtherIdBindingError OtherId(Party* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Party_name", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Name(Party* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Party_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Party* handle);
}