#!/usr/bin/env python3
"""Generate the raw Diplomat API and its Java-friendly owning facade."""

import argparse
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "bindings/kotlin/generated"
FFI = ROOT / "fatoora-ffi/src"

BIG_DECIMAL = "java.math.BigDecimal"

# Decimal text crosses the FFI as plain strings; the facade exposes BigDecimal.
# Parameters are keyed by (class, raw method) and must name every argument the
# FFI parses with `decimal(...)` (checked against fatoora-ffi in check_decimals).
DECIMAL_INPUTS = {
    ("Adjustment", "amount"): {"value"},
    ("Adjustment", "percentOf"): {"base", "percent"},
    ("Adjustment", "suppliedAmount"): {"value"},
    ("AllowanceCharge", "new_"): {"amount", "rate"},
    ("AllowanceCharge", "withPercentage"): {"base", "percent"},
    ("InvoiceBuilder", "addAllowance"): {"rate"},
    ("InvoiceBuilder", "addCharge"): {"rate"},
    ("InvoiceBuilder", "addLineItem"): {"quantity", "unitPrice", "vatRate"},
    ("InvoiceBuilder", "addTaxExemption"): {"rate"},
    ("InvoiceBuilder", "setPayableRoundingAmount"): {"amount"},
    ("InvoiceBuilder", "setPrepaidAmount"): {"amount"},
    ("InvoiceBuilder", "setTaxCurrency"): {"amount"},
    ("InvoiceLineItem", "new_"): {"quantity", "unitPrice", "rate"},
    ("LineItemBuilder", "setInvoicedQuantity"): {"value"},
    ("LineItemBuilder", "setItemGrossPrice"): {"value"},
    ("LineItemBuilder", "setItemNetPrice"): {"value"},
    ("LineItemBuilder", "setItemPriceBaseQuantity"): {"value"},
    ("LineItemBuilder", "setItemPriceDiscount"): {"value"},
    ("LineItemBuilder", "setLineAmountWithVat"): {"value"},
    ("LineItemBuilder", "setLineNetAmount"): {"value"},
    ("LineItemBuilder", "setLineVatAmount"): {"value"},
    ("LineItemBuilder", "setVatRate"): {"value"},
    ("VatBreakdown", "new_"): {"rate", "taxableAmount", "taxAmount"},
}
# Getters whose text is a core Decimal rendered with `to_string()`.
DECIMAL_OUTPUTS = {
    "Adjustment": {"baseAmount", "percentage", "value"},
    "AllowanceCharge": {"amount", "baseAmount", "percentage", "rate"},
    "InvoiceData": {"invoiceLevelCharge", "invoiceLevelDiscount", "taxCurrencyAmount"},
    "InvoiceLineItem": {
        "itemGrossPrice", "itemPriceBaseQuantity", "itemPriceDiscount", "quantity",
        "totalAmount", "unitPrice", "vatAmount", "vatRate",
    },
    "InvoiceTotals": {
        "allowanceTotal", "chargeTotal", "lineExtension", "payableAmount",
        "payableRoundingAmount", "prepaidAmount", "taxAmount", "taxInclusive",
        "taxableAmount",
    },
    "VatBreakdown": {"rate", "taxAmount", "taxableAmount"},
}
# Millisecond FFI arguments the facade takes as a java.time.Duration (plus a
# kotlin.time.Duration overload), keyed by (class, raw method):
# {raw parameter: facade parameter}. The KDoc replaces the FFI's millisecond one.
DURATION_INPUTS = {
    ("ZatcaClient", "withTimeout"): {"timeoutMs": "timeout"},
}
DURATION_DOCS = {
    ("ZatcaClient", "withTimeout"): """/**
 * Create a client whose requests fail with a network error once they take
 * longer than [timeout], from connecting until the response body is read.
 * The timeout is rounded up to whole milliseconds.
 *
 * @throws IllegalArgumentException if [timeout] is zero, negative, or longer
 *   than 4294967.295 seconds (`UInt.MAX_VALUE` milliseconds).
 */
""",
}
# Raw methods the facade exposes under an idiomatic name, with their KDoc,
# keyed by (class, raw method).
RENAMED_METHODS = {
    ("Bytes", "asSlice"): ("toByteArray", """/**
 * Returns a copy of the bytes. The array is independent of this owner and
 * stays valid after it is closed.
 */
"""),
}
# Zero-argument accessors with these prefixes are conversions or operations,
# not properties.
VERB_PREFIXES = ("to", "as", "into", "with", "build", "ensure")
VERBS = {"len", "isEmpty"}


