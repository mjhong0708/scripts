# pos2pot

Converts POSCAR to POTCAR

## Installation

Build from source:
```bash
cargo install --git https://git.hansgroup.synology.me/mjhong0708/pos2pot.git
```

If you want to install binary distribution, go to releases tab and download one that matches your machine. (HPC: glibc_2.17, other linux machines which use `CentOS>=8` or `Ubuntu>=18.04`: glibc_2.27)

## Usage

Before running the program, set the environment variable `POTCAR_PATH_PREFIX` to the path where the POTCAR files are stored.

For example, in `~/.bashrc`:

```bash
# This is .bashrc file
...
export POTCAR_PATH_PREFIX=/TGM/Apps/VASP/POTCAR/2.POTPAW.PBE.54.RECOMMEND
...
```

Run `pos2pot --help` to see help message.

### 1. Recommanded mode (Default)

This generates POTCAR from recommendation.

```bash
# default filename: POSCAR
user@example.com:~/work$ pos2pot
# custom filename
user@example.com:~/work$ pos2pot --filename my_file.vasp
```

### 2. Manual mode

Use the flag `--manual` to use the POTCARs other than the recommended ones.

If no other options are provided, the program will prompt you to select POTCARs.

```bash
# It will ask you to select POTCAR files!
user@example.com:~/work$ pos2pot --manual
```

Otherwise, supply the option `--potcars` in the format of `elem1=potcar1,elem2=potcar2,...`, then the program will use them.

```bash
# It will use the POTCARs provided by --potcars option.
user@example.com:~/work$ pos2pot --manual --potcars Pt=Pt,Cu=Cu_pv
```