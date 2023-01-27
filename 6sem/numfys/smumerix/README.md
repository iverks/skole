# Numfys fysikkbibliotek

## To teachers

The entry point is the make files. I provide the precompiled python library but im not sure if it will work on mac and linux. 
The python library will also be downloadable from pip `pip install smumerix`.
If it doesnt, follow steps below. Headers are reverse sorted with respect to chronological order, since i do the last steps the most often.


## Kompilering og kjøring

```
maturin develop
py .\_python\preex.py
```

## Aktivering

```
.env/Scripts/Activate.ps1
code .
```

## Originalt oppsett

```
py -m venv .env
pip install -f requirements.txt
```

## Installering av biblioteker

1. Installer rust og python
2. Installer maturin med pip (anbefaler å ha i en virtualenv)
