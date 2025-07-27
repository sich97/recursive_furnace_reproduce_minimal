# Steps to reproduce loco.rs issue #1555

Using the following dependencies:
- ring v0.17.14
- rustls v0.23.29
- rustls-webpki v0.103.4
- jsonwebtoken v9.3.1
- sqlx-core v0.8.6
- tokio-rustls v0.26.2
- lettre v0.11.17
- sqlx-sqlite v0.8.6
- sqlx-postgres v0.8.6
- sqlx v0.8.6
- sea-query-binder v0.7.0
- sea-orm v1.1.14
- sea-schema v0.16.2
- sea-orm-migration v1.1.14
- loco-rs v0.16.2

### These steps are not necesarily all required in order to reproduce, but they aim to be the minimally necessary steps to reproduce

## 1. Set up a new saas app with server side rendering and sqllite
(base) simon@POSEIDON:~/git$ loco new

✔ ❯ App name? · recursive_furnace_reproduce_minimal

✔ ❯ What would you like to build? · Saas App with server side rendering

✔ ❯ Select a DB Provider · Sqlite

✔ ❯ Select your background worker type · Async (in-process tokio async tasks)

🚂 Loco app generated successfully in:
/home/simon/git/recursive_furnace_reproduce_minimal

## 2. Run initial project once

## 3. Create global_materials table
cargo loco generate model global_materials user:references:created_by name:text hash:text^

## 4. Create global_recipes table
cargo loco generate model global_recipes user:references:created_by machine_fuel_consumption:double hash:text^

## 5. Create migration for input_materials join table between global_recipes table and global_materials table
cargo loco generate migration CreateJoinTableGlobal_recipesAndGlobal_materials quantity:int! chance:float!

## 6. (ERROR HERE) Run migration for the join table creation
cargo loco db migrate

