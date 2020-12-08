Before submitting a PR, and should probably run `./scripts/internal-tests.sh` and/or `./scripts/intergration-tests-all.sh` to make sure everything is fine.

#### Integration tests

Tests that involve databases, therefore, you will need a local installation or a remote instance with connection access.

#### Internal tests

Runs unit tests, `rustfmt`, `clippy` and `libfuzzer-sys` to enhance security and robustness.

#### Database credentials

In a testing environment, the following credentials are expected:

- **MS-SQL**: A database `oapth` and an user `sa` with password `yourStrong_Password`
- **Everything else**: A database `oapth` and an user `oapth` with password `oapth`

#### Databases

If you don't want to manually install and configure all databases in your system, checkout `scripts/podman-start.sh` where each database image is pulled and executed automatically.
