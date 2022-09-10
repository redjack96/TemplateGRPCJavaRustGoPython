package com.giacomolorenzo.rossi.template_grpc.server;

import com.giacomolorenzo.rossi.template_grpc.client.JavaGrpcClient;
import io.grpc.Server;
import io.grpc.ServerBuilder;
import io.grpc.examples.helloworld.GreeterGrpc;
import io.grpc.examples.helloworld.HelloReply;
import io.grpc.examples.helloworld.HelloRequest;
import io.grpc.stub.StreamObserver;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

// Esegui con:
// mvn verify
// mvn exec:java -q -D"com.giacomolorenzo.rossi.template_grpc.GrpcServer"
public class JavaGrpcServer {
    private static final Logger LOGGER = LoggerFactory.getLogger(JavaGrpcServer.class);
    public static final int PORT = 50051;

    private Server server;

    private void start() throws IOException {
       this.server = ServerBuilder.forPort(PORT)
               .addService(new GreeterImpl())
               .build()
               .start();
        LOGGER.info("Server started, listening on {}", PORT);
        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            System.err.println("*** shutting down gRPC server since JVM is shutting down");
            try {
                JavaGrpcServer.this.stop();
            } catch(InterruptedException i){
                i.printStackTrace(System.err); // SO that it prints on the System console
            }
            System.err.println("*** server shut down");
        }));
    }

    private void stop() throws InterruptedException {
        if (server != null) {
            server.shutdown().awaitTermination(30, TimeUnit.SECONDS);
        }
    }

    private void blockUntilShutdown() throws InterruptedException {
        if(server != null){
            server.awaitTermination();
        }
    }

    public static void main(String[] args) throws IOException, InterruptedException{
        final var server = new JavaGrpcServer();
        server.start();
        server.blockUntilShutdown();
    }

    static class GreeterImpl extends GreeterGrpc.GreeterImplBase{
        @Override
        public void sayHello(HelloRequest request, StreamObserver<HelloReply> responseObserver) {
            HelloReply reply = HelloReply.newBuilder()
                    .setMessage("Hello %s".formatted(request.getName()))
                    .build();
            responseObserver.onNext(reply); // delivers the HelloReply message
            responseObserver.onCompleted(); // calls the onCompleted
        }
    }
}