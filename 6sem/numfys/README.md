# Numfys fysikkbibliotek

## To teachers

The entry point is the make files. I provide the precompiled python library but im not sure if it will work on mac and linux. 
The python library will also be downloadable from pip `pip install smumerix`.
If it doesnt, follow steps below. Headers are reverse sorted with respect to chronological order, since i do the last steps the most often.


## Prosjektoppsett

## Kjernefunksjonalitet (smumerix-core)

Inneholder fysikkmagien. Kan bygges med `cargo build -p smumerix-core`, men bygges automatisk om du bygger et av de andre bibliotekene. Testene kjøres automatisk om du kjører `cargo test`

## Rust frontend (nopy)

Brukes for å gjøre ting som python ikke er spesielt bra til, primært å animere. `cargo run` defaulter til denne siden den er eneste exe i workspacen.

### Python pakke (smumerix)

#### Kompilering og kjøring

```
maturin develop -m /smumerix/Cargo.toml
py ./_python/<filnavn>.py
```

#### Aktivering av venv

```
Activate.ps1
code .
```

#### Originalt oppsett

```
py -m venv smumerix/env
pip install -f requirements.txt
```

#### Installering av biblioteker

1. Installer rust og python
2. Generer og aktiver venv
   1. `py -m venv smumerix/venv`
   2. `./smumerix/venv/` (eller `./Activate.ps1`)
3. Installer maturin med pip (anbefaler å ha i en virtualenv)
