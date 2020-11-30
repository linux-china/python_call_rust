run: build
   python word_suffix/demo.py

build:
   maturin develop

install: activate
   pip install maturin
   pip install cffi

activate:
   source venv/bin/activate

setup:
   virtualenv -p ~/.pyenv/versions/3.8.6/bin/python3.8 venv
