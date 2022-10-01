#!/bin/sh

if [ "$1" = "server" ]
then
   go run go/server/server.go
elif [ "$1" = "client" ]
then
   go run go/client/client.go
else
  echo "Utilizzo: sh $0 [server|client]"
fi
