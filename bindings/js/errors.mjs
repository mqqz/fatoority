/** Library error codes reported by `BindingError.code()`, mirroring Rust `ErrorKind`. */
export const ErrorCode = Object.freeze({
  InvalidInput: 1,
  Validation: 2,
  Parse: 3,
  Xml: 4,
  Crypto: 5,
  Io: 6,
  Network: 7,
  Unauthorized: 8,
  Internal: 9,
  Api: 10,
});

const kinds = new Map(Object.entries(ErrorCode).map(([kind, code]) => [code, kind]));
// Filled in below: a subclass constructed directly takes its own code.
const defaultCodes = new Map();

/** Base class of every error the package throws for a library failure. */
export class FatooraError extends Error {
  constructor(message, { code = defaultCodes.get(new.target) ?? null, details = {}, cause } = {}) {
    super(message, cause === undefined ? undefined : { cause });
    /** Numeric library code, or `null` when the failure did not come from Rust. */
    this.code = code;
    /** Variant name of `code`, or `null` when this version does not recognize it. */
    this.kind = kinds.get(code) ?? null;
    /** Structured details from the library, such as findings or an HTTP response body. */
    this.details = details;
  }
}

export class InvalidInputError extends FatooraError {}
export class ValidationError extends FatooraError {}
export class ParseError extends FatooraError {}
export class XmlError extends FatooraError {}
export class CryptoError extends FatooraError {}
export class IoError extends FatooraError {}
export class NetworkError extends FatooraError {}
export class UnauthorizedError extends FatooraError {}
export class InternalError extends FatooraError {}
export class ApiError extends FatooraError {}

const classes = new Map([
  [ErrorCode.InvalidInput, InvalidInputError],
  [ErrorCode.Validation, ValidationError],
  [ErrorCode.Parse, ParseError],
  [ErrorCode.Xml, XmlError],
  [ErrorCode.Crypto, CryptoError],
  [ErrorCode.Io, IoError],
  [ErrorCode.Network, NetworkError],
  [ErrorCode.Unauthorized, UnauthorizedError],
  [ErrorCode.Internal, InternalError],
  [ErrorCode.Api, ApiError],
]);
for (const [code, ErrorClass] of classes) defaultCodes.set(ErrorClass, code);

// Explicit names survive bundler minification of class identifiers.
for (const [ErrorClass, name] of [
  [FatooraError, 'FatooraError'], [InvalidInputError, 'InvalidInputError'],
  [ValidationError, 'ValidationError'], [ParseError, 'ParseError'], [XmlError, 'XmlError'],
  [CryptoError, 'CryptoError'], [IoError, 'IoError'], [NetworkError, 'NetworkError'],
  [UnauthorizedError, 'UnauthorizedError'], [InternalError, 'InternalError'], [ApiError, 'ApiError'],
]) {
  Object.defineProperty(ErrorClass.prototype, 'name', { value: name, writable: true, configurable: true });
}

/** The error subclass for a library code; `FatooraError` for an unrecognized code. */
export function errorClassForCode(code) {
  return classes.get(code) ?? FatooraError;
}

/**
 * Fetch, cancellation and response-body read failures in the JavaScript host.
 * `cause` is the original host error; `httpStatus` is set when a response arrived.
 */
export class TransportError extends NetworkError {
  constructor(message, cause, httpStatus = undefined) {
    super(message, { code: ErrorCode.Network, cause });
    this.httpStatus = httpStatus;
  }
}
Object.defineProperty(TransportError.prototype, 'name', { value: 'TransportError', writable: true, configurable: true });

/** Convert a generated `BindingError` into its typed error, reading it eagerly. */
export function fromBindingError(binding) {
  const code = binding.code();
  let details = {};
  try {
    const parsed = JSON.parse(binding.detailsJson());
    if (parsed !== null && typeof parsed === 'object') details = parsed;
  } catch { /* details are best-effort */ }
  const ErrorClass = errorClassForCode(code);
  return new ErrorClass(binding.message(), { code, details, cause: binding });
}
