package com.fatoora

/** Category of a native failure; [code] is the stable integer carried by [BindingException.code]. */
enum class ErrorKind(val code: Int) {
    InvalidInput(1),
    Validation(2),
    Parse(3),
    Xml(4),
    Crypto(5),
    Io(6),
    Network(7),
    Unauthorized(8),
    Internal(9),
    Api(10);

    companion object {
        /** The kind for a native error code, or `null` for a code this version does not know. */
        @JvmStatic
        fun fromCode(code: Int): ErrorKind? = values().firstOrNull { it.code == code }
    }
}

/**
 * Native error data copied into managed memory before the native error is freed.
 *
 * Failures with a known [kind] are thrown as the matching subclass, such as
 * [InvalidInputException] or [NetworkException]; catch `BindingException` to
 * handle every native failure.
 */
open class BindingException(val code: Int, override val message: String, val detailsJson: String) :
    RuntimeException(message) {
    /** The error category, or `null` when [code] is not known to this version. */
    val kind: ErrorKind? get() = ErrorKind.fromCode(code)
}

/** A caller-supplied value is malformed or out of range. */
class InvalidInputException(message: String, detailsJson: String) :
    BindingException(ErrorKind.InvalidInput.code, message, detailsJson)

/** Input is well-formed but fails invoice, CSR, or schema validation rules. */
class ValidationException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Validation.code, message, detailsJson)

/** Structured input, such as a properties file or an API response body, could not be parsed. */
class ParseException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Parse.code, message, detailsJson)

/** XML could not be serialized, parsed, or processed. */
class XmlException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Xml.code, message, detailsJson)

/** Key handling, signing, hashing, or certificate processing failed. */
class CryptoException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Crypto.code, message, detailsJson)

/** A file or other I/O operation failed. */
class IoException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Io.code, message, detailsJson)

/** The HTTP request or response transport failed. */
class NetworkException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Network.code, message, detailsJson)

/** The ZATCA API rejected the supplied credentials. */
class UnauthorizedException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Unauthorized.code, message, detailsJson)

/** An internal invariant or client state error, such as a consumed builder. */
class InternalException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Internal.code, message, detailsJson)

/** The ZATCA API returned an error or non-accepted response. */
class ApiException(message: String, detailsJson: String) :
    BindingException(ErrorKind.Api.code, message, detailsJson)

internal fun bindingException(code: Int, message: String, detailsJson: String): BindingException =
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
        null -> BindingException(code, message, detailsJson)
    }
