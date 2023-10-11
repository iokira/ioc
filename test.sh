#!/bin/bash

IOC=./target/debug/ioc
MAINC=./target/source/main.c
MAINS=./target/source/main.s
MAIN=./target/source/main

mkdir -p ./target/source

assert() {
    expected="$1"
    input="$2"

    echo $input > $MAINC
    $IOC $MAINC $MAINS
    gcc -z noexecstack $MAINS -o $MAIN
    $MAIN
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual" >&2
        exit 1
    fi
}

# ダブルクオーテーションの中でも*の後ろに空白文字があるとメタ文字と解釈されてファイル一覧に展開されるから注意

assert 0 "0;"
assert 42 "42;"
assert 41 " 12 + 34 -5 ;"
assert 47 "5 + 6 *7;"
assert 15 "5*(9-6);"
assert 4 "(3+5)/2;"
assert 10 "-10+20;"
assert 1 "1==1;"
assert 1 "2 *3!=3-1;"
assert 0 "32/(1==1)<2;"
assert 2 "a=1+1;a;"
assert 14 "a = 3; b = 5*6 - 8; a + b / 2;"
assert 15 "row = 3; column=5; row*column;"

echo OK
