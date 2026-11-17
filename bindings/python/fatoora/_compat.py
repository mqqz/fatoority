"""Deprecation helpers for the 0.6 transition. Everything here is removed in 0.7."""
from __future__ import annotations

import functools
import warnings
from typing import Any, Callable

REMOVAL = "0.7"


def warn(message: str, skip: int = 2) -> None:
    """Emit the package's standard deprecation warning.

    ``skip`` counts the fatoora frames between the user's code and this call
    (2 = a fatoora function called by the user calls ``warn``). ``warnings``
    itself skips importlib frames, so ``from fatoora import Signer`` points at
    the import line.
    """
    warnings.warn(f"{message}; it will be removed in {REMOVAL}", DeprecationWarning,
                  stacklevel=skip + 1)


def deprecated(replacement: str) -> Callable:
    """Mark a function or method deprecated in favour of ``replacement``.

    The wrapper keeps the original signature (``functools.wraps`` sets
    ``__wrapped__``) and records the message in ``__deprecated__`` (PEP 702).
    """
    def decorator(function):
        message = f"{function.__qualname__}() is deprecated; use {replacement} instead"

        @functools.wraps(function)
        def wrapper(*args, **kwargs):
            warn(message)
            return function(*args, **kwargs)

        wrapper.__deprecated__ = message
        return wrapper
    return decorator


def alias_getattr(module_name: str, aliases: dict[str, tuple[str, Callable[[], Any]]]):
    """Build a PEP 562 module ``__getattr__`` serving deprecated names."""
    def __getattr__(name: str):
        try:
            replacement, resolve = aliases[name]
        except KeyError:
            raise AttributeError(f"module {module_name!r} has no attribute {name!r}") from None
        warn(f"{module_name}.{name} is deprecated; use {replacement} instead")
        return resolve()
    return __getattr__
