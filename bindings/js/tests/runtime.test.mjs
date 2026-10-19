import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import * as sdk from '../index.mjs';
import * as raw from '../generated/index.mjs';
import wasm from '../generated/diplomat-wasm.mjs';
const loadFixture = (path) => readFileSync(new URL(`../../../fatoora-core/tests/fixtures/${path}`, import.meta.url), 'utf8');
const config = sdk.Config.new_(0);
const ccsid = () => sdk.CsidCompliance.create(0, '123', 'token', 'secret');
const pcsid = () => sdk.CsidProduction.create(0, null, 'token', 'secret');
const invoice = () => sdk.SignedInvoice.fromXml(loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml'));
const credentialsBody = { requestID: 123, binarySecurityToken: 'new-token', secret: 'new-secret' };
const validationBody = { validationResults: { status: 'PASS' }, reportingStatus: 'REPORTED' };
const json = (body, status = 200) => new Response(JSON.stringify(body), { status });
const code = (n) => (error) => error.cause instanceof sdk.BindingError && error.cause.code() === n;

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
      const address = sdk.Address.new_('SAU', 'الرياض', 'شارع الملك', '1234', '12222', value, null, null);
      assert.equal(address.additionalStreet()?.value() ?? null, value);
      assert.equal(address.city(), 'الرياض');
    }
  }
  assert.throws(() => sdk.Address.new_('SAU', 'bad\0city', 'street', '1234', '12222', null, null, null), code(1));
});

test('decimal values remain exact and consumed builders fail safely', () => {
  const b = sdk.InvoiceBuilder.new_(0, 1, null, null, null, null);
  b.setId('JS-1'); b.setUuid('js-uuid'); b.setIssueDatetime('2024-01-01T12:30:00Z');
  b.setCurrency('SAR'); b.setPreviousInvoiceHash('hash'); b.setInvoiceCounter(0n); b.setPaymentMeansCode('10'); b.setVatCategory(1);
  const a = sdk.Address.new_('SAU', 'Riyadh', 'King Fahd', '1234', '12222', null, null, null);
  b.setSeller('Acme Inc', a, '301121971500003', null, null);
  b.addLineItem('Item', '3', 'PCE', '0.3333', '15', 1);
  const inv = b.build();
  assert.equal(inv.totals().taxInclusive(), '1.15');
  assert.match(inv.xml(), />0.3333<\/cbc:PriceAmount>/);
  assert.throws(() => b.build(), code(1));
});

test('prepared request retains context after invoice consumption and WASM memory growth', () => {
  const inv = invoice();
  const request = raw.ZatcaClient.create(config).prepareReportSimplifiedInvoice(inv, pcsid(), false, 'ar');
  const bytes = request.body().asSlice();
  const original = JSON.parse(Buffer.from(bytes).toString());
  inv.intoXml();
  wasm.memory.grow(1);
  for (let i = 0; i < 20; i++) sdk.SigningKey.generate();
  assert.equal(JSON.parse(Buffer.from(request.body().asSlice()).toString()).invoiceHash, original.invoiceHash);
  const result = request.decode(200, [...Buffer.from(JSON.stringify(validationBody))]);
  assert.equal(result.httpStatus(), 200);
  assert.equal(result.outcome().value, 'Accepted');
});

test('fetch sees Rust headers and body; concurrent requests decode independently', async () => {
  const pending = [];
  const client = new sdk.ZatcaClient(config, { fetch: (url, init) => new Promise(resolve => pending.push({url, init, resolve})) });
  const a = client.reportSimplifiedInvoice(invoice(), pcsid(), false, 'ar');
  const b = client.postCcsidForPcsid(ccsid());
  assert.equal(pending.length, 2);
  const headers = new Headers(pending[0].init.headers);
  assert.equal(headers.get('accept-version'), 'V2');
  assert.equal(headers.get('accept-language'), 'ar');
  assert.equal(headers.get('authorization'), `Basic ${Buffer.from('token:secret').toString('base64')}`);
  assert.equal(pending[0].init.redirect, 'error');
  assert.equal(JSON.parse(new TextDecoder().decode(pending[1].init.body)).compliance_request_id, '123');
  pending[1].resolve(json(credentialsBody));
  assert.equal((await b).binarySecurityToken(), 'new-token');
  pending[0].resolve(json(validationBody));
  assert.equal((await a).httpStatus(), 200);
});

