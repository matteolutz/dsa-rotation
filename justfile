[group('project-agnostic')]
default:
    @just --list

[group("binaries")]
[arg("profile", pattern="release|debug")]
cli profile="debug":
    cargo run --bin cli {{ if profile == "release" { "--release" } else { "" } }}

[group("binaries")]
[working-directory: 'crates/ui']
ui script="dev":
    pnpm install
    pnpm tauri {{script}}

[group("development")]
[confirm("Are you sure you want to clean the cargo target directory? (y/N)")]
clean:
    cargo clean

[group("testing")]
test package="":
    cargo test {{ if package == "" { "" } else { f'-p {{package}}' } }} -- --nocapture
