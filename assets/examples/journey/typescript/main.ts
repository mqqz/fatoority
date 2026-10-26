import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { randomUUID } from 'node:crypto';
import { pathToFileURL } from 'node:url';
import { resolve } from 'node:path';
import {
  Address, BindingError, Config, CsidProduction, InvoiceBuilder, Signer, Xml, ZatcaClient,
  type ValidationResponse, type ValidationMessage,
} from 'fatoority';

const ENVIRONMENT = 0; // Integration Sandbox only.
interface Input {
  invoice_id: string;
  invoice_counter: string;
  previous_invoice_hash: string;
  seller: { name: string; vat_id: string; crn: string; city: string; street: string;
    building_number: string; postal_code: string; additional_number: string; district: string };
}
function required(name: string): string {
  const value = process.env[name]?.trim();
  if (!value) throw new Error(`Set ${name} first`);
  return value;
}
function saveJson(path: string, value: unknown): void {
  writeFileSync(path, JSON.stringify(value, null, 2) + '\n');
}
function messages(count: number, get: (index: number) => ValidationMessage) {
  return Array.from({ length: count }, (_, index) => {
    const item = get(index);
    return { code: item.code()?.value() ?? null, message: item.message()?.value() ?? null };
  });
}
export function handleResponse(response: ValidationResponse, output: string): void {
  const results = response.validationResults();
  const summary = {
    http_status: response.httpStatus(), outcome: response.outcome().value,
    reporting_status: response.reportingStatus()?.value() ?? null,
    validation_status: results.status()?.value() ?? null,
    info: messages(results.infoLen(), i => results.infoMessage(i)),
    warnings: messages(results.warningLen(), i => results.warningMessage(i)),
    errors: messages(results.errorLen(), i => results.errorMessage(i)),
  };
  saveJson(`${output}/response.json`, summary);
  console.log(JSON.stringify(summary, null, 2));
  response.ensureAccepted(); // A successful HTTP status alone is insufficient.
  console.log('Sandbox reporting accepted. Keep any warnings in response.json.');
}

export async function run(inputPath: string, submit: boolean, transport?: typeof fetch): Promise<void> {
  const certificate = readFileSync(required('ZATCA_CERT_PATH'), 'utf8');
  const key = readFileSync(required('ZATCA_KEY_PATH'), 'utf8');
  const token = submit ? required('ZATCA_TOKEN') : '';
  const secret = submit ? required('ZATCA_SECRET') : '';
  const data: Input = JSON.parse(readFileSync(inputPath, 'utf8'));
  const seller = data.seller;
  const uuid = randomUUID();
  const output = `runs/${uuid}`;
  mkdirSync('runs', { recursive: true });
  mkdirSync(output); // A new directory preserves earlier attempts.
  console.log(`Artifacts: ${output}`);
  const state: Record<string, unknown> = {
    uuid, invoice_id: data.invoice_id, stage: 'create', submitted: false,
  };
  saveJson(`${output}/state.json`, state);
  try {
    const builder = InvoiceBuilder.new_(0, 1, null, null, null, null); // Tax, simplified.
    builder.setId(data.invoice_id);
    builder.setUuid(uuid);
    builder.setIssueDatetime(new Date().toISOString().replace(/\.\d{3}Z$/, 'Z'));
    builder.setCurrency('SAR');
    builder.setInvoiceCounter(BigInt(data.invoice_counter));
    builder.setPreviousInvoiceHash(data.previous_invoice_hash);
    builder.setPaymentMeansCode('10');
    builder.setVatCategory(1); // Standard VAT category.
    const address = Address.new_('SA', seller.city, seller.street, seller.building_number,
      seller.postal_code, null, seller.additional_number, seller.district);
    builder.setSeller(seller.name, address, seller.vat_id, seller.crn, 'CRN');
    builder.addLineItem('دفتر', '2', 'PCE', '50', '15', 1);
    const invoice = builder.build();
    writeFileSync(`${output}/invoice.xml`, invoice.xml());
    console.log('Created: SAR 100 net + SAR 15 VAT = SAR 115 payable');
    state.stage = 'sign';
    const signed = Signer.fromPem(certificate, key).sign(invoice);
    const signedXml = signed.xml();
    writeFileSync(`${output}/signed.xml`, signedXml);
    state.invoice_hash = signed.invoiceHash();
    state.stage = 'validate';
    const config = Config.new_(ENVIRONMENT);
    const report = JSON.parse(Xml.validateZatca(config, signedXml, JSON.stringify({
      previous_invoice_hash: data.previous_invoice_hash,
    })));
    saveJson(`${output}/validation.json`, report);
    if (!report.is_valid) throw new Error('Local validation failed; inspect validation.json. Nothing was submitted.');
    console.log('Local validation passed; inspect validation.json for warnings');
    if (!submit) {
      state.stage = 'local_only';
      console.log('Local run complete. No reporting request sent. Use --submit with sandbox credentials.');
      return;
    }
    state.stage = 'report';
    state.submitted = true; // Attempted; a failed connection can leave the outcome unknown.
    saveJson(`${output}/state.json`, state);
    const client = new ZatcaClient(config, transport ? { fetch: transport } : {});
    const credentials = CsidProduction.create(ENVIRONMENT, null, token, secret);
    const response = await client.reportSimplifiedInvoice(signed, credentials, false, 'en');
    handleResponse(response, output);
    state.stage = 'accepted';
  } catch (error) {
    state.error_type = error instanceof Error ? error.name : 'UnknownError';
    if (error instanceof Error && error.cause instanceof BindingError) {
      saveJson(`${output}/error.json`, { code: error.cause.code(), details: JSON.parse(error.cause.detailsJson()) });
    }
    throw error;
  } finally {
    saveJson(`${output}/state.json`, state);
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  const args = process.argv.slice(2);
  let input = 'input.json';
  let submit = false;
  let valid = true;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--submit') submit = true;
    else if (args[i] === '--input' && args[i + 1]) input = args[++i];
    else valid = false;
  }
  if (!valid) {
    console.error('Usage: npm start -- [--input input.json] [--submit]');
    process.exitCode = 1;
  } else {
    run(input, submit).catch(error => {
      console.error(`Stopped: ${error instanceof Error ? error.message : String(error)}`);
      console.error('No automatic retry. Inspect saved artifacts before another run.');
      process.exitCode = 1;
    });
  }
}
