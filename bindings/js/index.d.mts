export * from './generated/index.mjs';
export * from './errors.mjs';
import type {
  AdjustmentKind, ClearanceMode, ComplianceRequest, Csr, Environment,
  InvoiceRequest, InvoiceSubType, InvoiceTypeKind, ProductionRequest,
  SignedInvoice, Text, ValidationResponse, VatCategory,
} from './generated/index.mjs';
import {
  Address as NativeAddress, Adjustment as NativeAdjustment,
  AllowanceCharge as NativeAllowanceCharge, ComplianceCsid as NativeComplianceCsid,
  Config as NativeConfig, ProductionCsid as NativeProductionCsid,
  CsrProperties as NativeCsrProperties, InvoiceBuilder as NativeInvoiceBuilder,
  InvoiceLineItem as NativeInvoiceLineItem, LineItemBuilder as NativeLineItemBuilder,
  OriginalInvoiceRef as NativeOriginalInvoiceRef, PaymentMeans as NativePaymentMeans,
  TaxExemption as NativeTaxExemption, VatBreakdown as NativeVatBreakdown,
} from './generated/index.mjs';

// ---------------------------------------------------------------------------
// Enum value names. Each union lists the `.value` strings of the enum class of
// the same name; tests/runtime.test.mjs checks them against the runtime.

export type AdjustmentKindValue = 'Allowance' | 'Charge';
export type ClearanceModeValue = 'Disabled' | 'Enabled';
export type ClearanceStatusValue = 'Unknown' | 'Cleared' | 'NotCleared';
export type EnvironmentValue = 'NonProduction' | 'Simulation' | 'Production';
export type InvoiceFlagValue = 'ThirdParty' | 'Nominal' | 'Export' | 'Summary' | 'SelfBilled';
export type InvoiceOutcomeValue = 'Unknown' | 'Accepted' | 'Rejected';
export type InvoiceSubTypeValue = 'Standard' | 'Simplified';
export type InvoiceTypeKindValue = 'Tax' | 'Prepayment' | 'CreditNote' | 'DebitNote';
export type MessageTypeValue = 'Unknown' | 'Info' | 'Warning' | 'Error';
export type ReportingStatusValue = 'Unknown' | 'Reported' | 'NotReported';
export type SeverityValue = 'Warning' | 'Error';
export type ValidationLayerValue = 'Unknown' | 'FieldChecks' | 'Xsd' | 'BusinessRules' | 'Signature' | 'Qr' | 'PreviousInvoiceHash';
export type ValidationLocationKindValue = 'Unknown' | 'Field' | 'XPath' | 'Xml';
export type ValidationStatusValue = 'Unknown' | 'Pass' | 'Warning' | 'Error';
export type VatCategoryValue = 'Exempt' | 'Standard' | 'Zero' | 'OutOfScope';
export type ZatcaStageValue = 'Unknown' | 'Xsd' | 'Cen' | 'Ksa' | 'Signature' | 'Qr' | 'PreviousInvoiceHash';
export type ZatcaStageStatusValue = 'Unknown' | 'NotRun' | 'Completed' | 'NotApplicable' | 'ContextRequired' | 'EvaluationFailed';

// ---------------------------------------------------------------------------
// Constructors. The generated `new_` statics are exposed as `create`.

