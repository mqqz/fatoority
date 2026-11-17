/** Library error codes reported by `BindingError.code()`, mirroring Rust `ErrorKind`. */
export declare const ErrorCode: Readonly<{
  InvalidInput: 1;
  Validation: 2;
  Parse: 3;
  Xml: 4;
  Crypto: 5;
  Io: 6;
  Network: 7;
  Unauthorized: 8;
  Internal: 9;
  Api: 10;
}>;
/** Variant name of a library error code. */
export type ErrorKind = keyof typeof ErrorCode;

export interface FatooraErrorOptions {
  code?: number | null;
  details?: Record<string, unknown>;
  cause?: unknown;
}

/**
 * Base class of every error the package throws for a library failure.
 * For failures reported by Rust, `cause` is the generated `BindingError`.
 */
export declare class FatooraError extends Error {
  constructor(message: string, options?: FatooraErrorOptions);
  /** Numeric library code, or `null` when the failure did not come from Rust. */
  readonly code: number | null;
  /** Variant name of `code`, or `null` when this version does not recognize it. */
  readonly kind: ErrorKind | null;
  /** Structured details, such as validation findings or an HTTP response body. */
  readonly details: Record<string, unknown>;
}
/** Code 1: a caller-supplied value is malformed or out of range. */
export declare class InvalidInputError extends FatooraError { readonly code: 1; readonly kind: 'InvalidInput'; }
/** Code 2: input is well-formed but fails invoice, CSR or schema rules. */
export declare class ValidationError extends FatooraError { readonly code: 2; readonly kind: 'Validation'; }
/** Code 3: structured input, such as a ZATCA API response body, could not be parsed. */
export declare class ParseError extends FatooraError { readonly code: 3; readonly kind: 'Parse'; }
/** Code 4: XML could not be serialized, parsed or processed. */
export declare class XmlError extends FatooraError { readonly code: 4; readonly kind: 'Xml'; }
/** Code 5: key handling, signing, hashing or certificate processing failed. */
export declare class CryptoError extends FatooraError { readonly code: 5; readonly kind: 'Crypto'; }
/** Code 6: an I/O operation failed. */
export declare class IoError extends FatooraError { readonly code: 6; readonly kind: 'Io'; }
/** Code 7: the HTTP request or response transport failed. */
export declare class NetworkError extends FatooraError { readonly code: 7; readonly kind: 'Network'; }
/** Code 8: the ZATCA API rejected the supplied credentials. */
export declare class UnauthorizedError extends FatooraError { readonly code: 8; readonly kind: 'Unauthorized'; }
/** Code 9: an internal invariant or client state error. */
export declare class InternalError extends FatooraError { readonly code: 9; readonly kind: 'Internal'; }
/** Code 10: the ZATCA API returned an error or non-accepted response. */
export declare class ApiError extends FatooraError { readonly code: 10; readonly kind: 'Api'; }

/**
 * Fetch, cancellation and response-body read failures in the JavaScript host.
 * `cause` is the original host error; `httpStatus` is set when a response arrived.
 */
export declare class TransportError extends NetworkError {
  constructor(message: string, cause?: unknown, httpStatus?: number);
  readonly httpStatus?: number;
}

/** The error subclass for a library code; `FatooraError` for an unrecognized code. */
export declare function errorClassForCode(code: number | null): typeof FatooraError;
