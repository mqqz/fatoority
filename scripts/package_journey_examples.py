#!/usr/bin/env python3
"""Build or check the downloadable sandbox walkthrough projects."""
import argparse
from io import BytesIO
from pathlib import Path
from zipfile import ZipFile, ZipInfo, ZIP_DEFLATED

ROOT = Path(__file__).resolve().parents[1] / "docs/assets/examples/journey"
PROJECTS = {
    "python": ("main.py", "requirements.txt"),
    "csharp": ("Program.cs", "Journey.csproj"),
    "typescript": ("main.ts", "package.json", "tsconfig.json"),
}


def archive(language, names):
    buffer = BytesIO()
    with ZipFile(buffer, "w", compression=ZIP_DEFLATED) as bundle:
        files = {name: (ROOT / language / name).read_bytes() for name in names}
        files["input.json"] = (ROOT / "input.json").read_bytes()
        files[".gitignore"] = b"runs/\n*.pem\n.env\n.venv/\nnode_modules/\ndist/\nbin/\nobj/\n"
        for name, data in sorted(files.items()):
            info = ZipInfo(name, date_time=(2026, 1, 1, 0, 0, 0))
            info.create_system = 3
            info.compress_type = ZIP_DEFLATED
            info.external_attr = 0o644 << 16
            bundle.writestr(info, data)
    return buffer.getvalue()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Fail if an archive differs from its source")
    args = parser.parse_args()
    for language, names in PROJECTS.items():
        path = ROOT / f"{language}.zip"
        expected = archive(language, names)
        if args.check:
            if not path.is_file() or path.read_bytes() != expected:
                raise SystemExit(f"Rebuild {path.name}: python scripts/package_journey_examples.py")
        else:
            path.write_bytes(expected)
        print(f"{path.name}: {'checked' if args.check else 'written'}")


if __name__ == "__main__":
    main()
