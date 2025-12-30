actually maturin works better with uv, it has some kind of integration, so:

1. Install uv
2. uv venv .venv
3. activate your venv (shell specific)
4. uv pip install maturin

Then manually instanciating the cargo project can be avoided, just use:

- maturin new project_name

A cli will ask you what kind of project you want to init, choose pyo3, that's it.

With this setup you don't even have to manually wire the pyfunction to the pymodule, you just define it inside a rust module with the pymodule proc macro.

You also got rust doc strings embedded in your python code, so every doc string you normally make on rust pyfunction will be displayed on hover from your intellisense in the python side and also no more "unresolved library import".

One last thing, you can also define the signature to be shown for python with some attribute macro.
