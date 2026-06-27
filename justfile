[arg("profile", pattern="release|debug")]
cli profile="debug":
    cargo run --bin cli {{ if profile == "release" { "--release" } else { "" } }}


[arg("profile", pattern="release|debug")]
[working-directory: 'crates/ui']
ui profile="debug":
    pnpm install
    pnpm tauri dev
