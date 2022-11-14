#!/bin/bash
set -e -x

for PYBIN in /opt/python/cp3[7891011]*/bin; do
    "${PYBIN}/pip" install maturin
    "${PYBIN}/maturin" build -i "${PYBIN}/python" --release --manylinux 2014
done

for wheel in target/wheels/*.whl; do
    auditwheel repair "${wheel}"
done
