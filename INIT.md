# HelloWorld nei 4 linguaggi
Le seguenti istruzioni si riferiscono alla creazione del programma hello-world nei 4 linguaggi, sfruttando dei build system se possibile (maven, cargo ecc.).
Non si riferiscono a grpc o protocol buffer.
## Go
Per installare Go:
- Ubuntu: `$ sudo apt install golang`
- Windows: https://go.dev/doc/install

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
Per installare java, è comodo lo strumento sdkman:
```console
$ curl -s "https://get.sdkman.io" | bash
$ source "$HOME/.sdkman/bin/sdkman-init.sh"
```
Bisogna scrivere esattamente i numeri della versione per installare la jdk corretta.
Ad esempio per installare jdk temurin 17 (attuale Long Term Support) con sdkman, prima trova la versione più attuale con:
```console
$ sdk list
```
E poi installala (ricontrollare i numeri di versione):
``` console
$ sdk install java 17.0.4.1-tem
```
---
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
Ubuntu: installa pip e virtualenv:
```console
$ sudo apt install python3-pip python3-virtualenv -y`
```

Crea un file main.py nella cartella python/

```python
print("Hello Python World")
```

Nella cartella python/:

```console
$ python main.py
```
## Rust
Per installare Rust:

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup self install stable
```

Nella cartella rust/ digita i seguenti comandi uno dopo l'altro per creare ed eseguire il progetto:
```console
$ cargo init
$ cargo run
```