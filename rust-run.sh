#!/bin/sh

BASEDIR=$(dirname "$0")

if [ "$1" = "server" ]
then
   cargo run --bin server --manifest-path "${BASEDIR}/rust/Cargo.toml"
elif [ "$1" = "client" ]
then
   cargo run --bin client --manifest-path "${BASEDIR}/rust/Cargo.toml"
else
  echo "Utilizzo: sh $0 [server|client]"
fi
