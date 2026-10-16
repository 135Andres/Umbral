<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/logo-dark.svg">
    <img src="assets/brand/logo-light.svg" alt="Umbral" width="220">
  </picture>
</p>

<h2 align="center">Tu proyecto continúa.<br>Aunque cambies de agente.</h2>

<p align="center"><a href="README.md">English</a> · <b>Español</b></p>

<p align="center">
  <a href="docs/candidates/CONTINUITY-PILOT.md"><img alt="Continuidad: en diseño" src="https://img.shields.io/badge/continuidad-en_dise%C3%B1o-6d28d9"></a>
  <a href="docs/versions/v0.2.md"><img alt="Instrumento disponible: v0.2" src="https://img.shields.io/badge/observer-v0.2-52525b"></a>
  <a href="LICENSE"><img alt="GPL-3.0-or-later" src="https://img.shields.io/badge/license-GPL--3.0--or--later-52525b"></a>
</p>

---

Deberías poder dejar una tarea, volver después o cambiar de herramienta de IA sin tener
que explicar otra vez todo tu proyecto. **Ese es el objetivo de Umbral; todavía no está
construido.** Esta página separa lo que puedes usar hoy de lo que hará al terminar el desarrollo.

## Hoy: el observador v0.2

Lo único disponible es una herramienta de línea de comandos que observa una carpeta:

- registra la carpeta sin escribir dentro de ella (`init`);
- toma una observación con la huella del contenido de cada archivo (`observe`);
- dice qué cambió entre las dos últimas observaciones (`changes`), el estado actual (`status`)
  y la historia de un archivo (`show`);
- distingue lo leído de nuevo de lo reutilizado, y dice «desconocido» cuando no pudo leer algo.

```sh
cargo run --locked -p umbral -- init /ruta/al/proyecto
cargo run --locked -p umbral -- observe /ruta/al/proyecto
# Trabaja en el proyecto y vuelve a observarlo.
cargo run --locked -p umbral -- observe /ruta/al/proyecto
cargo run --locked -p umbral -- changes /ruta/al/proyecto
```

Detalles en la [guía del observador](umbral/README.md) (en inglés), su
[contrato de salida](umbral/CONTRACT.md) y la [evidencia de la versión](docs/versions/v0.2.md).

**Lo que hoy no hace:** no guarda tareas, objetivos ni decisiones; no tiene MCP; no se conecta
a ningún agente; no ayuda a continuar una sesión. La v0.2 tampoco está cerrada: sus criterios
de aceptación siguen pendientes ([evidencia](docs/versions/v0.2.md)).

## Al terminar el desarrollo: continuidad entre agentes

<p align="center">
  <img src="assets/brand/continuity-es.svg" alt="Recorrido propuesto: trabajar en tu herramienta de IA, conservar contexto con referencias a sus fuentes y continuar con el mismo agente o con otro." width="960">
</p>

Lo que buscamos, aceptado en [UD-039](docs/decisions/DECISIONS.md) y **sin implementar**:

1. Conectas tu herramienta de IA a un repositorio Git una vez.
2. Trabajas como siempre; el agente guarda el objetivo, las decisiones, el siguiente paso
   y los archivos en los que se apoya.
3. En otra sesión, o con otro agente, pides continuar. Umbral entrega ese registro y marca
   qué archivos cambiaron desde que se guardó.

Que un archivo cambie significa **revísalo**; no demuestra que la conclusión anterior sea
falsa. Umbral no prueba que una IA entendió un archivo ni comprueba dependencias que nadie
registró.

| Pieza | Función | Estado |
|---|---|---|
| Programa local | Guardar y recuperar tareas; contrastar sus archivos con la copia actual | Diseño en revisión |
| Acceso MCP | Que los agentes usen el programa local | Planeado |
| Integración con un agente | Que pedir «continúa» llegue a Umbral en un cliente real | Planeada; hay que demostrar la invocación |

El orden es ese: programa local, MCP, integración. Al final se compara con Git más un buen
archivo de continuidad en una tarea real de Umbral (corrección, esfuerzo, contexto e
interrupciones innecesarias). No se promete ahorro de tiempo ni de tokens antes de medirlo.
Límites del piloto: [alcance](docs/candidates/CONTINUITY-PILOT.md) (en inglés).

## Encuentra lo que necesitas

| Necesidad | Entrada |
|---|---|
| Dirección actual y siguiente trabajo | [Dirección del proyecto](docs/canonical/PROJECT-DIRECTION.md) |
| Nuevo piloto de continuidad | [Alcance y orden de construcción](docs/candidates/CONTINUITY-PILOT.md) |
| Herramienta existente | [Guía del observador](umbral/README.md) |
| Decisiones y evidencia | [Decisiones](docs/decisions/DECISIONS.md) · [Versiones](docs/versions/README.md) · [Experimentos](experiments/) |
| Colaborar o trabajar como agente | [Contribuir](CONTRIBUTING.md) · [AGENTS.md](AGENTS.md) |
| Explorar la investigación | [Documentación](docs/README.md) · [Índice de investigación](research/RESEARCH-INDEX.md) |
| Informar de una vulnerabilidad | [Seguridad](SECURITY.md) |

Local. Código abierto. Tus archivos se quedan en tus herramientas.

[GPL-3.0-or-later](LICENSE) · [Decisión de licencia](docs/decisions/DECISIONS.md)
