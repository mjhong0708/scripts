#!/usr/bin/env python
from pathlib import Path
import argparse
import subprocess


def is_bin(crate_path: Path) -> bool:
    return (crate_path / "src/main.rs").is_file()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Install the binaries")
    parser.add_argument("--bin", type=str, default="all", help="Path to the crate to install")
    args = parser.parse_args()

    bin_crates = [crate for crate in Path("src").glob("*") if is_bin(crate)]
    if args.bin == "all":
        for crate in bin_crates:
            print("Installing {}".format(crate.name))
            subprocess.run(["cargo", "install", "--path", crate])
    else:
        for crate in bin_crates:
            if crate.name == args.bin:
                print("Installing {}".format(crate.name))
                subprocess.run(["cargo", "install", "--path", crate])
                break
        else:
            print("Crate not found")
            exit(1)
