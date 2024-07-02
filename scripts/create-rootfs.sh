#!/usr/bin/bash
#
#

# Enough args?
if [ $# -lt 1 ]; then
    echo 1>&2 "Usage: $(basename $0) DIR"
    exit 3
fi

# Exists already?
target=$1
if [ -d "$target" ]; then
    echo "Directory $target already exists"
    exit 1
fi

sudo debootstrap --variant=minbase jammy $target http://ftp.halifax.rwth-aachen.de/ubuntu/