export declare class Address extends NativeAddress {
  private constructor();
  static create(countryCode: string, city: string, street: string, buildingNumber: string, postalCode: string, additionalStreet: string | null, additionalNumber: string | null, district: string | null): Address;
  /** @deprecated Use `Address.create`; `new_` will be removed from the package root. */
  static new_(countryCode: string, city: string, street: string, buildingNumber: string, postalCode: string, additionalStreet: string | null, additionalNumber: string | null, district: string | null): Address;
}
export declare class AllowanceCharge extends NativeAllowanceCharge {
  private constructor();
  static create(kind: AdjustmentKind, amount: string, reason: string | null, reasonCode: string | null, category: VatCategory, rate: string, exemption: TaxExemption): AllowanceCharge;
  /** @deprecated Use `AllowanceCharge.create`; `new_` will be removed from the package root. */
  static new_(kind: AdjustmentKind, amount: string, reason: string | null, reasonCode: string | null, category: VatCategory, rate: string, exemption: TaxExemption): AllowanceCharge;
}
export declare class ComplianceCsid extends NativeComplianceCsid {
  private constructor();
  static create(environment: Environment, requestId: string | null, token: string, secret: string): ComplianceCsid;
  /** @deprecated Use `ComplianceCsid.create`; `new_` will be removed from the package root. */
  static new_(environment: Environment, requestId: string | null, token: string, secret: string): ComplianceCsid;
}
export declare class Config extends NativeConfig {
  private constructor();
  static create(env: Environment): Config;
  /** @deprecated Use `Config.create`; `new_` will be removed from the package root. */
  static new_(env: Environment): Config;
}
export declare class CsrProperties extends NativeCsrProperties {
  private constructor();
  static create(commonName: string, serialNumber: string, organizationIdentifier: string, organizationUnitName: string, organizationName: string, countryName: string, invoiceType: string, locationAddress: string, industryBusinessCategory: string): CsrProperties;
  /** @deprecated Use `CsrProperties.create`; `new_` will be removed from the package root. */
  static new_(commonName: string, serialNumber: string, organizationIdentifier: string, organizationUnitName: string, organizationName: string, countryName: string, invoiceType: string, locationAddress: string, industryBusinessCategory: string): CsrProperties;
}
export declare class InvoiceBuilder extends NativeInvoiceBuilder {
  private constructor();
  static create(kind: InvoiceTypeKind, subtype: InvoiceSubType, originalId: string | null, originalUuid: string | null, originalDate: string | null, reason: string | null): InvoiceBuilder;
  /** @deprecated Use `InvoiceBuilder.create`; `new_` will be removed from the package root. */
  static new_(kind: InvoiceTypeKind, subtype: InvoiceSubType, originalId: string | null, originalUuid: string | null, originalDate: string | null, reason: string | null): InvoiceBuilder;
}
export declare class InvoiceLineItem extends NativeInvoiceLineItem {
  private constructor();
  static create(description: string, quantity: string, unitCode: string, unitPrice: string, rate: string, category: VatCategory): InvoiceLineItem;
  /** @deprecated Use `InvoiceLineItem.create`; `new_` will be removed from the package root. */
  static new_(description: string, quantity: string, unitCode: string, unitPrice: string, rate: string, category: VatCategory): InvoiceLineItem;
}
export declare class LineItemBuilder extends NativeLineItemBuilder {
  private constructor();
  static create(): LineItemBuilder;
  /** @deprecated Use `LineItemBuilder.create`; `new_` will be removed from the package root. */
  static new_(): LineItemBuilder;
}
export declare class OriginalInvoiceRef extends NativeOriginalInvoiceRef {
  private constructor();
  static create(id: string, uuid: string | null, date: string | null, time: string | null, typeCode: string | null): OriginalInvoiceRef;
  /** @deprecated Use `OriginalInvoiceRef.create`; `new_` will be removed from the package root. */
  static new_(id: string, uuid: string | null, date: string | null, time: string | null, typeCode: string | null): OriginalInvoiceRef;
}
export declare class ProductionCsid extends NativeProductionCsid {
  private constructor();
  static create(environment: Environment, requestId: string | null, token: string, secret: string): ProductionCsid;
  /** @deprecated Use `ProductionCsid.create`; `new_` will be removed from the package root. */
  static new_(environment: Environment, requestId: string | null, token: string, secret: string): ProductionCsid;
}
export declare class PaymentMeans extends NativePaymentMeans {
  private constructor();
  static create(code: string, instruction: string | null): PaymentMeans;
  /** @deprecated Use `PaymentMeans.create`; `new_` will be removed from the package root. */
  static new_(code: string, instruction: string | null): PaymentMeans;
}
export declare class TaxExemption extends NativeTaxExemption {
  private constructor();
  static create(code: string | null, reason: string | null): TaxExemption;
  /** @deprecated Use `TaxExemption.create`; `new_` will be removed from the package root. */
  static new_(code: string | null, reason: string | null): TaxExemption;
}
export declare class VatBreakdown extends NativeVatBreakdown {
  private constructor();
  static create(category: VatCategory, rate: string, taxableAmount: string, taxAmount: string, exemption: TaxExemption): VatBreakdown;
  /** @deprecated Use `VatBreakdown.create`; `new_` will be removed from the package root. */
  static new_(category: VatCategory, rate: string, taxableAmount: string, taxAmount: string, exemption: TaxExemption): VatBreakdown;
}

// ---------------------------------------------------------------------------
// 0.5 names of members renamed to match the Rust core; removed in 0.7.

export declare class Adjustment extends NativeAdjustment {
  private constructor();
  /** @deprecated Use `Adjustment.percentOf`; removed in 0.7. */
  static percentage(base: string, percent: string): NativeAdjustment;
}
// Instance aliases merge into the generated classes, so values returned by
// any method keep them.
declare module './generated/Adjustment.mjs' {
  interface Adjustment {
    /** @deprecated Use `suppliedAmount`; removed in 0.7. */
    withSuppliedAmount(value: string): Adjustment;
    /** @deprecated Use `percentage`; removed in 0.7. */
    percent(): Text | null;
  }
}
declare module './generated/AllowanceCharge.mjs' {
  interface AllowanceCharge {
    /** @deprecated Use `omitVatRate`; removed in 0.7. */
    withoutVatRate(): AllowanceCharge;
  }
}
declare module './generated/VatBreakdown.mjs' {
  interface VatBreakdown {
    /** @deprecated Use `omitVatRate`; removed in 0.7. */
    withoutVatRate(): VatBreakdown;
  }
}

// ---------------------------------------------------------------------------
// Local validation report: plain data copied out of WASM by `Xml.validateZatca`.

