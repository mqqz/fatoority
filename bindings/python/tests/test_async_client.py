"""``AsyncZatcaClient`` against the loopback gateway of the blocking client's tests."""
import asyncio
import base64
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import runpy
import socket
import sys
from threading import Thread
import time

import httpx
import pytest

import fatoora
from fatoora import (AsyncZatcaClient, ClearanceMode, ComplianceCsid, Config, CsrProperties, Environment,
                     InvoiceOutcome, ProductionCsid, ReportingStatus, SignedInvoice, SigningKey,
                     ZatcaClient)
from fatoora.errors import ApiError, InvalidInputError, NetworkError, ParseError, UnauthorizedError
from test_api_responses import gateway

FIXTURES = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures"
SIMPLIFIED = FIXTURES / "invoices/sample-simplified-invoice.xml"
STANDARD = FIXTURES / "invoices/Standard/Invoice/Standard_Invoice.xml"
CSR_CONFIG = FIXTURES / "csr-configs/csr-config-example-EN.properties"
ENV = Environment.NON_PRODUCTION
INVOICE_BODY = json.dumps({"validationResults": {"status": "PASS"}, "reportingStatus": "REPORTED",
                           "clearanceStatus": "CLEARED"})
CSID_BODY = json.dumps({"requestID": 1234567890, "binarySecurityToken": "issued-token", "secret": "issued-secret",
                        "dispositionMessage": "ISSUED"})
# Headers set by the HTTP stack rather than by the prepared request.
TRANSPORT_HEADERS = {"host", "user-agent", "accept", "accept-encoding", "connection"}


def run(coroutine):
    return asyncio.run(coroutine)


def csr():
    with SigningKey.generate() as key:
        return CsrProperties.from_file(CSR_CONFIG).build(key, ENV)


def ccsid():
    return ComplianceCsid(ENV, token="test-token", secret="test-secret", request_id="1234567890")


def pcsid():
    return ProductionCsid(ENV, token="test-token", secret="test-secret")


# Each operation: (name, arguments, keyword arguments, gateway body, invoice operation).
OPERATIONS = [
    ("request_compliance_csid", lambda: (csr(), "123345"), {}, CSID_BODY),
    ("request_production_csid", lambda: (ccsid(),), {}, CSID_BODY),
    ("renew_csid", lambda: (pcsid(), csr(), "123345", "ar"), {}, CSID_BODY),
    ("check_invoice_compliance", lambda: (SignedInvoice.from_file(SIMPLIFIED), ccsid()), {}, INVOICE_BODY),
    ("report_simplified_invoice", lambda: (SignedInvoice.from_file(SIMPLIFIED), pcsid()),
     {"clearance": ClearanceMode.DISABLED, "accept_language": "en"}, INVOICE_BODY),
    ("clear_standard_invoice", lambda: (SignedInvoice.from_file(STANDARD), pcsid()),
     {"clearance": ClearanceMode.ENABLED, "accept_language": "ar"}, INVOICE_BODY),
]


def report(**client_options):
    async def main():
        with SignedInvoice.from_file(SIMPLIFIED) as invoice, pcsid() as credentials:
            async with AsyncZatcaClient(Config(ENV), **client_options) as client:
                return await client.report_simplified_invoice(invoice, credentials, clearance=ClearanceMode.DISABLED)
    return run(main())


@contextmanager
def server(monkeypatch, handler):
    """A loopback gateway whose ``do_POST`` is ``handler(request_handler)``."""
    Handler = type("Handler", (BaseHTTPRequestHandler,), {
        "do_POST": handler, "do_PATCH": handler, "do_GET": handler, "log_message": lambda self, *args: None})
    http = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = Thread(target=http.serve_forever, daemon=True)
    thread.start()
    monkeypatch.setenv("FATOORA_ZATCA_BASE_URL", f"http://127.0.0.1:{http.server_port}")
    try:
        yield
    finally:
        http.shutdown()
        http.server_close()
        thread.join()


