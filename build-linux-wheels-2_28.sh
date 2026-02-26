#!/bin/bash

# Run it with
# docker run --rm -v $(dirname $(pwd)):/io \
#   quay.io/pypa/manylinux_2_28_x86_64 /io/pyarc2/build-linux-wheels-2_28.sh

set -ex

yum -y update
yum -y install libusb wget git rpm-build rpmdevtools gcc-c++ udev libffi-devel

mkdir -p ${HOME}/rpmbuild/{BUILD,BUILDROOT,RPMS,SOURCES,SPECS,SRPMS}

mkdir /cesys
pushd /cesys
git clone https://github.com/arc-instruments/beastlink-rs
popd

cp /cesys/beastlink-rs/contrib/redhat/*.spec ${HOME}/rpmbuild/SPECS/

spectool -g -R ${HOME}/rpmbuild/SPECS/cesys-udk-lite.spec

pushd ${HOME}/rpmbuild/SPECS
rpmbuild -ba cesys-udk-lite.spec
rpmbuild -ba beastlink-free.spec
popd

pushd ${HOME}/rpmbuild/RPMS/x86_64
rpm -i beastlink-free-1.0-1.x86_64.rpm cesys-udk-lite-1.5.1-1.x86_64.rpm
popd

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

export PATH="${HOME}/.cargo/bin:${PATH}"
export WHLPLAT=manylinux_2_28_x86_64

cd /io/pyarc2
mkdir -p target/wheels

for PYVER in {311,312,313,314}; do
    PYBIN=/opt/python/cp${PYVER}-cp${PYVER}/bin
    PYEXEC="${PYBIN}/python"
    PIPEXEC="${PYBIN}/pip"
    "${PIPEXEC}" install -U poetry setuptools
    ${PYEXEC} -m poetry config virtualenvs.in-project false
    ${PYEXEC} -m poetry check
    ${PYEXEC} -m poetry update
    ${PYEXEC} -m poetry install

    # we'll check manylinux compliance with auditwheel later
    PYTHON_SYS_EXECUTABLE=${PYEXEC} PYO3_PYTHON=${PYEXEC} \
            ${PYEXEC} -m poetry run maturin build \
            --release --interpreter ${PYEXEC} \
            --out "$(pwd)/target/wheels" \
            --skip-auditwheel --compatibility off \
            --sdist
done

for whl in /io/pyarc2/target/wheels/*-linux*x86_64.whl; do
    # make sure we target the correct platform when repairing
    auditwheel repair --plat $WHLPLAT "$whl" -w /io/pyarc2/target/wheels || true
done