/** Where a finding points, discriminated by `kind`. */
export type ZatcaFindingLocation =
  | { readonly kind: 'Field' | 'XPath'; readonly path: string | null }
  | { readonly kind: 'Xml'; readonly line: number | null; readonly column: number | null }
  | { readonly kind: 'Unknown' };

/** One validation finding. */
export interface ZatcaFinding {
  /** Library-owned rule identifier, independent of message wording. */
  readonly code: string;
  readonly severity: SeverityValue;
  readonly layer: ValidationLayerValue;
  /** Human-readable text; the wording is not a stable API. */
  readonly message: string;
  /** Rule assertion that produced the finding, for business-rule findings. */
  readonly assertionSite: string | null;
  /** `null` when the source cannot locate the finding. */
  readonly location: ZatcaFindingLocation | null;
}

/** Pinned upstream rule source of a business-rule stage. */
export interface ZatcaRuleSource {
  /** File name of the upstream rule stylesheet. */
  readonly stylesheet: string;
  /** Hex SHA-256 digest of the pinned stylesheet. */
  readonly sha256: string;
}

/** Coverage and findings from one validation stage. */
export interface ZatcaStageReport {
  readonly stage: ZatcaStageValue;
  readonly status: ZatcaStageStatusValue;
  /** Pinned rule source, for the CEN and KSA business-rule stages. */
  readonly provenance: ZatcaRuleSource | null;
  /** Assertion sites whose selected contexts were all evaluated. */
  readonly evaluatedAssertions: readonly string[];
  readonly findings: readonly ZatcaFinding[];
}

/**
 * Results of local ZATCA validation with explicit stage coverage. Completion
 * and validity describe only the local profile; they do not establish
 * acceptance by ZATCA or certificate issuer trust.
 */
export interface ZatcaValidationReport {
  /** Version of the report's serialized format. */
  readonly schemaVersion: number;
  /** Identifier of the validation profile (the ZATCA SDK release it mirrors). */
  readonly profile: string;
  /** RFC 3339 instant used for time-dependent rules. */
  readonly evaluatedAt: string;
  /** Whether every required stage finished exactly once. */
  readonly isComplete: boolean;
  /** Whether any stage, including an incomplete stage, reported an error. */
  readonly hasErrors: boolean;
  /** Whether validation is complete and contains no error findings. */
  readonly isValid: boolean;
  /** Stage reports, in stage order. */
  readonly stages: readonly ZatcaStageReport[];
}

export interface ZatcaValidationOptions {
  /** Hash of the preceding invoice; without it the chain stage is incomplete. */
  previousInvoiceHash?: string | null;
  /** Instant for time-dependent rules; defaults to the host clock. */
  evaluatedAt?: string | Date | null;
}

export declare class Xml {
  private constructor();
  /** Check the XML against the UBL schema and business rules; throws on failure. */
  static validate(xml: string): boolean;
  /** Base64 invoice hash. */
  static hash(xml: string): string;
  /**
   * Run the local ZATCA validation profile. Inspect `isValid` before acceptance.
   * `options` may also be the raw options JSON string (`previous_invoice_hash`,
   * `evaluated_at`). The host clock is used when no instant is given.
   */
  static validateZatca(xml: string, options?: ZatcaValidationOptions | string | null): ZatcaValidationReport;
}

// ---------------------------------------------------------------------------
// Async API client.

export interface RequestOptions { signal?: AbortSignal; }
export interface TransportOptions extends RequestOptions { fetch?: typeof fetch; }
/** The Clearance-Status request header: a boolean, a `ClearanceMode` or its value name. */
export type ClearanceModeInput = boolean | ClearanceMode | ClearanceModeValue;
export function sendPrepared(request: InvoiceRequest, options?: TransportOptions): Promise<ValidationResponse>;
export function sendPrepared(request: ComplianceRequest, options?: TransportOptions): Promise<ComplianceCsid>;
export function sendPrepared(request: ProductionRequest, options?: TransportOptions): Promise<ProductionCsid>;
export class ZatcaClient {
  constructor(config: Config, options?: Pick<TransportOptions, 'fetch'>);
  requestComplianceCsid(csr: Csr, otp: string, options?: RequestOptions): Promise<ComplianceCsid>;
  requestProductionCsid(credentials: ComplianceCsid, options?: RequestOptions): Promise<ProductionCsid>;
  renewCsid(credentials: ProductionCsid, csr: Csr, otp: string, acceptLanguage?: string | null, options?: RequestOptions): Promise<ProductionCsid>;
  checkInvoiceCompliance(invoice: SignedInvoice, credentials: ComplianceCsid, options?: RequestOptions): Promise<ValidationResponse>;
  reportSimplifiedInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceStatus?: ClearanceModeInput, acceptLanguage?: string | null, options?: RequestOptions): Promise<ValidationResponse>;
  clearStandardInvoice(invoice: SignedInvoice, credentials: ProductionCsid, clearanceStatus?: ClearanceModeInput, acceptLanguage?: string | null, options?: RequestOptions): Promise<ValidationResponse>;
}
