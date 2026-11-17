using System;
using System.Runtime.InteropServices;

namespace Fatoora.Raw;

using Fatoora;
using Fatoora.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatOptionUInt
{
    [StructLayout(LayoutKind.Explicit)]
    private struct InnerUnion
    {
        [FieldOffset(0)] internal uint value;
    }

    private InnerUnion _inner;

    public DiplomatBool IsSome;

    public uint Value => IsSome ? _inner.value : throw new InvalidOperationException("Option is None");
}