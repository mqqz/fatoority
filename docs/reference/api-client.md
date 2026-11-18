---
title: ZATCA API Client and Response Reference
description: Reference for ZATCA credentials, invoice compliance, reporting, and clearance. Find method inputs, return values, response fields, acceptance checks, and custom transport options.
---

# ZATCA API client

The API client requests credentials and submits signed invoices to the selected
[environment](configuration.md). For a runnable integration, start with the
[API guide](../guides/api.md). This reference describes inputs, returned data, and
how to distinguish a completed request from an accepted invoice.

## Choose an operation

| Operation | Required input | Returns | Behavior |
| --- | --- | --- | --- |
| Issue compliance credentials | CSR and OTP | Compliance credentials (CCSID) | Sends the CSR as Base64-encoded PEM. |
| Issue production credentials | Compliance credentials with a request ID | Production credentials (PCSID) | Uses the compliance request ID to request production credentials. |
| Renew production credentials | Existing production credentials, new CSR, OTP, response language | Production credentials | Submits a renewal request. |
| Check invoice compliance | Signed invoice and compliance credentials | Validation response | Checks the invoice; a passing check does not establish reporting or clearance. |
| Report a simplified invoice | Signed simplified invoice, production credentials, clearance flag, response language | Validation response | Rejects standard invoices before sending. |
| Clear a standard invoice | Signed standard invoice, production credentials, clearance flag, response language | Validation response | Rejects simplified invoices before sending. |

Credentials must match the client's environment. Production credentials describe
the credential scope; they can belong to a test environment. The credential
constructor stores supplied values; it does not authenticate them or obtain a
certificate. Preserve the request ID returned during compliance onboarding.

## Input values

| Input | Expected value | Notes |
| --- | --- | --- |
| Configuration | Selected environment | Used for endpoints and credential environment checks. |
| CSR | Parsed or generated certificate signing request | See [CSR reference](csr.md). Supply the matching private key separately when signing invoices. |
| OTP | Text supplied for the onboarding or renewal operation | Sent in the OTP header. Invalid HTTP-header characters fail before transport. |
| Token | Binary security token returned by ZATCA | Used with the secret for HTTP Basic authentication. |
| Secret | Credential secret returned by ZATCA | Keep credentials out of logs. |
| Request ID | Optional text from credential issuance | Required to request production credentials from compliance credentials. |
| Invoice | Signed invoice object | The client sends its hash, UUID, and Base64 XML. |
| Clearance flag | `ClearanceMode` (`Enabled`/`Disabled`); a Boolean in Python, the JavaScript async client, and the Kotlin `*Async` helpers | Sends `Clearance-Status: 1` when enabled, `0` when disabled. It does not change the selected operation. |
| Response language | `ar`, `en`, or omitted | Only the exact value `ar` selects Arabic; every other value sends English. |

## Interpret an invoice response

A successful HTTP status means that a response body was decoded. **Check acceptance
before treating an invoice as reported or cleared.** The acceptance helper fails
for both rejected and unknown outcomes.

| Invoked operation | Accepted status | Rejected status |
| --- | --- | --- |
| Reporting | `REPORTED` | `NOT_REPORTED` |
| Clearance | `CLEARED` | `NOT_CLEARED` |
| Compliance | Validation status `PASS` or `WARNING` | Validation status `ERROR` |

Missing or unrecognized endpoint status produces an unknown outcome. A positive
status accompanied by validation errors also produces an unknown outcome.
Warnings alone do not prevent acceptance. Only the status for the operation
actually invoked determines its outcome.

| Returned field | Value | Meaning |
| --- | --- | --- |
| HTTP status | Optional integer | Actual response status, when attached by the client. |
| Outcome | Accepted, rejected, or unknown | Classification for the invoked operation. |
| Validation results | Result object | Contains message collections and optional overall status. |
| Reporting / clearance status | Optional status enum (Python: optional text) | Gateway status for each operation; unrecognized values map to `Other`/`Unknown`, and the bindings' `*_raw` getters return the exact text. |
| Seller / buyer QR status | Optional text | Raw QR status returned by the gateway. |
| Cleared invoice Base64 | Optional text | Exact encoded invoice returned by clearance. |
| Cleared invoice XML | Optional decoded text | Preserves returned text; does not parse XML or verify its signature. |

Missing cleared XML is independent of acceptance. A present empty Base64 value,
invalid Base64, NUL bytes, or invalid UTF-8 fails decoding.

Validation results contain information, warning, and error message collections,
which can be empty, plus an optional status. Each message has optional type, code,
category, message text, and status fields. Handle missing fields instead of
assuming that every response contains a complete diagnostic.

## HTTP failures and custom transport

Invoice operations treat every non-2xx response as an error, including 400 and
409. A duplicate-reporting 409 does not establish that an earlier submission was
rejected. A 208 clearance response follows the successful-response path and still
requires an acceptance check. Redirects are not followed; a 303 does not trigger
automatic reporting. A malformed successful body is a decoding error, and a failed
body read is a transport error.

No client-side request timeout applies by default. Construct the client with a
timeout to bound each request, from connecting until the response body is read.
A request that exceeds it fails as a network (transport) error; zero is rejected.
JavaScript has no timeout option: its client sends through the `fetch` you supply,
so bound requests there with an `AbortSignal`.

Credential issuance requires a successful response. Renewal additionally supports
the gateway's 428 renewal response when it contains a decodable credential body.

