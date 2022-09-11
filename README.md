# Compilare e/o eseguire i progetti 
## Go
Entra nella cartella go/server/ ed esegui:
```
```console
$ go run server.go
```
Entra nella cartella go/client/ ed esegui:
```console
$ go run client.go
```
## Java
Entra nella cartella java\template_grpc.
Per compilare e generare il codice gRPC:
```console
$ mvn clean verify
```
Per eseguire (-q acquieta Maven), su due terminali diversi:
```console
$ mvn exec:java -q -D"exec.mainClass"="com.giacomolorenzo.rossi.template_grpc.server.JavaGrpcServer"
$ mvn exec:java -q -D"exec.mainClass"="com.giacomolorenzo.rossi.template_grpc.client.JavaGrpcClient"
```

Se non funziona:
- assicurarsi che java -version sia la stessa versione di quella compilata indicata nel pom.xml
## Python
Crea un file main.py nella cartella python

```python
print("Hello Python World")
```
Nella cartella python/:

```console
$ py main.py
```
## Rust
Per compilare ed eseguire, entra nella cartella rust ed esegui nel primo terminale (ci vuole un po' la prima volta):
```console
$ cargo run --bin server
```
poi in un altro terminale, sempre nella cartella rust:
```console
$ cargo run --bin client
```