### Output from running the above command:
```
   Compiling ring v0.17.14
   Compiling rustls v0.23.29
   Compiling rustls-webpki v0.103.4
   Compiling jsonwebtoken v9.3.1
   Compiling sqlx-core v0.8.6
   Compiling tokio-rustls v0.26.2
   Compiling lettre v0.11.17
   Compiling sqlx-sqlite v0.8.6
   Compiling sqlx-postgres v0.8.6
   Compiling sqlx v0.8.6
   Compiling sea-query-binder v0.7.0
   Compiling sea-orm v1.1.14
   Compiling sea-schema v0.16.2
   Compiling sea-orm-migration v1.1.14
   Compiling loco-rs v0.16.2
   Compiling migration v0.1.0 (/home/simon/git/recursive_furnace_reproduce_minimal/migration)
   Compiling recursive_furnace_reproduce_minimal v0.1.0 (/home/simon/git/recursive_furnace_reproduce_minimal)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 23.95s
     Running `target/debug/recursive_furnace_reproduce_minimal-cli generate migration CreateJoinTableGlobal_recipesAndGlobal_materials 'quantity:int'\!'' 'chance:float'\!''`
added: "migration/src/m20250727_131039_create_join_table_global_recipes_and_global_materials.rs"
injected: "migration/src/lib.rs"
injected: "migration/src/lib.rs"
* Migration for `CreateJoinTableGlobal_recipesAndGlobal_materials` added! You can now apply it with `$ cargo loco db migrate`.

(base) simon@POSEIDON:~/git/recursive_furnace_reproduce_minimal$ cargo loco db migrate
   Compiling migration v0.1.0 (/home/simon/git/recursive_furnace_reproduce_minimal/migration)
warning: unused import: `schema::*`
 --> migration/src/m20250727_131039_create_join_table_global_recipes_and_global_materials.rs:1:37
  |
1 | use sea_orm_migration::{prelude::*, schema::*};
  |                                     ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `m`
 --> migration/src/m20250727_131039_create_join_table_global_recipes_and_global_materials.rs:8:24
  |
8 |     async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
  |                        ^ help: if this is intentional, prefix it with an underscore: `_m`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `m`
  --> migration/src/m20250727_131039_create_join_table_global_recipes_and_global_materials.rs:12:26
   |
12 |     async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
   |                          ^ help: if this is intentional, prefix it with an underscore: `_m`

   Compiling recursive_furnace_reproduce_minimal v0.1.0 (/home/simon/git/recursive_furnace_reproduce_minimal)
warning: `migration` (lib) generated 3 warnings (run `cargo fix --lib -p migration` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.84s
     Running `target/debug/recursive_furnace_reproduce_minimal-cli db migrate`
2025-07-27T13:10:52.941261Z  WARN app: loco_rs::boot: migrate: environment=development
2025-07-27T13:10:52.941572Z  INFO app: sea_orm_migration::migrator: Applying all pending migrations environment=development
2025-07-27T13:10:52.942874Z  INFO app: sea_orm_migration::migrator: Applying migration 'm20250727_131039_create_join_table_global_recipes_and_global_materials' environment=development

thread 'main' panicked at migration/src/m20250727_131039_create_join_table_global_recipes_and_global_materials.rs:9:9:
not yet implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library/std/src/panicking.rs:697:5
   1: core::panicking::panic_fmt
             at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library/core/src/panicking.rs:75:14
   2: core::panicking::panic
             at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library/core/src/panicking.rs:145:5
   3: <migration::m20250727_131039_create_join_table_global_recipes_and_global_materials::Migration as sea_orm_migration::MigrationTrait>::up::{{closure}}
             at ./migration/src/m20250727_131039_create_join_table_global_recipes_and_global_materials.rs:9:9
   4: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /home/simon/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/future.rs:124:9
   5: sea_orm_migration::migrator::exec_up::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sea-orm-migration-1.1.14/src/migrator.rs:386:31
   6: sea_orm_migration::migrator::MigratorTrait::up::{{closure}}::{{closure}}::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sea-orm-migration-1.1.14/src/migrator.rs:235:67
   7: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /home/simon/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/future.rs:124:9
   8: sea_orm_migration::migrator::exec_with_connection::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sea-orm-migration-1.1.14/src/migrator.rs:270:25
   9: sea_orm_migration::migrator::MigratorTrait::up::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sea-orm-migration-1.1.14/src/migrator.rs:237:10
  10: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /home/simon/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/future.rs:124:9
  11: loco_rs::db::migrate::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/loco-rs-0.16.2/src/db.rs:232:21
  12: loco_rs::boot::run_db::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/loco-rs-0.16.2/src/boot.rs:311:47
  13: loco_rs::cli::main::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/loco-rs-0.16.2/src/cli.rs:716:62
  14: recursive_furnace_reproduce_minimal_cli::main::{{closure}}
             at ./src/bin/main.rs:7:34
  15: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /home/simon/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/future/future.rs:124:9
  16: tokio::runtime::park::CachedParkThread::block_on::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/park.rs:285:60
  17: tokio::task::coop::with_budget
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/task/coop/mod.rs:167:5
  18: tokio::task::coop::budget
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/task/coop/mod.rs:133:5
  19: tokio::runtime::park::CachedParkThread::block_on
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/park.rs:285:31
  20: tokio::runtime::context::blocking::BlockingRegionGuard::block_on
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/context/blocking.rs:66:9
  21: tokio::runtime::scheduler::multi_thread::MultiThread::block_on::{{closure}}
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/scheduler/multi_thread/mod.rs:87:13
  22: tokio::runtime::context::runtime::enter_runtime
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/context/runtime.rs:65:16
  23: tokio::runtime::scheduler::multi_thread::MultiThread::block_on
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/scheduler/multi_thread/mod.rs:86:9
  24: tokio::runtime::runtime::Runtime::block_on_inner
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/runtime.rs:358:45
  25: tokio::runtime::runtime::Runtime::block_on
             at /home/simon/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.47.0/src/runtime/runtime.rs:328:13
  26: recursive_furnace_reproduce_minimal_cli::main
             at ./src/bin/main.rs:7:5
  27: core::ops::function::FnOnce::call_once
             at /home/simon/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.