Where exposed, prepared requests let you use your own HTTP transport. Preparing
performs input checks and builds an owned request without sending it. Send its
method, URL, headers, and body, then pass the actual HTTP status and response bytes
to its decoder. Headers contain credentials. The decoder preserves the operation
context needed for acceptance checks. Keep redirects disabled in custom transports.

<span id="zatcaclient"></span>
<span id="new"></span>
<span id="post_csr_for_ccsid"></span>
<span id="post_ccsid_for_pcsid"></span>
<span id="request_compliance_csid"></span>
<span id="request_production_csid"></span>
<span id="renew_csid"></span>
<span id="check_invoice_compliance"></span>
<span id="report_simplified_invoice"></span>
<span id="clear_standard_invoice"></span>
<span id="csidcredentials-csidcompliance-csidproduction"></span>
<span id="env"></span>
<span id="request_id"></span>
<span id="binary_security_token"></span>
<span id="secret"></span>
<span id="invoice-response-contract"></span>
<span id="duplicate-submissions-and-redirects"></span>
<span id="response-types"></span>
<span id="errors"></span>

<span id="new_1"></span>

## Language reference

=== "Rust"
    Import client, credential, and response types from `fatoora_core::api`.
    `ZatcaClient::new(config: Config)` returns `Result<ZatcaClient, ZatcaError>`.
    `ZatcaClient::with_timeout(config: Config, timeout: Duration)` does the same with
    a per-request timeout; a timeout surfaces as `ZatcaError::Http` (or
    `ZatcaError::ResponseRead` mid-body), both `ErrorKind::Network`.
    HTTP methods are async; call them with `.await`. Inputs below are borrowed.

    **HTTP operations**

    | Method on `client` | Returns after `.await` |
    | --- | --- |
    | `request_compliance_csid(csr: &Csr, otp: &str)` | `Result<ComplianceCsid, ZatcaError>` |
    | `request_production_csid(ccsid: &ComplianceCsid)` | `Result<ProductionCsid, ZatcaError>` |
    | `renew_csid(pcsid: &CsidCredentials<Production>, csr: &Csr, otp: &str, accept_language: Option<&str>)` | `Result<CsidCredentials<Production>, ZatcaError>` |
    | `check_invoice_compliance(invoice: &SignedInvoice, credentials: &CsidCredentials<Compliance>)` | `Result<ValidationResponse, ZatcaError>` |
    | `report_simplified_invoice(invoice: &SignedInvoice, credentials: &CsidCredentials<Production>, clearance_mode: ClearanceMode, accept_language: Option<&str>)` | `Result<ValidationResponse, ZatcaError>` |
    | `clear_standard_invoice(invoice: &SignedInvoice, credentials: &CsidCredentials<Production>, clearance_mode: ClearanceMode, accept_language: Option<&str>)` | `Result<ValidationResponse, ZatcaError>` |

    `ClearanceMode::Enabled` / `ClearanceMode::Disabled` send the `Clearance-Status`
    header as `1` / `0`; `ClearanceMode::from(flag)` converts a `bool`. The
    `prepare_report_simplified_invoice` / `prepare_clear_standard_invoice` variants
    take the same arguments.

    **Credentials**

    `CsidCredentials::<Compliance>::new(env, request_id, token, secret)` creates compliance
    credentials; use `Production` for production scope. `ComplianceCsid` and `ProductionCsid`
    are aliases for `CsidCredentials<Compliance>` and `CsidCredentials<Production>`.
    Arguments are `Environment`,
    `Option<String>`, and two `impl Into<String>` values. Construction returns the
    credential object directly.

    | Accessor | Returns |
    | --- | --- |
    | `credentials.env()` | `Environment` |
    | `credentials.request_id()` | `Option<&str>` |
    | `credentials.binary_security_token()` / `credentials.secret()` | `&str` |

    **Responses**

    | Method | Returns |
    | --- | --- |
    | `response.http_status()` | `Option<u16>` |
    | `response.outcome()` | `InvoiceOutcome::{Accepted, Rejected, Unknown}` |
    | `response.ensure_accepted()` | `Result<(), ZatcaError>` |
    | `response.validation_results()` | `&ValidationResults` |
    | `response.reporting_status()` | `Option<&ReportingStatus>`: `Reported`, `NotReported`, `Other(String)` |
    | `response.clearance_status()` | `Option<&ClearanceStatus>`: `Cleared`, `NotCleared`, `Other(String)` |
    | `response.qr_seller_status()` / `qr_buyer_status()` | `Option<&str>` |
    | `response.cleared_invoice_base64()` | `Option<&str>` |
    | `response.cleared_invoice_xml()` | `Result<Option<String>, ZatcaError>` |
    | `results.status()` | `Option<&ValidationStatus>`: `Pass`, `Warning`, `Error`, `Other(String)` |
    | `results.info_messages()` | `&MessageList`: `One`, `Many`, or `Empty` |
    | `results.warning_messages()` / `results.error_messages()` | `&[ValidationMessage]` |
    | `message.message_type()` | `Option<&MessageType>`: `Info`, `Warning`, `Error`, `Other(String)` |
    | `message.status()` | `Option<&ValidationStatus>` |
    | `message.code()` / `category()` / `message()` | `Option<&str>` |

    The status enums are non-exhaustive. A value they do not recognize is kept
    verbatim in `Other`; `as_str()` and `Display` return the exact wire text.
    These getters borrow the response. Serializing and deserializing the response body
    does not preserve HTTP status or operation context; the deserialized outcome is
    `Unknown`. `ZatcaError::NotAccepted` retains the response. For HTTP failures,
    `ZatcaError::http_status()` provides the status when available;
    `HttpResponseError::body()` and `validation_response()` expose the retained body
    and optional parsed response. See [error reference](errors.md).

    **Prepared requests**

    Prefix each HTTP method above with `prepare_` and use the same arguments. These
    synchronous methods return `Result<ComplianceRequest, ZatcaError>` for CSR issuance,
    `Result<ProductionRequest, ZatcaError>` for production issuance/renewal, and
    `Result<InvoiceRequest, ZatcaError>` for invoice operations.

    `prepared.request()` borrows an `HttpRequest`. Its `method()` and `url()` return
    `&str`, `headers()` returns `&[(String, String)]`, and `body()` returns `&[u8]`.
    `prepared.decode(status: u16, body: &[u8])` returns the corresponding response or
    credentials in `Result<_, ZatcaError>`. Preparing and decoding do not consume inputs.
    The request owns its data and can outlive the client, invoice, and credentials.

