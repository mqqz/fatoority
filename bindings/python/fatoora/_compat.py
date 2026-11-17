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


def deprecated_property(replacement: str, getter: Callable[[Any], Any], name: str) -> property:
    """A read-only property that warns, for a renamed 0.5 getter."""
    message = f"{name} is deprecated; use {replacement} instead"

    def fget(self):
        warn(message)
        return getter(self)

    fget.__deprecated__ = message  # type: ignore[attr-defined]
    return property(fget, doc=message)


class deprecated_classmethod:
    """A 0.5 factory classmethod whose name is now a dataclass field.

    Read on the class (``Adjustment.amount(10)``) it is the deprecated factory;
    read on an instance the field value in the instance ``__dict__`` wins,
    because this is a non-data descriptor. Install it after ``@dataclass`` so
    the field default is captured first.
    """

    def __init__(self, replacement: str, factory: Callable[..., Any], name: str):
        self.__deprecated__ = f"{name}() is deprecated; use {replacement} instead"
        self._factory = factory

    def __get__(self, obj, cls=None):
        if obj is not None:
            # Only reached for an instance without the field set (never for a
            # constructed value); behave like a missing attribute.
            raise AttributeError(self.__deprecated__)
        factory, message = self._factory, self.__deprecated__

        @functools.wraps(factory)
        def call(*args, **kwargs):
            warn(message)
            return factory(cls, *args, **kwargs)

        call.__deprecated__ = message  # type: ignore[attr-defined]
        return call


class ValueCompat:
    """0.5 owner methods kept on the frozen value models until 0.7.

    In 0.5 these models wrapped native handles that needed ``close()`` or a
    ``with`` block. They are plain values now; both are no-ops that warn.
    """

    __slots__ = ()

    def close(self) -> None:
        warn(f"{type(self).__name__}.close() is deprecated; {type(self).__name__} is a plain value "
             "and needs no cleanup")

    close.__deprecated__ = "close() is deprecated; the value needs no cleanup"  # type: ignore[attr-defined]

    def __enter__(self):
        warn(f"using {type(self).__name__} in a with block is deprecated; "
             f"{type(self).__name__} is a plain value and needs no cleanup")
        return self

    def __exit__(self, *args) -> None:
        return None
