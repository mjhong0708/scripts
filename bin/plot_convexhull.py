#!/usr/bin/env python
import argparse
from typing import Sequence

import ase.io
import matplotlib.pyplot as plt
import numpy as np
from scipy.spatial import ConvexHull


def make_composition_fn(elem: str):
    def composition_fn(atoms):
        symbols = atoms.get_chemical_symbols()
        return symbols.count(elem) / len(symbols)

    return composition_fn


def get_binary_convex_hull(
    images: Sequence[ase.Atoms], elem_to_count: str
) -> ConvexHull:
    composition_fn = make_composition_fn(elem_to_count)
    compositions = np.empty(len(images))
    energies = np.empty(len(images))
    for i, atoms in enumerate(images):
        compositions[i] = composition_fn(atoms)
        energies[i] = atoms.get_potential_energy() / len(atoms)

    min_comp_idx, max_comp_idx = np.argmin(compositions), np.argmax(compositions)
    ref_energy_0 = energies[min_comp_idx]
    ref_energy_1 = energies[max_comp_idx]

    formation_energies = (
        energies - (1 - compositions) * ref_energy_0 - compositions * ref_energy_1
    )
    points = np.c_[compositions, formation_energies]
    return ConvexHull(points)


def plot_convex_hull(hull: ConvexHull, ax=None, **kwargs):
    if ax is None:
        _, ax = plt.subplots(facecolor="w")
    x, y = hull.points[:, 0], hull.points[:, 1]
    ax.plot(x, y, "o", mew=0.6, mec="k", mfc="#CCCCCC")
    for simplex in hull.simplices:
        simp_x = hull.points[simplex][:, 0]
        simp_y = hull.points[simplex][:, 1]
        if np.all(simp_y <= 0):
            plt.plot(simp_x, simp_y, "ko-", ms=7.5)
    ax.set_xlabel("Composition", fontsize=13)
    ax.set_ylabel("Energy (eV/atom)", fontsize=13)
    if ax is None:
        plt.show()


if __name__ == "__main__":
    # Plotting style
    plt.rc("font", family="Arial", size=13)
    plt.rc("axes", linewidth=1.35, labelsize=15)
    plt.rc(("xtick.major", "xtick.minor", "ytick.major", "ytick.minor"), width=1.35)
    parser = argparse.ArgumentParser()
    help_msg = (
        "Input file which contains the all structures. "
        "It should be readable by ASE, and contain energy."
    )
    parser.add_argument("-f", "--file", required=True, help=help_msg)
    parser.add_argument(
        "-e", "--elem", required=True, help="Element to count composition."
    )
    parser.add_argument("-o", "--output", required=True, help="Output image file.")
    parser.add_argument("--save_data", action="store_true", help="Save data to file.")
    args = parser.parse_args()

    images = ase.io.read(args.file, ":")
    convex_hull = get_binary_convex_hull(images, args.elem)

    fig, ax = plt.subplots(facecolor="w")
    plot_convex_hull(convex_hull)
    plt.tight_layout()
    plt.savefig(args.output, dpi=600)

    if args.save_data:
        np.savetxt(
            "comp_vs_formation_energy.dat",
            convex_hull.points.T,
            header="Composition, E_formation",
        )
        np.savetxt(
            "index_on_hull.dat",
            convex_hull.vertices,
            fmt="%d",
            header="Index of points on hull",
        )
