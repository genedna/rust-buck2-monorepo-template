## Rust Monorepo Template with Buck2

### 1. 安装 Buck2

从 Buck2 Release 页面下载对应平台最新版本二进制文件， 解压后放移动到 `/usr/local/bin` 目录下，

```bash
$ wget https://github.com/facebook/buck2/releases/download/2024-02-01/buck2-x86_64-unknown-linux-musl.zst
$ zstd -d buck2-x86_64-unknown-linux-musl.zst
$ mv buck2-x86_64-unknown-linux-musl buck2
$ chmod +x buck2
$ sudo mv buck2 /usr/local/bin
```

### 2. 安装 [Reindeer](https://github.com/facebookincubator/reindeer)

```bash
$ cargo install --locked --git https://github.com/facebookincubator/reindeer reindeer
```

### 3. 使用 Buck2 对 Template 中的 rust-buck2 仓库进行测试

```bash
$ git clone https://github.com/genedna/rust-buck2-monorepo-template.git
$ cd rust-buck2-monorepo-template
$ reindeer --third-party-dir third-party vendor
$ reindeer --third-party-dir third-party buckify
$ buck2 build //third-party:rand#check
```

进行 `buck2 build` 和 `buck2 run` 命令测试

```bash
$ buck2 build //...
$ buck2 run //projects/rust-buck2:rust-buck2
Build ID: 12109586-d302-4cc0-ad2b-3fdfa6c5afaf
Jobs completed: 3. Time elapsed: 0.0s.
BUILD SUCCEEDED
A random value: 4120104088
```

### 4. 为 `rust-buck2` 添加新的第三方依赖

在代码中加入新的第三方依赖， 直接在文件中使用 `use` 并修改 `BUCK` 文件，

```rust
use rand::{thread_rng, Rng};
use semver::{Version, VersionReq};

fn main() {
    let mut rng = thread_rng();
    let x: u32 = rng.gen();

    println!("A random value: {}", x);

    let req = VersionReq::parse(">=1.2.3, <1.8.0").unwrap();
    let version = Version::parse("1.3.0").unwrap();
    assert!(req.matches(&version));
}
```

```rust
rust_binary(
    name = "rust-buck2",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/main.rs",
    deps = [
        "//third-party:rand",
        "//third-party:semver"
    ]
)
```

在 `third-party` 目录下的 Cargo.toml 文件添加新的依赖，

```toml
[dependencies]
rand = '0.8.5'
semver = '1.0.21'
```

