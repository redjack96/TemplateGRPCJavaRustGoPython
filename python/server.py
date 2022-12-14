from concurrent import futures
import logging
# se questo import non viene riconosciuto... boh prova a attivare venv e installa grpcio
import grpc
# vorrei tanto metterli in una cartella generated, ma non si riesce a importarli. Devono stare nella stessa cartella di questo file!!!
import helloworld_pb2
import helloworld_pb2_grpc


class Greeter(helloworld_pb2_grpc.GreeterServicer):
    def SayHello(self, request, context):
        print("Received request with param: %s" % request.name)
        return helloworld_pb2.HelloReply(message='Hello, %s!' % request.name)


def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    helloworld_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    print("Python server started at port 50051...")
    server.add_insecure_port('[::]:50051')
    server.start()
    server.wait_for_termination()


if __name__ == '__main__':
    logging.basicConfig()
    serve()
