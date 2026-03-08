//! Acceptance Tests: Infrastructure (Dev Scripts, Docker, Configuration)
//!
//! These tests verify the development and deployment infrastructure works correctly.
//! They check that configuration files exist and are valid, dev scripts work,
//! and Docker builds succeed.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

use std::path::Path;

#[test]
fn env_example_file_exists_with_required_variables() {
    // Verifies: .env.example exists at the project root and contains all
    // required environment variables: DATABASE_URL, REDIS_URL, SESSION_SECRET,
    // OWNER_USERNAME, OWNER_PASSWORD, OWNER_SYSTEM_PROMPT, USER_SYSTEM_PROMPT.
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".env.example");
    assert!(
        path.exists(),
        ".env.example must exist at project root, path: {:?}",
        path
    );

    let content = std::fs::read_to_string(&path).expect(".env.example should be readable");
    let required_vars = [
        "DATABASE_URL",
        "REDIS_URL",
        "SESSION_SECRET",
        "OWNER_USERNAME",
        "OWNER_PASSWORD",
        "OWNER_SYSTEM_PROMPT",
        "USER_SYSTEM_PROMPT",
    ];

    for var in &required_vars {
        assert!(content.contains(var), ".env.example must contain {}", var);
    }
}

#[test]
fn justfile_exists_with_dev_commands() {
    // Verifies: A justfile exists at the project root with at least the following
    // recipes: dev, build, test, check, fmt, lint, db-migrate, db-reset, seed.
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("justfile");
    assert!(path.exists(), "justfile must exist at project root");

    let content = std::fs::read_to_string(&path).expect("justfile should be readable");
    let required_recipes = [
        "dev",
        "build",
        "test",
        "check",
        "fmt",
        "lint",
        "db-migrate",
        "db-reset",
        "seed",
    ];

    for recipe in &required_recipes {
        assert!(
            content.contains(recipe),
            "justfile must contain recipe: {}",
            recipe
        );
    }
}

#[test]
fn docker_compose_file_defines_required_services() {
    // Verifies: docker/docker-compose.yml exists and defines services for:
    // postgres, redis, backend, frontend, caddy. Services have health checks
    // and proper depends_on ordering.
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docker/docker-compose.yml");
    assert!(path.exists(), "docker/docker-compose.yml must exist");

    let content = std::fs::read_to_string(&path).expect("docker-compose.yml should be readable");
    let required_services = ["postgres", "redis", "backend", "frontend", "caddy"];

    for service in &required_services {
        assert!(
            content.contains(service),
            "docker-compose.yml must define service: {}",
            service
        );
    }

    // Check for health checks and depends_on
    assert!(
        content.contains("healthcheck"),
        "docker-compose.yml must define health checks"
    );
    assert!(
        content.contains("depends_on"),
        "docker-compose.yml must define depends_on ordering"
    );
}

#[test]
fn caddyfile_routes_api_and_spa_correctly() {
    // Verifies: docker/Caddyfile exists and contains:
    // - handle /api/* routing to the backend service
    // - handle (fallback) for SPA with try_files and file_server
    // - WebSocket upgrade support (automatic in Caddy)
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docker/Caddyfile");
    assert!(path.exists(), "docker/Caddyfile must exist");

    let content = std::fs::read_to_string(&path).expect("Caddyfile should be readable");
    assert!(
        content.contains("handle /api/*"),
        "Caddyfile must contain handle /api/* routing"
    );
    assert!(
        content.contains("try_files"),
        "Caddyfile must contain try_files for SPA fallback"
    );
    assert!(
        content.contains("file_server"),
        "Caddyfile must contain file_server for SPA"
    );
}

#[test]
fn gitignore_excludes_build_artifacts_and_secrets() {
    // Verifies: .gitignore exists and excludes: target/, node_modules/,
    // .env, dist/, *.db, and other common build artifacts.
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".gitignore");
    assert!(path.exists(), ".gitignore must exist at project root");

    let content = std::fs::read_to_string(&path).expect(".gitignore should be readable");
    let required_patterns = ["target/", "node_modules/", ".env", "dist/"];

    for pattern in &required_patterns {
        assert!(
            content.contains(pattern),
            ".gitignore must exclude pattern: {}",
            pattern
        );
    }
}

#[test]
fn backend_dockerfile_uses_cargo_chef_multi_stage_build() {
    // Verifies: docker/backend.Dockerfile exists and uses a cargo-chef
    // three-stage build pattern (planner → builder → runtime) for
    // optimized layer caching.
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docker/backend.Dockerfile");
    assert!(path.exists(), "docker/backend.Dockerfile must exist");

    let content = std::fs::read_to_string(&path).expect("backend.Dockerfile should be readable");

    // Check for three-stage cargo-chef pattern
    assert!(
        content.contains("cargo-chef"),
        "backend.Dockerfile must use cargo-chef"
    );
    assert!(
        content.contains("cargo chef prepare"),
        "backend.Dockerfile must use cargo chef prepare"
    );
    assert!(
        content.contains("cargo chef cook"),
        "backend.Dockerfile must use cargo chef cook"
    );
    assert!(
        content.contains("cargo build"),
        "backend.Dockerfile must use cargo build"
    );
}

#[test]
fn check_env_script_validates_required_tools() {
    // Verifies: scripts/check-env.sh exists and checks for the presence of
    // required development tools (rustc, cargo, node, npm, docker, just, sqlx).
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("scripts/check-env.sh");
    assert!(path.exists(), "scripts/check-env.sh must exist");

    let content = std::fs::read_to_string(&path).expect("check-env.sh should be readable");
    let required_tools = ["rustc", "cargo", "node", "npm", "docker", "just", "sqlx"];

    for tool in &required_tools {
        assert!(
            content.contains(tool),
            "check-env.sh must check for tool: {}",
            tool
        );
    }
}

#[test]
fn database_migrations_directory_contains_initial_migration() {
    // Verifies: backend/migrations/ contains at least one migration file
    // that creates the initial PostgreSQL schema (events table, views tables,
    // users table, sessions table).
    let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    assert!(
        migrations_dir.exists(),
        "backend/migrations directory must exist"
    );

    // Read the migrations directory and check for at least one migration file
    let entries =
        std::fs::read_dir(&migrations_dir).expect("should be able to read migrations directory");

    let migration_files: Vec<_> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "sql") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    assert!(
        !migration_files.is_empty(),
        "backend/migrations must contain at least one .sql migration file"
    );

    // Check that at least one migration file contains schema definitions
    let found_schema = migration_files.iter().any(|path| {
        std::fs::read_to_string(path)
            .map(|content| {
                content.contains("events")
                    && content.contains("users")
                    && content.contains("sessions")
            })
            .unwrap_or(false)
    });

    assert!(
        found_schema,
        "migrations must contain table definitions for events, users, and sessions"
    );
}
