#!/usr/bin/env python

import click
import ase.io


@click.command()
@click.argument("src")
@click.argument("dst")
@click.option("-f", "--format", default=None, help="Format to convert")
@click.option("-i", "--index", default="-1", help="Index to convert")
def main(src, dst, format, index):
    atoms_or_traj = ase.io.read(src, index)
    ase.io.write(dst, atoms_or_traj, format=format)


if __name__ == "__main__":
    main()
