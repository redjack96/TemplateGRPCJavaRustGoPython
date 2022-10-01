#!/bin/sh

DIR=$(dirname "$0")

if    ! { [ "$1" = "server" ] || [ "$1" = "client" ]; }
then
    echo "Utilizzo: $0 [server|client]"
    exit
else
  echo "Compilo il file proto"
  protoc --go_out "${DIR}/go"  --go-grpc_out "${DIR}/go" "${DIR}/go/proto/hellogoworld.proto"
  if [ "$1" = "server" ]
  then
     go run go/server/server.go
  elif [ "$1" = "client" ]
  then
     go run go/client/client.go
  else
    echo "Utilizzo: sh $0 [server|client]"
  fi
fi