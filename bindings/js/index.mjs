import './patch.mjs';
import { ClearanceMode, ZatcaClient as NativeClient, Xml as NativeXml } from './generated/index.mjs';
import { TransportError } from './errors.mjs';

// Generated classes re-exported unchanged (after patch.mjs). The native
// validation report classes stay in `fatoority/raw`; `Xml` and `ZatcaClient`
// are replaced below. tests/runtime.test.mjs checks this list stays complete.
export {
  Address, Adjustment, AdjustmentKind, AllowanceCharge, BindingError, Bytes, BytesList,
  ClearanceMode, ClearanceStatus, ComplianceCsid, ComplianceRequest, Config, Csr, CsrProperties,
  Environment, FinalizedInvoice, InvoiceBuilder, InvoiceData, InvoiceFlag, InvoiceLineItem,
  InvoiceNote, InvoiceOutcome, InvoiceRequest, InvoiceSigner, InvoiceSubType, InvoiceTotals,
  InvoiceTypeKind, LineItemBuilder, MessageType, OriginalInvoiceRef, OtherId, Party, PaymentMeans,
  ProductionCsid, ProductionRequest, ReportingStatus, Severity, SignedInvoice, SigningKey,
  TaxExemption, Text, ValidationLayer, ValidationLocationKind, ValidationMessage,
  ValidationResponse, ValidationResults, ValidationStatus, VatBreakdown, VatCategory, VatId,
  ZatcaStage, ZatcaStageStatus,
} from './generated/index.mjs';
export {
  ErrorCode, FatooraError, InvalidInputError, ValidationError, ParseError, XmlError, CryptoError,
  IoError, NetworkError, UnauthorizedError, InternalError, ApiError, TransportError,
  errorClassForCode,
} from './errors.mjs';

/** Accept a boolean, a ClearanceMode or its value name for the Clearance-Status header. */
function clearanceMode(value) {
  if (value instanceof ClearanceMode) return value;
  if (typeof value === 'boolean') return value ? ClearanceMode.Enabled : ClearanceMode.Disabled;
  if (value === 'Enabled' || value === 'Disabled') return ClearanceMode[value];
  throw new TypeError('clearanceStatus must be a boolean or ClearanceMode');
}

/** Send an owned prepared request using the caller's fetch implementation. */
export async function sendPrepared(request, { fetch: fetchImpl = globalThis.fetch, signal } = {}) {
  // Complete every read of WASM memory before invoking user code or awaiting IO.
  const url = request.url;
  const init = {
    method: request.method, headers: JSON.parse(request.headersJson),
    body: Uint8Array.from(request.body().asSlice()), redirect: 'error', signal,
  };
  let response;
  try { response = await fetchImpl(url, init); }
  catch (cause) { throw new TransportError('ZATCA request failed', cause); }
  let body;
  try { body = new Uint8Array(await response.arrayBuffer()); }
  catch (cause) { throw new TransportError('Could not read ZATCA response', cause, response.status); }
  // Non-2xx statuses must reach Rust; HTTP success is not invoice acceptance.
  return request.decode(response.status, body);
}

/** Async convenience API. Preparation and decoding remain in Rust. */
export class ZatcaClient {
  #native;
  #fetch;
  constructor(config, { fetch: fetchImpl = globalThis.fetch } = {}) {
    this.#native = NativeClient.create(config);
    this.#fetch = fetchImpl;
  }
  async requestComplianceCsid(csr, otp, options = {}) {
    return this.#send(this.#native.prepareRequestComplianceCsid(csr, otp), options);
  }
  async requestProductionCsid(credentials, options = {}) {
    return this.#send(this.#native.prepareRequestProductionCsid(credentials), options);
  }
  async renewCsid(credentials, csr, otp, acceptLanguage = null, options = {}) {
    return this.#send(this.#native.prepareRenewCsid(credentials, csr, otp, acceptLanguage), options);
  }
  async checkInvoiceCompliance(invoice, credentials, options = {}) {
    return this.#send(this.#native.prepareCheckInvoiceCompliance(invoice, credentials), options);
  }
  async reportSimplifiedInvoice(invoice, credentials, clearanceStatus = false, acceptLanguage = null, options = {}) {
    return this.#send(this.#native.prepareReportSimplifiedInvoice(invoice, credentials, clearanceMode(clearanceStatus), acceptLanguage), options);
  }
  async clearStandardInvoice(invoice, credentials, clearanceStatus = true, acceptLanguage = null, options = {}) {
    return this.#send(this.#native.prepareClearStandardInvoice(invoice, credentials, clearanceMode(clearanceStatus), acceptLanguage), options);
  }
  #send(request, options) { return sendPrepared(request, { ...options, fetch: this.#fetch }); }
}

function location(finding) {
  const kind = finding.locationKind;
  if (kind === null) return null;
  switch (kind.value) {
    case 'Field': case 'XPath': return { kind: kind.value, path: finding.locationPath };
    case 'Xml': return { kind: 'Xml', line: finding.locationLine, column: finding.locationColumn };
    default: return { kind: kind.value };
  }
}

function finding(native) {
  return {
    code: native.code, severity: native.severity.value, layer: native.layer.value,
    message: native.message, assertionSite: native.assertionSite, location: location(native),
  };
}

function stage(native) {
  const provenance = native.provenance();
  return {
    stage: native.stage.value,
    status: native.status.value,
    provenance: provenance === null ? null : { stylesheet: provenance.stylesheet, sha256: provenance.sha256 },
    evaluatedAssertions: Array.from({ length: native.evaluatedAssertionsLen }, (_, i) => native.evaluatedAssertion(i)),
    findings: Array.from({ length: native.findingsLen }, (_, i) => finding(native.finding(i))),
  };
}

/** Copy a native report into plain data once, so no WASM handle outlives the call. */
function report(native) {
  return {
    schemaVersion: native.schemaVersion,
    profile: native.profile,
    evaluatedAt: native.evaluatedAt,
    isComplete: native.isComplete,
    hasErrors: native.hasErrors,
    isValid: native.isValid,
    stages: Array.from({ length: native.stagesLen }, (_, i) => stage(native.stage(i))),
  };
}

function optionsJson(options) {
  if (options === null || options === undefined) return null;
  if (typeof options === 'string') return options;
  if (typeof options !== 'object' || Array.isArray(options)) {
    throw new TypeError('options must be an object, a JSON string or null');
  }
  const { previousInvoiceHash = null, evaluatedAt = null } = options;
  return JSON.stringify({
    previous_invoice_hash: previousInvoiceHash,
    evaluated_at: evaluatedAt instanceof Date ? evaluatedAt.toISOString() : evaluatedAt,
  });
}

/** Supply the host clock once; explicit evaluated_at remains reproducible. */
function withClock(json) {
  // Let Rust produce its error for malformed, non-object or oversized input.
  // Check the original bytes before normalization so whitespace cannot evade the limit.
  if (json === null) return JSON.stringify({ evaluated_at: new Date().toISOString() });
  if (new TextEncoder().encode(json).length > 4096) return json;
  let options;
  try { options = JSON.parse(json); }
  catch { return json; }
  if (options === null || typeof options !== 'object' || Array.isArray(options)) return json;
  options.evaluated_at ??= new Date().toISOString();
  return JSON.stringify(options);
}

export class Xml {
  constructor() { throw new TypeError('Xml has only static methods'); }
  static validate(xml) { return NativeXml.validate(xml); }
  static hash(xml) { return NativeXml.hash(xml); }
  static validateZatca(xml, options = null) {
    return report(NativeXml.validateZatca(xml, withClock(optionsJson(options))));
  }
}
