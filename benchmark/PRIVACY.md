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
- Los datos se envían a una API REST, que a su vez los reenvía a un webhook de Discord en formato de mensaje.
- No se almacenan en bases de datos ni se comparten con terceros fuera del webhook de Discord.

## Opcionalidad en el envío de datos  
- Al finalizar la prueba, se te preguntará si deseas enviar los datos al servidor.  
- Si eliges **"y"**, la información se enviará y se publicará en un webhook de Discord.  
- Si eliges **"n"**, los datos no se enviarán y se descartarán inmediatamente.  

## Glosario de términos  

- **Single-thread**: Prueba de rendimiento donde solo se usa un núcleo lógico del procesador para ejecutar la tarea.  
- **Multi-thread**: Prueba de rendimiento que utiliza múltiples núcleos lógicos del procesador para ejecutar la tarea en paralelo.  
- **Score (Puntuación de rendimiento)**: Valor que representa el rendimiento del CPU en base al tiempo que tarda en completar la tarea.  

### **Términos adicionales para el glosario**  
- **CPU (Unidad Central de Procesamiento)**: Componente principal de un computador que ejecuta instrucciones y procesa datos.  
- **Frecuencia del CPU**: Velocidad a la que opera el procesador, medida en megahercios (MHz) o gigahercios (GHz).  
- **Núcleos lógicos**: Subunidades dentro de un procesador que permiten la ejecución simultánea de múltiples procesos.  
- **API REST**: Interfaz que permite la comunicación entre sistemas mediante peticiones HTTP.  
- **Webhook**: Mecanismo que permite a una aplicación enviar información automáticamente a otra aplicación en tiempo real.  
- **Discord**: Plataforma de comunicación en línea que permite enviar mensajes, realizar llamadas de voz y video, y compartir contenido en comunidades.  
- **Host (nombre de host)**: Identificador único de un dispositivo dentro de una red.  
- **Benchmark**: Prueba de rendimiento utilizada para medir y comparar la velocidad de un sistema o componente de hardware.  

## ¿Cómo se calculan los scores?  

El score se obtiene con la siguiente fórmula:

score = (ITERATIONS / tiempo_en_segundos) / 100_000.0

Donde:
- **ITERATIONS** es la cantidad de operaciones realizadas en la prueba.
- **tiempo_en_segundos** es el tiempo total que tomó ejecutar la prueba.

Un menor tiempo de ejecución da un mayor score, lo que indica mejor rendimiento del CPU.
