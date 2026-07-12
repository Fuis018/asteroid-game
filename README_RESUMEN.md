# Asteroid Shooter — Resumen Ejecutivo

## Qué es
Asteroid Shooter es un videojuego 3D de combate espacial desarrollado en Rust con Bevy 0.19. El jugador controla una nave en vuelo 6DOF mientras combate a un enemigo principal y a oleadas de UFOs.

## Estado actual
- Jugador: 700 de salud.
- Enemigo: 3000 de salud.
- UFOs: escala 0.75, velocidad 15, comportamiento de retroceso y strafe.
- Enemigo: recorrido amplio tipo figura de ocho, giro limitado a 45°.
- Checkpoints de UFOs: 2100 y 1200 HP.
- Segunda oleada: 6 UFOs en formación rectangular (3 a cada lado).
- Victoria: al destruir al enemigo, aunque queden UFOs vivos.

## Controles
- WASD: rotar la nave.
- Shift: acelerar.
- Ctrl: freno.
- Q/E: roll.
- Espacio: disparar.
- H: mostrar/ocultar colliders.
- R: reiniciar tras victoria o derrota.

## Tecnologías
- Rust
- Bevy 0.19
- avian3d 0.7
- bevy_asset_loader 0.27
- rand 0.9

## Verificación
- cargo check → OK.
