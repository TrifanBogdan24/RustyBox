# Rustybox
Describe your solution for the homework.

## Verify

Run the following commands to test your homework:

You will have to install NodeJS (it is installed in the codespace)

```bash
# Clone tests repository
git submodule update --init 

# Update tests repository to the lastest version
cd tests
git pull 
cd ..

# Install loadash
npm install lodash
```

## Install rustybox

```bash
cargo install --path .
```

## Run tests

```bash
cd tests
# Run all tests 
./run_all.sh

# Run single test
./run_all.sh pwd/pwd.sh
```# Rustybox
Describe your solution for the homework.

## Verify

Run the following commands to test your homework:

You will have to install NodeJS (it is installed in the codespace)

```bash
# Clone tests repository
git submodule update --init 

# Update tests repository to the lastest version
cd tests
git pull 
cd ..

# Install loadash
npm install lodash
```

## Install rustybox

```bash
cargo install --path .
```

## Run tests

```bash
cd tests
# Run all tests 
./run_all.sh

# Run single test
./run_all.sh pwd/pwd.sh
```

## Use of CARGO

Daca importam biblioteca **`chrono`** in programul nostru,
nu o sa mai putem compila codul folsind `rustc main.rs`,
ci va trebui sa ne folosim managerul de pachete **`cargo`**,
car va linka dependintele din program

> Nu uitati sa dati `cargo install --path .` dupa fiecare modificare a codului sursa

Pentru a testa tema : in directorul mare
```# compileaza programul si afiseaza erori si warning-uri
cargo build
```

Pentru a rula :
```# compileaza programul si afiseaza erori si warning-uri
# executa fisierul executabil
cargo run echo hello
```

`cargo run` = `cargo build` + `./rust_executable`

Pentru a rula doar executabilul (mai intai trebuie sa dam `cargo build`) :
```# executa fisierul executabil 
rustybox echo hello
```
