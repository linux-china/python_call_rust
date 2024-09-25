export PATH := justfile_directory() + "/.venv/bin:" + env_var('PATH')

run: build
   python3 word_suffix/demo.py

build:
   maturin develop

setup:
   uv venv --python 3.11
   ./.venv/bin/pip3 -m ensurepip
   pip3 install maturin
   pip3 install cffi