执行 [Reindeer](https://github.com/facebookincubator/reindeer) 的 `vendor` 和 `buckify` 子命令， 重新生成 `third-party` 下的 `BUCK` 文件

```bash
$ reindeer --third-party-dir third-party vendor
$ reindeer --third-party-dir third-party buckify
$ buck2 build //...
```

### 5. 新建 Rust 项目

如果添加新的 Rust 项目，直接在 `projects` 使用 `cargo new` 命令创建新的项目， 然后在项目目录下新建 `BUCK` 文件中， 参照 `rust-buck2` 项目添加新的 `rust_binary`， 重新执行 `reindeer` 的 `vendor` 和 `buckify` 子命令， 然后执行 `buck2 build` 命令进行构建。

### 6. 新建 Library 项目

在 `projects` 目录下新建 `rust-library` 项目， 然后在项目目录下新建 `BUCK` 文件中。

```bash
$ cargo new rust-library
$ cd rust-library
$ touch BUCK
$ rm Cargo.toml
```

```rust
rust_library(
    name = "rust-library",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/lib.rs",
    deps = [
        "//third-party:semver"
    ],
    visibility = ["PUBLIC"],
)
```

在代码中使用 `semver` 实现一个检查版本的简单函数，

```rust
use semver::{Version, VersionReq};

pub fn version_match(version: &str, requirement: &str) -> bool {
    let version = Version::parse(version).unwrap();
    let requirement = VersionReq::parse(requirement).unwrap();
    requirement.matches(&version)
}
```

在 `third-party` 目录下的 Cargo.toml 文件添加新的依赖，

```toml
[dependencies]
rand = '0.8.5'
semver = '1.0.21'
```

在 `rust-buck2` 中引入 `rust-library` 项目，

```rust
use rand::{thread_rng, Rng};
use rust_library::version_match;

fn main() {
    let mut rng = thread_rng();
    let x: u32 = rng.gen();

    println!("A random value: {}", x);

    let m = version_match("1.0.0", ">= 1.0.0");
    match m {
        true => println!("1.0.0 >= 1.0.0: true"),
        false => println!("1.0.0 >= 1.0.0: false"),
    }

    let m = version_match("0.3.0", ">= 1.0.0");
    match m {
        true => println!("0.3.0 >= 1.0.0: true"),
        false => println!("0.3.0 >= 1.0.0: false"),
    }
}
```

```rust
rust_binary(
    name = "rust-buck2",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/main.rs",
    deps = [
        "//third-party:rand",
        "//projects/rust-library:rust-library",
    ]
)
```

执行 [Reindeer](https://github.com/facebookincubator/reindeer) 的 `vendor` 和 `buckify` 子命令， 重新生成 `third-party` 下的 `BUCK` 文件

```bash
$ reindeer --third-party-dir third-party vendor
$ reindeer --third-party-dir third-party buckify
$ buck2 build //...
$ buck2 run //projects/rust-buck2:rust-buck2
Build ID: 9635b6fe-4d41-4c9b-8487-304c258887f2
Jobs completed: 3. Time elapsed: 0.0s.
BUILD SUCCEEDED
A random value: 3596158885
1.0.0 >= 1.0.0: true
0.3.0 >= 1.0.0: false
```

### 7. 使用 Buck2 执行 Rust 单元测试

在 `toolchains` 目录下的 `BUCK` 文件加入 `remote_test_execution_toolchain`，

```rust
load("@prelude//toolchains:remote_test_execution.bzl", "remote_test_execution_toolchain")

remote_test_execution_toolchain(
    name = "remote_test_execution",
    visibility = ["PUBLIC"],
)
```

因为 `rust_library` 和 `rust_test` 都要用相同的 `deps`， 所以在 `rust_library` 中加入 `shared_deps` 避免重复，

```rust
shared_deps = [
    "//third-party:semver",
]

rust_library(
    name = "rust-library",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/lib.rs",
    tests = [
        "//projects/rust-library:rust-library-test",
    ],
    deps = shared_deps,
    visibility = ["PUBLIC"],
)

rust_test(
    name = "rust-library-test",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/lib.rs",
    deps = shared_deps,
    visibility = ["PUBLIC"],
)
```

执行 `buck2 test` 命令进行测试，

```bash
$ buck2 test //...
✓ Pass: root//projects/rust-library:rust-library-test (0.0s)
---- STDOUT ----

running 1 test
test tests::version_match_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


---- STDERR ----

Build ID: dbc61ebd-700f-49a4-8d33-28d5bd535051
Jobs completed: 4. Time elapsed: 0.0s.
Tests finished: Pass 1. Fail 0. Fatal 0. Skip 0. Build failure 0
```

---

## 关于 Monorepo / Monolithic 和 Buck2

1. [Buck](https://buck.build) 和 [Buck2](https://buck2.build) 就像 [Bazel](https://bazel.build) 一样是配合 **Monorepo / Monolithic** 代码仓库演化而来的大型构建系统， 当前 **Reindeer** 版本还是面向 **Monorepo** 的代码组织模型。 如果使用 [Buck2](https://buck2.build) 进行构建， 建议使用 **Monorepo / Monolithic** 方式组织代码。 
2. [Mega](https://github.com/web3infra-foundation/mega) 是使用 **Rust** 正在开发的 **Monorepo / Monolithic Codebase Platform**， 是 [Google Piper](https://cacm.acm.org/magazines/2016/7/204032-why-google-stores-billions-of-lines-of-code-in-a-single-repository/fulltext) 非官方的开源实现。 [Mega](https://github.com/web3infra-foundation/mega) 系统选择适配 [Buck2](https://buck2.build) ， 支持开发团队使用 [Mega](https://github.com/web3infra-foundation/mega) + [Buck2](https://buck2.build) 构建一个完整工作流。

### 1. 马道长之 Buck2 系列文章

- [使用 Buck2 编译构建 Rust 工程](https://maquanyi.com/articles/buck2-rust-hello-world)
- [使用 Reindeer 生成 Rust 项目 BUCK](https://maquanyi.com/articles/buck2-rust-reindeer)
- [使用 Buck2 和 Reindeer 构建 Rust Monorepo 工程](https://maquanyi.com/articles/buck2-rust-reindeer-monorepo)

### 2. References 

- [Buck2 - A large-scale build tool. The successor to Buck. Ready for users ∈ {C++, Python, Rust, Erlang, OCaml}](https://buck2.build)
- [Reindeer - Building Cargo Packages with Buck](https://github.com/facebookincubator/reindeer)
- [Bootstrap rustc with Buck2](https://github.com/themarwhal/buck2-rust)
- [Why Google Stores Billions of Lines of Code in a Single Repository](https://cacm.acm.org/magazines/2016/7/204032-why-google-stores-billions-of-lines-of-code-in-a-single-repository/fulltext)
- [Sapling - A Scalable, User-Friendly Source Control System](https://sapling-scm.com)