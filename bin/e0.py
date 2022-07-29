#!/usr/bin/env python
import re
from pathlib import Path
from typing import List

re_sci_num = r"[\d+]?\+?-?.\d+E[\+-]?\d+"


def read_energies(dirname: str) -> List[float]:
    dir_path = Path(dirname)

    def is_result_line(line):
        return "E0" in line

    def get_energy(line):
        energy_frag = re.findall("E0=\s+" + re_sci_num, line)[0]
        energy = float(re.findall(re_sci_num, energy_frag)[0])
        return energy

    stdout_lines = (dir_path / "OSZICAR").read_text().splitlines()
    result_lines = filter(is_result_line, stdout_lines)

    energies = map(get_energy, result_lines)
    return list(energies)


if __name__ == "__main__":
    try:
        e0_energies = read_energies(".")
        last_e0_energy = e0_energies[-1]
        print(f"{last_e0_energy:.6f}")
    except IndexError:
        print("No SCF calculation found.")
    except FileNotFoundError:
        print("vasprun.xml not found.")
        exit(1)
