"""The Python implementation of the GRPC helloworld.Greeter client."""

import logging
import grpc
import helloworld_pb2
import helloworld_pb2_grpc

# Prerequisiti: nella cartella python crea un virtual environment (per evitare conflitti con altre versioni di python installate)
# Installa i virtualenv dalla versione attuale di python
# $ python -m pip install virtualenv
# crea un virtual enviroment
# $ virtualenv venv
# -- SOLO WINDOWS
# $ .\venv\Scripts\activate
# -- SOLO UNIX
# $ source venv/bin/activate
# A questo punto si è entrati nel virtual environment
# $ python -m pip install --upgrade pip
# Installa i seguenti package
# $ python -m pip install grpcio grpcio-tools
# Dalla cartella radice (python/) Genera il codice con:
# $ python -m grpc_tools.protoc -Iproto/ --python_out=. --grpc_python_out=. proto/hello.proto

def run():
    # la with evita di usare la channel.close()
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = helloworld_pb2_grpc.GreeterStub(channel)
        response = stub.SayHello(helloworld_pb2.HelloRequest(name='giacomo'))
    print("Greeter client received: " + response.message)

if __name__ == '__main__':
    logging.basicConfig()
    run()