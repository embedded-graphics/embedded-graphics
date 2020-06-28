#!/bin/bash

# Generate or check readmes
#
# Taken from https://github.com/porglezomp/pixel-canvas/blob/develop/gen-readme.sh

set +e

if [ "$1" = "--check" ]; then   # ./readme.sh --check <crate>
    crate=$2

    diff -q <(cargo readme -r $2 | $0 --filter) ./$2/README.md || (
       printf "\033[1;31mREADME for $2 needs to be re-generated.\n"
       printf "Run ./readme.sh $2 to regenerate.\033[0m\n"
       exit 1
    )
elif [ "$1" = "--filter" ]; then    # ./readme.sh --filter
    sed -E '/\[`.*`\]: .*(struct|enum|trait|type|fn|index)\./d' |
    sed -e 's/\[`\([^]]*\)`\]/\1/g' |
    sed -E 's/\[(.*)\]\(.*(struct|enum|trait|type|fn|index).*\)/\1/g'
else    # ./readme.sh <crate>
    crate=$1

    cargo readme -r $crate | $0 --filter > ./$crate/README.md
fi
