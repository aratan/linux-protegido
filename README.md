# Linux Protegido

Herramienta CLI en Rust para coordinar copias de seguridad en Linux. La fase actual valida configuración y comprueba la disponibilidad de Restic; todavía no ejecuta backups ni modifica archivos del usuario.

## Estado

**Fase 2 — comprobación de Restic:** el subcomando `restic-version` ejecuta únicamente `restic version`, captura su salida y comprueba el código de salida. No inicializa repositorios, no lee credenciales y no crea copias.

## Requisitos

- Rust estable y Cargo.
- Restic instalado para probar `restic-version`.
- Arch Linux u otra distribución Linux.

## Compilar y probar

```bash
cargo check
cargo test
cargo run -- --config config/example.toml config-check
cargo run -- --config config/example.toml status
cargo run -- restic-version
```

Para activar logs de depuración:

```bash
RUST_LOG=debug cargo run -- restic-version
```

Si Restic no está instalado, el comando informa de que no encuentra el ejecutable en `PATH`. Si está instalado, muestra la versión devuelta por Restic.

## Seguridad

- `restic-version` solo consulta la versión: no ejecuta operaciones sobre repositorios ni archivos.
- Esta versión todavía no ejecuta operaciones de backup o restauración.
- No guardes contraseñas ni claves en archivos TOML.
- Prueba primero con datos ficticios y un repositorio aislado.

## Hoja de ruta

1. CLI y validación TOML.
2. Logging y errores contextualizados.
3. Comprobación de Restic.
4. Integración segura con Restic (timeouts, códigos de salida y sin exponer secretos).
5. Backup y restauración de prueba.
6. Verificación y política de retención.
7. Timers de systemd e instalador.

## Licencia

MIT.
