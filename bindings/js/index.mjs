export * from './generated/index.mjs';
import { ClearanceMode, ZatcaClient as NativeClient, Xml as NativeXml } from './generated/index.mjs';

/** Accept the documented boolean or a ClearanceMode for the Clearance-Status header. */
function clearanceMode(value) {
  if (value instanceof ClearanceMode) return value;
  if (typeof value !== 'boolean') throw new TypeError('clearanceStatus must be a boolean or ClearanceMode');
  return value ? ClearanceMode.Enabled : ClearanceMode.Disabled;
}

/** Fetch failures have no Rust response; decoding failures retain Diplomat BindingError. */
export class TransportError extends Error {
  constructor(message, cause, httpStatus = undefined) {
    super(message, { cause });
    this.name = 'TransportError';
    this.httpStatus = httpStatus;
  }
}

/** Send an owned prepared request using the caller's fetch implementation. */
export async function sendPrepared(request, { fetch: fetchImpl = globalThis.fetch, signal } = {}) {
  // Complete every read of WASM memory before invoking user code or awaiting IO.
  const url = request.url();
  const init = {
    method: request.method(), headers: JSON.parse(request.headersJson()),
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
  async postCsrForCcsid(csr, otp, options = {}) {
    return this.#send(this.#native.preparePostCsrForCcsid(csr, otp), options);
  }
  async postCcsidForPcsid(credentials, options = {}) {
    return this.#send(this.#native.preparePostCcsidForPcsid(credentials), options);
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

export class Xml extends NativeXml {
  /** Supply the host clock once; explicit evaluated_at remains reproducible. */
  static validateZatca(config, xml, optionsJson = null) {
    // Let Rust produce BindingError for malformed, non-object or oversized input.
    // Check the original bytes before normalization so whitespace cannot evade the limit.
    if (optionsJson !== null && new TextEncoder().encode(optionsJson).length > 4096) {
      return NativeXml.validateZatca(config, xml, optionsJson);
    }
    let options;
    try { options = optionsJson === null ? {} : JSON.parse(optionsJson); }
    catch { return NativeXml.validateZatca(config, xml, optionsJson); }
    if (options === null || typeof options !== 'object' || Array.isArray(options)) {
      return NativeXml.validateZatca(config, xml, optionsJson);
    }
    options.evaluated_at ??= new Date().toISOString();
    return NativeXml.validateZatca(config, xml, JSON.stringify(options));
  }
}
