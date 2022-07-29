#!/usr/bin/env python
import re
from pathlib import Path
from typing import List

import ase.io
import numpy as np

re_sci_num = r"[\d+]?\+?-?.\d+E[\+-]?\d+"


def read_energies(calc_dir: str) -> List[float]:
    dir_path = Path(calc_dir)

    def is_result_line(line):
        return "E0" in line

    def get_energy(line):
        energy_frag = re.findall(r"E0=\s+" + re_sci_num, line)[0]
        energy = float(re.findall(re_sci_num, energy_frag)[0])
        return energy

    stdout_lines = (dir_path / "OSZICAR").read_text().splitlines()
    result_lines = filter(is_result_line, stdout_lines)

    energies = map(get_energy, result_lines)
    return list(energies)


def get_forces(calc_dir):
    calc_dir = Path(calc_dir)
    poscar = ase.io.read(calc_dir / "POSCAR")
    outcar = (calc_dir / "OUTCAR").read_text().splitlines()

    constraints = poscar.constraints

    force_lines_idx = [i for i, line in enumerate(outcar) if "TOTAL-FORCE" in line]
    force_slices = [outcar[i + 2 : i + 2 + len(poscar)] for i in force_lines_idx]
    forces = np.stack([np.loadtxt(f)[:, 3:] for f in force_slices])

    if constraints:
        fix_idx = constraints[0].index
        forces[:, fix_idx, :] = 0

    return forces


if __name__ == "__main__":
    try:
        traj = ase.io.read("vasprun.xml", ":")
    except FileNotFoundError:
        print("vasprun.xml not found")
        exit(1)

    if len(traj) == 0:
        print("No SCF loop found.")
        exit(0)

    E0 = np.array(read_energies("."))
    F = get_forces(".")
    F_max = np.sqrt(F ** 2).sum(axis=2).max(axis=1)

    rel_E0 = E0 - E0[0]
    dE0 = np.concatenate([np.array([E0[0]]), E0[1:] - E0[:-1]])

    header = f"{'Step':<10}    {'F_max (eV/A)':<15} " f"{'E0 (eV)':<15} {'E - E0 (eV)':<15} {'dE (eV)':<15}"
    print(header)
    print("=" * len(header))
    for i in range(len(traj)):
        line = f"{i:<10d}    {F_max[i]:<15.5f} " f"{E0[i]:<+15.5f} {rel_E0[i]:<+15.5f} {dE0[i]:<+15.5f}"
        print(line)