@pytest.mark.parametrize("name,arguments,options,body", OPERATIONS, ids=[op[0] for op in OPERATIONS])
def test_every_method_sends_what_the_blocking_client_sends(monkeypatch, name, arguments, options, body):
    sync_headers, async_headers = {}, {}
    inputs = arguments()  # calls do not consume their inputs
    with gateway(monkeypatch, 200, body, headers=sync_headers):
        with ZatcaClient(Config(ENV)) as client:
            expected = getattr(client, name)(*inputs, **options)

    async def main():
        async with AsyncZatcaClient(Config(ENV)) as client:
            return await getattr(client, name)(*inputs, **options)

    with gateway(monkeypatch, 200, body, headers=async_headers):
        result = run(main())
    assert type(result) is type(expected)
    if body is CSID_BODY:
        assert (result.binary_security_token, result.secret, result.request_id) == \
            ("issued-token", "issued-secret", "1234567890")
        assert result.env is ENV
    else:
        assert result.http_status == 200
        assert result.outcome == expected.outcome == InvoiceOutcome.ACCEPTED
        result.ensure_accepted()
    for headers in (sync_headers, async_headers):
        for header in TRANSPORT_HEADERS:
            headers.pop(header, None)
    assert async_headers == sync_headers
    if name != "request_compliance_csid":
        assert async_headers["authorization"] == "Basic " + base64.b64encode(b"test-token:test-secret").decode()


def test_method_signatures_match_the_blocking_client():
    import inspect
    for name, *_ in OPERATIONS:
        method = getattr(AsyncZatcaClient, name)
        assert inspect.iscoroutinefunction(method)
        def shape(function):
            return [(p.name, p.kind, p.default) for p in inspect.signature(function).parameters.values()]
        assert shape(method) == shape(getattr(ZatcaClient, name))


def test_response_bodies_are_decoded_by_the_core(monkeypatch):
    xml = "<Invoice>cleared</Invoice>"
    body = json.dumps({"validationResults": {"status": "PASS"}, "reportingStatus": "REPORTED",
                       "clearedInvoice": base64.b64encode(xml.encode()).decode()})
    with gateway(monkeypatch, 202, body), report() as response:
        assert response.http_status == 202
        assert response.reporting_status is ReportingStatus.REPORTED
        assert response.cleared_invoice_xml == xml


@pytest.mark.parametrize("status", [400, 401, 409, 503])
def test_http_error_statuses_raise_the_core_errors(monkeypatch, status):
    body = json.dumps({"validationResults": {"status": "ERROR"}, "reportingStatus": "NOT_REPORTED"})
    with gateway(monkeypatch, status, body), pytest.raises(UnauthorizedError if status == 401 else ApiError) as caught:
        report()
    assert caught.value.details["type"] == "api_response"
    assert caught.value.details["http_status"] == status
    assert caught.value.details["body"] == body


def test_malformed_success_body_raises_parse_error(monkeypatch):
    with gateway(monkeypatch, 200, "not json"), pytest.raises(ParseError):
        report()


def test_redirects_are_not_followed(monkeypatch):
    seen = []

    def handler(self):
        seen.append(self.path)
        if self.command == "POST":
            self.rfile.read(int(self.headers["Content-Length"]))
            self.send_response(303)
            self.send_header("Location", "/followed")
            self.send_header("Content-Length", "0")
        else:
            self.send_response(200)
            self.send_header("Content-Length", "0")
        self.end_headers()

    with server(monkeypatch, handler), pytest.raises(ApiError) as caught:
        report()
    assert caught.value.details["http_status"] == 303
    assert seen == ["/invoices/reporting/single"]


def test_injected_client_redirect_setting_is_overridden(monkeypatch):
    seen = []

    def handler(self):
        seen.append(self.path)
        self.rfile.read(int(self.headers.get("Content-Length", 0)))
        self.send_response(307)
        self.send_header("Location", "/followed")
        self.send_header("Content-Length", "0")
        self.end_headers()

    async def main():
        async with httpx.AsyncClient(follow_redirects=True) as http:
            return await report_with(http)

    with server(monkeypatch, handler), pytest.raises(ApiError):
        run(main())
    assert seen == ["/invoices/reporting/single"]


