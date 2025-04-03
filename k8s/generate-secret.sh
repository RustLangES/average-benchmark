#!/bin/bash

# Obtener la ruta del script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ENV_FILE="$SCRIPT_DIR/../.env"

if [ ! -f "$ENV_FILE" ]; then
    echo "❌ Error: Archivo .env no encontrado en $ENV_FILE"
    exit 1
fi

# Cargar las variables del .env de forma segura
set -o allexport
source "$ENV_FILE"
set +o allexport

if [ -z "$DISCORD_WEBHOOK_URL" ]; then
    echo "❌ Error: DISCORD_WEBHOOK_URL no está definida en el archivo .env"
    exit 1
fi

SECRET_FILE="$SCRIPT_DIR/secret.yaml"

# Eliminar el archivo si ya existe
rm -f "$SECRET_FILE"

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
kubectl apply -f "$SCRIPT_DIR/namespace.yaml"
kubectl apply -f "$SECRET_FILE"
kubectl apply -f "$SCRIPT_DIR/deployment.yaml"
kubectl apply -f "$SCRIPT_DIR/service.yaml"