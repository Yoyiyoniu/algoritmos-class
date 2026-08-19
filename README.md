# Hola persona que que lee esto,

Este proyecto usa `rust-script` debido a que rust vanilla no permite ejecutar archivos por si solos,
asique si quieres ejecutar un script hazlo de la siguiente forma

```bash
rust-script $file_route
```


## Instalacion de los paquetes

```bash
cargo install rust-script
```

nota: si tienes problemas y no se ejecuta recuerda cargar el `~/.cargo/bin`

```bash
fish_add_path ~/.cargo/bin  # en caso de fish

source ~/.cargo/bin         # en caso de una terminal comun :D  
```

# Muy importante!
Todo se trabajara dentro de `./src/bin` debido a que aqui si se puede ejecutar adecuadamente el codigo y rust no se pone delicado.
