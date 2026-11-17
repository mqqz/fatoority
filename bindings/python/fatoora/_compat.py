"""Deprecation helpers for the 0.6 transition. Everything here is removed in 0.7."""
from __future__ import annotations

import dis
import functools
import sys
import warnings
from typing import TYPE_CHECKING, Any, Callable, Optional, TypeVar

T = TypeVar("T")

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


# --- getter -> property transition -------------------------------------------------
#
# 0.5 exposed getters as methods (``invoice.id()``); 0.6 makes them properties
# (``invoice.id``). One name cannot be both, so ``getter`` (a ``property``
# subclass) looks at the instruction in the caller that is reading the attribute:
#
# * ``obj.name(...)`` compiles to a *method load*: ``LOAD_METHOD`` on 3.9-3.11,
#   ``LOAD_ATTR`` with the low oparg bit set on 3.12+. For that read only, the
#   descriptor returns a zero-argument callable that warns and returns the 0.5
#   value. pytest's assertion rewriting splits ``assert obj.name() == x`` into
#   ``@py_assert1 = obj.name; @py_assert2 = @py_assert1()``; that exact shape
#   (a store into a pytest ``@py_assert*`` temporary that is immediately called
#   with no arguments) is recognised too.
# * Every other read -- ``obj.name``, ``getattr(obj, "name")``, ``attrgetter`` --
#   returns the plain value, exactly what the property returns after 0.7,
#   ``None`` included. New code never sees a wrapper type.
#
# The check reads the caller's un-specialised bytecode (``co_code``), so it is
# deterministic, and it is verified once at import time. When it cannot be
# verified (another interpreter, a future bytecode change) the descriptor always
# returns the plain value, i.e. the 0.7 behaviour. The 0.5 spelling it cannot
# serve is keeping the bound getter to call later (``f = obj.name; f()``).

_PYTEST_TEMP = "@py_assert"  # '@' cannot appear in a user-written identifier
_SKIP = frozenset({"CACHE", "EXTENDED_ARG", "NOP", "PUSH_NULL", "PRECALL"})
_CALLS = frozenset({"CALL", "CALL_FUNCTION", "CALL_METHOD"})


@functools.lru_cache(maxsize=1024)
def _pytest_temp_call(code, offset: int) -> bool:
    """``LOAD_ATTR`` at ``offset`` feeds ``@py_assertN = <attr>; @py_assertN()``."""
    if not any(name.startswith(_PYTEST_TEMP) for name in code.co_varnames + code.co_names):
        return False
    instructions = [i for i in dis.get_instructions(code) if i.opname not in _SKIP]
    for index, instruction in enumerate(instructions):
        if instruction.offset == offset:
            break
    else:
        return False
    rest = instructions[index + 1:index + 4]
    if not rest:
        return False
    store = rest.pop(0)
    if store.opname == "STORE_FAST_LOAD_FAST":  # 3.13+ superinstruction
        names = store.argval
        if not (isinstance(names, tuple) and names[0] == names[1]):
            return False
        name = names[0]
    elif store.opname.startswith("STORE_") and rest and rest[0].opname.startswith("LOAD_") \
            and rest[0].argval == store.argval:
        name = store.argval
        rest.pop(0)
    else:
        return False
    if not (isinstance(name, str) and name.startswith(_PYTEST_TEMP)) or not rest:
        return False
    return rest[0].opname in _CALLS and rest[0].arg == 0


def _method_load_checker() -> Optional[Callable[[Any], bool]]:
    if sys.implementation.name != "cpython":
        return None
    if sys.version_info < (3, 12):
        method_op = dis.opmap.get("LOAD_METHOD")
        attr_op = dis.opmap.get("LOAD_ATTR")
        if method_op is None:
            return None

        def check(frame) -> bool:
            code = frame.f_code
            index = frame.f_lasti
            op = code.co_code[index]
            return op == method_op or (op == attr_op and _pytest_temp_call(code, index))
    else:
        attr_op = dis.opmap.get("LOAD_ATTR")
        if attr_op is None:
            return None

        def check(frame) -> bool:
            code = frame.f_code
            raw = code.co_code
            index = frame.f_lasti
            if raw[index] != attr_op:
                return False
            return bool(raw[index + 1] & 1) or _pytest_temp_call(code, index)

    def guarded(frame) -> bool:
        try:
            return check(frame)
        except Exception:
            return False
    return guarded


_is_method_load = _method_load_checker()


class getter(property):
    """Read-only property that still answers the 0.5 call form until 0.7."""

    def __init__(self, fget: Callable, legacy: Optional[Callable] = None) -> None:
        super().__init__(fget, None, None, fget.__doc__)
        self._legacy = legacy
        self._name = fget.__name__

    def __set_name__(self, owner, name) -> None:
        self._name = name

    def __get__(self, obj, owner=None):
        if obj is None:
            return self
        if _is_method_load is not None and _is_method_load(sys._getframe(1)):
            value = (self._legacy or self.fget)(obj)
            message = (f"calling {type(obj).__name__}.{self._name}() is deprecated; "
                       f"read the {self._name!r} property instead")

            def legacy_call():
                warn(message)
                return value
            return legacy_call
        return self.fget(obj)


def _prop(legacy: Optional[Callable] = None) -> Callable[[Callable], getter]:
    """``@prop()`` / ``@prop(legacy=fn)``: build a :class:`getter`.

    ``legacy`` computes the 0.5 return value (for example a ``str`` where the
    property now returns ``datetime``) for the deprecated call form.
    """
    def decorator(function):
        return getter(function, legacy)
    return decorator


if TYPE_CHECKING:
    # Type checkers see a plain read-only attribute of the getter's return type.
    def prop(legacy: Optional[Callable[[Any], Any]] = None) -> Callable[[Callable[[Any], T]], T]: ...
else:
    prop = _prop


def _verify_method_load_checker() -> bool:
    class _Probe:
        value = getter(lambda self: 1)

    probe = _Probe()
    try:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            called = probe.value()
        return probe.value == 1 and called == 1
    except Exception:
        return False


if _is_method_load is not None and not _verify_method_load_checker():
    _is_method_load = None


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
