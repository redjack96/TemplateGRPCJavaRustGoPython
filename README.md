# Compilare e/o eseguire i progetti
Per scaricare protoc:
- Ubuntu: `sudo apt install -y protobuf-compiler`
- MacOS: `brew install protobuf`
- Windows: Scarica la più recente da https://github.com/protocolbuffers/protobuf/releases/

## Go
Per Intellij: installa il plugin "Go".

Prima bisogna scaricare le dipendenze. Entra nella cartella go ed esegui:
```console
$ go get google.golang.org/grpc
$ go get -u google.golang.org/protobuf/cmd/protoc-gen-go
$ go get -u google.golang.org/grpc/cmd/protoc-gen-go-grpc
```
Verranno installati 6 pacchetti.

Compila il file proto con protoc (dalla cartella go/):
``` consoele
$ protoc --go_out .  --go-grpc_out . proto/hellogoworld.proto
```
Entra nella cartella go/server ed esegui:
```console
$ go run server.go
```
Entra nella cartella go/client ed esegui:
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
Per Intellij: install il Plugin "Python"
### Prerequisiti
Nella cartella python/ bisogna creare un virtual environment per evitare conflitti con altre versioni di python installate.

Ubuntu:
- Se non hai pip, installalo con `sudo apt install python3-pip`.
- installa virtualenv con `sudo apt install python3-virtualenv`
- usa python3 invece di python

Windows: 
- Installa i virtualenv dalla versione attuale di python, ad esempio `python -m pip install virtualenv`

Crea un virtual enviroment (viene creata una cartella venv, ignorata dal .gitignore):
``` console
$ virtualenv venv
```
Dalla cartella che contiene venv/, avvia il virtual environment:
- Per Windows: `$ .\venv\Scripts\activate`
- Per Unix: `$ source venv/bin/activate`

A questo punto si è entrati nel virtual environment e puoi eseguire `python`, indipendentemente da quale versione di python hai installato. Bisogna sempre usare lo script activate per fare tutto il resto!

Solo IntelliJ: a questo punto è conveniente importare il modulo python:
- Ctrl+Alt+Shift+S > Modules > + > scegli la cartella Python > vai avanti (includi Django)
- Ctrl+Alt+Shift+S > seleziona il modulo Python > Scheda Dependencies > Seleziona python3.x invece al posto della JDK.

Aggiorna pip e installa i package grpcio e grpcio-tools.
``` console
$ python -m pip install --upgrade pip
$ python -m pip install grpcio grpcio-tools
```
Dalla cartella radice (python/) Genera il codice con:
``` console
$ python -m grpc_tools.protoc -Iproto helloworld.proto --python_out . --grpc_python_out .
```
Non è possibile inserire i file generati in una cartella a parte, altrimenti gli import di python non funzioneranno.
### Avvio server e client python
Dalla cartella che contiene venv/, avvia il virtual environment (se già è attivo, skippa):
- Per Windows: `$ .\venv\Scripts\activate`
- Per Unix: `$ source venv/bin/activate`

Esegui `python server.py` e poi `python client.py` su due terminali diversi.
Per uscire dal virtual environment, ovvero per togliere (venv) all'inizio della riga di comando, usa il comando
``` console
$ deactivate
```
Il comando deactivate è disponibile solo quando è attivo il virtual environment, cioè quando vedi (venv) nella linea di comando.
## Rust
Per Intellij: installa il plugin "Rust".

Per compilare ed eseguire, entra nella cartella rust ed esegui nel primo terminale (ci vuole un po' la prima volta):
```console
$ cargo run --bin server
```
poi in un altro terminale, sempre nella cartella rust:
```console
$ cargo run --bin client
```

# Script lang-run
Una volta configurati i linguaggi, è possibile usare i corrispottivi script. 
Per avviare un server, eseguire UNO tra questi comandi

```console
$ sh go-run.sh server
$ sh java-run.sh server
$ sh rust-run.sh server
$ sh python-run.sh server
```

Per avviare un client eseguire uno qualsiasi tra:

```console
$ sh go-run.sh client
$ sh java-run.sh client
$ sh rust-run.sh client
$ sh python-run.sh client
```