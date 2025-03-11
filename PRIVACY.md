# Política de Privacidad

Esta aplicación recopila información sobre tu CPU y sistema operativo para generar un reporte de rendimiento (benchmark). 

## Datos recopilados
- Marca y modelo del CPU
- Frecuencia del CPU
- Identificador del proveedor del CPU
- Número de núcleos lógicos
- Puntuaciones de rendimiento (single-thread y multi-thread)
- Nombre del host del sistema
- Sistema operativo

## Uso de los datos
- Los datos se envían a una API, que a su vez los reenvía a un webhook de Discord en formato de mensaje.
- No se almacenan en bases de datos ni se comparten con terceros fuera del webhook de Discord.

## Opcionalidad en el envío de datos  
- Al finalizar la prueba, se te preguntará si deseas enviar los datos al servidor.  
- Si eliges **"y"**, la información se enviará y se publicará en un webhook de Discord.  
- Si eliges **"n"**, los datos no se enviarán y se descartarán inmediatamente.  
