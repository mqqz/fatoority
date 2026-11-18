#!/usr/bin/env python3
"""Install the pinned Kotlin generator, including optional-result/runtime fixes."""

import argparse
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
REVISION = "0cca8412630402302b4bd720842101b144a3e18d"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT / ".tools/diplomat-kotlin")
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as directory:

        def run(*command):
            subprocess.run(command, cwd=directory, check=True)

        run("git", "init", "--quiet")
        run(
            "git",
            "fetch",
            "--depth=1",
            "https://github.com/rust-diplomat/diplomat.git",
            REVISION,
        )
        run("git", "checkout", "--detach", "FETCH_HEAD")
        run("git", "apply", str(ROOT / "scripts/patches/diplomat-kotlin.patch"))
        run("cargo", "test", "-p", "diplomat-tool", "--locked", "kotlin")
        run(
            "cargo",
            "install",
            "--path",
            str(Path(directory) / "tool"),
            "--locked",
            "--root",
            str(args.root.resolve()),
            "--force",
        )


if __name__ == "__main__":
    main()
