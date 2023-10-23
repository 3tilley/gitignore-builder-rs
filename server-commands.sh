#/bin/bash

set -eu

port=$1

xh :$port/ignores lang==python
