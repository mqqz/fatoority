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
                [m for m in re.findall(r"^    fun (\w+)\(\): (\w+) \{", content, re.M) if m[0] != "toNative"],
            )

    def managed(ty):
        if ty.endswith("?"):
            return managed(ty[:-1]) + "?"
        return {
            "UByte": "Int",
            "UShort": "Int",
            "UInt": "Long",
            "ULong": "java.math.BigInteger",
            "UByteArray": "ByteArray",
        }.get(ty, ty)

    def argument(name, ty):
        if ty.rstrip("?") in enums:
            raw = f"com.fatoora.raw.{ty.rstrip('?')}.valueOf"
            return f"{name}?.let {{ {raw}(it.name) }}" if ty.endswith("?") else f"{raw}({name}.name)"
        if ty.endswith("?") and managed(ty) != ty:
            return f"{name}?.let {{ value -> {argument('value', ty[:-1])} }}"
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

    def result(expr, ty):
        if ty.rstrip("?") in enums:
            value = f"{ty.rstrip('?')}.valueOf"
            return f"{expr}?.let {{ {value}(it.name) }}" if ty.endswith("?") else f"{value}({expr}.name)"
        if ty.endswith("?") and managed(ty) != ty:
            return f"{expr}?.let {{ value -> {result('value', ty[:-1])} }}"
        if ty.rstrip("?") in public:
            return f"{expr}?.let(::{ty[:-1]})" if ty.endswith("?") else f"{ty}({expr})"
        return {
            "UByte": f"{expr}.toInt()",
            "UShort": f"{expr}.toInt()",
            "UInt": f"{expr}.toLong()",
            "ULong": f"java.math.BigInteger({expr}.toString())",
            "UByteArray": f"{expr}.toByteArray()",
        }.get(ty, expr)

    output = {}
    for path, content in sources.items():
        name = path.stem
        if name not in public:
            continue
        methods = []
        statics = []
        for match in re.finditer(
            r"^(    |        )fun (\w+)\(([^\n]*)\): ([^\n{]+) \{", content, re.M
        ):
            indent, method, params, ret = match.groups()
            if method == "finalize":
                continue
            is_static = len(indent) == 8
            pairs = (
                [tuple(x.split(": ", 1)) for x in params.split(", ")] if params else []
            )
            ret = ret.strip()
            fallible = ret.startswith("Result<")
            ty = ret[7:-1] if fallible else ret
            # Default only trailing optional arguments, so Java overloads stay unambiguous.
            optional = len(pairs)
            while optional and pairs[optional - 1][1].endswith("?"):
                optional -= 1
            decl = ", ".join(
                f"{n}: {managed(t)}" + (" = null" if i >= optional else "")
                for i, (n, t) in enumerate(pairs)
            )
            guards = ([] if is_static else ["this"]) + [
                n for n, t in pairs if t.rstrip("?") in public
            ]
            args = ", ".join(argument(n, t) for n, t in pairs)
            call = (
                f"{'com.fatoora.raw.' + name if is_static else 'raw'}.{method}({args})"
            )
            if fallible:
                call = f"unwrap({call})"
            expression = result(call, ty)
            lines = []
            if is_static:
                lines.append("@JvmStatic")
            if optional < len(pairs):
                lines.append("@JvmOverloads")
            # Avoid Kotlin keyword escapes in Java source: new_ becomes create.
            exposed = "create" if method == "new_" else method
            lines.append(
                f"fun {exposed}({decl}): {managed(ty)} = nativeCall(listOfNotNull({', '.join(guards)})) {{"
            )
            lines.append(f"    {expression}")
            lines.append("}")
            (statics if is_static else methods).append("\n".join(lines))
        text = f"""// Generated by scripts/generate_kotlin.py; do not edit.
@file:OptIn(ExperimentalUnsignedTypes::class)
package com.fatoora

class {name} internal constructor(internal val raw: com.fatoora.raw.{name}) : NativeResource() {{
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
        text += (
            "\n\n".join("    " + m.replace("\n", "\n    ") for m in methods) + "\n}\n"
        )
        output[Path("facade/com/fatoora") / f"{name}.kt"] = text.encode()
    for name, (variants, enum_methods) in sorted(enums.items()):
        body = ", ".join(variants)
        if enum_methods:
            body += ";\n\n" + "\n\n".join(
                f"    fun {method}(): {managed(ty)} =\n        "
                + result(f"com.fatoora.raw.{name}.valueOf(name).{method}()", ty)
                for method, ty in enum_methods
            ) + "\n"
            text = f"enum class {name} {{\n    {body}}}\n"
        else:
            text = f"enum class {name} {{ {body} }}\n"
        output[Path("facade/com/fatoora") / f"{name}.kt"] = (
            "// Generated by scripts/generate_kotlin.py; do not edit.\npackage com.fatoora\n\n" + text
        ).encode()
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
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
