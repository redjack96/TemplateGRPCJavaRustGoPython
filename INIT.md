# HelloWorld nei 4 linguaggi
## Go
Entra nella cartella go/.
Crea un modulo:
```console
$ go mod init template.grpc
```
Crea un file main.go:

```go
package main

import "fmt"

func main() {
	fmt.Println("Hello Go World!")
}

```

```console
$ go run main.go
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

Per compilare e installare nella cartella .m2: (se invece vuoi solo creare il jar, usa mvn clean package)

```console
$ mvn clean install -DskipTests
```

Per eseguire (-q acquieta Maven):
```console
$ mvn exec:java -q -D"exec.mainClass"="com.giacomolorenzo.rossi.template_grpc.App"
```
[Sconsigliato] se non usi dipendenze:

```console
$ java -cp target/template_grpc-0.0.1-SNAPSHOT.jar com.giacomolorenzo.rossi.template_grpc.App
```

Se non funziona:
- assicurarsi che java -version sia la stessa versione di quella compilata indicata nel pom.xml
## Python
Crea un file main.py nella cartella python/

```python
print("Hello Python World")
```

Nella cartella python/:

```console
$ py main.py
```
## Rust
Nella cartella rust/ digita i seguenti comandi uno dopo l'altro per creare ed eseguire il progetto:
```console
$ cargo init
$ cargo run
```