#!/usr/bin/env bash

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir

cd "$SRCDIR" # "$SRCDIR" ensures that this script can be run from anywhere.

scp -r frontend/static/* minecraftuser@hetzner:/home/minecraftuser/static_web/pw

