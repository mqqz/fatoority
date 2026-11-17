export * from './generated/index.mjs';
import type { ClearanceMode, Config, Csr, CsidCompliance, CsidProduction, SignedInvoice, ValidationResponse, InvoiceRequest, ComplianceRequest, ProductionRequest } from './generated/index.mjs';
import { Xml as NativeXml } from './generated/index.mjs';
export interface RequestOptions { signal?: AbortSignal; }
export interface TransportOptions extends RequestOptions { fetch?: typeof fetch; }
export class TransportError extends Error { readonly httpStatus?: number; }
export function sendPrepared(request: InvoiceRequest, options?: TransportOptions): Promise<ValidationResponse>;
export function sendPrepared(request: ComplianceRequest, options?: TransportOptions): Promise<CsidCompliance>;
export function sendPrepared(request: ProductionRequest, options?: TransportOptions): Promise<CsidProduction>;
export class ZatcaClient {
  constructor(config: Config, options?: Pick<TransportOptions, 'fetch'>);
  postCsrForCcsid(csr: Csr, otp: string, options?: RequestOptions): Promise<CsidCompliance>;
  postCcsidForPcsid(credentials: CsidCompliance, options?: RequestOptions): Promise<CsidProduction>;
  renewCsid(credentials: CsidProduction, csr: Csr, otp: string, acceptLanguage?: string | null, options?: RequestOptions): Promise<CsidProduction>;
  checkInvoiceCompliance(invoice: SignedInvoice, credentials: CsidCompliance, options?: RequestOptions): Promise<ValidationResponse>;
  reportSimplifiedInvoice(invoice: SignedInvoice, credentials: CsidProduction, clearanceStatus?: boolean | ClearanceMode, acceptLanguage?: string | null, options?: RequestOptions): Promise<ValidationResponse>;
  clearStandardInvoice(invoice: SignedInvoice, credentials: CsidProduction, clearanceStatus?: boolean | ClearanceMode, acceptLanguage?: string | null, options?: RequestOptions): Promise<ValidationResponse>;
}
export class Xml extends NativeXml {
  static validateZatca(config: Config, xml: string, optionsJson?: string | null): string;
}