test('all credential operations, including renewal 428, share Rust decoding', async () => {
  const csr = sdk.CsrProperties.fromPropertiesStr(loadFixture('csr-configs/csr-config-example-EN.properties')).build(sdk.SigningKey.generate(), 0);
  const calls = [];
  const client = new sdk.ZatcaClient(config, { fetch: async (url, init) => {
    calls.push({url, init});
    return init.method === 'PATCH' ? json({value: credentialsBody}, 428) : json(credentialsBody);
  }});
  assert.equal((await client.postCsrForCcsid(csr, '123456')).env(), 0);
  assert.equal((await client.renewCsid(pcsid(), csr, '123456')).requestId().value(), '123');
  assert.equal(new Headers(calls[0].init.headers).get('otp'), '123456');
  assert.equal(calls[1].init.method, 'PATCH');
});

test('HTTP failures decode in Rust; network, body-read and cancellation failures stay transport errors', async () => {
  for (const status of [400, 401, 409, 500]) {
    const client = new sdk.ZatcaClient(config, { fetch: async () => json(validationBody, status) });
    await assert.rejects(client.reportSimplifiedInvoice(invoice(), pcsid()), (e) => {
      assert.equal(JSON.parse(e.cause.detailsJson()).http_status, status); return true;
    });
  }
  const malformed = new sdk.ZatcaClient(config, { fetch: async () => new Response('bad') });
  await assert.rejects(malformed.reportSimplifiedInvoice(invoice(), pcsid()), code(3));
  const broken = new sdk.ZatcaClient(config, { fetch: async () => ({ status: 200, arrayBuffer: async () => { throw new Error('read'); } }) });
  await assert.rejects(broken.postCcsidForPcsid(ccsid()), e => e instanceof sdk.TransportError && e.httpStatus === 200);
  const abort = new AbortController(); abort.abort();
  const cancelled = new sdk.ZatcaClient(config, { fetch: async (_url, {signal}) => { signal.throwIfAborted(); } });
  await assert.rejects(cancelled.postCcsidForPcsid(ccsid(), {signal: abort.signal}), e => e instanceof sdk.TransportError && e.cause.name === 'AbortError');
});

test('validation uses supplied time and the public facade supplies a default', () => {
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  assert.throws(() => raw.Xml.validateZatca(config, xml, null), e => e.cause.message().includes('evaluated_at'));
  const a = sdk.Xml.validateZatca(config, xml, JSON.stringify({evaluated_at: '2025-01-01T00:00:00Z'}));
  assert.equal(JSON.parse(a).evaluated_at, '2025-01-01T00:00:00Z');
  assert.ok(JSON.parse(sdk.Xml.validateZatca(config, xml)).evaluated_at);
});

test('clearance and compliance use their own acceptance fields', async () => {
  const client = new sdk.ZatcaClient(config, {fetch: async (url) => url.includes('/clearance/')
    ? json({validationResults: {status: 'PASS'}, clearanceStatus: 'CLEARED', reportingStatus: 'NOT_REPORTED'})
    : json({validationResults: {status: 'WARNING'}, reportingStatus: 'NOT_REPORTED'})});
  const standard = sdk.SignedInvoice.fromXml(loadFixture('sdk-parity/cases/standard-invoice/sdk-signed.xml'));
  assert.equal((await client.clearStandardInvoice(standard, pcsid())).outcome().value, 'Accepted');
  assert.equal((await client.checkInvoiceCompliance(invoice(), ccsid())).outcome().value, 'Accepted');
  await assert.rejects(client.clearStandardInvoice(invoice(), pcsid()));
});

test('compressed schemas preserve valid Arabic input and reject schema-invalid XML', () => {
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  assert.equal(sdk.Xml.validate(config, xml), true);
  const invalid = xml.replaceAll('cbc:ProfileID', 'cbc:InvalidProfileID');
  assert.notEqual(invalid, xml);
  assert.throws(() => sdk.Xml.validate(config, invalid), e => e.cause instanceof sdk.BindingError);
  // Failed runs must not corrupt the cached compiled schema.
  assert.equal(sdk.Xml.validate(config, xml), true);
});

test('documented examples and signing work in the optimized artifact', async () => {
  const examples = await import('../examples/sdk.mjs');
  const csr = examples.generateCsr(loadFixture('csr-configs/csr-config-example-EN.properties'));
  assert.match(csr.csrPem, /BEGIN CERTIFICATE REQUEST/);
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  assert.ok(examples.validateInvoice(xml, undefined, '2025-01-01T00:00:00Z').stages);
  const response = await examples.reportInvoice(xml, 'token', 'secret', {fetch: async () => json(validationBody)});
  assert.equal(response.outcome().value, 'Accepted');
  const cert = readFileSync(new URL('../../../fatoora-core/tests/fixtures/sdk-parity/credentials/certificate.der', import.meta.url));
  const key = readFileSync(new URL('../../../fatoora-core/tests/fixtures/sdk-parity/credentials/private-key.der', import.meta.url));
  const signer = sdk.Signer.fromDer([...cert], [...key]);
  const signed = examples.signInvoice(xml, signer.certificatePem(), sdk.SigningKey.fromDer([...key]).toPem());
  assert.equal(sdk.Xml.hash(signed), sdk.Xml.hash(xml));
  assert.ok(sdk.SignedInvoice.fromXml(signed).signature());
});

