all: kpointsgen vasp-slurm-generator chgdiff libscripts check_convergence pos2pot ndstat qst

install:
	cp target/release/kpointsgen bin && \
	cp target/release/vasp-slurm-generator bin && \
	cp target/release/chgdiff bin && \
	cp target/release/check_convergence bin && \
	cp target/release/pos2pot bin && \
	cp target/release/ndstat bin && \
	cp target/release/qst bin

kpointsgen:	src/kpointsgen/*
	cd src/kpointsgen && \
	cargo build --release && \
	cd ../..;

vasp-slurm-generator: src/vasp-slurm-generator/*
	cd src/vasp-slurm-generator && \
	cargo build --release && \
	cd ../..;

chgdiff: src/chgdiff/*
	cd src/chgdiff && \
	cargo build --release && \
	cd ../..;

check_convergence: src/check_convergence/*
	cd src/check_convergence && \
	cargo build --release && \
	cd ../..;

pos2pot: src/pos2pot/*
	cd src/pos2pot && \
	cargo build --release && \
	cd ../..;

ndstat: src/ndstat/*
	cd src/ndstat && \
	cargo build --release && \
	cd ../..;

qst: src/qst/*
	cd src/qst && \
	cargo build --release && \
	cd ../..;

libscripts: src/libscripts/*
	cd src/libscripts && \
	cargo build --release && \
	cd ../..;
