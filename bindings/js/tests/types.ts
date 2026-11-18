import {
  Address, Adjustment, ClearanceMode, Config, Environment, FatooraError, InvalidInputError, InvoiceBuilder,
  InvoiceSubType, InvoiceTypeKind, LineItemBuilder, PaymentMeans, ReportingStatus, Severity, NetworkError, ErrorKind,
  ZatcaClient, ComplianceCsid, Xml, sendPrepared,
  type EnvironmentValue, type ProductionCsid, type SeverityValue, type SignedInvoice,
  type ValidationResponse, type ZatcaFinding, type ZatcaStageReport, type ZatcaValidationReport,
} from 'fatoority';
import { ZatcaClient as RawClient, Config as RawConfig } from 'fatoority/raw';

const config = Config.create(Environment.NonProduction);
const rawConfig: Config = RawConfig.create(Environment.NonProduction); // raw instances are root instances
const credentials = ComplianceCsid.create(Environment.NonProduction, '1', 'token', 'secret');
const client = new ZatcaClient(config, {fetch});
const response: Promise<ProductionCsid> = client.requestProductionCsid(credentials, {signal: new AbortController().signal});
const prepared = RawClient.create(rawConfig).prepareRequestProductionCsid(credentials);
const sent: Promise<ProductionCsid> = sendPrepared(prepared);
declare const invoice: SignedInvoice, production: ProductionCsid;
const reported: Promise<ValidationResponse> = client.reportSimplifiedInvoice(invoice, production, ClearanceMode.Disabled);
const cleared: Promise<ValidationResponse> = client.clearStandardInvoice(invoice, production, ClearanceMode.Enabled);
// @ts-expect-error the Clearance-Status header takes a ClearanceMode, not a boolean
client.clearStandardInvoice(invoice, production, true);
// @ts-expect-error nor the variant name
client.clearStandardInvoice(invoice, production, 'Enabled');
// @ts-expect-error nor may it be omitted
client.reportSimplifiedInvoice(invoice, production);

const builder: InvoiceBuilder = InvoiceBuilder.create(InvoiceTypeKind.Tax, InvoiceSubType.Simplified, null, null, null, null);
builder.addPaymentMeans(PaymentMeans.create('10', null));
builder.setSeller('Seller', Address.create('SA', 'Riyadh', 'Street', '1234', '12222', null, null, null), '301121971500003', null, null);
const line = LineItemBuilder.create();
const rebate: Adjustment = Adjustment.percentOf('250', '10').suppliedAmount('25');
const percentage: string | null = rebate.percentage; // value getters are properties
const amount: string = rebate.value; // decimal amounts are strings
// @ts-expect-error 0.5 getter calls are a compile error
rebate.value();
const city: string = Address.create('SA', 'Riyadh', 'Street', '1234', '12222', null, null, null).city;
const district: string | null = Address.create('SA', 'Riyadh', 'Street', '1234', '12222', null, null, 'Olaya').district;
const environmentProperty: Environment = rawConfig.env;
// @ts-expect-error properties are read-only
rawConfig.env = Environment.Production;
// @ts-expect-error the 0.5 names were removed in 0.7
Adjustment.percentage('250', '10');
// @ts-expect-error the 0.5 names were removed in 0.7
rebate.withSuppliedAmount('25');
// @ts-expect-error `new_` was removed in 0.7
Config.new_(Environment.NonProduction);
line.addAllowance(rebate);
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
  if (error instanceof NetworkError) { const kind: 'Network' = error.kind; const network: FatooraError = error; void [kind, network]; }
  if (error instanceof FatooraError && error.kind === ErrorKind.Network) { const kind: ErrorKind = error.kind; void kind; }
  if (error instanceof FatooraError) { const kind: ErrorKind | null = error.kind; const details: Record<string, unknown> = error.details; void [kind, details]; }
}
const crypto: FatooraError = FatooraError.fromCode(5, 'message', { details: {} });

declare const validation: ValidationResponse;
const status: ReportingStatus | null = validation.reportingStatus;
const statusText: string | null = validation.reportingStatusRaw;
const httpStatus: number | null = validation.httpStatus;
const results = validation.validationResults(); // native owners stay methods
const xml: string = invoice.xml;
const counter: bigint = invoice.data().invoiceCounter;
const url: string = prepared.url;

void [response, sent, reported, cleared, line, legacy, valid, bogus, severity, environment, crypto,
  percentage, amount, city, district, environmentProperty, status, statusText, httpStatus, results, xml, counter, url];
