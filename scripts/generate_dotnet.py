#!/usr/bin/env python3
"""Generate the C# bindings: raw Diplomat output plus the idiomatic facade pass.

The Diplomat C# fork (mqqz/diplomat, see the dotnet README) emits one managed
wrapper class per FFI opaque. Those wrappers already own the native handle,
implement ``IDisposable`` and track borrow lifetimes, so instead of wrapping
them a second time this script rewrites the generated wrappers in place:

* value getters (zero-argument instance methods returning a string, number,
  bool, enum or optional ``Text``) become read-only properties;
  methods returning native-owned ``IDisposable`` snapshots stay methods so
  ownership (``using``) remains explicit, as do ``To*``/``Into*`` conversions;
* optional ``Text`` results are copied into ``string`` and released at once;
* every decimal the Rust FFI parses (``decimal(param, ...)``) or returns
  (a ``fatoora_core`` getter typed ``Decimal``) is ``decimal`` in C#,
  converted with the invariant culture (never through ``double``);
* native errors are thrown as ``FfiException`` subclasses (see
  ``bindings/dotnet/src/Errors.cs``) and ``BindingError`` becomes internal;
* the Rust ``new`` constructors become ``Create`` factory methods, matching
  the Kotlin and JavaScript packages; ``New`` and the members in ``RENAMED``
  keep ``[Obsolete]`` aliases until 0.7.

Decimal members are derived from the Rust sources, so new FFI members are
picked up without editing this script.
"""
import argparse
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
FFI = ROOT / "fatoora-ffi/src"
CORE = ROOT / "fatoora-core/src"

VALUE_TYPES = {"string", "bool", "byte", "ushort", "int", "uint", "ulong", "nuint"}
METHOD_PREFIXES = ("To", "Into", "As")
SIGNATURE = re.compile(
    r"^    public (?P<static>static )?(?P<ret>[\w<>\[\]?]+) (?P<name>\w+)\((?P<params>.*)\)$")
THROW = "throw new BindingErrorException(new BindingError(result.Err));"
# FFI members renamed in 0.6 to match the core: (class, new member) -> old name.
# Adjustment.Percentage (the 0.5 factory, now PercentOf) cannot keep an alias
# because the Percentage property now uses that name.
RENAMED = {
    ("Adjustment", "SuppliedAmount"): "WithSuppliedAmount",
    ("Adjustment", "Percentage"): "Percent",
    ("AllowanceCharge", "OmitVatRate"): "WithoutVatRate",
    ("VatBreakdown", "OmitVatRate"): "WithoutVatRate",
}
# Classes whose constructor was already called Create in 0.5, so get no New alias.
CREATED = {"ComplianceCsid", "ProductionCsid", "ZatcaClient"}


def pascal(snake):
    return "".join(part[:1].upper() + part[1:] for part in snake.split("_"))


def camel(snake):
    value = pascal(snake)
    value = value[:1].lower() + value[1:]
    return "@" + value if value in {"base", "string", "object", "params"} else value


def rust_functions():
    """Yield (opaque, function, params, body, bridged) for every method in fatoora-ffi."""
    for path in sorted(FFI.glob("*.rs")):
        current = None
        functions = []
        for line in path.read_text().splitlines():
            impl = re.match(r"^\s*impl (?:ffi::)?(\w+) \{", line)
            if impl:
                current = impl.group(1)
                continue
            function = re.match(r"^(\s*)(pub )?fn (\w+)\(", line)
            if function and current:
                bridged = function.group(1) == " " * 8 and bool(function.group(2))
                functions.append([current, function.group(3), bridged, []])
            elif re.match(r"^(    )?\}", line):
                current = None
            if functions and current:
                functions[-1][3].append(line)
        for opaque, name, bridged, lines in functions:
            body = "\n".join(lines)
            signature = body.split("{", 1)[0]
            arguments = signature[signature.index("(") + 1:signature.rindex(")")]
            params = re.findall(r"(?:^|,)\s*(?:mut )?(\w+)\s*:", arguments)
            yield opaque, name, params, body, bridged