async def report_with(http, **options):
    with SignedInvoice.from_file(SIMPLIFIED) as invoice, pcsid() as credentials:
        client = AsyncZatcaClient(Config(ENV), http_client=http, **options)
        return await client.report_simplified_invoice(invoice, credentials, clearance=ClearanceMode.DISABLED)


def test_timeout_raises_network_error(monkeypatch):
    with gateway(monkeypatch, 200, INVOICE_BODY, delay=1.0):
        started = time.monotonic()
        with pytest.raises(NetworkError) as caught:
            report(timeout=0.1)
        assert time.monotonic() - started < 0.9
    assert caught.value.details["type"] == "network"
    assert isinstance(caught.value.__cause__, asyncio.TimeoutError)


def test_generous_timeout_still_succeeds(monkeypatch):
    from datetime import timedelta
    with gateway(monkeypatch, 200, INVOICE_BODY), report(timeout=timedelta(seconds=30)) as response:
        assert response.outcome == InvoiceOutcome.ACCEPTED


@pytest.mark.parametrize("timeout,error", [(0, ValueError), (-1, ValueError), (float("nan"), ValueError),
                                           ("5", TypeError), (True, TypeError)])
def test_invalid_timeouts_are_rejected_like_the_blocking_client(timeout, error):
    with pytest.raises(error):
        AsyncZatcaClient(Config(ENV), timeout=timeout)
    with pytest.raises(error):
        ZatcaClient(Config(ENV), timeout=timeout)


def test_injected_client_timeouts_raise_network_error(monkeypatch):
    async def main():
        async with httpx.AsyncClient(timeout=0.1) as http:
            return await report_with(http)

    with gateway(monkeypatch, 200, INVOICE_BODY, delay=1.0), pytest.raises(NetworkError) as caught:
        run(main())
    assert isinstance(caught.value.__cause__, httpx.TimeoutException)


def test_stalled_invoice_body_is_a_response_read_error(monkeypatch):
    def handler(self):
        self.rfile.read(int(self.headers["Content-Length"]))
        self.send_response(200)
        self.send_header("Content-Length", "100")
        self.end_headers()
        self.wfile.flush()
        time.sleep(1.0)

    with server(monkeypatch, handler), pytest.raises(NetworkError) as caught:
        report(timeout=0.2)
    assert caught.value.details == {"type": "api_response_read", "http_status": 200,
                                    "message": caught.value.details["message"]}


def test_truncated_body(monkeypatch):
    def handler(self):
        self.rfile.read(int(self.headers["Content-Length"]))
        self.send_response(401)
        self.send_header("Content-Length", "100")
        self.end_headers()
        self.wfile.write(b'{"partial"')

    # An invoice operation reports the failed read.
    with server(monkeypatch, handler), pytest.raises(NetworkError) as caught:
        report()
    assert caught.value.details["type"] == "api_response_read"
    assert caught.value.details["http_status"] == 401

    # A CSID operation decodes an empty body, as the native client does.
    async def renew():
        async with AsyncZatcaClient(Config(ENV)) as client:
            return await client.renew_csid(pcsid(), csr(), "123345")

    with server(monkeypatch, handler), pytest.raises(UnauthorizedError) as caught:
        run(renew())
    assert caught.value.details["type"] == "api_unauthorized"


def test_transport_failure_raises_network_error(monkeypatch):
    with socket.socket() as probe:
        probe.bind(("127.0.0.1", 0))
        port = probe.getsockname()[1]
    monkeypatch.setenv("FATOORA_ZATCA_BASE_URL", f"http://127.0.0.1:{port}")
    with pytest.raises(NetworkError) as caught:
        report()
    assert caught.value.code == 7
    assert caught.value.details["type"] == "network"
    assert caught.value.details["diagnostics"][0]["message"]
    assert isinstance(caught.value.__cause__, httpx.TransportError)


