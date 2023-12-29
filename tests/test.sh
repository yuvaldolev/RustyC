#!/bin/bash

SCRIPT_PATH="$(realpath ${BASH_SOURCE[0]})"
TESTS_DIRECTORY="$(dirname $SCRIPT_PATH)"
PROJECT_DIRECTORY="$(dirname $TESTS_DIRECTORY)"
RUSTYC_PATH="$PROJECT_DIRECTORY/target/debug/rustyc"

RESULT=1

fail() {
    RESULT=0
}

assert() {
    local expected="$1"
    local input="$2"

    $RUSTYC_PATH "$input" > test.s
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
}

pushd "$PROJECT_DIRECTORY"
cargo build
popd >/dev/null

pushd "$TESTS_DIRECTORY" >/dev/null
assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 107 "     111 +    5                              -              9"
popd >/dev/null

if [[ 1 == $RESULT ]]; then
    echo OK
fi
