# `Just a command runner` script
# Web: https://github.com/casey/just
# Man: https://just.systems/man/en/
#
# Generate Bash completion script:
# `just --completions bash > ~/.local/share/bash-completion/completions/just`
# and reload console

# by default (no params), list the recipes
default:
    @just --list

# debug-build given example
b *ARGS:
    cargo b --example {{ARGS}}
    cargo size --example {{ARGS}} -- -B

# release-build given example
br *ARGS:
    cargo b --release --example {{ARGS}}
    cargo size --example {{ARGS}} --release -- -B

# https://docs.rust-embedded.org/book/start/qemu.html
# debug-build and run in QEMU, eg: just qemu rtwins
qemu *ARGS:
    cargo b --example {{ARGS}} --features "qemu"
    cargo size --example {{ARGS}} --features "qemu" -- -B
    qemu-system-arm \
        -cpu cortex-m3 \
        -machine lm3s6965evb \
        -nographic \
        -semihosting-config enable=on,target=native \
        -kernel target/thumbv7m-none-eabi/debug/examples/{{ARGS}}

# runs QEMU with Gdb server
qemu-gdbserv *ARGS:
    cargo b --example {{ARGS}} --features "qemu"
    cargo size --example {{ARGS}} --features "qemu" -- -B
    qemu-system-arm \
        -cpu cortex-m3 \
        -machine lm3s6965evb \
        -nographic \
        -semihosting-config enable=on,target=native \
        -gdb tcp::3333 \
        -S \
        -kernel target/thumbv7m-none-eabi/debug/examples/{{ARGS}}

# attach Gdb to the QEMU gdb server
gdb-qemu *ARGS:
    gdb-multiarch -tui -ex "target remote :3333" -ex "b main" -q target/thumbv7m-none-eabi/debug/examples/{{ARGS}}
