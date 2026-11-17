export * from './generated/index.mjs';
import type { ClearanceMode, Config, Csr, ComplianceCsid, ProductionCsid, SignedInvoice, ValidationResponse, InvoiceRequest, ComplianceRequest, ProductionRequest, ZatcaValidationReport } from './generated/index.mjs';
import { Xml as NativeXml } from './generated/index.mjs';
export interface RequestOptions { signal?: AbortSignal; }
export interface TransportOptions extends RequestOptions { fetch?: typeof fetch; }
export class TransportError extends Error { readonly httpStatus?: number; }
export function sendPrepared(request: InvoiceRequest, options?: TransportOptions): Promise<ValidationResponse>;
export function sendPrepared(request: ComplianceRequest, options?: TransportOptions): Promise<ComplianceCsid>;
export function sendPrepared(request: ProductionRequest, options?: TransportOptions): Promise<ProductionCsid>;
export class ZatcaClient {
  constructor(config: Config, options?: Pick<TransportOptions, 'fetch'>);
  requestComplianceCsid(csr: Csr, otp: string, options?: RequestOptions): Promise<ComplianceCsid>;
  requestProductionCsid(credentials: ComplianceCsid, options?: RequestOptions): Promise<ProductionCsid>;
  renewCsid(credentials: ProductionCsid, csr: Csr, otp: string, acceptLanguage?: string | null, options?: RequestOptions): Promise<ProductionCsid>;
  checkInvoiceCompliance(invoice: SignedInvoice, credentials: ComplianceCsid, options?: RequestOptions): Promise<ValidationResponse>;
  reportSimplifiedInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceStatus?: boolean | ClearanceMode, acceptLanguage?: string | null, options?: RequestOptions): Promise<ValidationResponse>;
  clearStandardInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceStatus?: boolean | ClearanceMode, acceptLanguage?: string | null, options?: RequestOptions): Promise<ValidationResponse>;
}
export class Xml extends NativeXml {
  static validateZatca(xml: string, optionsJson?: string | null): ZatcaValidationReport;
}
