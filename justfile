run: build
   python word_suffix/demo.py

build:
   maturin develop

install:
   pip install --upgrade maturin
   pip install --upgrade cffi
