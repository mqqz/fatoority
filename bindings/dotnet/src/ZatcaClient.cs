using System;

namespace Fatoora;

#nullable enable

public partial class ZatcaClient
{
    private const uint MaxTimeoutMilliseconds = uint.MaxValue;

    /// <summary>
    /// Create a client whose requests fail with a network error once they take
    /// longer than <paramref name="timeout"/>, from connecting until the response
    /// body is read. The timeout is rounded up to whole milliseconds.
    /// </summary>
    /// <exception cref="ArgumentNullException"><paramref name="config"/> is null.</exception>
    /// <exception cref="ArgumentOutOfRangeException">
    /// <paramref name="timeout"/> is zero, negative (including
    /// <see cref="System.Threading.Timeout.InfiniteTimeSpan"/>), or longer than
    /// 4294967.295 seconds (<see cref="uint.MaxValue"/> milliseconds).
    /// </exception>
    /// <exception cref="FatooraException"></exception>
    public static ZatcaClient WithTimeout(Config config, TimeSpan timeout)
    {
        ArgumentNullException.ThrowIfNull(config);
        return WithTimeoutMilliseconds(config, TimeoutMilliseconds(timeout));
    }

    /// <summary>A positive timeout in whole milliseconds, rounded up, that fits the native u32.</summary>
    internal static uint TimeoutMilliseconds(TimeSpan timeout)
    {
        if (timeout <= TimeSpan.Zero)
        {
            throw new ArgumentOutOfRangeException(nameof(timeout), timeout, "timeout must be positive");
        }
        long milliseconds = timeout.Ticks / TimeSpan.TicksPerMillisecond
            + (timeout.Ticks % TimeSpan.TicksPerMillisecond == 0 ? 0 : 1);
        if (milliseconds > MaxTimeoutMilliseconds)
        {
            throw new ArgumentOutOfRangeException(nameof(timeout), timeout,
                "timeout must be at most 4294967.295 seconds");
        }
        return (uint)milliseconds;
    }
}
