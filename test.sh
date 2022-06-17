#!/bin/bash

set -euxo pipefail

export CROSS_CUSTOM_TOOLCHAIN=1
exec cross run -vv --target powerpc64le-unknown-linux-gnu
