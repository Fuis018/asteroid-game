# Asteroid Shooter — Todo / Estado del Proyecto

## Estado general
✅ Proyecto funcional y compilando.

## Funcionalidades completadas
- Movimiento de la nave del jugador en 6DOF.
- Disparo recto del jugador.
- Enemigo con movimiento amplio y fluido.
- Fases de salud para spawnear UFOs.
- UFOs con comportamiento activo, retroceso y movimiento lateral.
- Colliders visibles con H.
- Condición de victoria al destruir al enemigo.

## Ajustes actuales de gameplay
- Player health: 700
- Enemy health: 3000
- UFOs spawned at enemy health 2100 and 1200
- UFO scale: 0.75
- UFO speed: 15
- Enemy movement speed: 22.5
- Enemy patrol orbit radius: 700
- Enemy turn smoothing limit: 45°
- Enemy turret count: 4

## Estructura relevante
- src/enemy.rs — lógica del enemigo, checkpoints, spawns de UFOs.
- src/ufo.rs — movimiento, disparo y comportamiento de UFOs.
- src/player.rs — movimiento y disparo del jugador.
- src/collision.rs — colisiones, victoria/derrota y limpieza de entidades.
- src/debug_colliders.rs — visualización de colliders.
- src/constants.rs — valores de balance y tuning.

## Cómo probarlo
1. Ejecutar cargo run.
2. Probar las fases del enemigo con el daño progresivo.
3. Pulsar H para ver colliders.
4. Reiniciar con R al terminar una partida.
