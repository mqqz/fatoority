"""``AsyncZatcaClient``: the ZATCA gateway client for ``asyncio``, sending through httpx.

The native library prepares each request and decodes each response, exactly as
for the blocking :class:`~fatoora.ZatcaClient`; only the HTTP exchange runs in
Python. httpx is an optional dependency (``pip install "fatoority[async]"``),
imported when a client is created.
"""
from __future__ import annotations

import asyncio
from datetime import timedelta
import json
from typing import TYPE_CHECKING, Any, Optional, Union

from .api import (ClearanceMode, ComplianceCsid, Config, Csr, ProductionCsid, SignedInvoice,
                  ValidationResponse, ZatcaClient, _call, _clearance_mode, _timeout_ms)
from .errors import InvalidInputError, NetworkError

if TYPE_CHECKING:
    import httpx

_INSTALL_HINT = 'AsyncZatcaClient requires httpx; install it with: pip install "fatoority[async]"'


def _import_httpx():
    try:
        import httpx
    except ImportError as exc:
        raise ImportError(_INSTALL_HINT, name="httpx") from exc
    return httpx


def _network_error(message: str, cause: BaseException) -> NetworkError:
    # The shape of the core's transport errors (details type "network").
    return NetworkError(f"{message}: {cause}" if str(cause) else message,
                        details={"type": "network", "diagnostics": [{"message": str(cause) or message}]})


def _read_error(status: int, cause: BaseException) -> NetworkError:
    # The core's ``ZatcaError::ResponseRead``: headers arrived, the body did not.
    text = str(cause) or type(cause).__name__
    return NetworkError(f"Failed to read ZATCA response (HTTP {status}): {text}",
                        details={"type": "api_response_read", "http_status": status, "message": text})


