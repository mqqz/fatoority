package com.fatoora

/** Category of a library failure; [code] is the stable numeric error code carried by [FatooraException.code]. */
enum class ErrorKind(val code: Int) {
    /** A caller-supplied value is malformed or out of range. */
    InvalidInput(1),
    /** Input is well-formed but fails invoice, CSR, or schema validation rules. */
    Validation(2),
    /** Structured input, such as a properties file or an API response body, could not be parsed. */
    Parse(3),
    /** XML could not be serialized, parsed, or processed. */
    Xml(4),
    /** Key handling, signing, hashing, or certificate processing failed. */
    Crypto(5),
    /** A file or other I/O operation failed. */
    Io(6),
    /** The HTTP request or response transport failed. */
    Network(7),
    /** The ZATCA API rejected the supplied credentials. */
    Unauthorized(8),
    /** An internal invariant or client state error. */
    Internal(9),
    /** The ZATCA API returned an error or non-accepted response. */
    Api(10);

    companion object {
        /** The kind for a numeric error code, or `null` for a code this version does not know. */
        @JvmStatic
        fun fromCode(code: Int): ErrorKind? = entries.firstOrNull { it.code == code }
    }
}

/**
 * Base class of every exception the library throws for a native failure.
 *
 * Failures with a known [kind] are thrown as the matching subclass, such as
 * [InvalidInputException] or [NetworkException]; catch `FatooraException` to
 * handle every library failure. An unknown code throws this class with a `null`
 * [kind]. The native error is copied into managed memory before it is freed,
 * so the exception needs no cleanup.
 */
open class FatooraException @JvmOverloads constructor(val code: Int, override val message: String, val detailsJson: String = "{}") :
    RuntimeException(message) {
    /** The error category, or `null` when [code] is not known to this version. */
    val kind: ErrorKind? get() = ErrorKind.fromCode(code)

    companion object {
        /** Create the subclass that matches [code]; `FatooraException` for an unknown code. */
        @JvmStatic
        @JvmOverloads
        fun fromCode(code: Int, message: String, detailsJson: String = "{}"): FatooraException =
            when (ErrorKind.fromCode(code)) {
                ErrorKind.InvalidInput -> InvalidInputException(message, detailsJson)
                ErrorKind.Validation -> ValidationException(message, detailsJson)
                ErrorKind.Parse -> ParseException(message, detailsJson)
                ErrorKind.Xml -> XmlException(message, detailsJson)
                ErrorKind.Crypto -> CryptoException(message, detailsJson)
                ErrorKind.Io -> IoException(message, detailsJson)
                ErrorKind.Network -> NetworkException(message, detailsJson)
                ErrorKind.Unauthorized -> UnauthorizedException(message, detailsJson)
                ErrorKind.Internal -> InternalException(message, detailsJson)
                ErrorKind.Api -> ApiException(message, detailsJson)
                null -> FatooraException(code, message, detailsJson)
            }
    }
}

/** A caller-supplied value is malformed or out of range (code 1). */
class InvalidInputException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.InvalidInput.code, message, detailsJson)

/** Input is well-formed but fails invoice, CSR, or schema validation rules (code 2). */
class ValidationException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Validation.code, message, detailsJson)

/** Structured input, such as a properties file or an API response body, could not be parsed (code 3). */
class ParseException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Parse.code, message, detailsJson)

/** XML could not be serialized, parsed, or processed (code 4). */
class XmlException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Xml.code, message, detailsJson)

/** Key handling, signing, hashing, or certificate processing failed (code 5). */
class CryptoException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Crypto.code, message, detailsJson)

/** A file or other I/O operation failed (code 6). */
class IoException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Io.code, message, detailsJson)

/** The HTTP request or response transport failed (code 7). */
class NetworkException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Network.code, message, detailsJson)

/** The ZATCA API rejected the supplied credentials (code 8). */
class UnauthorizedException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Unauthorized.code, message, detailsJson)

/** An internal invariant or client state error, such as a consumed builder (code 9). */
class InternalException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Internal.code, message, detailsJson)

/** The ZATCA API returned an error or non-accepted response (code 10). */
class ApiException @JvmOverloads constructor(message: String, detailsJson: String = "{}") :
    FatooraException(ErrorKind.Api.code, message, detailsJson)
