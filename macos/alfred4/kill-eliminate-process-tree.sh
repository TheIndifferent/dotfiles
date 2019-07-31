#!/bin/bash

root="$1"

function recursiveKill() {
  local children="$( pgrep -P $1 )"
  if [[ -n $children ]]
  then
    for p in $children
    do
      recursiveKill "$p"
    done
  fi
  kill -9 "$1"
}

recursiveKill "$root"
