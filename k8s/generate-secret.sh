#!/bin/bash

# Obtener la ruta del script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Definir la ruta del archivo .env
ENV_FILE="$SCRIPT_DIR/../.env"

# Verificar si el archivo .env existe
if [ ! -f "$ENV_FILE" ]; then
    echo "❌ Error: Archivo .env no encontrado en $ENV_FILE"
    exit 1
fi

# Cargar las variables del .env
export $(grep -v '^#' "$ENV_FILE" | xargs)

# Verificar si la variable está definida
if [ -z "$DISCORD_WEBHOOK_URL" ]; then
    echo "❌ Error: DISCORD_WEBHOOK_URL no está definida en el archivo .env"
    exit 1
fi

# Definir la salida del secret.yaml
SECRET_FILE="$SCRIPT_DIR/secret.yaml"

# Generar el archivo de secreto
cat << EOF > "$SECRET_FILE"
apiVersion: v1
kind: Secret
metadata:
  name: discord-webhook-secret
  namespace: average-benchmark
type: Opaque
stringData:
  DISCORD_WEBHOOK_URL: "$DISCORD_WEBHOOK_URL"
EOF

echo "✅ Secreto generado correctamente en $SECRET_FILE"

# Aplicar los manifiestos de Kubernetes
k8s kubectl apply -f "$SCRIPT_DIR/namespace.yaml"
k8s kubectl apply -f "$SECRET_FILE"
k8s kubectl apply -f "$SCRIPT_DIR/deployment.yaml"
k8s kubectl apply -f "$SCRIPT_DIR/service.yaml"