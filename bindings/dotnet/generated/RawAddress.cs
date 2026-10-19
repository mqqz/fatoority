using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Address
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_with_province", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAddressBindingError WithProvince(Address* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_province", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError Province(Address* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultAddressBindingError New(DiplomatSliceU8 countryCode, DiplomatSliceU8 city, DiplomatSliceU8 street, DiplomatSliceU8 buildingNumber, DiplomatSliceU8 postalCode, DiplomatOptionSliceU8 additionalStreet, DiplomatOptionSliceU8 additionalNumber, DiplomatOptionSliceU8 district);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_city", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError City(Address* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_street", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError Street(Address* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_building_number", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError BuildingNumber(Address* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_postal_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError PostalCode(Address* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_country_code", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidBindingError CountryCode(Address* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_additional_street", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError AdditionalStreet(Address* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_additional_number", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError AdditionalNumber(Address* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_district", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultTextBindingError District(Address* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Address_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Address* handle);
}