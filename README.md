# Average Benchmark

El proyecto consta de dos programas uno para hacer una prueba de rendimiento del cpu obteniendo una puntuación y el otro se encarga de recibir los resultados de los usuarios y publicarlos en canal de discord usando el web hook, agregando un rate limit para evitar que un usuario envié multiples benchmarks.

## Ejecución usando docker

```sh
docker run --rm -it theaverageunknown/cpu-benchmark
```

```sh
docker run --rm -e DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/..." theaverageunknown/cpu-benchmark-api
```

## 1. Construcción de Imágenes de docker

Para construir las imágenes de Docker, ejecuta los siguientes comandos en el directorio raíz del proyecto donde se encuentra el `Dockerfile`.

### Construir la imagen de la API

```sh
docker build -t api-image --target api .
```

## 2. Ejecución de imágenes de docker

```sh
docker run --rm -it benchmark-image
```

## Ejecución usando nix

Si estas en nix abre una nix shell con las herramientas de desarrollo de rust con el flake usando:

```sh
nix develop
```

para ejecutar el benchmark usa:

```sh
cargo run --bin benchmark
```

y para ejecutar la api

```sh
DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/..." cargo run --bin api
```

o colocar un .env con la variable `DISCORD_WEBHOOK_URL` en la raíz del proyecto.

Si quieres cambiar el dominio del backend puede usar la variable de entorno de tu sistema `BACKEND_URL`
Ejemplo:
```
export BACKEND_URL="http://localhost:8080" && cargo run --bin average-benchmark --release
```
