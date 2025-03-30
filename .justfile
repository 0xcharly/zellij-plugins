import '.justfile.incl'

set shell := ['fish', '-c']

[doc('List all available commands')]
[private]
default:
    @just --list

[doc('Update the given flake inputs')]
[group('nix')]
update +inputs:
    for input in {{ inputs }}; do nix flake update --flake {{ justfile_directory() }} $input; done

[doc('Update all "distribution" inputs (nixpkgs, etc.)')]
[group('nix')]
update-nixpkgs:
    @just update nixpkgs nixpkgs-darwin

[doc('Update all "toolchain" inputs (flake-parts, etc.)')]
[group('nix')]
update-toolchain:
    @just update flake-parts git-hooks-nix treefmt-nix

[doc('Update all inputs')]
[group('nix')]
update-all:
    nix flake update

[group('dev')]
build *flavor="":
    cargo build --target=wasm32-wasip1 {{ flavor }}

[group('dev')]
debug:
    @just build
    env WASMTIME_BACKTRACE_DETAILS=1 zellij --layout {{ justfile_directory() }}/.launch-debug.kdl

[doc('Run pre-commit checks on all files')]
[group('devshell')]
fix:
    pre-commit run --all-files
