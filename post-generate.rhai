// Rename template files.
file::rename(".github/workflows/ci.yaml.template", ".github/workflows/ci.yaml");
file::rename(".github/workflows/release.yaml.template", ".github/workflows/release.yaml");
file::rename("Cargo.toml.template", "Cargo.toml");
file::rename("README.md.template", "README.md");
file::rename("src/bin/debug.rs.template", "src/bin/debug.rs");
file::rename("src/bin/run.rs.template", "src/bin/run.rs");
file::rename("src/core/window.rs.template", "src/core/window.rs");
file::rename("src/menu/main.rs.template", "src/menu/main.rs");

// Generate `Cargo.lock`.
system::command("cargo", ["update", "--package", variable::get("project-name")]);
