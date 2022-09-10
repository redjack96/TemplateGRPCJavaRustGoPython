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
Entra nella cartella java.
In questo caso uso l'archetype "java-archetype" per generare immediatamente il progetto. Attenzione a non inserire '-' nei groupId o artifactId.

```console
$ mvn archetype:generate /
    -DgroupId="com.giacomolorenzo.rossi"  /
    -DartifactId="template_grpc" /
    -DarchetypeGroupId="io.github.oliviercailloux" /
    -DarchetypeArtifactId="java-archetype" /
    -DinteractiveMode=false
```

Aggiungi il seguente plugin sul pom.xml:

```markdown
<plugin>
    <groupId>org.codehaus.mojo</groupId>
    <artifactId>exec-maven-plugin</artifactId>
    <version>1.2.1</version>
    <configuration>
        <mainClass>com.giacomolorenzo.rossi.template_grpc.client.JavaGrpcClient</mainClass> <!-- Cambia qui se necessario -->
    </configuration>
</plugin>
```

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
Per compilare ed eseguire, entra nella cartella rust/server/ ed esegui:
```console
$ cargo run
```
poi entra nella cartella rust/client/ ed esegui di nuovo:
```console
$ cargo run
```
