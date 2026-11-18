#!/usr/bin/env python3
"""Run native C# regressions and a fresh NuGet consumer with no project reference."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib
from xml.sax.saxutils import escape

from package_dotnet import ROOT, native_target


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--package-dir", type=Path, help="Also verify an already built NuGet package")
    parser.add_argument("--package-only", action="store_true", help="Test only the fresh package consumer; requires --package-dir")
    args = parser.parse_args()
    if args.package_only and not args.package_dir:
        parser.error("--package-only requires --package-dir")
    dotnet = os.environ.get("DOTNET", "dotnet")
    _, filename = native_target()
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve()
    env = os.environ.copy()
    for key in ("LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH"):
        env.pop(key, None)
    def run(command, cwd=ROOT):
        subprocess.run(command, cwd=cwd, env=env, check=True, timeout=300)
    if not args.package_only:
        run([dotnet, "build", str(ROOT / "bindings/dotnet/examples/Examples.csproj")])
        run(["cargo", "build", "-p", "fatoora-ffi", "--locked"])
        run([dotnet, "run", "--project", str(ROOT / "bindings/dotnet/tests/Smoke.csproj"),
             f"-p:NativeLibrary={target / 'debug' / filename}", "--", str(ROOT)])
    if args.package_dir:
        version = tomllib.loads((ROOT / "fatoora-ffi/Cargo.toml").read_text())["package"]["version"]
        with tempfile.TemporaryDirectory() as directory:
            consumer = Path(directory)
            shutil.copyfile(ROOT / "bindings/dotnet/tests/Program.cs", consumer / "Program.cs")
            (consumer / "Consumer.csproj").write_text(f'''<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><Nullable>enable</Nullable><UseAppHost>false</UseAppHost></PropertyGroup>
  <ItemGroup><PackageReference Include="Fatoority" Version="{version}" /></ItemGroup>
</Project>''')
            local = escape(str(args.package_dir.resolve()), {chr(34): '&quot;'})
            # Exact source mapping keeps Fatoority local while allowing SDK targeting packs.
            (consumer / "NuGet.Config").write_text(f'''<configuration>
  <packageSources><clear/><add key="local" value="{local}"/><add key="nuget.org" value="https://api.nuget.org/v3/index.json"/></packageSources>
  <packageSourceMapping>
    <packageSource key="local"><package pattern="Fatoority"/></packageSource>
    <packageSource key="nuget.org"><package pattern="*"/></packageSource>
  </packageSourceMapping>
</configuration>''')
            env["NUGET_PACKAGES"] = str(consumer / "packages")
            run([dotnet, "run", "--project", str(consumer / "Consumer.csproj"), "--", str(ROOT)], cwd=consumer)


if __name__ == "__main__":
    main()
