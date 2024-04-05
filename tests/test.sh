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

    $RUSTYC_PATH -- "$input" > test.s
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

assert 0 "{ return 0; }"
assert 42 "{ return 42; }"
assert 5 "{ return 3 + 2; }"
assert 21 "{ return 5+20-4; }"
assert 107 "{    return     111 +    5                              -              9;      }"
assert 56 "{ return 8* 7; }"
assert 47 "{ return 5 + 6 * 7; }"
assert 100 "{   return 200 /2; }"
assert 15 "{ return 5* (9 -6); }"
assert 4 "{ return (3+5)/2; }"
assert 78 "{ return (34 + 5) * 2; }"
assert 10 "{ return -10+20; }"
assert 10 "{ return - -10; }"
assert 10 "{ return - - +10; }"
assert 8 "{ return -(5 + 3 * 44) + 145; }"

assert 0 "{ return 0 == 1; }"
assert 1 "{ return 42==42; }"

assert 1 "{ return 0!=1; }"
assert 0 "{ return 42!=42; }"

assert 1 "{ return 0<1; }"
assert 0 "{ return 1<1; }"
assert 0 "{ return 2           < 1; }"
assert 1 "{ return 0<=1; }"
assert 1 "{ return 1<=1; }"
assert 0 "{ return 2<=1; }"

assert 1 "{ return 1>0; }"
assert 0 "{ return 1 >  1; }"
assert 0 "{ return 1>2; }"
assert 1 "{ return 1>=0; }"
assert 1 "{ return 1>=1; }"
assert 0 "{ return 1>=2; }"

assert 3 "{ a=3; return a; }"
assert 8 "{ a=3; z=5; return a+z; }"
assert 6 "{ a=b=3; return a+b; }"
assert 3 "{ foo=3; return foo; }"
assert 8 "{ foo123=3; _bar=5; return foo123+_bar; }"

assert 1 "{ return 1; 2; 3; }"
assert 2 "{ 1; return 2; 3; }"
assert 3 "{ 1; 2; return 3; }"
assert 2 "{ return 2; 5 + 6; 9 - 3; }"
assert 11 "{ 2; return 5 + 6; 9 - 3; }"
assert 6 "{ 2; 5 + 6; return 9 - 3; }"

assert 3 "{ {1; {2;} return 3; } }"

assert 5 "{ ; ;; return 5; }"

assert 3 "{ if (0) return 2; return 3; }"
assert 3 "{ if (1-1) return 2; return 3; }"
assert 2 "{ if (1) return 2; return 3; }"
assert 2 "{ if (2-1) return 2; return 3; }"
assert 4 "{ if (0) { 1; 2; return 3; } else { return 4; } }"
assert 3 "{ if (1) { 1; 2; return 3; } else { return 4; } }"

popd >/dev/null

if [[ 1 == $RESULT ]]; then
    echo OK
    exit 0
fi

exit 1
