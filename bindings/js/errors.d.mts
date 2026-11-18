/** Category of a library failure, mirroring Rust `ErrorKind`; each value is its own name. */
export declare const ErrorKind: Readonly<{
  InvalidInput: 'InvalidInput';
  Validation: 'Validation';
  Parse: 'Parse';
  Xml: 'Xml';
  Crypto: 'Crypto';
  Io: 'Io';
  Network: 'Network';
  Unauthorized: 'Unauthorized';
  Internal: 'Internal';
  Api: 'Api';
}>;
/** Category of a library failure, such as `'InvalidInput'`. */
export type ErrorKind = (typeof ErrorKind)[keyof typeof ErrorKind];

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
  /** Numeric error code, or `null` when the failure did not come from the library. */
  readonly code: number | null;
  /** `ErrorKind` of `code`, or `null` when this version does not recognize it. */
  readonly kind: ErrorKind | null;
  /** Structured details, such as validation findings or an HTTP response body. */
  readonly details: Record<string, unknown>;
  /** Create the subclass that matches `code`; `FatooraError` for an unrecognized code. */
  static fromCode(code: number | null, message: string, options?: Omit<FatooraErrorOptions, 'code'>): FatooraError;
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
/**
 * Code 7: the HTTP request or response transport failed. When `fetch` or the
 * response-body read fails, `cause` is the host error and `details` follows the
 * library schema (`network`, or `api_response_read` with `http_status`).
 */
export declare class NetworkError extends FatooraError { readonly code: 7; readonly kind: 'Network'; }
/** Code 8: the ZATCA API rejected the supplied credentials. */
export declare class UnauthorizedError extends FatooraError { readonly code: 8; readonly kind: 'Unauthorized'; }
/** Code 9: an internal invariant or client state error. */
export declare class InternalError extends FatooraError { readonly code: 9; readonly kind: 'Internal'; }
/** Code 10: the ZATCA API returned an error or non-accepted response. */
export declare class ApiError extends FatooraError { readonly code: 10; readonly kind: 'Api'; }
