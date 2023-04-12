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

b *ARGS:
    cargo b --example {{ARGS}}
    cargo size --example {{ARGS}} -- -B

br *ARGS:
    cargo b --release --example {{ARGS}}
    cargo size --example {{ARGS}} --release -- -B

# build dbg and run in QEMU, eg: just qemu rtwins
qemu *ARGS:
    cargo b --example {{ARGS}} --features "qemu"
    cargo size --example {{ARGS}} --features "qemu" -- -B
    qemu-system-arm \
        -cpu cortex-m3 \
        -machine lm3s6965evb \
        -nographic \
        -semihosting-config enable=on,target=native \
        -kernel target/thumbv7m-none-eabi/debug/examples/{{ARGS}}

# TODO: rtwins shows nothing when run by `timeout`
# TODO: hello hangs and shows nothing when run by `timeout`

qemuto *ARGS:
    cargo b --example {{ARGS}}
    cargo size --example {{ARGS}} -- -B
    timeout 3s qemu-system-arm \
        -cpu cortex-m3 \
        -machine lm3s6965evb \
        -nographic \
        -semihosting-config enable=on,target=native \
        -kernel target/thumbv7m-none-eabi/debug/examples/{{ARGS}}
