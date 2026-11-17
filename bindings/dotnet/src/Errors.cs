using System;

namespace Fatoora;

#nullable enable

/// <summary>Base class for every error raised by the Fatoora bindings.</summary>
public class FatooraException : Exception
{
    public FatooraException(string message) : base(message) { }
    public FatooraException(string message, Exception? innerException) : base(message, innerException) { }
}

/// <summary>Native error category; values match the FFI error codes.</summary>
public enum FfiErrorKind
{
    InvalidInput = 1,
    Validation = 2,
    Parse = 3,
    Xml = 4,
    Crypto = 5,
    Io = 6,
    Network = 7,
    Unauthorized = 8,
    Internal = 9,
    Api = 10,
}

/// <summary>
/// Error reported by the native library. Catch a subclass such as
/// <see cref="InvalidInputException"/> to handle one category.
/// </summary>
/// <remarks>
/// The native error is copied when the exception is created, so the exception
/// holds no native resources and needs no disposal.
/// </remarks>
public class FfiException : FatooraException
{
    public FfiException(string message, int code, string detailsJson = "{}") : base(message)
    {
        Code = code;
        DetailsJson = detailsJson;
    }

    /// <summary>Numeric FFI error code.</summary>
    public int Code { get; }

    /// <summary>The error category, or <c>null</c> for an unknown code.</summary>
    public FfiErrorKind? Kind => Enum.IsDefined(typeof(FfiErrorKind), Code) ? (FfiErrorKind)Code : null;

    /// <summary>Structured error details as a JSON object.</summary>
    public string DetailsJson { get; }

    internal static FfiException FromNative(BindingError error)
    {
        using (error)
        {
            return Create(error.Code, error.Message, error.DetailsJson);
        }
    }

    /// <summary>Create the exception subclass that matches <paramref name="code"/>.</summary>
    public static FfiException Create(int code, string message, string detailsJson = "{}") => code switch
    {
        (int)FfiErrorKind.InvalidInput => new InvalidInputException(message, code, detailsJson),
        (int)FfiErrorKind.Validation => new ValidationException(message, code, detailsJson),
        (int)FfiErrorKind.Parse => new ParseException(message, code, detailsJson),
        (int)FfiErrorKind.Xml => new XmlException(message, code, detailsJson),
        (int)FfiErrorKind.Crypto => new CryptoException(message, code, detailsJson),
        (int)FfiErrorKind.Io => new IoException(message, code, detailsJson),
        (int)FfiErrorKind.Network => new NetworkException(message, code, detailsJson),
        (int)FfiErrorKind.Unauthorized => new UnauthorizedException(message, code, detailsJson),
        (int)FfiErrorKind.Internal => new InternalException(message, code, detailsJson),
        (int)FfiErrorKind.Api => new ApiException(message, code, detailsJson),
        _ => new FfiException(message, code, detailsJson),
    };
}

/// <summary>An argument was rejected (code 1).</summary>
public class InvalidInputException : FfiException
{
    public InvalidInputException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>Invoice or builder validation failed (code 2).</summary>
public class ValidationException : FfiException
{
    public ValidationException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>Input could not be parsed (code 3).</summary>
public class ParseException : FfiException
{
    public ParseException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>XML processing failed (code 4).</summary>
public class XmlException : FfiException
{
    public XmlException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>A cryptographic operation failed (code 5).</summary>
public class CryptoException : FfiException
{
    public CryptoException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>A file or I/O operation failed (code 6).</summary>
public class IoException : FfiException
{
    public IoException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>The ZATCA API could not be reached (code 7).</summary>
public class NetworkException : FfiException
{
    public NetworkException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>The ZATCA API rejected the credentials (code 8).</summary>
public class UnauthorizedException : FfiException
{
    public UnauthorizedException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>An internal error occurred (code 9).</summary>
public class InternalException : FfiException
{
    public InternalException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}

/// <summary>The ZATCA API returned an error response (code 10).</summary>
public class ApiException : FfiException
{
    public ApiException(string message, int code, string detailsJson = "{}") : base(message, code, detailsJson) { }
}
