#!/bin/bash

# Run it with
# docker run --rm -v $(dirname $(pwd)):/io \
#   quay.io/pypa/manylinux_2_24_x86_64 /io/pyarc2/build-linux-wheels.sh

set -ex

apt update

apt-get -y install libusb-1.0
apt install -y /io/cesys-udk-lite_1.5.1-1.deb /io/beastlink-free_1.0-1.deb

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

export PATH="${HOME}/.cargo/bin:${PATH}"

cd /io/pyarc2

for PYBIN in /opt/python/cp{38,39,310}*/bin; do
    PYEXEC="${PYBIN}/python"
    PIPEXEC="${PYBIN}/pip"
    "${PIPEXEC}" install -U poetry
    ${PYEXEC} -m poetry config virtualenvs.in-project false
    ${PYEXEC} -m poetry check
    ${PYEXEC} -m poetry update
    ${PYEXEC} -m poetry install

    # we'll check manylinux compliance with auditwheel later
    PYTHON_SYS_EXECUTABLE=${PYEXEC} PYO3_PYTHON=${PYEXEC} \
            ${PYEXEC} -m poetry run maturin build \
            --release --interpreter ${PYEXEC} \
            --manylinux off
done

for whl in /io/pyarc2/target/wheels/*.whl; do
    auditwheel repair "$whl" -w /io/pyarc2/target/wheels
done
