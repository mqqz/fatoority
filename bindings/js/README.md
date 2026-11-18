# Fatoority JavaScript / TypeScript

ES modules backed by Diplomat 0.17.0 and WebAssembly. Supports Node 22+ and modern browsers with WebAssembly, Web Crypto, FinalizationRegistry and top-level await.

Install from npm:

```sh
npm install fatoority
```

To build from source, run from the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install diplomat-tool --version 0.17.0 --locked
npm ci --prefix bindings/js
python3 scripts/generate_bindings.py
python3 scripts/build_js.py
npm run check --prefix bindings/js
node bindings/js/tests/runtime.test.mjs
python3 scripts/check_js_package.py
```

Build a local tarball with `npm pack` in `bindings/js`. Version tags publish the tested package to npm through GitHub Actions.

See the [JS/TS reference](https://docs.fatoority.com/reference/bindings/javascript/) for complete installation, browser deployment and API examples.

```js
import { Config, Environment, SigningKey, ComplianceCsid, ZatcaClient } from 'fatoority';

const key = SigningKey.generate();
const config = Config.create(Environment.NonProduction); // or Simulation, Production
const client = new ZatcaClient(config);
const compliance = ComplianceCsid.create(Environment.NonProduction, 'request-id', 'token', 'secret');
const production = await client.requestProductionCsid(compliance, {
  signal: AbortSignal.timeout(30_000),
});
```

API methods prepare requests and decode responses in Rust; the facade calls `fetch`. Inject a fetch implementation with `new ZatcaClient(config, {fetch})`. Requests refuse redirects. HTTP errors go to Rust decoding, and a successful invoice HTTP response still requires `response.ensureAccepted()` or inspection of `response.outcome`.

For your own transport, import `ZatcaClient` from `fatoority/raw`, call a `prepare*` method, and read `method`, `url`, `headersJson` (an array of header pairs), and `body().toUint8Array()`. Complete with `request.decode(status, bytes)`, or use `sendPrepared(request, {fetch, signal})`. The request owns its data and the operation/environment needed for decoding. It contains authorization headers and must not be logged.

The package root exposes the generated classes with Rust names in camelCase; a Rust `new` constructor is `create`. Value getters are read-only properties (`invoice.xml`, `address.city`, `totals.taxAmount`, `response.reportingStatus`); members that take arguments, return another native object (`invoice.data()`), convert or consume stay methods. Optional text is `string | null`. Decimal arguments and outputs are exact decimal strings (never `number`), and invoice counters are `bigint`. Enumerated codes are enum classes such as `Environment.NonProduction`, `VatCategory.Standard`, `AdjustmentKind.Charge` and `ClearanceMode.Disabled` (`.value` is the variant name); each enum has a string-literal union type such as `VatCategoryValue`. Response statuses such as `reportingStatus` are an enum (with `Unknown` for unrecognized values) or `null`; `reportingStatusRaw` and the other `*Raw` properties hold the exact text. The async `ZatcaClient` reporting and clearance methods, like the `fatoority/raw` `prepare*` methods, require a `ClearanceMode`. Optional strings use `null`, distinct from `''`. Consuming operations such as `build()`, signing and `intoXml()` reject reuse. Generated owners are released by FinalizationRegistry; do not manually free pointers or share handles between workers. Binary data is a `Uint8Array`: `Bytes.toUint8Array()` returns an independent copy, and byte arguments such as `SigningKey.fromDer(der)` take a `Uint8Array` (or `Buffer`). The facade snapshots outgoing bytes before awaiting the network.

Library failures throw one `FatooraError` subclass per `ErrorKind` (`InvalidInputError`, `ValidationError`, `ParseError`, `XmlError`, `CryptoError`, `IoError`, `NetworkError`, `UnauthorizedError`, `InternalError`, `ApiError`) with `code`, `kind`, `message`, parsed `details`, and the `BindingError` as `cause`. `fatoority/raw` throws the same classes. Network, cancellation and response-body read failures throw `NetworkError` with the host error as `cause` and `details` in the library schema (`network`, or `api_response_read` with `http_status`).

`Xml.validateZatca(xml, {previousInvoiceHash, evaluatedAt})` returns the `ZatcaValidationReport` as a plain, typed object (`isValid`, `stages[].findings[]` with `severity`, `layer`, `code`, `message`, `location`) and supplies the host clock when `evaluatedAt` is absent. An explicit timestamp makes validation reproducible. The raw API returns the native report and requires `evaluated_at` on WASM. Filesystem helpers are excluded from JS; read files in the host and pass their contents.

The browser loader resolves `fatoora_ffi.wasm` relative to the package modules. Deploy the generated modules and WASM together, or configure the bundler to copy the WASM at that URL. Direct gateway requests depend on the gateway's CORS policy. Keep signing keys and credentials in the host environment appropriate to your application.

If you set `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS`, retain `--cfg getrandom_backend="custom"`: these override `.cargo/config.toml` and otherwise break the WASM random backend.

The WASM build uses getrandom's custom backend with Web Crypto. `xsd-schema` transitively enables Chrono's wasm-bindgen features; the pinned Binaryen step removes unused descriptor exports and maps its retained UTF-8 throw helper to Diplomat's equivalent host import. The build fails if any unsupported import remains, including a JS clock or wasm-bindgen object runtime. Native transport and clock behavior use the normal Rust dependencies.

## Licensing

This package follows the native CLI/FFI archives and Python wheels: the compiled
validator includes SDK-derived rule predicates and messages, and the upstream
notices are distributed with it. This is not an entirely MIT/Apache-2.0 artifact.

Original project code is `MIT OR Apache-2.0`. The 257 translated assertion sites
and imported messages retain the terms recorded for ZATCA SDK `238-R3.4.8`,
including LGPL v3. The included GPL v3 text is incorporated by LGPL v3; its
presence is not a separate relicensing of all project code. The SDK's CEN
stylesheet claims LGPL v3 through EUPL compatibility; the exact upstream revision
and conversion basis remain unverified, as recorded in the existing native notices.

The npm `license` field points to [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
Read that file and [licenses/zatca/NOTICE.md](licenses/zatca/NOTICE.md), alongside
[LGPL v3](licenses/zatca/LICENSE-LGPL-3.0.txt) and
[GPL v3](licenses/zatca/LICENSE-GPL-3.0.txt). Copying notices alone does not resolve
all distribution obligations; downstream distribution must account for the
applicable source and relinking requirements. The source and build instructions
are in the [Fatoority repository](https://github.com/mqqz/fatoority), with versioned
source under the corresponding release tag. No ZATCA SDK JAR is bundled.

## WASM size

The measured package is 3.525 MB raw and 1.512 MB gzipped, down from an 8.974 MB Rust release artifact. `scripts/build_js.py` selects `wasm-release` (`opt-level = "z"`, LTO, one codegen unit and symbol stripping), runs pinned Binaryen `wasm-opt -Oz`, and rejects artifacts above 4 MB raw or 1.8 MB gzip.

The 15 UBL schemas originally totalled 2,996,782 bytes. WASM embeds them with lossless compression; each build verifies that decompression reproduces every source byte. Unicode regex behavior remains enabled. Run `python3 scripts/check_wasm_size.py --json` to inspect code/data sections and transfer size. Schema compilation still requires decompressed data in memory.
