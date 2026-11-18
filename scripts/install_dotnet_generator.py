#!/usr/bin/env python3
"""Install the pinned C# generator from the reviewed fork revision."""
import argparse
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
REVISION = "2687a45709a8dbce5ac018a0c4704d45da680c72"
SOURCE = "https://github.com/mqqz/diplomat.git"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT / ".tools/diplomat-fork")
    parser.add_argument("--source", default=SOURCE,
                        help="git repository to fetch REVISION from (e.g. a local fork clone)")
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as directory:
        source = Path(directory)
        def run(*command):
            subprocess.run(command, cwd=source, check=True)
        run("git", "init", "--quiet")
        run("git", "fetch", "--depth=1", args.source, REVISION)
        run("git", "checkout", "--detach", "FETCH_HEAD")
        run("cargo", "test", "-p", "diplomat-tool", "--locked", "dotnet")
        run("cargo", "install", "--path", str(source / "tool"), "--locked",
            "--root", str(args.root.resolve()), "--force")


if __name__ == "__main__":
    main()
