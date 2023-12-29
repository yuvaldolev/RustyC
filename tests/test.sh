#!/bin/bash

SCRIPT_PATH="$(realpath ${BASH_SOURCE[0]})"
TESTS_DIR="$(dirname $SCRIPT_PATH)"

RESULT=1

fail() {
    RESULT=0
}

assert() {
    local expected="$1"
    local input="$2"

    pushd "$TESTS_DIR" > /dev/null

    cargo run -- "$input" > test.s
    local rustc_status="$?"
    if [[ 0 != $rustc_status ]]; then
        fail
        return
    fi

    clang -o test test.s 
    ./test
    local actual="$?"

    if [[ "$actual" == "$expected" ]]; then
        echo "$input => $actual"
    else
        echo "$input => expected $expected, got $actual"
        fail
    fi

    popd > /dev/null
}

assert 0 0
assert 42 42
assert 21 "5+20-4"

if [[ 1 == $RESULT ]]; then
    echo OK
fi
