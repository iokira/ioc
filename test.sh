#!/bin/bash

assert() {
    expected="$1"
    input="$2"

    echo $input > ./target/source/main.c
    ./target/debug/ioc target/source/main.c target/source/main.s
    gcc -o target/source/main target/source/main.s
    ./target/source/main
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual" >&2
        exit 1
    fi
}

assert 0 0
assert 42 42

echo OK