def camel(name):
    head, *rest = name.split("_")
    return head + "".join(part.title() for part in rest)


def check_decimals():
    """Fail when the FFI parses a decimal argument the facade still types as String."""
    parsed = set()
    for path in FFI.glob("*.rs"):
        owner = method = None
        for line in path.read_text().splitlines():
            if m := re.match(r"\s*impl (\w+)", line):
                owner = m.group(1)
            if m := re.match(r"\s*pub fn (\w+)", line):
                method = "new_" if m.group(1) == "new" else camel(m.group(1))
            for argument in re.findall(r"decimal\(\s*(\w+)\s*,", line):
                if owner and method and owner != "ffi":
                    parsed.add((owner, method, camel(argument)))
    declared = {(o, m, a) for (o, m), args in DECIMAL_INPUTS.items() for a in args}
    missing = parsed - declared
    if missing:
        raise SystemExit(f"Decimal FFI arguments missing from DECIMAL_INPUTS: {sorted(missing)}")


def kdoc(content, position, indent):
    """Return the KDoc block that immediately precedes position, re-indented."""
    before = content[:position].rstrip()
    if not before.endswith("*/"):
        return ""
    start = before.rfind("/**")
    lines = [line.strip() for line in before[start:].splitlines()]
    body = [lines[0]] + [(" " + line if line.startswith("*") else " * " + line) for line in lines[1:]]
    return "\n".join(indent + line for line in body) + "\n"