=== "Python"
    Import `ZatcaClient`, `ComplianceCsid`, `ProductionCsid`, `ClearanceMode`, and
    `InvoiceOutcome` from `fatoora`. Construct `ZatcaClient(config: Config, *, timeout=None)`;
    `timeout` is seconds (`int`/`float`) or a `datetime.timedelta`, and a request
    that exceeds it raises `NetworkError`. HTTP calls block until they
    finish and raise classified exceptions on failure; see [errors](errors.md).

    **HTTP operations**

    | Method on `client` | Returns |
    | --- | --- |
    | `request_compliance_csid(csr: Csr, otp: str)` | `ComplianceCsid` |
    | `request_production_csid(ccsid: ComplianceCsid)` | `ProductionCsid` |
    | `renew_csid(pcsid: ProductionCsid, csr: Csr, otp: str, accept_language: Optional[str] = None)` | `ProductionCsid` |
    | `check_invoice_compliance(invoice: SignedInvoice, ccsid: ComplianceCsid)` | `ValidationResponse` |
    | `report_simplified_invoice(invoice: SignedInvoice, pcsid: ProductionCsid, *, clearance: ClearanceMode, accept_language: Optional[str] = None)` | `ValidationResponse` |
    | `clear_standard_invoice(invoice: SignedInvoice, pcsid: ProductionCsid, *, clearance: ClearanceMode, accept_language: Optional[str] = None)` | `ValidationResponse` |

    `clearance` (required) and `accept_language` are keyword-only.
    `ClearanceMode.DISABLED` sends `Clearance-Status: 0` and `ClearanceMode.ENABLED`
    sends `1`.

    **Credentials**

    `ComplianceCsid(env: Environment, *, token: str, secret: str, request_id: Optional[str] = None)`
    builds compliance credentials from stored values; `ProductionCsid(...)` takes the
    same arguments. The `env` property is an `Environment`; `binary_security_token` and
    `secret` are `str`; `request_id` is `Optional[str]`, `None` when absent.

    **Responses**

    | Property or method | Value |
    | --- | --- |
    | `response.http_status` | `Optional[int]` |
    | `response.outcome` | `InvoiceOutcome.ACCEPTED`, `.REJECTED`, or `.UNKNOWN` |
    | `response.ensure_accepted()` | `None`; raises `ApiError` if not accepted. |
    | `response.validation_results` | `ValidationResults` |
    | `response.reporting_status` | `Optional[ReportingStatus]`: `REPORTED`, `NOT_REPORTED` |
    | `response.clearance_status` | `Optional[ClearanceStatus]`: `CLEARED`, `NOT_CLEARED` |
    | `response.qr_seller_status` / `qr_buyer_status` | `Optional[str]` |
    | `response.cleared_invoice_base64` | `Optional[str]` |
    | `response.cleared_invoice_xml` | `Optional[str]`; raises `ParseError` for invalid content. |
    | `results.status` | `Optional[ValidationStatus]`: `PASS`, `WARNING`, `ERROR` |
    | `results.info_messages` / `warning_messages` / `error_messages` | `list[ValidationMessage]` |
    | `message.message_type` | `Optional[MessageType]`: `INFO`, `WARNING`, `ERROR` |
    | `message.status` | `Optional[ValidationStatus]` |
    | `message.code` / `category` / `message` | `Optional[str]` |

    The status enums are `str` subclasses whose values are the wire text, so
    `response.reporting_status == "REPORTED"` holds. A value this version does not
    recognize is kept verbatim, like the core's `Other(String)`: it is an instance of
    the enum whose `name` is `"OTHER"` and whose `value` is the text ZATCA sent.

    Client, credential, response, result, and message objects support `with` and
    `close()`. Child results are independently owned. Calls do not consume their
    inputs. The public Python facade provides the blocking HTTP methods above;
    prepared-request methods are not exposed by this facade.

