using System;
using System.Globalization;

namespace Fatoora;

#nullable enable

/// <summary>Exact conversions between <see cref="decimal"/> and the native decimal text.</summary>
/// <remarks>
/// The native decimal and <see cref="decimal"/> share the 96-bit mantissa and
/// 0-28 scale range, so the invariant-culture text round trip is lossless.
/// </remarks>
internal static class FatooraDecimal
{
    private const NumberStyles Style = NumberStyles.AllowLeadingSign | NumberStyles.AllowDecimalPoint;

    internal static string Format(decimal value) => value.ToString(CultureInfo.InvariantCulture);

    // Generated code calls this only after a null check.
    internal static string Format(decimal? value) => Format(value!.Value);

    internal static decimal Parse(string value) => decimal.Parse(value, Style, CultureInfo.InvariantCulture);
}
