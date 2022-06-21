#!/bin/sh

function tp() {
    result=$(tp-cli -s $1);
    if [ "$result" = "" ]; then
        ;
    else
        cd $result;
    fi;
}