=== "JavaScript"
    Import `ZatcaClient`, `ComplianceCsid`, `ProductionCsid`, and `InvoiceOutcome`
    from `fatoority`. Create `new ZatcaClient(config, { fetch })`; the second argument
    is optional and defaults to `globalThis.fetch`. Calls return promises; use `await`.

    **HTTP operations**

    | Method on `client` | Promise resolves to |
    | --- | --- |
    | `requestComplianceCsid(csr, otp, options = {})` | `ComplianceCsid` |
    | `requestProductionCsid(credentials, options = {})` | `ProductionCsid` |
    | `renewCsid(credentials, csr, otp, acceptLanguage = null, options = {})` | `ProductionCsid` |
    | `checkInvoiceCompliance(invoice, credentials, options = {})` | `ValidationResponse` |
    | `reportSimplifiedInvoice(invoice, credentials, clearanceStatus = false, acceptLanguage = null, options = {})` | `ValidationResponse` |
    | `clearStandardInvoice(invoice, credentials, clearanceStatus = true, acceptLanguage = null, options = {})` | `ValidationResponse` |

    Pass `Csr` for `csr`, `SignedInvoice` for `invoice`, strings for OTP/language, and
    a boolean, a `ClearanceMode` (`ClearanceMode.Disabled`/`.Enabled`) or its name (`'Disabled'`/`'Enabled'`) for `clearanceStatus`. Compliance issuance uses no credential; compliance checks
    and production issuance take `ComplianceCsid`; reporting, clearance, and renewal
    take `ProductionCsid`. Each `options` accepts `{ signal: AbortSignal }` for cancellation.
    Transport failures reject with `TransportError` (a `NetworkError`); response/acceptance failures reject with the
    `FatooraError` subclass for their code, such as `ApiError` or `UnauthorizedError`. See [JavaScript HTTP setup](bindings/javascript.md#api-requests)
    for browser CORS and custom fetch behavior.

    **Credentials**

    `ComplianceCsid.create(environment, requestId, token, secret)` returns compliance
    credentials; `ProductionCsid.create` returns production credentials. Pass an `Environment`
    (`Environment.NonProduction`, `.Simulation`, or `.Production`), a string or `null`
    request ID, and string token/secret. The `env` property is the `Environment`;
    `binarySecurityToken` and `secret` are strings and `requestId` is a string or `null`.

    **Responses**

    Value getters are read-only properties; members with parentheses are methods.

    | Member | Value |
    | --- | --- |
    | `response.httpStatus` | Number or `null` |
    | `response.outcome` | `InvoiceOutcome.Accepted`, `.Rejected`, or `.Unknown` |
    | `response.ensureAccepted()` | `undefined`; throws unless accepted. |
    | `response.validationResults()` | `ValidationResults` |
    | `response.reportingStatus` | `ReportingStatus` (`Reported`, `NotReported`, `Unknown`) or `null` |
    | `response.clearanceStatus` | `ClearanceStatus` (`Cleared`, `NotCleared`, `Unknown`) or `null` |
    | `response.reportingStatusRaw` / `clearanceStatusRaw` / `qrSellerStatus` / `qrBuyerStatus` | String or `null` |
    | `response.clearedInvoiceBase64` / `clearedInvoiceXml` | String or `null`; reading the XML can throw. |
    | `results.status` | `ValidationStatus` (`Pass`, `Warning`, `Error`, `Unknown`) or `null` |
    | `results.statusRaw` | String or `null` |
    | `results.infoLen` / `warningLen` / `errorLen` | Number of messages |
    | `results.infoMessage(index)` / `warningMessage(index)` / `errorMessage(index)` | `ValidationMessage` |
    | `message.messageType` | `MessageType` (`Info`, `Warning`, `Error`, `Unknown`) or `null` |
    | `message.status` | `ValidationStatus` or `null` |
    | `message.messageTypeRaw` / `statusRaw` / `code` / `category` / `message` | String or `null` |

    Status enums are `null` when the field is absent and `Unknown` when it is
    present but not recognized by this binding version; the matching `*Raw`
    property holds the exact wire string. Compare with `=== ReportingStatus.Reported`, or read
    the variant name from `.value`.

    Message indexes are zero-based and must be less than the corresponding length.
    Child handles are independently owned. Calls do not consume inputs. See
    [JavaScript resource handling](bindings/javascript.md) for binary data lifetimes.

    **Prepared requests**

    For custom transport, import `ZatcaClient as RequestClient` from `fatoority/raw` and
    create it with `RequestClient.create(config)`. Its synchronous methods are
    `prepareRequestComplianceCsid`, `prepareRequestProductionCsid`, `prepareRenewCsid`,
    `prepareCheckInvoiceCompliance`, `prepareReportSimplifiedInvoice`, and
    `prepareClearStandardInvoice`. They take the same operation arguments as above,
    without `options`; supply a `ClearanceMode` (not a boolean) and nullable language
    explicitly.
    They return `ComplianceRequest`, `ProductionRequest`, or `InvoiceRequest` according
    to the operation.

    `request.method`, `url`, and `headersJson` are strings; `body()` returns
    `Bytes`. `request.decode(status, body)` takes an HTTP status number and an array of
    byte numbers and returns the corresponding credentials or validation response.
    Alternatively, `await sendPrepared(request, { fetch, signal })` from `fatoority`
    sends and decodes the request. Preparation owns its data; it does not send HTTP.

=== "C#"
    Use the `Fatoora` namespace. `ZatcaClient.Create(config: Config)` returns a client;
    `ZatcaClient.WithTimeout(config, timeout: TimeSpan)` adds a per-request timeout,
    rounded up to whole milliseconds, that raises `NetworkException` when exceeded;
    a zero, negative, or over-`uint.MaxValue`-millisecond timeout throws
    `ArgumentOutOfRangeException`.
    HTTP methods block; errors throw `FfiException` subclasses such as
    `NetworkException` or `UnauthorizedException`. Use `using` for owned objects,
    including child responses. Scalar getters are read-only properties.

    **HTTP operations**

    | Method on `client` | Inputs in order | Returns |
    | --- | --- | --- |
    | `RequestComplianceCsid` | `Csr csr`, `string otp` | `ComplianceCsid` |
    | `RequestProductionCsid` | `ComplianceCsid credentials` | `ProductionCsid` |
    | `RenewCsid` | `ProductionCsid credentials`, `Csr csr`, `string otp`, `string? acceptLanguage` | `ProductionCsid` |
    | `CheckInvoiceCompliance` | `SignedInvoice invoice`, `ComplianceCsid credentials` | `ValidationResponse` |
    | `ReportSimplifiedInvoice` | `SignedInvoice invoice`, `ProductionCsid credentials`, `ClearanceMode clearanceMode`, `string? acceptLanguage` | `ValidationResponse` |
    | `ClearStandardInvoice` | `SignedInvoice invoice`, `ProductionCsid credentials`, `ClearanceMode clearanceMode`, `string? acceptLanguage` | `ValidationResponse` |

    Pass `null` explicitly when omitting response language.

    **Credentials**

    `ComplianceCsid.Create(environment, requestId, token, secret)` returns
    compliance credentials; `ProductionCsid.Create` returns production credentials.
    Pass a `ZatcaEnvironment` (`NonProduction`, `Simulation`, or `Production`), a
    nullable `string` request ID, and `string` token/secret. The `Env` property
    returns `ZatcaEnvironment`; `BinarySecurityToken` and `Secret` return `string`;
    `RequestId` returns `string?`.

    **Responses**

    | Member | Returns |
    | --- | --- |
    | `response.HttpStatus` | `ushort?` |
    | `response.Outcome` | `InvoiceOutcome` (`Accepted`, `Rejected`, `Unknown`) |
    | `response.EnsureAccepted()` | No value; throws unless accepted. |
    | `response.ValidationResults()` | `ValidationResults` |
    | `response.ReportingStatus` | `ReportingStatus?` (`Reported`, `NotReported`, `Unknown`) |
    | `response.ClearanceStatus` | `ClearanceStatus?` (`Cleared`, `NotCleared`, `Unknown`) |
    | `response.ReportingStatusRaw` / `ClearanceStatusRaw` / `QrSellerStatus` / `QrBuyerStatus` | `string?` |
    | `response.ClearedInvoiceBase64` / `ClearedInvoiceXml` | `string?`; XML decoding can throw. |
    | `results.Status` | `ValidationStatus?` (`Pass`, `Warning`, `Error`, `Unknown`) |
    | `results.StatusRaw` | `string?` |
    | `results.InfoLen` / `WarningLen` / `ErrorLen` | `nuint` message count |
    | `results.InfoMessage(index)` / `WarningMessage(index)` / `ErrorMessage(index)` | `ValidationMessage` |
    | `message.MessageType` | `MessageType?` (`Info`, `Warning`, `Error`, `Unknown`) |
    | `message.Status` | `ValidationStatus?` |
    | `message.MessageTypeRaw` / `StatusRaw` / `Code` / `Category` / `Message` | `string?` |

    Status enums are `null` when the field is absent and `Unknown` when it is present
    but not recognized by this binding version; the matching `*Raw` property returns
    the exact wire string.

    Use zero-based `nuint` indexes below the corresponding message count. Child
    objects are independently owned. Methods do not consume their inputs.

    **Prepared requests**

    Use the six methods `PrepareRequestComplianceCsid`,
    `PrepareRequestProductionCsid`, `PrepareRenewCsid`,
    `PrepareCheckInvoiceCompliance`, `PrepareReportSimplifiedInvoice`,
    and `PrepareClearStandardInvoice` with the same inputs as their HTTP
    counterparts. They return `ComplianceRequest` for compliance credentials,
    `ProductionRequest` for production credentials/renewal, and `InvoiceRequest` for
    invoice operations, without sending HTTP.

    The `request.Method`, `Url`, and `HeadersJson` properties
    return `string`; `Body()` returns owned `Bytes`.
    `request.Decode(status, body)` accepts `ushort` HTTP status and
    `byte[]` response body, and returns the matching credential or response type.
    Prepared requests own their input data and may outlive the original client and inputs.

=== "Kotlin"
    Import types from `com.fatoora`. `ZatcaClient.create(config)` takes `Config` and
    returns a client; `ZatcaClient.withTimeout(config, timeout)` takes a
    `kotlin.time.Duration` (such as `30.seconds`) or `java.time.Duration` and adds a
    per-request timeout, rounded up to whole milliseconds, that raises
    `NetworkException` when exceeded; a zero, negative, or too-long timeout throws
    `IllegalArgumentException`.
    HTTP methods block and errors raise exceptions. Use `.use { ... }` or `close()` for owned objects,
    including child responses.

    **HTTP operations**

    | Method on `client` | Inputs in order | Returns |
    | --- | --- | --- |
    | `requestComplianceCsid` | `Csr csr`, `String otp` | `ComplianceCsid` |
    | `requestProductionCsid` | `ComplianceCsid credentials` | `ProductionCsid` |
    | `renewCsid` | `ProductionCsid credentials`, `Csr csr`, `String otp`, `String? acceptLanguage` | `ProductionCsid` |
    | `checkInvoiceCompliance` | `SignedInvoice invoice`, `ComplianceCsid credentials` | `ValidationResponse` |
    | `reportSimplifiedInvoice` | `SignedInvoice invoice`, `ProductionCsid credentials`, `ClearanceMode clearanceMode`, `String? acceptLanguage` | `ValidationResponse` |
    | `clearStandardInvoice` | `SignedInvoice invoice`, `ProductionCsid credentials`, `ClearanceMode clearanceMode`, `String? acceptLanguage` | `ValidationResponse` |

    Response language defaults to `null`; an overload allows it to be omitted.

    **Credentials**

    `ComplianceCsid.create(environment, requestId, token, secret)` returns
    compliance credentials; `ProductionCsid.create` returns production credentials.
    Pass an `Environment` (`NonProduction`, `Simulation`, or `Production`), a nullable
    `String` request ID, and `String` token/secret. The `env` property is an
    `Environment`; `binarySecurityToken`, `secret` and the nullable `requestId` are
    `String` properties.

    **Responses**

    | Member | Returns |
    | --- | --- |
    | `response.httpStatus` | `Int?` |
    | `response.outcome` | `InvoiceOutcome` (`Accepted`, `Rejected`, `Unknown`) |
    | `response.ensureAccepted()` | No value; throws unless accepted. |
    | `response.validationResults()` | `ValidationResults` |
    | `response.reportingStatus` | Nullable `ReportingStatus` (`Reported`, `NotReported`, `Unknown`) |
    | `response.clearanceStatus` | Nullable `ClearanceStatus` (`Cleared`, `NotCleared`, `Unknown`) |
    | `response.reportingStatusRaw` / `clearanceStatusRaw` / `qrSellerStatus` / `qrBuyerStatus` | `String?` |
    | `response.clearedInvoiceBase64` / `clearedInvoiceXml` | `String?`; XML decoding can throw. |
    | `results.status` | Nullable `ValidationStatus` (`Pass`, `Warning`, `Error`, `Unknown`) |
    | `results.statusRaw` | `String?` |
    | `results.infoLen` / `warningLen` / `errorLen` | `java.math.BigInteger` message count |
    | `results.infoMessage(index)` / `warningMessage(index)` / `errorMessage(index)` | `ValidationMessage` |
    | `message.messageType` | Nullable `MessageType` (`Info`, `Warning`, `Error`, `Unknown`) |
    | `message.status` | Nullable `ValidationStatus` |
    | `message.messageTypeRaw` / `statusRaw` / `code` / `category` / `message` | `String?` |

    Status enums are `null` when the field is absent and `Unknown` when it is present
    but not recognized by this binding version; the matching `*Raw` property holds
    the exact wire string.

    Use zero-based `java.math.BigInteger` indexes below the corresponding message count. Child
    objects are independently owned. Methods do not consume their inputs.

    **Prepared requests**

    Use the six methods `prepareRequestComplianceCsid`,
    `prepareRequestProductionCsid`, `prepareRenewCsid`,
    `prepareCheckInvoiceCompliance`, `prepareReportSimplifiedInvoice`,
    and `prepareClearStandardInvoice` with the same inputs as their HTTP
    counterparts. They return `ComplianceRequest` for compliance credentials,
    `ProductionRequest` for production credentials/renewal, and `InvoiceRequest` for
    invoice operations, without sending HTTP.

    The `request.method`, `url`, and `headersJson` properties
    are `String`; `body()` returns owned `Bytes`.
    `request.decode(status, body)` accepts `Int` HTTP status and
    `ByteArray` response body, and returns the matching credential or response type.
    Prepared requests own their input data and may outlive the original client and inputs.

=== "Java"
    Import types from `com.fatoora`. `ZatcaClient.create(config)` takes `Config` and
    returns a client; `ZatcaClient.withTimeout(config, Duration.ofSeconds(30))` takes a
    `java.time.Duration` and adds a per-request timeout, rounded up to whole
    milliseconds, that raises `NetworkException` when exceeded; a zero, negative, or
    too-long timeout throws `IllegalArgumentException`.
    HTTP methods block and errors raise exceptions. Use try-with-resources or `close()` for owned objects,
    including child responses.

    **HTTP operations**

    | Method on `client` | Inputs in order | Returns |
    | --- | --- | --- |
    | `requestComplianceCsid` | `Csr csr`, `String otp` | `ComplianceCsid` |
    | `requestProductionCsid` | `ComplianceCsid credentials` | `ProductionCsid` |
    | `renewCsid` | `ProductionCsid credentials`, `Csr csr`, `String otp`, `String (nullable) acceptLanguage` | `ProductionCsid` |
    | `checkInvoiceCompliance` | `SignedInvoice invoice`, `ComplianceCsid credentials` | `ValidationResponse` |
    | `reportSimplifiedInvoice` | `SignedInvoice invoice`, `ProductionCsid credentials`, `ClearanceMode clearanceMode`, `String (nullable) acceptLanguage` | `ValidationResponse` |
    | `clearStandardInvoice` | `SignedInvoice invoice`, `ProductionCsid credentials`, `ClearanceMode clearanceMode`, `String (nullable) acceptLanguage` | `ValidationResponse` |

    Response language defaults to `null`; an overload allows it to be omitted.

    **Credentials**

    `ComplianceCsid.create(environment, requestId, token, secret)` returns
    compliance credentials; `ProductionCsid.create` returns production credentials.
    Pass an `Environment` (`NonProduction`, `Simulation`, or `Production`), a nullable
    `String` request ID, and `String` token/secret. `getEnv()` returns `Environment`;
    `getBinarySecurityToken()`, `getSecret()` and the nullable `getRequestId()`
    return `String`.

    **Responses**

    | Method | Returns |
    | --- | --- |
    | `response.getHttpStatus()` | `Integer (nullable)` |
    | `response.getOutcome()` | `InvoiceOutcome` (`Accepted`, `Rejected`, `Unknown`) |
    | `response.ensureAccepted()` | No value; throws unless accepted. |
    | `response.validationResults()` | `ValidationResults` |
    | `response.getReportingStatus()` | Nullable `ReportingStatus` (`Reported`, `NotReported`, `Unknown`) |
    | `response.getClearanceStatus()` | Nullable `ClearanceStatus` (`Cleared`, `NotCleared`, `Unknown`) |
    | `response.getReportingStatusRaw()` / `getClearanceStatusRaw()` / `getQrSellerStatus()` / `getQrBuyerStatus()` | Nullable `String` |
    | `response.getClearedInvoiceBase64()` / `getClearedInvoiceXml()` | Nullable `String`; XML decoding can throw. |
    | `results.getStatus()` | Nullable `ValidationStatus` (`Pass`, `Warning`, `Error`, `Unknown`) |
    | `results.getStatusRaw()` | Nullable `String` |
    | `results.getInfoLen()` / `getWarningLen()` / `getErrorLen()` | `java.math.BigInteger` message count |
    | `results.infoMessage(index)` / `warningMessage(index)` / `errorMessage(index)` | `ValidationMessage` |
    | `message.getMessageType()` | Nullable `MessageType` (`Info`, `Warning`, `Error`, `Unknown`) |
    | `message.getStatus()` | Nullable `ValidationStatus` |
    | `message.getMessageTypeRaw()` / `getStatusRaw()` / `getCode()` / `getCategory()` / `getMessage()` | Nullable `String` |

    Status enums are `null` when the field is absent and `Unknown` when it is present
    but not recognized by this binding version; the matching `get*Raw()` getter returns
    the exact wire string.

    Use zero-based `java.math.BigInteger` indexes below the corresponding message count. Child
    objects are independently owned. Methods do not consume their inputs.

    **Prepared requests**

    Use the six methods `prepareRequestComplianceCsid`,
    `prepareRequestProductionCsid`, `prepareRenewCsid`,
    `prepareCheckInvoiceCompliance`, `prepareReportSimplifiedInvoice`,
    and `prepareClearStandardInvoice` with the same inputs as their HTTP
    counterparts. They return `ComplianceRequest` for compliance credentials,
    `ProductionRequest` for production credentials/renewal, and `InvoiceRequest` for
    invoice operations, without sending HTTP.

    `request.getMethod()`, `getUrl()`, and `getHeadersJson()`
    return `String`; `body()` returns owned `Bytes`.
    `request.decode(status, body)` accepts `int` HTTP status and
    `byte[]` response body, and returns the matching credential or response type.
    Prepared requests own their input data and may outlive the original client and inputs.

=== "C"
    Include `fatoora.h`. Every operation returns `FatooraStatus`; `FATOORA_OK` means
    its output was written. Other statuses indicate failure; inspect the optional
    `FatooraError`. Calls borrow inputs and block while sending HTTP.

    **Client and credentials**

    ```c
    FatooraStatus fatoora_zatca_client_new(const FatooraConfig *config,
        FatooraZatcaClient **out,
        FatooraError **error);
    /* Like fatoora_zatca_client_new; each request fails with
     * FATOORA_NETWORK after timeout_ms milliseconds. Zero is rejected. */
    FatooraStatus fatoora_zatca_client_with_timeout(const FatooraConfig *config,
        uint32_t timeout_ms,
        FatooraZatcaClient **out,
        FatooraError **error);
    FatooraStatus fatoora_zatca_client_request_compliance_csid(const FatooraZatcaClient *self,
        const FatooraCsr *csr,
        const char *otp,
        FatooraComplianceCsid **out,
        FatooraError **error);
    FatooraStatus fatoora_zatca_client_request_production_csid(const FatooraZatcaClient *self,
        const FatooraComplianceCsid *credentials,
        FatooraProductionCsid **out,
        FatooraError **error);
    FatooraStatus fatoora_zatca_client_renew_csid(const FatooraZatcaClient *self,
        const FatooraProductionCsid *credentials,
        const FatooraCsr *csr,
        const char *otp,
        const char *accept_language,
        FatooraProductionCsid **out,
        FatooraError **error);
    FatooraStatus fatoora_zatca_client_check_invoice_compliance(const FatooraZatcaClient *self,
        const FatooraSignedInvoice *invoice,
        const FatooraComplianceCsid *credentials,
        FatooraValidationResponse **out,
        FatooraError **error);
    FatooraStatus fatoora_zatca_client_report_simplified_invoice(const FatooraZatcaClient *self,
        const FatooraSignedInvoice *invoice,
        const FatooraProductionCsid *credentials,
        FatooraClearanceMode clearance_mode,
        const char *accept_language,
        FatooraValidationResponse **out,
        FatooraError **error);
    FatooraStatus fatoora_zatca_client_clear_standard_invoice(const FatooraZatcaClient *self,
        const FatooraSignedInvoice *invoice,
        const FatooraProductionCsid *credentials,
        FatooraClearanceMode clearance_mode,
        const char *accept_language,
        FatooraValidationResponse **out,
        FatooraError **error);
    FatooraStatus fatoora_compliance_csid_new(FatooraEnvironment environment,
        const char *request_id,
        const char *token,
        const char *secret,
        FatooraComplianceCsid **out,
        FatooraError **error);
    FatooraStatus fatoora_production_csid_new(FatooraEnvironment environment,
        const char *request_id,
        const char *token,
        const char *secret,
        FatooraProductionCsid **out,
        FatooraError **error);
    ```

    Environment values are `FATOORA_NON_PRODUCTION`, `FATOORA_SIMULATION`, and
    `FATOORA_PRODUCTION`. `clearance_mode` is `FATOORA_CLEARANCE_DISABLED` or
    `FATOORA_CLEARANCE_ENABLED` (the Clearance-Status header). Any other enum value
    returns `FATOORA_INVALID_INPUT`.
    Use `NULL` for absent request ID or language. Text inputs are UTF-8 strings.
    Credential getters start with `fatoora_compliance_csid_` or
    `fatoora_production_csid_`, take `const` credential pointer, output pointer, and
    `FatooraError **error`. `env` writes `FatooraEnvironment`; `request_id`, `binary_security_token`,
    and `secret` write owned `char *`, with `NULL` for absent request ID.

    **Responses**

    Each getter takes `const FatooraValidationResponse *self`, the output argument
    below, then `FatooraError **error`. `ensure_accepted` takes only `self` and `error`.

    | Function | Output |
    | --- | --- |
    | `fatoora_validation_response_http_status` | `uint16_t *out`, `bool *has_value` |
    | `fatoora_validation_response_outcome` | `FatooraInvoiceOutcome *out` |
    | `fatoora_validation_response_ensure_accepted` | No output; `FATOORA_OK` only if accepted. |
    | `fatoora_validation_response_validation_results` | `FatooraValidationResults **out` |
    | `fatoora_validation_response_reporting_status` | `FatooraReportingStatus *out`, `bool *has_value` |
    | `fatoora_validation_response_reporting_status_raw` | `char **out` |
    | `fatoora_validation_response_clearance_status` | `FatooraClearanceStatus *out`, `bool *has_value` |
    | `fatoora_validation_response_clearance_status_raw` | `char **out` |
    | `fatoora_validation_response_qr_seller_status` | `char **out` |
    | `fatoora_validation_response_qr_buyer_status` | `char **out` |
    | `fatoora_validation_response_cleared_invoice_base64` | `char **out` |
    | `fatoora_validation_response_cleared_invoice_xml` | `char **out`; decoding can fail. |

    Optional text returns `NULL` for absence, distinct from a present empty string.
    Status enum getters set `*has_value` to `false` when the field is absent; a present
    value this binding version does not recognize is written as the `*_UNKNOWN`
    enumerator (such as `FATOORA_REPORTING_STATUS_UNKNOWN`), and the `_raw` getter
    returns the exact wire string.
    Result getters use the `fatoora_validation_results_` prefix:
    `status(self, FatooraValidationStatus *out, bool *has_value, error)` returns the
    optional status and `status_raw(self, char **out, error)` its text;
    `info_len`, `warning_len`, and `error_len` use `size_t *out`;
    `info_message`, `warning_message`, and `error_message` take `self`, zero-based
    `size_t index`, `FatooraValidationMessage **out`, and `error`. Indexes must be below
    the corresponding length.

    Message getters use `fatoora_validation_message_` with suffixes `message_type_raw`,
    `code`, `category`, `message`, and `status_raw`. Each takes `self`, `char **out`, and
    `error`, returning optional owned text. `message_type` writes
    `FatooraMessageType *out` and `status` writes `FatooraValidationStatus *out`, each
    with `bool *has_value`.

    **Prepared requests**

    Add `prepare_` after `fatoora_zatca_client_` in the six HTTP function names above.
    Inputs are identical; the output changes to `FatooraComplianceRequest **` for CSR
    issuance, `FatooraProductionRequest **` for production issuance/renewal, or
    `FatooraInvoiceRequest **` for invoice operations.

    Request accessors use `fatoora_compliance_request_`, `fatoora_production_request_`,
    or `fatoora_invoice_request_`. `method`, `url`, and `headers_json` take `self`,
    `char **out`, and `error`; `body` uses `FatooraBytes **out`.
    `decode` takes `self`, `uint16_t status`, `const uint8_t *body`, `size_t body_len`,
    the matching credential/response output pointer, and `error`.
    The request owns its data and does not send HTTP until your transport does so.

    **Ownership**

    Outputs and response children are independent owned copies. Release text with
    `fatoora_string_free`; release handles with their matching `_free` function, such
    as `fatoora_zatca_client_free`, `fatoora_compliance_csid_free`,
    `fatoora_production_csid_free`, `fatoora_validation_response_free`,
    `fatoora_validation_results_free`, `fatoora_validation_message_free`, or
    `fatoora_invoice_request_free`. See [C ownership and errors](bindings/c.md).

## Related reference

- [Environment configuration](configuration.md)
- [CSR and private keys](csr.md)
- [Invoice signing](invoice-signing.md)
- [Error handling](errors.md)