def facade(sources):
    owners = {p.stem for p in sources if "internal val handle: Pointer" in sources[p]}
    public = owners - {"BindingError"}
    # Raw enums are mirrored in com.fatoora by name, so facade signatures never
    # expose com.fatoora.raw types.
    enums = {}
    for path, content in sources.items():
        match = re.search(r"^enum class (\w+)(?:\(val inner: Int\))? \{\n(.*?);", content, re.M | re.S)
        if match:
            enums[match.group(1)] = (
                re.findall(r"^    (\w+)(?:\(-?\d+\))?,?$", match.group(2), re.M),
                [
                    (m.group(1), m.group(2), kdoc(content, m.start(), "    "))
                    for m in re.finditer(r"^    fun (\w+)\(\): (\w+) \{", content, re.M)
                    if m.group(1) != "toNative"
                ],
                kdoc(content, match.start(), ""),
            )
    seen_inputs = set()
    seen_outputs = set()
    seen_durations = set()
    seen_renames = set()

    def managed(ty, decimal=False):
        if ty.endswith("?"):
            return managed(ty[:-1], decimal) + "?"
        if decimal:
            return BIG_DECIMAL
        return {
            "UByte": "Int",
            "UShort": "Int",
            "UInt": "Long",
            "ULong": "java.math.BigInteger",
            "UByteArray": "ByteArray",
        }.get(ty, ty)

    def argument(name, ty, decimal=False):
        if decimal:
            return f"{name}?.let(::decimalText)" if ty.endswith("?") else f"decimalText({name})"
        if ty.rstrip("?") in enums:
            raw = f"com.fatoora.raw.{ty.rstrip('?')}.valueOf"
            return f"{name}?.let {{ {raw}(it.name) }}" if ty.endswith("?") else f"{raw}({name}.name)"
        if ty.endswith("?") and managed(ty) != ty:
            return f"{name}?.let {{ value -> {argument('value', ty[:-1], decimal)} }}"
        if ty.rstrip("?") in public:
            return name + ("?.raw" if ty.endswith("?") else ".raw")
        return {
            "String": f"checkedText({name})",
            "String?": f"{name}?.let(::checkedText)",
            "UByte": f"checkedUnsigned({name}.toLong(), 255).toUByte()",
            "UShort": f"checkedUnsigned({name}.toLong(), 65535).toUShort()",
            "UInt": f"checkedUnsigned({name}, 4294967295L).toUInt()",
            "ULong": f"checkedCounter({name})",
            "UByteArray": f"{name}.asUByteArray()",
        }.get(ty, name)

    def result(expr, ty, decimal=False):
        if decimal:
            return f"{expr}?.let(::decimalOf)" if ty.endswith("?") else f"decimalOf({expr})"
        if ty.rstrip("?") in enums:
            value = f"{ty.rstrip('?')}.valueOf"
            return f"{expr}?.let {{ {value}(it.name) }}" if ty.endswith("?") else f"{value}({expr}.name)"
        if ty.endswith("?") and managed(ty) != ty:
            return f"{expr}?.let {{ value -> {result('value', ty[:-1], decimal)} }}"
        if ty.rstrip("?") in public:
            return f"{expr}?.let(::{ty[:-1]})" if ty.endswith("?") else f"{ty}({expr})"
        return {
            "UByte": f"{expr}.toInt()",
            "UShort": f"{expr}.toInt()",
            "UInt": f"{expr}.toLong()",
            "ULong": f"java.math.BigInteger({expr}.toString())",
            "UByteArray": f"{expr}.toByteArray()",
        }.get(ty, expr)

    def is_value(ty):
        # Copied values are safe to read as properties; owners stay explicit calls.
        return managed(ty).rstrip("?") in {
            "String", BIG_DECIMAL, "Boolean", "Int", "Long", "java.math.BigInteger",
        } | set(enums)

    def is_property(method, pairs, ty, is_static):
        return (
            not is_static
            and not pairs
            and ty != "Unit"
            and method not in VERBS
            and not re.match(rf"(?:{'|'.join(VERB_PREFIXES)})[A-Z]", method)
            and method not in VERB_PREFIXES
            and is_value(ty)
        )

    def getter_annotations(method):
        # Kotlin already maps isX to a Java isX() getter; keep hasX() natural too.
        if re.match(r"has[A-Z]", method):
            return [f'@get:JvmName("{method}")']
        return []

    output = {}
    for path, content in sources.items():
        name = path.stem
        if name not in public:
            continue
        properties = []
        methods = []
        statics = []
        members = list(re.finditer(
            r"^(    |        )fun (\w+)\(([^\n]*)\): ([^\n{]+) \{", content, re.M
        ))
        getters = {m.group(2): m.group(4).strip() for m in members if len(m.group(1)) == 4 and not m.group(3)}
        # `x_present`/`x` pairs: a fallible optional string the FFI cannot return
        # as Option<()>; the facade merges them into one nullable `x` and hides
        # `xPresent`.
        optional_pairs = {
            m for m, ret in getters.items()
            if ret == "Result<String>" and getters.get(m + "Present") in {"Boolean", "Result<Boolean>"}
        }
        hidden = {m + "Present" for m in optional_pairs}
        for match in members:
            indent, method, params, ret = match.groups()
            if method == "finalize" or method in hidden:
                continue
            is_static = len(indent) == 8
            doc = kdoc(content, match.start(), "")
            pairs = (
                [tuple(x.split(": ", 1)) for x in params.split(", ")] if params else []
            )
            ret = ret.strip()
            fallible = ret.startswith("Result<")
            ty = ret[7:-1] if fallible else ret
            decimal_args = DECIMAL_INPUTS.get((name, method), set())
            durations = DURATION_INPUTS.get((name, method), {})
            if durations:
                if {n: t for n, t in pairs if n in durations} != {n: "UInt" for n in durations}:
                    raise SystemExit(f"{name}.{method} has no UInt argument(s) {sorted(durations)}")
                seen_durations.add((name, method))
                doc = DURATION_DOCS[(name, method)]
            unknown = decimal_args - {n for n, _ in pairs}
            if unknown:
                raise SystemExit(f"{name}.{method} has no decimal argument(s) {sorted(unknown)}")
            if decimal_args:
                seen_inputs.add((name, method))
            decimal_out = method in DECIMAL_OUTPUTS.get(name, set()) and not pairs and not is_static
            if decimal_out:
                if ty.rstrip("?") != "String":
                    raise SystemExit(f"{name}.{method} returns {ty}, not decimal text")
                seen_outputs.add((name, method))
            # Default only trailing optional arguments, so Java overloads stay unambiguous.
            optional = len(pairs)
            while optional and pairs[optional - 1][1].endswith("?"):
                optional -= 1
            guards = ([] if is_static else ["this"]) + [
                n for n, t in pairs if t.rstrip("?") in public
            ]
            # Avoid Kotlin keyword escapes in Java source: new_ becomes create.
            exposed = "create" if method == "new_" else method
            if (name, method) in RENAMED_METHODS:
                exposed, doc = RENAMED_METHODS[(name, method)]
                seen_renames.add((name, method))

            decl = ", ".join(
                (
                    f"{durations[n]}: java.time.Duration" if n in durations
                    else f"{n}: {managed(t, n in decimal_args)}"
                ) + (" = null" if i >= optional else "")
                for i, (n, t) in enumerate(pairs)
            )
            args = ", ".join(
                f"timeoutMillis({durations[n]})" if n in durations
                else argument(n, t, n in decimal_args)
                for n, t in pairs
            )
            call = f"{'com.fatoora.raw.' + name if is_static else 'raw'}.{method}({args})"
            if fallible:
                call = f"unwrap({call})"
            returns = managed(ty, decimal_out)
            value = result(call, ty, decimal_out)
            if method in optional_pairs:
                has = method + "Present"
                present = f"raw.{has}()" if getters[has] == "Boolean" else f"unwrap(raw.{has}())"
                ty, returns = ty + "?", returns + "?"
                value = f"if ({present}) {value} else null"
            body = f"nativeCall(listOfNotNull({', '.join(guards)})) {{\n    {value}\n}}"
            if is_property(method, pairs, ty, is_static):
                lines = getter_annotations(method)
                lines.append(f"val {method}: {returns}")
                lines.append(f"    get() = {body.replace(chr(10), chr(10) + '    ')}")
                properties.append(doc + "\n".join(lines))
                continue
            lines = []
            if is_static:
                lines.append("@JvmStatic")
            if optional < len(pairs):
                lines.append("@JvmOverloads")
            lines.append(f"fun {exposed}({decl}): {returns} = {body}")
            (statics if is_static else methods).append(doc + "\n".join(lines))
            if durations:
                # Kotlin-only overload: value-class parameters mangle the JVM name.
                kotlin_decl = decl
                for facade_name in durations.values():
                    kotlin_decl = kotlin_decl.replace(
                        f"{facade_name}: java.time.Duration", f"{facade_name}: kotlin.time.Duration"
                    )
                forwarded = ", ".join(
                    f"javaDuration({durations[n]})" if n in durations else n for n, _ in pairs
                )
                (statics if is_static else methods).append(
                    f"/** [{exposed}] taking a [kotlin.time.Duration]. */\n"
                    f"@JvmSynthetic\nfun {exposed}({kotlin_decl}): {returns} = {exposed}({forwarded})"
                )
        text = f"""// Generated by scripts/generate_kotlin.py; do not edit.
@file:OptIn(ExperimentalUnsignedTypes::class)
package com.fatoora

{kdoc(content, content.index(f"class {name} internal constructor"), "")}class {name} internal constructor(internal val raw: com.fatoora.raw.{name}) : NativeResource() {{
    init {{
        check(raw.owned && raw.selfEdges.isEmpty()) {{ "Facade requires an independent owned handle" }}
        raw.owned = false
        val handle = raw.handle
        attach {{ com.fatoora.raw.{name}.lib.fatoora_{name}_destroy(handle) }}
    }}
"""
        if statics:
            text += (
                "    companion object {\n"
                + "\n\n".join(
                    "        " + m.replace("\n", "\n        ") for m in statics
                )
                + "\n    }\n"
            )
        members = properties + methods
        if members:
            text += "\n" + "\n\n".join("    " + m.replace("\n", "\n    ") for m in members) + "\n"
        text += "}\n"
        output[Path("facade/com/fatoora") / f"{name}.kt"] = text.encode()
    missing_inputs = set(DECIMAL_INPUTS) - seen_inputs
    missing_outputs = {(o, m) for o, ms in DECIMAL_OUTPUTS.items() for m in ms} - seen_outputs
    missing_durations = set(DURATION_INPUTS) - seen_durations
    missing_renames = set(RENAMED_METHODS) - seen_renames
    if missing_renames:
        raise SystemExit(f"renamed raw methods not found: {sorted(missing_renames)}")
    if missing_durations:
        raise SystemExit(f"Unknown duration methods: {sorted(missing_durations)}")
    if missing_inputs or missing_outputs:
        raise SystemExit(f"Unknown decimal methods: {sorted(missing_inputs | missing_outputs)}")
    for name, (variants, enum_methods, doc) in sorted(enums.items()):
        body = ", ".join(variants)
        if enum_methods:
            members = []
            for method, ty, method_doc in enum_methods:
                expression = result(f"com.fatoora.raw.{name}.valueOf(name).{method}()", ty)
                members.append(
                    f"{method_doc}    val {method}: {managed(ty)}\n        get() = {expression}"
                )
            body += ";\n\n" + "\n\n".join(members) + "\n"
            text = f"{doc}enum class {name} {{\n    {body}}}\n"
        else:
            text = f"{doc}enum class {name} {{ {body} }}\n"
        output[Path("facade/com/fatoora") / f"{name}.kt"] = (
            "// Generated by scripts/generate_kotlin.py; do not edit.\npackage com.fatoora\n\n" + text
        ).encode()
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    check_decimals()
    tool = os.environ.get(
        "DIPLOMAT_TOOL_KOTLIN",
        str(
            ROOT
            / ".tools/diplomat-kotlin/bin"
            / ("diplomat-tool.exe" if os.name == "nt" else "diplomat-tool")
        ),
    )
    with tempfile.TemporaryDirectory() as directory:
        subprocess.run(
            [
                tool,
                "--entry",
                "fatoora-ffi/src/lib.rs",
                "--config",
                "lib_name=raw",
                "--config",
                "kotlin.domain=com.fatoora",
                "--config",
                "kotlin.dylib_name=fatoora_ffi",
                "--config",
                "kotlin.use_finalizers_not_cleaners=true",
                "kotlin",
                directory,
            ],
            cwd=ROOT,
            check=True,
        )
        raw = {
            p: p.read_text()
            for p in (Path(directory) / "src/main/kotlin").rglob("*.kt")
        }
        if not raw:
            raise SystemExit("Kotlin generator produced no sources")
        sources = {
            Path("raw/com/fatoora/raw") / p.name: ("\n".join(line.rstrip() for line in s.splitlines()).rstrip() + "\n").encode()
            for p, s in raw.items()
        }
        sources.update(facade(raw))
        existing = {p.relative_to(OUTPUT): p.read_bytes() for p in OUTPUT.rglob("*.kt")}
        if args.check:
            if sources != existing:
                raise SystemExit(
                    "Stale Kotlin bindings; run scripts/generate_kotlin.py"
                )
        else:
            for obsolete in existing.keys() - sources.keys():
                (OUTPUT / obsolete).unlink()
            for name, content in sources.items():
                (OUTPUT / name).parent.mkdir(parents=True, exist_ok=True)
                (OUTPUT / name).write_bytes(content)


if __name__ == "__main__":
    main()
