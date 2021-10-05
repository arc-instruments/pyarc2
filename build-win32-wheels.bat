python -m poetry run maturin build
for /R %%a in (target\wheels\*.whl) DO python -m delvewheel repair %%a -w target\wheels\

