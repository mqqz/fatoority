"""Exceptions raised by the fatoora bindings.

Every library failure raises a :class:`FatooraError` subclass chosen by its
:class:`ErrorKind`; an error code this version does not recognize raises the
base :class:`FatooraError` with ``kind`` set to ``None``.
"""

from __future__ import annotations

from enum import IntEnum
from typing import Any, ClassVar


class ErrorKind(IntEnum):
    """Category of a library failure; each value is the stable numeric error code."""

    INVALID_INPUT = 1
    VALIDATION = 2
    PARSE = 3
    XML = 4
    CRYPTO = 5
    IO = 6
    NETWORK = 7
    UNAUTHORIZED = 8
    INTERNAL = 9
    API = 10


def _kind(code: int | None) -> ErrorKind | None:
    try:
        return ErrorKind(code)
    except ValueError:
        return None


class FatooraError(RuntimeError):
    """Base class of every error the fatoora bindings raise for a library failure.

    ``code`` is the numeric error code, ``kind`` its :class:`ErrorKind` (or
    ``None`` for a code this version does not recognize) and ``details`` the
    structured details as a ``dict``. A subclass constructed directly takes its
    own code.
    """

    _default_kind: ClassVar[ErrorKind | None] = None

    code: int | None
    kind: ErrorKind | None
    details: dict[str, Any]

    def __init__(self, message: str, *, code: int | None = None, details: dict[str, Any] | None = None) -> None:
        super().__init__(message)
        if code is None and self._default_kind is not None:
            code = int(self._default_kind)
        self.code = code
        self.kind = _kind(code)
        self.details = details if details is not None else {}

    @property
    def message(self) -> str:
        """The human-readable message; do not parse it."""
        return str(self.args[0]) if self.args else ""

    @staticmethod
    def from_code(code: int | None, message: str, details: dict[str, Any] | None = None) -> FatooraError:
        """Create the subclass that matches ``code``; ``FatooraError`` for an unknown code."""
        kind = _kind(code)
        cls = _CLASSES[kind] if kind is not None else FatooraError
        return cls(message, code=code, details=details)


class InvalidInputError(FatooraError, ValueError):
    """A caller-supplied value is malformed or out of range (code 1)."""

    _default_kind = ErrorKind.INVALID_INPUT


class ValidationError(FatooraError):
    """Input is well-formed but fails invoice, CSR or schema rules (code 2)."""

    _default_kind = ErrorKind.VALIDATION


class ParseError(FatooraError):
    """Structured input, such as an API response body, could not be parsed (code 3)."""

    _default_kind = ErrorKind.PARSE


class XmlError(FatooraError):
    """XML could not be serialized, parsed or processed (code 4)."""

    _default_kind = ErrorKind.XML


class CryptoError(FatooraError):
    """Key handling, signing, hashing or certificate processing failed (code 5)."""

    _default_kind = ErrorKind.CRYPTO


class IoError(FatooraError, OSError):
    """A file or other I/O operation failed (code 6)."""

    _default_kind = ErrorKind.IO


class NetworkError(FatooraError):
    """The HTTP request or response transport failed (code 7)."""

    _default_kind = ErrorKind.NETWORK


class UnauthorizedError(FatooraError):
    """The ZATCA API rejected the supplied credentials (code 8)."""

    _default_kind = ErrorKind.UNAUTHORIZED


class InternalError(FatooraError):
    """An internal invariant or client state error (code 9)."""

    _default_kind = ErrorKind.INTERNAL


class ApiError(FatooraError):
    """The ZATCA API returned an error or non-accepted response (code 10)."""

    _default_kind = ErrorKind.API


_CLASSES: dict[ErrorKind, type[FatooraError]] = {
    cls._default_kind: cls  # type: ignore[misc]
    for cls in (
        InvalidInputError,
        ValidationError,
        ParseError,
        XmlError,
        CryptoError,
        IoError,
        NetworkError,
        UnauthorizedError,
        InternalError,
        ApiError,
    )
}

__all__ = [
    "ApiError",
    "CryptoError",
    "ErrorKind",
    "FatooraError",
    "InternalError",
    "InvalidInputError",
    "IoError",
    "NetworkError",
    "ParseError",
    "UnauthorizedError",
    "ValidationError",
    "XmlError",
]
