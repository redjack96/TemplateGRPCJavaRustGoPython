package main

// Esegui con: go run client.go

import (
	"context"
	"flag"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"log"
	pb "template.grpc/template.grpc/proto/helloworld"
	"time"
)

// here we define some constants
const (
	port           = "50051"
	defaultAddress = "localhost:" + port
	defaultName    = "go"
)

// here some flags for command line parsing!!! They cannot be constants, because are returned by flag.String
// similar to rust's clap crate, but this is in the go std library
var (
	// 1: name of command line flag, 2: default value of flag, 3: usage message for the flag
	addr = flag.String("addr", defaultAddress, "The address to connect to")
	name = flag.String("name", defaultName, "Name to greet")
)

// Run it (from the client directory) like:
// go run .\client.go --name=giacomo
func main() {
	flag.Parse()
	// Set up connection to server

	conn, err := grpc.Dial(*addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did non connect: %v", err)
	}
	defer func(conn *grpc.ClientConn) {
		err := conn.Close()
		if err != nil {
			log.Fatalf("cannot close connection: %v", err)
		}
	}(conn) // runs immediately this function (like in JavaScript). To be more precise, it runs the lambda function at the end of the main function.

	// pb stand for ProtocolBuffer
	c := pb.NewGreeterClient(conn)

	// Contact server and print out its response; cancel is a function
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	r, err := c.SayHello(ctx, &pb.HelloRequest{Name: *name}) // with {} we define the field of the struct HelloRequest. This struct is generated by protoc.
	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}
	log.Printf("Greeting received: %s", r.GetMessage())
}
