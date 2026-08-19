#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_NAME="$(basename "$0")"
readonly ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly ROOT_TOML="$ROOT_DIR/Cargo.toml"

log() { printf '[%s] %s\n' "$(date '+%H:%M:%S')" "$*"; }
die() { log "ERROR: $*" >&2; exit 1; }

usage() {
    cat <<EOF
Uso: $SCRIPT_NAME <nombre1> [nombre2 ...]

Crea mini sub-proyectos Cargo (cada uno con su propio Cargo.toml y
src/main.rs) y los registra como miembros del workspace en $ROOT_TOML.

Cada sub-proyecto se ejecuta con:  cargo run -p <nombre>

Ejemplos:
  $SCRIPT_NAME algo-burbuja
  $SCRIPT_NAME algo-quick algo-merge algo-insertion
EOF
}

root_package_name() {
    # Extrae el nombre del package raíz ([package] ... name = "..."), si existe.
    # En un virtual workspace (sin [package]) devuelve vacío.
    [[ -f "$ROOT_TOML" ]] || return 0
    local line in_pkg=0
    local re='^name[[:space:]]*=[[:space:]]*"([^"]+)"'
    while IFS= read -r line; do
        if [[ "$line" == "[package]" ]]; then
            in_pkg=1
        elif [[ "$line" == \[* ]]; then
            in_pkg=0
        fi
        if ((in_pkg)) && [[ "$line" =~ $re ]]; then
            printf '%s' "${BASH_REMATCH[1]}"
            return 0
        fi
    done < "$ROOT_TOML"
}

validate_name() {
    local name="$1"
    local root_pkg
    # Nombre de paquete Cargo: minúscula inicial y mínimo 2 caracteres,
    # después solo minúsculas, dígitos, '-' o '_'.
    if ! [[ "$name" =~ ^[a-z][a-z0-9_-]+$ ]]; then
        die "'$name' no es un nombre válido (empieza con minúscula; solo minúsculas, dígitos, '-' o '_'; mínimo 2 caracteres)"
    fi
    if [[ -e "$ROOT_DIR/$name" ]]; then
        die "ya existe '$ROOT_DIR/$name'"
    fi
    # Un sub-proyecto con el mismo nombre que el package raíz dejaría dos
    # packages con el mismo nombre y rompería todo el workspace.
    root_pkg="$(root_package_name)"
    if [[ -n "$root_pkg" && "$name" == "$root_pkg" ]]; then
        die "'$name' coincide con el nombre del package raíz del workspace"
    fi
}

ensure_workspace() {
    # cargo new registra el sub-proyecto en los members del workspace por sí
    # solo; aquí solo creamos la sección [workspace] si todavía no existe.
    if ! grep -q '^\[workspace\]' "$ROOT_TOML"; then
        log "Añadiendo sección [workspace] a Cargo.toml"
        printf '\n[workspace]\nmembers = []\n' >> "$ROOT_TOML"
    fi
}

write_main() {
    local dir="$1"
    cat > "$dir/src/main.rs" <<'EOF'
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    println!("{}", input);
}
EOF
}

new_subproject() {
    local name="$1"
    log "Creando sub-proyecto '$name'..."
    cargo new --vcs none "$name" >/dev/null
    write_main "$name"
    log "Listo. Ejecuta con: cargo run -p $name"
}

if [[ $# -eq 0 ]]; then
    usage
    exit 1
fi

for arg in "$@"; do
    case "$arg" in
        -h|--help) usage; exit 0 ;;
    esac
done

cd "$ROOT_DIR"
ensure_workspace
for name in "$@"; do
    validate_name "$name"
    new_subproject "$name"
done
