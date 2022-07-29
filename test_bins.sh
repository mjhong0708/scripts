#!/bin/bash

BULK_RELAX=".dev/vasp-reference-calculations/0_optimization/0_bulk"
SLAB_RELAX=".dev/vasp-reference-calculations/0_optimization/1_slab"

CDD_DIR=".dev/vasp-reference-calculations/2_charge_density_difference"
CHG_AB="$CDD_DIR/1_Pt-H2O/CHGCAR"
CHG_A="$CDD_DIR/2_Pt/CHGCAR"
CHG_B="$CDD_DIR/3_H2O/CHGCAR"

function test_check_ef {
    CURR_DIR=$(pwd);
    cd $BULK_RELAX;
    cargo run --release --bin check_ef;
    cd $CURR_DIR;
    cd $SLAB_RELAX;
    cargo run --release --bin check_ef;
    cd $CURR_DIR;
}

echo "Testing check_ef...";
test_check_ef;


echo "Testing charge density difference...";
cargo run --bin chgdiff --release -- $CHG_AB --ref1 $CHG_A --ref2 $CHG_B;