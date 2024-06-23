# ======================== development commands ============================= #

doc:
    cargo doc --open

# === lints === #

lint:
    cargo clippy --all-targets

# === watch command output === #

watch:
    cargo watch --clear --why \
      --exec 'check --all-features --all-targets'

watch-short:
    cargo watch --clear --why \
      --exec 'check --message-format=short --all-features --all-targets'

watch-test:
    cargo watch --clear --why \
      --exec 'test --all-features --all-targets'
