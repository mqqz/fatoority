/** Category of a library failure, mirroring Rust `ErrorKind`; each value is its own name. */
export const ErrorKind = Object.freeze({
  InvalidInput: 'InvalidInput',
  Validation: 'Validation',
  Parse: 'Parse',
  Xml: 'Xml',
  Crypto: 'Crypto',
  Io: 'Io',
  Network: 'Network',
  Unauthorized: 'Unauthorized',
  Internal: 'Internal',
  Api: 'Api',
});

// Stable numeric codes, shared with every other binding and the C API.
const codes = new Map([
  [ErrorKind.InvalidInput, 1], [ErrorKind.Validation, 2], [ErrorKind.Parse, 3], [ErrorKind.Xml, 4],
  [ErrorKind.Crypto, 5], [ErrorKind.Io, 6], [ErrorKind.Network, 7], [ErrorKind.Unauthorized, 8],
  [ErrorKind.Internal, 9], [ErrorKind.Api, 10],
]);
const kinds = new Map([...codes].map(([kind, code]) => [code, kind]));
// Filled in below: a subclass constructed directly takes its own code.
const defaultCodes = new Map();
const classes = new Map();

/** Base class of every error the package throws for a library failure. */
export class FatooraError extends Error {
  constructor(message, { code = defaultCodes.get(new.target) ?? null, details = {}, cause } = {}) {
    super(message, cause === undefined ? undefined : { cause });
    /** Numeric error code, or `null` when the failure did not come from the library. */
    this.code = code;
    /** `ErrorKind` of `code`, or `null` when this version does not recognize it. */
    this.kind = kinds.get(code) ?? null;
    /** Structured details, such as validation findings or an HTTP response body. */
    this.details = details;
  }

  /** Create the subclass that matches `code`; `FatooraError` for an unrecognized code. */
  static fromCode(code, message, { details = {}, cause } = {}) {
    const ErrorClass = classes.get(code) ?? FatooraError;
    return new ErrorClass(message, { code, details, cause });
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

for (const [kind, ErrorClass] of [
  [ErrorKind.InvalidInput, InvalidInputError], [ErrorKind.Validation, ValidationError],
  [ErrorKind.Parse, ParseError], [ErrorKind.Xml, XmlError], [ErrorKind.Crypto, CryptoError],
  [ErrorKind.Io, IoError], [ErrorKind.Network, NetworkError], [ErrorKind.Unauthorized, UnauthorizedError],
  [ErrorKind.Internal, InternalError], [ErrorKind.Api, ApiError],
]) {
  classes.set(codes.get(kind), ErrorClass);
  defaultCodes.set(ErrorClass, codes.get(kind));
  // Explicit names survive bundler minification of class identifiers.
  Object.defineProperty(ErrorClass.prototype, 'name', { value: `${kind}Error`, writable: true, configurable: true });
}
Object.defineProperty(FatooraError.prototype, 'name', { value: 'FatooraError', writable: true, configurable: true });

/** Internal: convert a generated `BindingError` into its typed error, reading it eagerly. */
export function fromBindingError(binding) {
  let details = {};
  try {
    const parsed = JSON.parse(binding.detailsJson());
    if (parsed !== null && typeof parsed === 'object') details = parsed;
  } catch { /* details are best-effort */ }
  return FatooraError.fromCode(binding.code(), binding.message(), { details, cause: binding });
}
