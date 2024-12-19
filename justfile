export PATH := justfile_directory() + '/.venv/bin:' + env_var('PATH')

build:
  maturin develop

hello:
  uv run hello.py

release:
  maturin develop --release
  maturin build --release
