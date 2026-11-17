using System;
using System.Runtime.InteropServices;

namespace Fatoora.Raw;

using Fatoora;
using Fatoora.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatOptionClearanceStatus
{
    [StructLayout(LayoutKind.Explicit)]
    private struct InnerUnion
    {
        [FieldOffset(0)] internal ClearanceStatus value;
    }

    private InnerUnion _inner;

    public DiplomatBool IsSome;

    public ClearanceStatus Value => IsSome ? _inner.value : throw new InvalidOperationException("Option is None");
}