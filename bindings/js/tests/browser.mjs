import { chromium } from 'playwright';
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
const root = new URL('../', import.meta.url);
let redirected = 0;
const server = createServer(async (req, res) => {
  if (req.url === '/redirect') { res.writeHead(302, {location: '/unexpected'}).end(); return; }
  if (req.url === '/unexpected') { redirected++; res.end(); return; }
  if (req.url === '/slow') { return; }
  try {
    const path = req.url === '/' ? 'tests/browser.html' : req.url.slice(1);
    const url = new URL(path, root);
    if (!url.href.startsWith(root.href)) { res.writeHead(403).end(); return; }
    const data = await readFile(url);
    res.setHeader('Content-Type', path.endsWith('.mjs') ? 'text/javascript' : path.endsWith('.wasm') ? 'application/wasm' : 'text/html');
    res.end(data);
  } catch { res.writeHead(404).end(); }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const browser = await chromium.launch({headless: true, ...(process.env.CHROMIUM_PATH ? {executablePath: process.env.CHROMIUM_PATH} : {})});
try {
  const page = await browser.newPage();
  await page.goto(`http://127.0.0.1:${server.address().port}/`);
  const result = await page.evaluate(async () => {
    const sdk = await import('/index.mjs');
    const raw = await import('/raw.mjs');
    const key = sdk.SigningKey.generate();
    const config = sdk.Config.create(sdk.Environment.NonProduction);
    const credentials = sdk.ComplianceCsid.create(sdk.Environment.NonProduction, '123', 'token', 'secret');
    const prepared = raw.ZatcaClient.create(config).prepareRequestProductionCsid(credentials);
    const response = await sdk.sendPrepared(prepared, {fetch: async (_url, init) => {
      if (new Headers(init.headers).get('authorization') !== 'Basic dG9rZW46c2VjcmV0') throw new Error('bad auth');
      return new Response(JSON.stringify({requestID: 2, binarySecurityToken: 'new', secret: 'secret'}));
    }});
    const client = new sdk.ZatcaClient(config, {fetch: (_url, init) => fetch('/redirect', init)});
    let redirectBlocked = false;
    try { await client.requestProductionCsid(credentials); } catch (e) { redirectBlocked = e instanceof sdk.NetworkError && e.details.type === 'network'; }
    const controller = new AbortController();
    const slow = new sdk.ZatcaClient(config, {fetch: (_url, init) => fetch('/slow', init)});
    const pending = slow.requestProductionCsid(credentials, {signal: controller.signal});
    controller.abort();
    let aborted = false;
    try { await pending; } catch (e) { aborted = e instanceof sdk.NetworkError && e.cause.name === 'AbortError'; }
    let typedError = false;
    try { sdk.Address.create('SAU', '\0', '', '', '', null, null, null); } catch (e) { typedError = e instanceof sdk.InvalidInputError && e.code === 1; }
    const der = key.toDer().toUint8Array();
    const bytes = der instanceof Uint8Array && sdk.SigningKey.fromDer(der).toPem() === key.toPem();
    return {pem: key.toPem(), bytes, token: response.binarySecurityToken, redirectBlocked, aborted, typedError};
  });
  assert.match(result.pem, /BEGIN PRIVATE KEY/);
  assert.equal(result.bytes, true);
  assert.equal(result.token, 'new');
  assert.equal(result.redirectBlocked, true);
  assert.equal(result.aborted, true);
  assert.equal(result.typedError, true);
  assert.equal(redirected, 0);
  console.log('Chromium: WASM loading, randomness, Uint8Array bytes, prepared requests, redirect refusal, cancellation and typed errors passed');
} finally {
  await browser.close();
  server.closeAllConnections();
  await new Promise(resolve => server.close(resolve));
}
