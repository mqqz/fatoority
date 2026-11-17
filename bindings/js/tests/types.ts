import {
  Address, ClearanceMode, Config, Environment, FatooraError, InvalidInputError, InvoiceBuilder,
  InvoiceSubType, InvoiceTypeKind, LineItemBuilder, PaymentMeans, Severity, TransportError, NetworkError,
  ZatcaClient, ComplianceCsid, Xml, errorClassForCode, ErrorCode, sendPrepared,
  type EnvironmentValue, type ErrorKind, type ProductionCsid, type SeverityValue, type SignedInvoice,
  type ValidationResponse, type ZatcaFinding, type ZatcaStageReport, type ZatcaValidationReport,
} from 'fatoority';
import { ZatcaClient as RawClient, Config as RawConfig } from 'fatoority/raw';

const config = Config.create(Environment.NonProduction);
const rawConfig: Config = RawConfig.new_(Environment.NonProduction); // raw instances are root instances
const credentials = ComplianceCsid.create(Environment.NonProduction, '1', 'token', 'secret');
const client = new ZatcaClient(config, {fetch});
const response: Promise<ProductionCsid> = client.requestProductionCsid(credentials, {signal: new AbortController().signal});
const prepared = RawClient.create(rawConfig).prepareRequestProductionCsid(credentials);
const sent: Promise<ProductionCsid> = sendPrepared(prepared);
declare const invoice: SignedInvoice, production: ProductionCsid;
const reported: Promise<ValidationResponse> = client.reportSimplifiedInvoice(invoice, production, ClearanceMode.Disabled);
const cleared: Promise<ValidationResponse> = client.clearStandardInvoice(invoice, production, true);
const named: Promise<ValidationResponse> = client.clearStandardInvoice(invoice, production, 'Enabled');

const builder: InvoiceBuilder = InvoiceBuilder.create(InvoiceTypeKind.Tax, InvoiceSubType.Simplified, null, null, null, null);
builder.addPaymentMeans(PaymentMeans.create('10', null));
builder.setSeller('Seller', Address.create('SA', 'Riyadh', 'Street', '1234', '12222', null, null, null), '301121971500003', null, null);
const line = LineItemBuilder.create();
// @ts-expect-error generated constructors are not public
new Config();

const report: ZatcaValidationReport = Xml.validateZatca('<Invoice/>', {previousInvoiceHash: 'hash', evaluatedAt: new Date()});
const legacy: ZatcaValidationReport = Xml.validateZatca('<Invoice/>', '{"evaluated_at":"2025-01-01T00:00:00Z"}');
const valid: boolean = report.isValid;
const stages: readonly ZatcaStageReport[] = report.stages;
const errors: ZatcaFinding[] = stages.flatMap(stage => stage.findings).filter(finding => finding.severity === 'Error');
for (const finding of errors) {
  const location = finding.location;
  if (location?.kind === 'Xml') { const line: number | null = location.line; void line; }
  else if (location?.kind === 'Field' || location?.kind === 'XPath') { const path: string | null = location.path; void path; }
}
// @ts-expect-error severities are a closed union
const bogus: SeverityValue = 'Fatal';
const severity: SeverityValue = Severity.Error.value as SeverityValue;
const environment: EnvironmentValue = 'Simulation';

try { Config.create(Environment.Production); }
catch (error) {
  if (error instanceof InvalidInputError) { const code: 1 = error.code; void code; }
  if (error instanceof TransportError) { const status: number | undefined = error.httpStatus; const network: NetworkError = error; void [status, network]; }
  if (error instanceof FatooraError) { const kind: ErrorKind | null = error.kind; const details: Record<string, unknown> = error.details; void [kind, details]; }
}
const ErrorClass: typeof FatooraError = errorClassForCode(ErrorCode.Crypto);

void [response, sent, reported, cleared, named, line, legacy, valid, bogus, severity, environment, ErrorClass];
