using System;
using System.Runtime.InteropServices;

namespace Fatoora.Raw;

using Fatoora;
using Fatoora.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultPaymentMeansBindingError
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal PaymentMeans* ok;
        [FieldOffset(0)] internal BindingError* err;
    }

    private InnerUnion _inner;

    public DiplomatBool IsOk;
    public unsafe PaymentMeans* Ok => IsOk ? _inner.ok : throw new InvalidOperationException("Result does not contain Ok value");
    public unsafe BindingError* Err => !IsOk ? _inner.err : throw new InvalidOperationException("Result does not contain Err value");
}