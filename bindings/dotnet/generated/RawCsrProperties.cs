using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct CsrProperties
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsrProperties_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsrPropertiesBindingError New(DiplomatSliceU8 commonName, DiplomatSliceU8 serialNumber, DiplomatSliceU8 organizationIdentifier, DiplomatSliceU8 organizationUnitName, DiplomatSliceU8 organizationName, DiplomatSliceU8 countryName, DiplomatSliceU8 invoiceType, DiplomatSliceU8 locationAddress, DiplomatSliceU8 industryBusinessCategory);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsrProperties_from_properties_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsrPropertiesBindingError FromPropertiesStr(DiplomatSliceU8 properties);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsrProperties_parse_csr_config_file", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsrPropertiesBindingError ParseCsrConfigFile(DiplomatSliceU8 path);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsrProperties_build", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultCsrBindingError Build(CsrProperties* handle, SigningKey* key, ZatcaEnvironment env);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_CsrProperties_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(CsrProperties* handle);
}