#!/usr/bin/env python3
"""Generate the checked-in bindings with diplomat-tool 0.17.0."""
import os
import re
import shutil
import argparse
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Fail if checked-in bindings differ")
    args = parser.parse_args()
    tool = os.environ.get("DIPLOMAT_TOOL", "diplomat-tool")
    for backend, output in (
        ("js", "bindings/js/generated"),
        ("c", "bindings/c/generated"),
        ("cpp", "bindings/cpp"),
        ("nanobind", "bindings/python/native/generated"),
    ):
        with tempfile.TemporaryDirectory() as generated:
            subprocess.run([
                tool, "--entry", "fatoora-ffi/src/lib.rs",
                *(["--config", "lib_name=_native", "--config", "custom_extra_code_location=fatoora-ffi/src"] if backend == "nanobind" else []),
                backend, generated,
            ], cwd=ROOT, check=True)
            if backend == "js":
                # Diplomat's default loader has no custom host-import hook.
                (Path(generated) / "diplomat-wasm.mjs").write_bytes(
                    (ROOT / "scripts/js-wasm.mjs").read_bytes())
                # ESM declarations must resolve under NodeNext as well as bundlers.
                for declaration in Path(generated).glob("*.d.ts"):
                    content = re.sub(r'"(\./[^"\n]+?)"',
                        lambda m: '"' + m[1].removesuffix('.d.ts') + '.mjs"', declaration.read_text())
                    content = content.replace("'./diplomat-runtime'", "'./diplomat-runtime.mjs'")
                    declaration.with_suffix('.mts').write_text(content)
                    declaration.unlink()
            if backend == "nanobind":
                # diplomat-tool 0.17.0 hard-codes the feature-test `somelib` namespace in
                # the callback caster; fixed upstream by rust-diplomat/diplomat#1308.
                common = Path(generated) / "include/diplomat_nanobind_common.hpp"
                common.write_text(common.read_text().replace("somelib::", "_native::"))
            expected = {p.relative_to(generated) for p in Path(generated).rglob("*") if p.is_file()}
            for existing in (ROOT / output).rglob("*"):
                if "examples" not in existing.relative_to(ROOT / output).parts and existing.is_file() and existing.suffix in (".h", ".hpp", ".cpp", ".mjs", ".ts", ".mts") and existing.relative_to(ROOT / output) not in expected:
                    if args.check:
                        raise SystemExit(f"Obsolete generated file: {existing.relative_to(ROOT)}")
                    existing.unlink()
            for source in Path(generated).rglob("*"):
                if not source.is_file():
                    continue
                destination = ROOT / output / source.relative_to(generated)
                if args.check:
                    if not destination.exists() or source.read_bytes() != destination.read_bytes():
                        raise SystemExit(f"Stale generated binding: {destination.relative_to(ROOT)}")
                else:
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    destination.write_bytes(source.read_bytes())
    subprocess.run([
        sys.executable, str(ROOT / "scripts/generate_js_facade.py"),
        *(["--check"] if args.check else []),
    ], cwd=ROOT, check=True)
    subprocess.run([
        sys.executable, str(ROOT / "scripts/generate_c_facade.py"),
        *(["--check"] if args.check else []),
    ], cwd=ROOT, check=True)
    dotnet_tool = os.environ.get("DIPLOMAT_TOOL_DOTNET", str(ROOT / ".tools/diplomat-fork/bin" /
        ("diplomat-tool.exe" if os.name == "nt" else "diplomat-tool")))
    if not shutil.which(dotnet_tool) and not args.check:
        print("C# generator not installed; run scripts/install_dotnet_generator.py")
    else:
        subprocess.run([
            sys.executable, str(ROOT / "scripts/generate_dotnet.py"),
            *(["--check"] if args.check else []),
        ], cwd=ROOT, check=True)


    kotlin_tool = os.environ.get("DIPLOMAT_TOOL_KOTLIN", str(ROOT / ".tools/diplomat-kotlin/bin" /
        ("diplomat-tool.exe" if os.name == "nt" else "diplomat-tool")))
    if not shutil.which(kotlin_tool) and not args.check:
        print("Kotlin generator not installed; run scripts/install_kotlin_generator.py")
    else:
        subprocess.run([
            sys.executable, str(ROOT / "scripts/generate_kotlin.py"),
            *(["--check"] if args.check else []),
        ], cwd=ROOT, check=True)


if __name__ == "__main__":
    main()