def decimal_members():
    """Return (inputs, outputs) as {(Class, Method): {param}} and {(Class, Method)}."""
    core_getters = set(re.findall(
        r"pub fn (\w+)\(&self\) -> (?:Option<|Result<)?Decimal\b",
        "\n".join(p.read_text() for p in CORE.rglob("*.rs"))))
    functions = list(rust_functions())
    decimals = {(o, n): set(re.findall(r"\bdecimal\(\s*(\w+)\s*,", b)) for o, n, _, b, _ in functions}
    params = {(o, n): p for o, n, p, _, _ in functions}
    changed = True
    while changed:  # Follow parameters forwarded to private helpers.
        changed = False
        for opaque, name, _, body, _ in functions:
            for callee, args in re.findall(r"\bSelf::(\w+)\(([^()]*)\)", body):
                callee_params = params.get((opaque, callee), [])
                for arg, param in zip((a.strip() for a in args.split(",")), callee_params):
                    if param in decimals[(opaque, callee)] and arg not in decimals[(opaque, name)]:
                        decimals[(opaque, name)].add(arg)
                        changed = True
    inputs, outputs = {}, set()
    for opaque, name, own, body, bridged in functions:
        if not bridged:
            continue
        key = (opaque, pascal(name))
        found = decimals[(opaque, name)] & set(own)
        if found:
            inputs[key] = {camel(p) for p in found}
        signature = body.split("{", 1)[0]
        if "&self" in signature and re.search(r"(DiplomatWrite|Option<Box<Text>>)", signature):
            called = set(re.findall(r"\.(\w+)\(\)", body))
            if called & core_getters:
                outputs.add(key)
    return inputs, outputs


def split_params(params):
    return [p.strip() for p in params.split(",")] if params.strip() else []


def transform_class(name, text, inputs, outputs, enums, renamed):
    lines = text.split("\n")
    result = []
    index = 0
    seen = set()
    while index < len(lines):
        line = lines[index]
        match = SIGNATURE.match(line)
        if not match or match["name"] in {name, "Dispose"}:
            result.append(line)
            index += 1
            continue
        end = lines.index("    }", index)
        body = lines[index + 2:end]
        method, ret, params = match["name"], match["ret"], split_params(match["params"])
        key = (name, method)
        seen.add(key)

        # Decimal inputs.
        for param in inputs.get(key, ()):
            position = next((i for i, p in enumerate(params) if p.split()[-1] == param), None)
            if position is None or params[position].split()[0] not in {"string", "string?"}:
                raise SystemExit(f"{name}.{method}: decimal parameter {param} not found")
            params[position] = params[position].replace("string", "decimal", 1)
            body = [b for b in body if f"throw new ArgumentNullException(nameof({param}))" not in b]
            body = [b.replace(f"Diplomat.Utf8.Clone({param})", f"Diplomat.Utf8.Clone(FatooraDecimal.Format({param}))")
                    for b in body]
            if not any(f"FatooraDecimal.Format({param})" in b for b in body):
                raise SystemExit(f"{name}.{method}: unexpected conversion of {param}")

        # Text results become strings; decimal results become decimals.
        is_decimal = key in outputs
        if is_decimal and ret not in {"string", "Text", "Text?"}:
            raise SystemExit(f"{name}.{method}: decimal result has unexpected type {ret}")
        if ret in {"Text", "Text?"}:
            helper = ("TakeDecimal" if is_decimal else "Take") + ("" if ret == "Text" else "Optional")
            ret = ("decimal" if is_decimal else "string") + ("?" if ret == "Text?" else "")
            body = [re.sub(r"return (.*new )Text\((.*)\);$",
                           rf"return global::Fatoora.Text.{helper}(\1global::Fatoora.Text(\2));", b)
                    for b in body]
            text_doc = ["/// <returns>", "/// A <c>Text</c> allocated on Rust side.", "/// </returns>"]
            if [r.strip() for r in result[-3:]] == text_doc:
                del result[-3:]
        elif is_decimal:
            ret = "decimal"
            body = [b.replace("return writeable.ToUnicode();",
                              "return FatooraDecimal.Parse(writeable.ToUnicode());") for b in body]
        if match["ret"] != ret and not any("FatooraDecimal.Parse(" in b or ".Text.Take" in b for b in body):
            raise SystemExit(f"{name}.{method}: unexpected result conversion")
        if is_decimal and not any("FatooraDecimal.Parse(" in b or ".TakeDecimal" in b for b in body):
            raise SystemExit(f"{name}.{method}: unexpected decimal result conversion")

        exposed, old = method, RENAMED.get((name, method))
        if old:
            renamed.add((name, method))
        if match["static"] and method == "New":
            exposed, old = "Create", None if name in CREATED else "New"
        base = ret.rstrip("?")
        is_property = (not match["static"] and not params and ret != "void"
                       and not any(method.startswith(p) and method[len(p):][:1].isupper()
                                   for p in METHOD_PREFIXES)
                       and (base in VALUE_TYPES or base == "decimal" or base in enums))
        if is_property:
            if method == name:
                raise SystemExit(f"{name}.{method}: property would share its class name")
            result.append(f"    public {ret} {exposed}")
            result.append("    {")
            result.append("        get")
            result.append("        {")
            result.extend(("    " + b) if b else b for b in body)
            result.append("        }")
            result.append("    }")
            if old:
                result.append("")
                result.append(f'    [Obsolete("Use {exposed}; {old} will be removed in 0.7.")]')
                result.append(f"    public {ret} {old} => {exposed};")
        else:
            result.append(f"    public {match['static'] or ''}{ret} {exposed}({', '.join(params)})")
            result.append("    {")
            result.extend(body)
            result.append("    }")
            if old:
                names = ", ".join(p.split()[-1] for p in params)
                result.append("")
                result.append(f'    [Obsolete("Use {exposed}; {old} will be removed in 0.7.")]')
                result.append(f"    public {match['static'] or ''}{ret} {old}({', '.join(params)}) => {exposed}({names});")
        index = end + 1
    text = "\n".join(result)
    text = text.replace(THROW, "throw FfiException.FromNative(new BindingError(result.Err));")
    text = text.replace('/// <exception cref="BindingErrorException"></exception>',
                        '/// <exception cref="FfiException"></exception>')
    return text, seen