class AsyncZatcaClient:
    """Asynchronous ZATCA gateway client for ``asyncio``.

    It has the methods of :class:`~fatoora.ZatcaClient`, as coroutines, and
    returns the same objects and raises the same errors. The native library
    builds and decodes each request; httpx sends it, without following
    redirects.

    ``timeout`` bounds each request, from sending it until the response body is
    read, in seconds (``int``/``float``) or as a ``datetime.timedelta``. A
    request that exceeds it raises ``NetworkError``. The default ``None``
    applies no client-side timeout.

    Pass ``http_client`` to send through your own ``httpx.AsyncClient`` (to
    share its connection pool, proxies or TLS settings); its own timeouts also
    apply, and closing this client leaves it open. Without one, the client
    creates an ``httpx.AsyncClient`` (with no httpx timeouts) and closes it in
    :meth:`aclose`. Use ``async with`` to close the client when done.

    Requires httpx: ``pip install "fatoority[async]"``.
    """

    def __init__(self, config: Config, *, timeout: Optional[Union[float, timedelta]] = None,
                 http_client: Optional["httpx.AsyncClient"] = None) -> None:
        httpx = _import_httpx()
        if http_client is not None and not isinstance(http_client, httpx.AsyncClient):
            raise TypeError("http_client must be an httpx.AsyncClient or None")
        self._timeout = None if timeout is None else _timeout_ms(timeout) / 1000
        self._httpx = httpx
        self._native: Optional[ZatcaClient] = ZatcaClient(config)
        self._owns_http = http_client is None
        self._http: "httpx.AsyncClient" = (
            httpx.AsyncClient(timeout=None, follow_redirects=False) if http_client is None else http_client)

    async def __aenter__(self) -> "AsyncZatcaClient":
        self._client()
        return self

    async def __aexit__(self, *args: Any) -> None:
        await self.aclose()

    async def aclose(self) -> None:
        """Close the client, and the ``httpx.AsyncClient`` it created (not one passed in)."""
        native, self._native = self._native, None
        if native is None:
            return
        native.close()
        if self._owns_http:
            await self._http.aclose()

    def __repr__(self) -> str:
        if self._native is None:
            return "<AsyncZatcaClient closed>"
        return f"AsyncZatcaClient(timeout={self._timeout!r})"

    def _client(self) -> ZatcaClient:
        if self._native is None:
            raise InvalidInputError("object is closed", details={"type": "binding_error"})
        return self._native

    def _prepare(self, method: str, *args):
        return self._client()._invoke(method, *args)

    async def request_compliance_csid(self, csr: Csr, otp: str) -> ComplianceCsid:
        """Exchange a CSR and OTP for compliance CSID credentials."""
        request = self._prepare("prepare_request_compliance_csid", csr, otp)
        return ComplianceCsid._wrap(await self._send(request, invoice=False))

    async def request_production_csid(self, ccsid: ComplianceCsid) -> ProductionCsid:
        """Exchange compliance credentials for production CSID credentials."""
        request = self._prepare("prepare_request_production_csid", ccsid)
        return ProductionCsid._wrap(await self._send(request, invoice=False))

    async def renew_csid(self, pcsid: ProductionCsid, csr: Csr, otp: str,
                         accept_language: Optional[str] = None) -> ProductionCsid:
        request = self._prepare("prepare_renew_csid", pcsid, csr, otp, accept_language)
        return ProductionCsid._wrap(await self._send(request, invoice=False))

    async def check_invoice_compliance(self, invoice: SignedInvoice, ccsid: ComplianceCsid) -> ValidationResponse:
        request = self._prepare("prepare_check_invoice_compliance", invoice, ccsid)
        return ValidationResponse._wrap(await self._send(request, invoice=True))

    async def report_simplified_invoice(self, invoice: SignedInvoice, pcsid: ProductionCsid, *,
                                        clearance: ClearanceMode,
                                        accept_language: Optional[str] = None) -> ValidationResponse:
        """Report a simplified invoice. ``clearance`` sets the ``Clearance-Status`` header."""
        mode = _clearance_mode(clearance)
        request = self._prepare("prepare_report_simplified_invoice", invoice, pcsid, mode, accept_language)
        return ValidationResponse._wrap(await self._send(request, invoice=True))

    async def clear_standard_invoice(self, invoice: SignedInvoice, pcsid: ProductionCsid, *,
                                     clearance: ClearanceMode,
                                     accept_language: Optional[str] = None) -> ValidationResponse:
        """Clear a standard invoice. ``clearance`` sets the ``Clearance-Status`` header."""
        mode = _clearance_mode(clearance)
        request = self._prepare("prepare_clear_standard_invoice", invoice, pcsid, mode, accept_language)
        return ValidationResponse._wrap(await self._send(request, invoice=True))

    async def _send(self, request, *, invoice: bool):
        """Send a prepared request and decode the reply, as the native client does.

        A failure before the response headers is a ``NetworkError``. A body
        that cannot be read is a ``NetworkError`` for invoice operations; the
        CSID endpoints decode an empty body instead (a stalled body, past the
        timeout, is a ``NetworkError`` everywhere). Every HTTP status, a
        redirect included, reaches the native decoder.
        """
        httpx = self._httpx
        http = self._http
        # Read the native request completely before awaiting.
        built = http.build_request(_call(request.method), _call(request.url),
                                   headers=[tuple(pair) for pair in json.loads(_call(request.headers_json))],
                                   content=_call(request.body))
        status: Optional[int] = None

        async def exchange() -> bytes:
            nonlocal status
            response = await http.send(built, stream=True, follow_redirects=False)
            try:
                status = response.status_code
                return await response.aread()
            finally:
                await response.aclose()

        try:
            if self._timeout is None:
                body = await exchange()
            else:
                body = await asyncio.wait_for(exchange(), self._timeout)
        except (asyncio.TimeoutError, httpx.TimeoutException) as exc:
            if status is not None and invoice:
                raise _read_error(status, exc) from exc
            raise _network_error("ZATCA request timed out", exc) from exc
        except (httpx.HTTPError, httpx.InvalidURL, OSError) as exc:
            if status is None:
                raise _network_error("ZATCA request failed", exc) from exc
            if invoice:
                raise _read_error(status, exc) from exc
            body = b""
        assert status is not None
        return _call(request.decode, status, body)
