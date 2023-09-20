#!/bin/bash

IOC=./target/debug/ioc
MAINC=./target/source/main.c
MAINS=./target/source/main.s
MAIN=./target/source/main

assert() {
    expected="$1"
    input="$2"

    echo $input > $MAINC
    $IOC $MAINC $MAINS
    gcc $MAINS -o $MAIN
    $MAIN
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
assert 41 " 12 + 34 -5 "

echo OK
