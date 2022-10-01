#!/bin/sh

if [ "$1" = "server" ]
then
   cargo run --bin server --manifest-path rust/Cargo.toml
elif [ "$1" = "client" ]
then
   cargo run --bin client --manifest-path rust/Cargo.toml
else
  echo "Utilizzo: sh $0 [server|client]"
fi
