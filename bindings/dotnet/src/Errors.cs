using System;

namespace Fatoora;

#nullable enable

/// <summary>Category of a library failure; each value is the stable numeric error code.</summary>
public enum ErrorKind
{
    /// <summary>A caller-supplied value is malformed or out of range.</summary>
    InvalidInput = 1,
    /// <summary>Input is well-formed but fails invoice, CSR, or schema validation rules.</summary>
    Validation = 2,
    /// <summary>Structured input, such as a properties file or an API response body, could not be parsed.</summary>
    Parse = 3,
    /// <summary>XML could not be serialized, parsed, or processed.</summary>
    Xml = 4,
    /// <summary>Key handling, signing, hashing, or certificate processing failed.</summary>
    Crypto = 5,
    /// <summary>A file or other I/O operation failed.</summary>
    Io = 6,
    /// <summary>The HTTP request or response transport failed.</summary>
    Network = 7,
    /// <summary>The ZATCA API rejected the supplied credentials.</summary>
    Unauthorized = 8,
    /// <summary>An internal invariant or client state error.</summary>
    Internal = 9,
    /// <summary>The ZATCA API returned an error or non-accepted response.</summary>
    Api = 10,
}

/// <summary>
/// Base class of every exception the Fatoora bindings throw for a library failure.
/// Catch a subclass such as <see cref="InvalidInputException"/> to handle one
/// <see cref="ErrorKind"/>; an unknown code throws this class with a <c>null</c>
/// <see cref="Kind"/>.
/// </summary>
/// <remarks>
/// The native error is copied when the exception is created, so the exception
/// holds no native resources and needs no disposal.
/// </remarks>
public class FatooraException : Exception
{
    public FatooraException(string message, int code, string detailsJson = "{}") : base(message)
    {
        Code = code;
        DetailsJson = detailsJson;
    }

    /// <summary>Numeric error code.</summary>
    public int Code { get; }

    /// <summary>The error category, or <c>null</c> for a code this version does not know.</summary>
    public ErrorKind? Kind => Enum.IsDefined(typeof(ErrorKind), Code) ? (ErrorKind)Code : null;

    /// <summary>Structured error details as a JSON object.</summary>
    public string DetailsJson { get; }

    internal static FatooraException FromNative(BindingError error)
    {
        using (error)
        {
            return FromCode(error.Code, error.Message, error.DetailsJson);
        }
    }

    /// <summary>Create the subclass that matches <paramref name="code"/>; <see cref="FatooraException"/> for an unknown code.</summary>
    public static FatooraException FromCode(int code, string message, string detailsJson = "{}") => code switch
    {
        (int)ErrorKind.InvalidInput => new InvalidInputException(message, detailsJson),
        (int)ErrorKind.Validation => new ValidationException(message, detailsJson),
        (int)ErrorKind.Parse => new ParseException(message, detailsJson),
        (int)ErrorKind.Xml => new XmlException(message, detailsJson),
        (int)ErrorKind.Crypto => new CryptoException(message, detailsJson),
        (int)ErrorKind.Io => new IoException(message, detailsJson),
        (int)ErrorKind.Network => new NetworkException(message, detailsJson),
        (int)ErrorKind.Unauthorized => new UnauthorizedException(message, detailsJson),
        (int)ErrorKind.Internal => new InternalException(message, detailsJson),
        (int)ErrorKind.Api => new ApiException(message, detailsJson),
        _ => new FatooraException(message, code, detailsJson),
    };
}

/// <summary>A caller-supplied value is malformed or out of range (code 1).</summary>
public class InvalidInputException : FatooraException
{
    public InvalidInputException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.InvalidInput, detailsJson) { }
}

/// <summary>Input is well-formed but fails invoice, CSR, or schema validation rules (code 2).</summary>
public class ValidationException : FatooraException
{
    public ValidationException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Validation, detailsJson) { }
}

/// <summary>Structured input could not be parsed (code 3).</summary>
public class ParseException : FatooraException
{
    public ParseException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Parse, detailsJson) { }
}

/// <summary>XML could not be serialized, parsed, or processed (code 4).</summary>
public class XmlException : FatooraException
{
    public XmlException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Xml, detailsJson) { }
}

/// <summary>Key handling, signing, hashing, or certificate processing failed (code 5).</summary>
public class CryptoException : FatooraException
{
    public CryptoException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Crypto, detailsJson) { }
}

/// <summary>A file or other I/O operation failed (code 6).</summary>
public class IoException : FatooraException
{
    public IoException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Io, detailsJson) { }
}

/// <summary>The HTTP request or response transport failed (code 7).</summary>
public class NetworkException : FatooraException
{
    public NetworkException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Network, detailsJson) { }
}

/// <summary>The ZATCA API rejected the supplied credentials (code 8).</summary>
public class UnauthorizedException : FatooraException
{
    public UnauthorizedException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Unauthorized, detailsJson) { }
}

/// <summary>An internal invariant or client state error (code 9).</summary>
public class InternalException : FatooraException
{
    public InternalException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Internal, detailsJson) { }
}

/// <summary>The ZATCA API returned an error or non-accepted response (code 10).</summary>
public class ApiException : FatooraException
{
    public ApiException(string message, string detailsJson = "{}") : base(message, (int)ErrorKind.Api, detailsJson) { }
}
