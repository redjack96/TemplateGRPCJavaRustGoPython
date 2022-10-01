#!/bin/sh

if [ "$1" = "server" ]
then
   mvn verify -q -f java/template_grpc
   mvn exec:java -q -D"exec.mainClass"="com.giacomolorenzo.rossi.template_grpc.server.JavaGrpcServer" -D"exec.args"="giacomo localhost:50051" -f java/template_grpc
elif [ "$1" = "client" ]
then
   mvn verify -q -f java/template_grpc
   mvn exec:java -q -D exec.mainClass="com.giacomolorenzo.rossi.template_grpc.client.JavaGrpcClient" -f java/template_grpc
else
  echo "Utilizzo: sh $0 [server|client]"
fi