def transform(sources):
    """Apply the facade pass to the raw generator output (name -> text)."""
    inputs, outputs = decimal_members()
    enums = {n[:-3] for n, t in sources.items() if re.search(r"^public enum ", t, re.M)}
    sources = dict(sources)
    del sources["BindingErrorException.cs"]
    seen = set()
    renamed = set()
    for file, text in sources.items():
        if file.startswith(("Raw", "Diplomat")) or "\npublic partial class " not in text:
            continue
        name = file[:-3]
        text, members = transform_class(name, text, inputs, outputs, enums, renamed)
        seen |= members
        if name == "BindingError":
            text = text.replace("public partial class BindingError", "internal partial class BindingError")
        sources[file] = text
    missing = (set(inputs) | outputs) - seen
    if missing:
        raise SystemExit(f"Decimal members missing from C# output: {sorted(missing)}")
    if set(RENAMED) - renamed:
        raise SystemExit(f"Renamed members missing from C# output: {sorted(set(RENAMED) - renamed)}")
    if any("BindingErrorException" in t for t in sources.values()):
        raise SystemExit("Unrewritten BindingErrorException reference")
    return {name: text.encode() for name, text in sources.items()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    output = ROOT / "bindings/dotnet/generated"
    with tempfile.TemporaryDirectory() as temporary:
        subprocess.run([
            os.environ.get("DIPLOMAT_TOOL_DOTNET", str(ROOT / ".tools/diplomat-fork/bin" /
                ("diplomat-tool.exe" if os.name == "nt" else "diplomat-tool"))),
            "--entry", "fatoora-ffi/src/lib.rs",
            "--config", "lib_name=fatoora_ffi",
            "--config", "dotnet.namespace=Fatoora",
            "--config", "dotnet.diplomat_str_as_string=true",
            "--config", "dotnet.exception_message_method=Message",
            "dotnet", temporary,
        ], cwd=ROOT, check=True)
        raw = {p.name: p.read_bytes().decode() for p in Path(temporary).glob("*.cs")}
        if not raw:
            raise SystemExit("Generator produced no C# sources")
        sources = transform(raw)
        existing = {p.name: p.read_bytes() for p in output.glob("*.cs")}
        if args.check:
            if sources != existing:
                raise SystemExit("Stale C# bindings; run scripts/generate_dotnet.py")
        else:
            output.mkdir(parents=True, exist_ok=True)
            for obsolete in existing.keys() - sources.keys():
                (output / obsolete).unlink()
            for name, content in sources.items():
                (output / name).write_bytes(content)


if __name__ == "__main__":
    main()
