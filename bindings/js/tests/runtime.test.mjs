import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import * as sdk from '../index.mjs';
import * as raw from '../raw.mjs';
import * as generated from '../generated/index.mjs';
import wasm from '../generated/diplomat-wasm.mjs';
import { integerBooleanMethods } from '../patch.mjs';
import { optionalResults, properties } from '../members.mjs';
const loadFixture = (path) => readFileSync(new URL(`../../../fatoora-core/tests/fixtures/${path}`, import.meta.url), 'utf8');
const config = sdk.Config.create(sdk.Environment.NonProduction);
const ccsid = () => sdk.ComplianceCsid.create(sdk.Environment.NonProduction, '123', 'token', 'secret');
const pcsid = () => sdk.ProductionCsid.create(sdk.Environment.NonProduction, null, 'token', 'secret');
const invoice = () => sdk.SignedInvoice.fromXml(loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml'));
const credentialsBody = { requestID: 123, binarySecurityToken: 'new-token', secret: 'new-secret' };
const validationBody = { validationResults: { status: 'PASS' }, reportingStatus: 'REPORTED' };
const json = (body, status = 200) => new Response(JSON.stringify(body), { status });
const code = (n) => (error) => error.constructor === sdk.FatooraError.fromCode(n, '').constructor && error.code === n
  && error.cause instanceof sdk.BindingError && error.cause.code() === n;

test('artifact imports only supported host functions and generates real keys', () => {
  const module = new WebAssembly.Module(readFileSync(new URL('../fatoora_ffi.wasm', import.meta.url)));
  assert.deepEqual(WebAssembly.Module.imports(module).map(i => `${i.module}.${i.name}`), ['env.fatoora_random_fill', 'env.diplomat_throw_error_js']);
  const key = sdk.SigningKey.generate();
  assert.match(key.toPem(), /BEGIN PRIVATE KEY/);
  assert.notEqual(key.toPem(), sdk.SigningKey.generate().toPem());
  assert.equal(sdk.SigningKey.fromPem(key.toPem()).toPem(), key.toPem());
});

test('optional strings preserve absent, empty and Arabic across repeated calls', () => {
  for (let i = 0; i < 100; i++) {
    for (const value of [null, '', 'شارع إضافي']) {
      const address = sdk.Address.create('SAU', 'الرياض', 'شارع الملك', '1234', '12222', value, null, null);
      assert.equal(address.additionalStreet, value);
      assert.equal(address.city, 'الرياض');
    }
  }
  assert.throws(() => sdk.Address.create('SAU', 'bad\0city', 'street', '1234', '12222', null, null, null), code(1));
});

test('decimal values remain exact and consumed builders fail safely', () => {
  const b = sdk.InvoiceBuilder.create(sdk.InvoiceTypeKind.Tax, sdk.InvoiceSubType.Simplified, null, null, null, null);
  b.setId('JS-1'); b.setUuid('js-uuid'); b.setIssueDatetime('2024-01-01T12:30:00Z');
  b.setCurrency('SAR'); b.setPreviousInvoiceHash('hash'); b.setInvoiceCounter(0n); b.addPaymentMeans(sdk.PaymentMeans.create('10', null)); b.setVatCategory(sdk.VatCategory.Standard);
  const a = sdk.Address.create('SAU', 'Riyadh', 'King Fahd', '1234', '12222', null, null, null);
  b.setSeller('Acme Inc', a, '301121971500003', null, null);
  b.addLineItem('Item', '3', 'PCE', '0.3333', '15', sdk.VatCategory.Standard);
  const inv = b.build();
  assert.equal(inv.totals().taxInclusive, '1.15');
  assert.match(inv.xml, />0.3333<\/cbc:PriceAmount>/);
  assert.throws(() => b.build(), code(1));
});

test('prepared request retains context after invoice consumption and WASM memory growth', () => {
  const inv = invoice();
  const request = raw.ZatcaClient.create(config).prepareReportSimplifiedInvoice(inv, pcsid(), sdk.ClearanceMode.Disabled, 'ar');
  const bytes = request.body().toUint8Array();
  const original = JSON.parse(Buffer.from(bytes).toString());
  assert.match(inv.zatcaKeySignature, /^[A-Za-z0-9+/]+=*$/);
  inv.intoXml();
  assert.throws(() => inv.zatcaKeySignature, code(1));
  wasm.memory.grow(1);
  for (let i = 0; i < 20; i++) sdk.SigningKey.generate();
  assert.equal(JSON.parse(Buffer.from(request.body().toUint8Array()).toString()).invoiceHash, original.invoiceHash);
  const result = request.decode(200, new TextEncoder().encode(JSON.stringify(validationBody)));
  assert.equal(result.httpStatus, 200);
  assert.equal(result.outcome.value, 'Accepted');
  assert.equal(result.reportingStatus, sdk.ReportingStatus.Reported);
  assert.equal(result.reportingStatusRaw, 'REPORTED');
  assert.equal(result.validationResults().status, sdk.ValidationStatus.Pass);
  assert.equal(result.clearanceStatus, null);
  assert.equal(result.clearanceStatusRaw, null);
  const unknown = request.decode(200, [...Buffer.from(JSON.stringify({validationResults: {status: 'PARTIAL'}, reportingStatus: 'LATER'}))]);
  assert.equal(unknown.reportingStatus, sdk.ReportingStatus.Unknown);
  assert.equal(unknown.reportingStatusRaw, 'LATER');
  assert.equal(unknown.validationResults().status, sdk.ValidationStatus.Unknown);
  const decode = (body) => request.decode(200, [...Buffer.from(JSON.stringify({validationResults: {}, ...body}))]);
  assert.equal(result.clearedInvoiceXml, null);
  assert.equal(decode({clearedInvoice: Buffer.from('<Invoice/>').toString('base64')}).clearedInvoiceXml, '<Invoice/>');
  assert.throws(() => decode({clearedInvoice: '!!!'}).clearedInvoiceXml, code(3));
});

test('fetch sees Rust headers and body; concurrent requests decode independently', async () => {
  const pending = [];
  const client = new sdk.ZatcaClient(config, { fetch: (url, init) => new Promise(resolve => pending.push({url, init, resolve})) });
  const a = client.reportSimplifiedInvoice(invoice(), pcsid(), sdk.ClearanceMode.Disabled, 'ar');
  const b = client.requestProductionCsid(ccsid());
  assert.equal(pending.length, 2);
  const headers = new Headers(pending[0].init.headers);
  assert.equal(headers.get('accept-version'), 'V2');
  assert.equal(headers.get('accept-language'), 'ar');
  assert.equal(headers.get('authorization'), `Basic ${Buffer.from('token:secret').toString('base64')}`);
  assert.equal(pending[0].init.redirect, 'error');
  assert.equal(JSON.parse(new TextDecoder().decode(pending[1].init.body)).compliance_request_id, '123');
  pending[1].resolve(json(credentialsBody));
  assert.equal((await b).binarySecurityToken, 'new-token');
  pending[0].resolve(json(validationBody));
  assert.equal((await a).httpStatus, 200);
});

test('all credential operations, including renewal 428, share Rust decoding', async () => {
  const csr = sdk.CsrProperties.fromStr(loadFixture('csr-configs/csr-config-example-EN.properties')).build(sdk.SigningKey.generate(), sdk.Environment.NonProduction);
  const calls = [];
  const client = new sdk.ZatcaClient(config, { fetch: async (url, init) => {
    calls.push({url, init});
    return init.method === 'PATCH' ? json({value: credentialsBody}, 428) : json(credentialsBody);
  }});
  assert.equal((await client.requestComplianceCsid(csr, '123456')).env, sdk.Environment.NonProduction);
  assert.equal((await client.renewCsid(pcsid(), csr, '123456')).requestId, '123');
  assert.equal(new Headers(calls[0].init.headers).get('otp'), '123456');
  assert.equal(calls[1].init.method, 'PATCH');
});

test('HTTP failures decode in Rust; network, body-read and cancellation failures stay transport errors', async () => {
  for (const status of [400, 401, 409, 500]) {
    const client = new sdk.ZatcaClient(config, { fetch: async () => json(validationBody, status) });
    await assert.rejects(client.reportSimplifiedInvoice(invoice(), pcsid(), sdk.ClearanceMode.Disabled), (e) => {
      assert.ok(e instanceof sdk.FatooraError);
      assert.equal(e.details.http_status, status);
      assert.equal(JSON.parse(e.cause.detailsJson()).http_status, status); return true;
    });
  }
  const malformed = new sdk.ZatcaClient(config, { fetch: async () => new Response('bad') });
  await assert.rejects(malformed.reportSimplifiedInvoice(invoice(), pcsid(), sdk.ClearanceMode.Disabled), code(3));
  const broken = new sdk.ZatcaClient(config, { fetch: async () => ({ status: 200, arrayBuffer: async () => { throw new Error('read'); } }) });
  await assert.rejects(broken.requestProductionCsid(ccsid()), e => e instanceof sdk.NetworkError && e.code === 7 && e.kind === sdk.ErrorKind.Network
    && e.details.type === 'api_response_read' && e.details.http_status === 200 && e.cause.message === 'read');
  const abort = new AbortController(); abort.abort();
  const cancelled = new sdk.ZatcaClient(config, { fetch: async (_url, {signal}) => { signal.throwIfAborted(); } });
  await assert.rejects(cancelled.requestProductionCsid(ccsid(), {signal: abort.signal}), e => e instanceof sdk.NetworkError && e.details.type === 'network' && e.cause.name === 'AbortError');
});

test('validation uses supplied time and the public facade supplies a default', () => {
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  assert.throws(() => raw.Xml.validateZatca(xml, null), e => e instanceof sdk.FatooraError && e.message.includes('evaluated_at'));
  const a = sdk.Xml.validateZatca(xml, JSON.stringify({evaluated_at: '2025-01-01T00:00:00Z'}));
  assert.equal(Object.getPrototypeOf(a), Object.prototype);
  assert.equal(a.evaluatedAt, '2025-01-01T00:00:00Z');
  assert.equal(a.stages.length, 6);
  assert.equal(a.stages[0].stage, 'Xsd');
  assert.equal(a.stages[0].status, sdk.ZatcaStageStatus.Completed.value);
  assert.equal(a.isValid, a.isComplete && !a.hasErrors);
  assert.equal(typeof a.isValid, 'boolean');
  assert.ok(sdk.Xml.validateZatca(xml).evaluatedAt);
  for (const options of [{evaluatedAt: '2025-01-01T00:00:00Z'}, {evaluatedAt: new Date('2025-01-01T00:00:00Z')}]) {
    assert.match(sdk.Xml.validateZatca(xml, options).evaluatedAt, /^2025-01-01T00:00:00(\.000)?Z$/);
  }
  const unchained = sdk.Xml.validateZatca(xml, {evaluatedAt: '2025-01-01T00:00:00Z'});
  const chained = sdk.Xml.validateZatca(xml, {previousInvoiceHash: 'NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==', evaluatedAt: '2025-01-01T00:00:00Z'});
  assert.equal(unchained.stages.at(-1).status, 'ContextRequired');
  assert.notEqual(chained.stages.at(-1).status, 'ContextRequired');
  assert.throws(() => sdk.Xml.validateZatca(xml, 42), TypeError);
});

test('validation report is plain data matching the native report', () => {
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml')
    .replace('<cbc:InvoiceTypeCode name="0200000">388', '<cbc:InvoiceTypeCode name="0200000">389');
  const options = {evaluated_at: '2025-01-01T00:00:00Z'};
  const report = sdk.Xml.validateZatca(xml, {evaluatedAt: options.evaluated_at});
  const native = JSON.parse(raw.Xml.validateZatca(xml, JSON.stringify(options)).toJson());
  assert.deepEqual(structuredClone(report), report);
  assert.deepEqual(JSON.parse(JSON.stringify(report)), report);
  assert.equal(report.schemaVersion, native.schema_version);
  assert.equal(report.profile, native.profile);
  assert.deepEqual([report.isComplete, report.hasErrors, report.isValid], [native.is_complete, native.has_errors, native.is_valid]);
  assert.equal(report.isValid, false);
  const snake = (value) => value.replace(/[a-z](?=[A-Z])/g, '$&_').toLowerCase();
  report.stages.forEach((stage, i) => {
    const expected = native.stages[i];
    assert.equal(snake(stage.stage), expected.stage);
    assert.equal(snake(stage.status), expected.status);
    assert.deepEqual(stage.provenance, expected.provenance ?? null);
    assert.deepEqual(stage.evaluatedAssertions, expected.evaluated_assertions);
    assert.deepEqual(stage.findings.map(f => [f.code, snake(f.severity), snake(f.layer), f.message, f.assertionSite]),
      expected.findings.map(f => [f.code, f.severity, f.layer, f.message, f.assertion_site ?? null]));
  });
  const findings = report.stages.flatMap(stage => stage.findings);
  const typeCode = findings.find(f => f.code === 'BR-KSA-05');
  assert.equal(typeCode.severity, 'Error');
  assert.equal(typeCode.layer, 'BusinessRules');
  assert.equal(typeCode.location.kind, 'XPath');
  assert.match(typeCode.location.path, /InvoiceTypeCode/);
  assert.equal(findings.find(f => f.code === 'SIGNATURE_DIGEST').location, null);
});

test('clearance and compliance use their own acceptance fields', async () => {
  const client = new sdk.ZatcaClient(config, {fetch: async (url) => url.includes('/clearance/')
    ? json({validationResults: {status: 'PASS'}, clearanceStatus: 'CLEARED', reportingStatus: 'NOT_REPORTED'})
    : json({validationResults: {status: 'WARNING'}, reportingStatus: 'NOT_REPORTED'})});
  const standard = sdk.SignedInvoice.fromXml(loadFixture('sdk-parity/cases/standard-invoice/sdk-signed.xml'));
  assert.equal((await client.clearStandardInvoice(standard, pcsid(), sdk.ClearanceMode.Enabled)).outcome.value, 'Accepted');
  assert.equal((await client.checkInvoiceCompliance(invoice(), ccsid())).outcome.value, 'Accepted');
  await assert.rejects(client.clearStandardInvoice(invoice(), pcsid(), sdk.ClearanceMode.Enabled), code(9));
  assert.equal((await client.clearStandardInvoice(standard, pcsid(), sdk.ClearanceMode.Enabled)).clearanceStatus, sdk.ClearanceStatus.Cleared);
  for (const mode of [true, false, 'Enabled', 'Disabled', 1, undefined]) {
    await assert.rejects(client.clearStandardInvoice(standard, pcsid(), mode), TypeError);
    await assert.rejects(client.reportSimplifiedInvoice(invoice(), pcsid(), mode), TypeError);
  }
});

test('compressed schemas preserve valid Arabic input and reject schema-invalid XML', () => {
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  assert.equal(sdk.Xml.validate(xml), true);
  const invalid = xml.replaceAll('cbc:ProfileID', 'cbc:InvalidProfileID');
  assert.notEqual(invalid, xml);
  assert.throws(() => sdk.Xml.validate(invalid), e => e instanceof sdk.FatooraError && e.cause instanceof sdk.BindingError);
  // Failed runs must not corrupt the cached compiled schema.
  assert.equal(sdk.Xml.validate(xml), true);
});

test('documented examples and signing work in the optimized artifact', async () => {
  const examples = await import('../examples/sdk.mjs');
  const csr = examples.generateCsr(loadFixture('csr-configs/csr-config-example-EN.properties'));
  assert.match(csr.csrPem, /BEGIN CERTIFICATE REQUEST/);
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  assert.equal(examples.validateInvoice(xml, undefined, '2025-01-01T00:00:00Z').stages.length, 6);
  const response = await examples.reportInvoice(xml, 'token', 'secret', {fetch: async () => json(validationBody)});
  assert.equal(response.outcome.value, 'Accepted');
  const cert = readFileSync(new URL('../../../fatoora-core/tests/fixtures/sdk-parity/credentials/certificate.der', import.meta.url));
  const key = readFileSync(new URL('../../../fatoora-core/tests/fixtures/sdk-parity/credentials/private-key.der', import.meta.url));
  const signer = sdk.InvoiceSigner.fromDer(cert, key);
  const signed = examples.signInvoice(xml, signer.certificatePem, sdk.SigningKey.fromDer(key).toPem());
  assert.equal(sdk.Xml.hash(signed), sdk.Xml.hash(xml));
  assert.ok(sdk.SignedInvoice.fromXml(signed).signature);
});

test('binary data is a Uint8Array copy in and out', () => {
  const key = sdk.SigningKey.generate();
  const bytes = key.toDer();
  assert.equal(bytes.asSlice, undefined);
  assert.equal(raw.Bytes, sdk.Bytes);
  const der = bytes.toUint8Array();
  assert.ok(der instanceof Uint8Array);
  assert.ok(der.length > 0);
  // Each call returns an independent copy outside WASM memory.
  const again = bytes.toUint8Array();
  assert.notEqual(der.buffer, again.buffer);
  assert.notEqual(der.buffer, wasm.memory.buffer);
  der.fill(0);
  assert.deepEqual(bytes.toUint8Array(), again);
  // Inputs accept a Uint8Array, a Buffer or an array of byte values.
  assert.deepEqual(sdk.SigningKey.fromDer(again).toDer().toUint8Array(), again);
  assert.deepEqual(sdk.SigningKey.fromDer(Buffer.from(again)).toDer().toUint8Array(), again);
  assert.deepEqual(sdk.SigningKey.fromDer([...again]).toDer().toUint8Array(), again);
  assert.throws(() => sdk.SigningKey.fromDer(new Uint8Array([1, 2, 3])), e => e instanceof sdk.FatooraError);
});

test('validation options preserve Rust errors before host clock injection', () => {
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  for (const options of ['{', '', 'null', 'true', '42', '"text"', '[]', '[1]', '{"evaluated_at":42}', ' '.repeat(4096) + '{}', JSON.stringify({unknown: 'ع'.repeat(2048)})]) {
    let expected;
    try { generated.Xml.validateZatca(xml, options); }
    catch (error) { expected = error; }
    assert.ok(expected?.cause instanceof sdk.BindingError, options.slice(0, 40));
    assert.throws(() => sdk.Xml.validateZatca(xml, options), code(expected.cause.code()));
  }
  for (const options of [null, '{}', '{"evaluated_at":null}', '{"evaluated_at":"2025-01-01T00:00:00Z"}']) {
    assert.equal(sdk.Xml.validateZatca(xml, options).stages.length, 6);
  }
});

test('imported signed invoice hashes its stored XML', () => {
  for (const kind of ['standard', 'simplified']) {
    const xml = loadFixture(`sdk-parity/cases/${kind}-invoice/sdk-signed.xml`);
    assert.equal(sdk.SignedInvoice.fromXml(xml).hash, sdk.Xml.hash(xml));
  }
});

test('optional line fields and repeated identifiers survive WASM import', () => {
  const b = sdk.InvoiceBuilder.create(sdk.InvoiceTypeKind.Tax, sdk.InvoiceSubType.Simplified, null, null, null, null);
  b.setId('JS-fields'); b.setUuid('uuid'); b.setIssueDatetime('2024-01-01T12:30:00Z');
  b.setCurrency('SAR'); b.setPreviousInvoiceHash('hash'); b.setInvoiceCounter(1n); b.setVatCategory(sdk.VatCategory.Standard);
  b.setSeller('Seller', sdk.Address.create('SA', 'Riyadh', 'Street', '1234', '12222', null, null, null), '301121971500003', 'first', 'CRN');
  b.addSellerIdentifier('second', 'OTH');
  const line = sdk.LineItemBuilder.create();
  line.setItemName('case'); line.setInvoicedQuantity('30'); line.setItemNetPrice('100');
  line.setItemPriceBaseQuantity('12'); line.setVatRate('15'); line.setVatCategory(sdk.VatCategory.Standard);
  line.addAllowance(sdk.Adjustment.percentOf('250', '10').withReason('rebate', '95'));
  b.addItem(line.build());
  b.addPaymentMeans(sdk.PaymentMeans.create('30', 'one').withInstructionNote('two').withAccount('bank', 'net 30'));
  const invoice = b.build();
  const parsed = sdk.FinalizedInvoice.fromXml(invoice.xml);
  assert.equal(parsed.data().lineItem(0).totalAmount, '225');
  assert.equal(parsed.data().lineItem(0).itemPriceBaseQuantity, '12');
  assert.equal(parsed.data().seller().identifiersLen, 2);
  assert.equal(parsed.data().seller().identifier(1).value, 'second');
  assert.equal(parsed.data().paymentMeans(0).instructionNotesLen, 2);
  assert.equal(parsed.data().paymentMeans(0).instructionNoteAt(1), 'two');
  assert.equal(parsed.xml, invoice.xml);
});

test('signed import rejects the SDK fixture with a shifted QR timestamp', () => {
  assert.throws(() => sdk.SignedInvoice.fromXml(
    loadFixture('sdk-parity-typed/cases/standard-invoice/sdk-signed.xml')),
    e => e instanceof sdk.FatooraError && e.cause instanceof sdk.BindingError && e.message.includes('QR_TIMESTAMP'));
});

test('signed import preserves XML with line-wrapped certificates', () => {
  for (const kind of ['standard', 'simplified']) {
    const xml = loadFixture(`sdk-parity/cases/${kind}-invoice/sdk-signed.xml`);
    const wrapped = xml.replace(/(<ds:X509Certificate>)([^<]+)(<\/ds:X509Certificate>)/,
      (_, start, certificate, end) => start + certificate.match(/.{1,64}/g).join(' \t\r\n') + end);
    assert.notEqual(wrapped, xml);
    const original = sdk.SignedInvoice.fromXml(xml);
    const parsed = sdk.SignedInvoice.fromXml(wrapped);
    assert.equal(parsed.xml, wrapped);
    assert.equal(parsed.qrCode, original.qrCode);
  }
});

test('library failures throw one error class per code', () => {
  assert.equal(sdk.FatooraError.fromCode(null, 'm').constructor, sdk.FatooraError);
  const future = sdk.FatooraError.fromCode(99, 'm', {details: {type: 'future'}});
  assert.equal(future.constructor, sdk.FatooraError);
  assert.deepEqual([future.code, future.kind, future.details], [99, null, {type: 'future'}]);
  const names = ['InvalidInputError', 'ValidationError', 'ParseError', 'XmlError', 'CryptoError', 'IoError',
    'NetworkError', 'UnauthorizedError', 'InternalError', 'ApiError'];
  assert.deepEqual(Object.keys(sdk.ErrorKind), names.map(n => n.replace(/Error$/, '')));
  for (const [index, kind] of Object.values(sdk.ErrorKind).entries()) {
    const n = index + 1;
    const ErrorClass = sdk[`${kind}Error`];
    const error = sdk.FatooraError.fromCode(n, 'm');
    assert.equal(error.constructor, ErrorClass);
    assert.equal(error.code, n);
    assert.ok(error instanceof sdk.FatooraError && error instanceof Error);
    assert.equal(error.name, `${kind}Error`);
    assert.equal(error.kind, kind);
    // Constructed directly, a subclass carries its own code and kind.
    const direct = new ErrorClass('m');
    assert.equal(direct.code, n);
    assert.equal(direct.kind, kind);
  }
  const base = new sdk.FatooraError('m');
  assert.equal(base.code, null);
  assert.equal(base.kind, null);
  let error;
  try { sdk.Address.create('SAU', 'bad\0city', 'street', '1234', '12222', null, null, null); }
  catch (caught) { error = caught; }
  assert.ok(error instanceof sdk.InvalidInputError);
  assert.equal(error.message, 'text must not contain NUL');
  assert.deepEqual(error.details, {type: 'binding_error'});
  assert.equal(error.kind, 'InvalidInput');
  assert.doesNotMatch(error.stack.split('\n')[1], /patch\.mjs|errors\.mjs/);
  assert.throws(() => sdk.SigningKey.fromPem('not a key'), e => e instanceof sdk.FatooraError && e.code === e.cause.code());
  // fatoority/raw shares the generated classes, so it throws the same types.
  assert.throws(() => raw.Address.create('SAU', '\0', '', '', '', null, null, null), sdk.InvalidInputError);
});

test('create replaces new_ and the package root exports a deliberate API', () => {
  const withCreate = Object.entries(generated).filter(([, value]) => typeof value?.create === 'function').map(([name]) => name);
  assert.equal(withCreate.length, 14);
  for (const name of withCreate) {
    assert.equal(generated[name].new_, undefined, name);
    if (name !== 'ZatcaClient') assert.equal(sdk[name], generated[name]);
  }
  assert.ok(sdk.Config.create(sdk.Environment.Simulation) instanceof sdk.Config);
  assert.equal(sdk.Config.create('Production').env, sdk.Environment.Production);
  const replaced = ['Xml', 'ZatcaClient', 'ZatcaValidationReport', 'ZatcaStageReport', 'ZatcaFinding', 'ZatcaRuleSource'];
  for (const [name, value] of Object.entries(generated)) {
    if (replaced.includes(name)) assert.notEqual(sdk[name], value, name);
    else assert.equal(sdk[name], value, name);
  }
  assert.deepEqual(Object.keys(raw).sort(), Object.keys(generated).sort());
});

test('enum value unions in index.d.mts match the runtime enum classes', () => {
  const declarations = readFileSync(new URL('../index.d.mts', import.meta.url), 'utf8');
  const unions = new Map([...declarations.matchAll(/^export type (\w+)Value = (.+);$/gm)]
    .map(([, name, body]) => [name, [...body.matchAll(/'(\w+)'/g)].map(m => m[1])]));
  const enums = Object.entries(generated).filter(([, value]) => typeof value?.getAllEntries === 'function');
  assert.deepEqual([...unions.keys()].sort(), enums.map(([name]) => name).sort());
  for (const [name, Enum] of enums) {
    const values = [...Enum.getAllEntries()].map(([value]) => value);
    assert.deepEqual(unions.get(name), values, name);
    for (const value of values) assert.equal(new Enum(value).value, value);
  }
});

test('generated boolean getters return booleans', () => {
  const declared = {};
  for (const name of Object.keys(generated)) {
    let source;
    try { source = readFileSync(new URL(`../generated/${name}.d.mts`, import.meta.url), 'utf8'); }
    catch { continue; }
    // `*Present` halves of optional-string pairs are removed, not adapted.
    const hidden = Object.values(optionalResults[name] ?? {});
    const methods = [...source.matchAll(/^    (\w+)\(\): boolean;$/gm)].map(m => m[1])
      .filter(method => !hidden.includes(method));
    if (methods.length) declared[name] = methods;
  }
  assert.deepEqual(integerBooleanMethods, declared);
  const report = raw.Xml.validateZatca(loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml'), '{"evaluated_at":"2025-01-01T00:00:00Z"}');
  assert.equal(typeof report.isValid, 'boolean');
  assert.equal(sdk.PaymentMeans.create('10', null).hasAccount, false);
  assert.equal(sdk.Xml.validate(loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml')), true);
});

test('renamed members have only their core names', () => {
  const rebate = sdk.Adjustment.percentOf('250', '10');
  assert.equal(rebate.value, '25');
  assert.equal(rebate.percentage, '10');
  assert.equal(rebate.suppliedAmount('25').value, '25');
  assert.throws(() => rebate.suppliedAmount('24').value, sdk.ValidationError);
  const subtotal = sdk.VatBreakdown.create(sdk.VatCategory.OutOfScope, '0', '10', '0', sdk.TaxExemption.create('VATEX-SA-OOS', 'reason'));
  assert.equal(subtotal.omitVatRate().hasVatRate, false);
  // The 0.5 names, deprecated in 0.6, are gone in 0.7.
  assert.equal(Object.hasOwn(sdk.Adjustment, 'percentage'), false);
  for (const name of ['withSuppliedAmount', 'percent']) assert.equal(name in sdk.Adjustment.prototype, false, name);
  for (const Class of [sdk.AllowanceCharge, sdk.VatBreakdown]) assert.equal('withoutVatRate' in Class.prototype, false);
});

test('value getters are read-only properties and optional text is a plain string', () => {
  for (const [name, members] of Object.entries(properties)) {
    for (const member of members) {
      const descriptor = Object.getOwnPropertyDescriptor(generated[name].prototype, member);
      assert.equal(typeof descriptor?.get, 'function', `${name}.${member}`);
      assert.equal(descriptor.set, undefined, `${name}.${member}`);
    }
  }
  const address = sdk.Address.create('SA', 'Riyadh', 'Street', '1234', '12222', null, '5678', 'Olaya');
  assert.equal(address.city, 'Riyadh');
  assert.equal(address.district, 'Olaya');
  assert.equal(address.additionalStreet, null);
  assert.equal(address.province, null);
  assert.equal(address.withProvince('Riyadh Region').province, 'Riyadh Region');
  // The 0.5 call form is a hard break: the property value is not callable.
  assert.throws(() => address.city(), TypeError);
  assert.throws(() => { 'use strict'; address.city = 'Jeddah'; }, TypeError);
  for (const [name, pairs] of Object.entries(optionalResults)) {
    for (const [member, has] of Object.entries(pairs)) {
      const descriptor = Object.getOwnPropertyDescriptor(generated[name].prototype, member);
      assert.equal(typeof descriptor?.get, 'function', `${name}.${member}`);
      assert.equal(has in generated[name].prototype, false, `${name}.${has}`);
    }
  }
  const rebate = sdk.Adjustment.percentOf('250', '10').withReason('rebate', null);
  assert.deepEqual([rebate.value, rebate.baseAmount, rebate.percentage, rebate.reason, rebate.reasonCode],
    ['25', '250', '10', 'rebate', null]);
  const invoice = sdk.SignedInvoice.fromXml(loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml'));
  assert.equal(typeof invoice.data().invoiceCounter, 'bigint');
  assert.equal(invoice.data().vatCategory, sdk.VatCategory.Standard);
  assert.equal(typeof invoice.totals().payableAmount, 'string');
});

test('property getters throw the typed errors at the caller', () => {
  const invoice = sdk.SignedInvoice.fromXml(loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml'));
  invoice.intoXml();
  let error;
  try { void invoice.qrCode; }
  catch (caught) { error = caught; }
  assert.ok(error instanceof sdk.FatooraError, String(error));
  assert.doesNotMatch(error.stack.split('\n')[1], /patch\.mjs|errors\.mjs/);
});