def test_preparation_errors_are_raised_before_sending(monkeypatch):
    async def main():
        async with AsyncZatcaClient(Config(ENV)) as client:
            with SignedInvoice.from_file(SIMPLIFIED) as invoice:
                await client.report_simplified_invoice(
                    invoice, ProductionCsid(Environment.SIMULATION, token="t", secret="s"),
                    clearance=ClearanceMode.DISABLED)

    monkeypatch.setenv("FATOORA_ZATCA_BASE_URL", "http://127.0.0.1:9")
    with pytest.raises(fatoora.InternalError) as caught:
        run(main())
    assert caught.value.details["type"] == "client_state"


def test_clearance_is_a_required_clearance_mode():
    async def main(**options):
        async with AsyncZatcaClient(Config(ENV)) as client:
            await client.report_simplified_invoice(None, None, **options)

    with pytest.raises(TypeError, match="'clearance'"):
        run(main())
    with pytest.raises(TypeError, match="ClearanceMode"):
        run(main(clearance=True))


def test_owned_http_client_is_closed():
    async def main():
        async with AsyncZatcaClient(Config(ENV)) as client:
            http = client._http
            assert not http.is_closed
        assert http.is_closed
        await client.aclose()  # idempotent
        with pytest.raises(InvalidInputError, match="closed"):
            await client.request_production_csid(ccsid())
        assert repr(client) == "<AsyncZatcaClient closed>"

    run(main())


def test_injected_http_client_is_reused_and_left_open(monkeypatch):
    requests = []

    async def record(request):
        requests.append(request)

    async def main():
        async with httpx.AsyncClient(event_hooks={"request": [record]}) as http:
            async with AsyncZatcaClient(Config(ENV), http_client=http) as client:
                with SignedInvoice.from_file(SIMPLIFIED) as invoice, pcsid() as credentials:
                    for _ in range(2):
                        response = await client.report_simplified_invoice(
                            invoice, credentials, clearance=ClearanceMode.DISABLED)
                        response.ensure_accepted()
            assert not http.is_closed
        assert http.is_closed

    with gateway(monkeypatch, 200, INVOICE_BODY):
        run(main())
    assert len(requests) == 2


def test_http_client_must_be_an_httpx_async_client():
    with pytest.raises(TypeError, match="httpx.AsyncClient"):
        AsyncZatcaClient(Config(ENV), http_client=httpx.Client())


def test_concurrent_requests(monkeypatch):
    async def main():
        async with AsyncZatcaClient(Config(ENV)) as client:
            with SignedInvoice.from_file(SIMPLIFIED) as invoice, pcsid() as credentials:
                return await asyncio.gather(*(
                    client.report_simplified_invoice(invoice, credentials, clearance=ClearanceMode.DISABLED)
                    for _ in range(5)))

    started = time.monotonic()
    with gateway(monkeypatch, 200, INVOICE_BODY, delay=0.3):
        responses = run(main())
    assert time.monotonic() - started < 1.2
    assert [r.outcome for r in responses] == [InvoiceOutcome.ACCEPTED] * 5


def test_missing_httpx_raises_import_error_with_install_hint():
    # A fresh interpreter where ``import httpx`` fails: importing fatoora still works.
    import subprocess
    import textwrap
    script = textwrap.dedent("""
        import sys
        sys.modules["httpx"] = None
        import fatoora
        assert "AsyncZatcaClient" in fatoora.__all__
        try:
            fatoora.AsyncZatcaClient(fatoora.Config())
        except ImportError as error:
            assert error.name == "httpx", error.name
            print(error)
        else:
            raise AssertionError("AsyncZatcaClient did not raise ImportError")
    """)
    result = subprocess.run([sys.executable, "-c", script], capture_output=True, text=True, timeout=30)
    assert result.returncode == 0, result.stdout + result.stderr
    assert 'pip install "fatoority[async]"' in result.stdout


def test_async_api_example_against_local_gateway(monkeypatch):
    example = Path(__file__).resolve().parents[1] / "examples/api_async.py"
    with gateway(monkeypatch, 200, INVOICE_BODY):
        namespace = runpy.run_path(str(example), run_name="__main__")
    assert namespace["response"].outcome == InvoiceOutcome.ACCEPTED
    namespace["response"].close()
