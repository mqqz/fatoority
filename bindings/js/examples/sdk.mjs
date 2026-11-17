// --8<-- [start:csr]
import { CsrProperties, Environment, SigningKey } from 'fatoority';

export function generateCsr(propertiesText) {
  const key = SigningKey.generate();
  const csr = CsrProperties.fromStr(propertiesText).build(key, Environment.NonProduction);
  return { csrPem: csr.toPem(), keyPem: key.toPem() };
}
// --8<-- [end:csr]

// --8<-- [start:signing]
import { InvoiceSigner } from 'fatoority';

export function signInvoice(xml, certificatePem, keyPem) {
  return InvoiceSigner.fromPem(certificatePem, keyPem).signXml(xml);
}
// --8<-- [end:signing]

// --8<-- [start:validation]
import { Xml } from 'fatoority';

export function validateInvoice(xml, previousHash, evaluatedAt) {
  return Xml.validateZatca(xml, JSON.stringify({
    previous_invoice_hash: previousHash,
    evaluated_at: evaluatedAt, // Omit to use the JavaScript host clock.
  }));
}
// Inspect report.isValid() and report.stage(i) before accepting an invoice;
// report.toJson() returns the same report as JSON.
// --8<-- [end:validation]

// --8<-- [start:api]
import { Config as ApiConfig, ProductionCsid, Environment as ApiEnvironment, SignedInvoice, ZatcaClient } from 'fatoority';

export async function reportInvoice(signedXml, token, secret, { signal, fetch: transport = globalThis.fetch } = {}) {
  const client = new ZatcaClient(ApiConfig.new_(ApiEnvironment.Production), { fetch: transport });
  const credentials = ProductionCsid.create(ApiEnvironment.Production, null, token, secret);
  const invoice = SignedInvoice.fromXml(signedXml);
  const response = await client.reportSimplifiedInvoice(invoice, credentials, false, 'en', { signal });
  response.ensureAccepted();
  return response;
}
// --8<-- [end:api]
