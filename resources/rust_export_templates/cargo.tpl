[package]
name = "{{LABEL}}"
license = "MIT"
authors = [{{AUTHORS}}]
description = "{{TITLE}}"
version = "{{VERSION}}"
edition = "2024"
default-run = "{{LABEL}}"

[lib]]
name = "{{LABEL}}"
path = "src/lib.rs"

[dependencies]
env_logger = "0.11"
log = "0.4"
lorebook = { version = "{{LOREBOOK_VERSION}}", path = "{{LOREBOOK_PATH}}" }
mimalloc = "0.1"

[build-dependencies]
chrono = "0.4"

[profile.dev]
opt-level = 1
debug = true

[profile.release]
opt-level = 3
strip = true
lto = true
codegen-units = 1
panic = "abort"
debug-assertions = false
overflow-checks = false
package."*".opt-level = "s"

