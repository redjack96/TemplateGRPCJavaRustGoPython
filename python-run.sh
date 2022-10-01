#!/bin/sh

# cartella in cui risiede questo script
BASEDIR=$(dirname "$0")
DIR="${BASEDIR}/python"

if    ! { [ "$1" = "server" ] || [ "$1" = "client" ]; }
then
    echo "Utilizzo: $0 [server|client]"
    exit
else
    . "${DIR}/venv/bin/activate" # analogo di bash: source venv/bin/activate
    echo "Compilo il file proto"
    python -m grpc_tools.protoc -I"${DIR}/proto/" --python_out="${DIR}" --grpc_python_out="${DIR}" "${DIR}/proto/helloworld.proto"
    if [ "$1" = "server" ]
    then
      python python/server.py
    elif [ "$1" = "client" ]
    then
      mkdir -p python/generated
      python python/client.py
    fi
    deactivate
fi