test('validation options preserve Rust errors before host clock injection', () => {
  const xml = loadFixture('sdk-parity/cases/simplified-invoice/sdk-signed.xml');
  for (const options of ['{', '', 'null', 'true', '42', '"text"', '[]', '[1]', '{"evaluated_at":42}', ' '.repeat(4096) + '{}', JSON.stringify({unknown: 'ع'.repeat(2048)})]) {
    let expected;
    try { raw.Xml.validateZatca(config, xml, options); }
    catch (error) { expected = error; }
    assert.ok(expected?.cause instanceof sdk.BindingError, options.slice(0, 40));
    assert.throws(() => sdk.Xml.validateZatca(config, xml, options), code(expected.cause.code()));
  }
  for (const options of [null, '{}', '{"evaluated_at":null}', '{"evaluated_at":"2025-01-01T00:00:00Z"}']) {
    assert.ok(JSON.parse(sdk.Xml.validateZatca(config, xml, options)).stages);
  }
});

test('imported signed invoice hashes its stored XML', () => {
  for (const kind of ['standard', 'simplified']) {
    const xml = loadFixture(`sdk-parity/cases/${kind}-invoice/sdk-signed.xml`);
    assert.equal(sdk.SignedInvoice.fromXml(xml).hashBase64(), sdk.Xml.hash(xml));
  }
});

test('optional line fields and repeated identifiers survive WASM import', () => {
  const b = sdk.InvoiceBuilder.new_(0, 1, null, null, null, null);
  b.setId('JS-fields'); b.setUuid('uuid'); b.setIssueDatetime('2024-01-01T12:30:00Z');
  b.setCurrency('SAR'); b.setPreviousInvoiceHash('hash'); b.setInvoiceCounter(1n); b.setVatCategory(1);
  b.setSeller('Seller', sdk.Address.new_('SA', 'Riyadh', 'Street', '1234', '12222', null, null, null), '301121971500003', 'first', 'CRN');
  b.addSellerIdentifier('second', 'OTH');
  const line = sdk.LineItemBuilder.new_();
  line.setItemName('case'); line.setInvoicedQuantity('30'); line.setItemNetPrice('100');
  line.setItemPriceBaseQuantity('12'); line.setVatRate('15'); line.setVatCategory(1);
  line.addAllowance(sdk.Adjustment.percentage('250', '10').withReason('rebate', '95'));
  b.addItem(line.build());
  b.addPaymentMeans(sdk.PaymentMeans.new_('30', 'one').withInstructionNote('two').withAccount('bank', 'net 30'));
  const invoice = b.build();
  const parsed = sdk.FinalizedInvoice.fromXml(invoice.xml());
  assert.equal(parsed.data().lineItem(0).totalAmount(), '225');
  assert.equal(parsed.data().lineItem(0).itemPriceBaseQuantity().value(), '12');
  assert.equal(parsed.data().seller().identifiersLen(), 2);
  assert.equal(parsed.data().seller().identifier(1).value(), 'second');
  assert.equal(parsed.data().paymentMeans(0).instructionNotesLen(), 2);
  assert.equal(parsed.data().paymentMeans(0).instructionNoteAt(1).value(), 'two');
  assert.equal(parsed.xml(), invoice.xml());
});

test('signed import rejects the SDK fixture with a shifted QR timestamp', () => {
  assert.throws(() => sdk.SignedInvoice.fromXml(
    loadFixture('sdk-parity-typed/cases/standard-invoice/sdk-signed.xml')),
    e => e.cause instanceof sdk.BindingError && e.cause.message().includes('QR_TIMESTAMP'));
});

test('signed import preserves XML with line-wrapped certificates', () => {
  for (const kind of ['standard', 'simplified']) {
    const xml = loadFixture(`sdk-parity/cases/${kind}-invoice/sdk-signed.xml`);
    const wrapped = xml.replace(/(<ds:X509Certificate>)([^<]+)(<\/ds:X509Certificate>)/,
      (_, start, certificate, end) => start + certificate.match(/.{1,64}/g).join(' \t\r\n') + end);
    assert.notEqual(wrapped, xml);
    const original = sdk.SignedInvoice.fromXml(xml);
    const parsed = sdk.SignedInvoice.fromXml(wrapped);
    assert.equal(parsed.xml(), wrapped);
    assert.equal(parsed.qrCode(), original.qrCode());
  }
});
