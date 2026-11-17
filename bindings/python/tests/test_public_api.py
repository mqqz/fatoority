"""The public Python API contract, recorded in ``public_api.json``.

Regenerate the contract after an intentional API change with::

    python bindings/python/tests/test_public_api.py --update

and review the JSON diff. For each exported name the contract records:

* classes: the public constructor (``__init__``) and each public member as ``"property"``, ``"field"`` (dataclass),
  ``"member"`` (enum) or the parameter list of a method, where ``"*"`` marks
  the start of keyword-only parameters;
* functions: the parameter list;
* ``__deprecated__``: every name that still works but warns (removed in 0.7),
  including the class aliases served by the module ``__getattr__``.
"""
from __future__ import annotations

import dataclasses
import enum
import inspect
import json
from pathlib import Path
import sys

CONTRACT = Path(__file__).with_name("public_api.json")


def _parameters(function) -> list[str]:
    result = []
    keyword_only = False
    for name, parameter in inspect.signature(function).parameters.items():
        if name in ("self", "cls"):
            continue
        if parameter.kind is inspect.Parameter.KEYWORD_ONLY and not keyword_only:
            result.append("*")
            keyword_only = True
        if parameter.kind is inspect.Parameter.VAR_POSITIONAL:
            name, keyword_only = "*" + name, True
        elif parameter.kind is inspect.Parameter.VAR_KEYWORD:
            name = "**" + name
        result.append(name)
    return result


def _is_deprecated(value) -> bool:
    return hasattr(value, "__deprecated__")


def _class_contract(cls, deprecated: list[str]) -> dict:
    members: dict[str, object] = {}
    if issubclass(cls, enum.Enum):
        return {name: "member" for name in cls.__members__}
    if dataclasses.is_dataclass(cls):
        members.update({f.name: "field" for f in dataclasses.fields(cls) if not f.name.startswith("_")})
    # Only members defined by the package (not by object, int, Enum, ...).
    owners = [k for k in reversed(cls.__mro__) if k.__module__.startswith("fatoora")]
    for owner in owners:
        for name, raw in vars(owner).items():
            if name == "__init__" and owner.__name__ != "_Owned":
                members[name] = _parameters(raw)  # public constructor
                continue
            if name.startswith("_") or name in members:
                continue
            value = raw.__func__ if isinstance(raw, (classmethod, staticmethod)) else raw
            if isinstance(value, property):
                members[name] = "property"
            elif callable(value):
                members[name] = _parameters(value)
                if _is_deprecated(value):
                    deprecated.append(f"{cls.__name__}.{name}")
    return dict(sorted(members.items()))


def generate() -> dict:
    import fatoora
    from fatoora import api

    deprecated: list[str] = []
    contract: dict[str, object] = {}
    for name in sorted(fatoora.__all__):
        value = getattr(fatoora, name)
        if inspect.isclass(value):
            if value.__module__ == "fatoora.errors":
                contract[name] = {}
                continue
            contract[name] = _class_contract(value, deprecated)
        else:
            contract[name] = _parameters(value)
            if _is_deprecated(value):
                deprecated.append(name)
    deprecated.extend(sorted(api.DEPRECATED_ALIASES))
    contract["__deprecated__"] = sorted(set(deprecated))
    return contract


def test_public_api_matches_contract():
    expected = json.loads(CONTRACT.read_text())
    actual = generate()
    assert actual == expected, (
        "public API changed; if intentional run "
        "`python bindings/python/tests/test_public_api.py --update` and review the diff")


def test_contract_names_resolve_to_one_object():
    import fatoora
    from fatoora import api

    for name in fatoora.__all__:
        value = getattr(fatoora, name)
        if getattr(value, "__module__", "") == "fatoora.api":
            assert getattr(api, name) is value


def test_deprecated_aliases_resolve_to_the_new_classes():
    import warnings
    import fatoora
    from fatoora import api

    for alias, (replacement, _) in api.DEPRECATED_ALIASES.items():
        assert alias not in fatoora.__all__
        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            assert getattr(fatoora, alias) is getattr(fatoora, replacement)
            assert getattr(api, alias) is getattr(fatoora, replacement)
        assert [w.category for w in caught] == [DeprecationWarning, DeprecationWarning]


def test_distribution_has_no_legacy_loader_or_runtime_dependency():
    from importlib.metadata import distribution
    import fatoora

    dist = distribution("fatoority")
    files = {str(p) for p in dist.files}
    assert "fatoora/_lib.py" not in files
    assert "fatoora/native.py" not in files
    assert "fatoora/fatoora_ffi.h" not in files
    assert not any(requirement.lower().startswith("cffi") for requirement in dist.requires or [])
    assert not hasattr(fatoora, "FfiLibrary")


def test_public_function_annotations_resolve():
    from typing import get_type_hints
    from fatoora import api

    for name, function in inspect.getmembers(api, inspect.isfunction):
        if not name.startswith("_") and function.__module__ == api.__name__:
            get_type_hints(function)
    for name, cls in inspect.getmembers(api, inspect.isclass):
        if cls.__module__ != api.__name__:
            continue
        for member in vars(cls).values():
            target = member.fget if isinstance(member, property) else getattr(member, "__func__", member)
            if inspect.isfunction(target):
                get_type_hints(target, vars(api))


if __name__ == "__main__":
    if sys.argv[1:] != ["--update"]:
        sys.exit("usage: test_public_api.py --update")
    CONTRACT.write_text(json.dumps(generate(), indent=2, ensure_ascii=False) + "\n")
    print(f"wrote {CONTRACT}")
