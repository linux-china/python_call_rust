export PATH := justfile_directory() + "/.venv/bin:" + env_var('PATH')

run: build
    python3 word_suffix/demo.py

build:
    maturin develop
