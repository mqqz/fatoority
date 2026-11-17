using System;
using System.Runtime.InteropServices;
using Fatoora;
using Fatoora.Diplomat;

namespace Fatoora.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Config
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Config_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultConfigBindingError New(ZatcaEnvironment env);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Config_env", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultZatcaEnvironmentBindingError Env(Config* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "fatoora_Config_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Config* handle);
}