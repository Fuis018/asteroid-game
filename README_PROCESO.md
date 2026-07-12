# Asteroid Shooter — Proceso de Elaboración

## Estado actual del proyecto
El proyecto ya está funcional y compilando correctamente. La versión actual incluye un jugador con vuelo 6DOF, un enemigo con varias fases de combate, UFOs que aparecen en checkpoints de salud y un sistema de depuración para ver colliders.

## Cambios principales incorporados
- Salud del jugador ajustada a 700.
- Salud del enemigo ajustada a 3000.
- El enemigo ahora patrulla con un recorrido amplio y fluido, con giros limitados a 45°.
- Se añadieron checkpoints de salud en 2100 y 1200 para generar UFOs.
- Los UFOs aparecen alrededor de la nave enemiga y se mueven con un patrón más artificial/strafeo.
- Los UFOs retroceden y se separan ligeramente cuando están cerca del jugador.
- El collider de los UFOs se ha movido y ampliado para que quede más visible.
- La victoria se activa cuando el enemigo desaparece, sin depender de que todos los UFOs ya hayan muerto.
- El modo de depuración con H muestra los colliders del jugador, enemigo, UFOs, asteroides y láseres.

## Arquitectura actual
- Bevy 0.19 + avian3d 0.7 para renderizado, física y colisiones.
- ECS modular en módulos como player, enemy, ufo, collision, debug_colliders, particles, ui y asteroids.
- Estados de juego: Loading → Playing → Victory/Defeat → Restart.

## Verificación reciente
- cargo check → compilación correcta.
