# Linux Protegido

Herramienta CLI en Rust para coordinar copias de seguridad en Linux. En esta primera etapa solo valida configuración; todavía no ejecuta backups ni modifica archivos del usuario.

## Estado

**MVP inicial — fase 1:** CLI, lectura TOML y logging. Restic, restauración, verificación y systemd se incorporarán después de probar esta base.

## Requisitos

- Rust estable y Cargo.
- Arch Linux u otra distribución Linux.

## Compilar y probar

```bash
cargo check
cargo test
cargo run -- --config config/example.toml config-check
cargo run -- --config config/example.toml status
```

Para activar logs de depuración:

```bash
RUST_LOG=debug cargo run -- --config config/example.toml config-check
```

## Seguridad

- Esta versión no ejecuta Restic ni realiza operaciones de backup.
- No guardes contraseñas ni claves en archivos TOML.
- Prueba primero con datos ficticios y un repositorio aislado.

## Hoja de ruta

1. CLI y validación TOML.
2. Errores tipados y logging.
3. Integración segura con Restic (timeouts, códigos de salida y sin exponer secretos).
4. Backup y restauración de prueba.
5. Verificación y política de retención.
6. Timers de systemd e instalador.

## Licencia

MIT.
