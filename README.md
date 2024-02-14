## Rust Monorepo Template with Buck2

### 安装 Buck2

从 Buck2 Release 页面下载对应平台最新版本二进制文件， 解压后放移动到 `/usr/local/bin` 目录下，

```bash
$ wget https://github.com/facebook/buck2/releases/download/2024-02-01/buck2-x86_64-unknown-linux-musl.zst
$ zstd -d buck2-x86_64-unknown-linux-musl.zst
$ mv buck2-x86_64-unknown-linux-musl buck2
$ chmod +x buck2
$ sudo mv buck2 /usr/local/bin
```

### 安装 [Reindeer](https://github.com/facebookincubator/reindeer)

```bash
$ cargo install --locked --git https://github.com/facebookincubator/reindeer reindeer
```

### 使用 Buck2 对 Template 中的 rust-buck2 仓库进行测试

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

### 添加项目的依赖

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

```
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

执行 `vendor` 和 `buckify` 子命令， 重新生成 `third-party` 下的 `BUCK` 文件

```bash
$ reindeer --third-party-dir third-party vendor
$ reindeer --third-party-dir third-party buckify
$ buck2 build //...
```

### 添加新的 Rust 项目

如果添加新的 Rust 项目，直接在 `projects` 进行添加项目。

---

> [Buck]((https://buck.build)) 和 [Buck2](https://buck2.build) 就像 [Bazel](https://bazel.build) 一样是配合 Monorepo / Monolithic 代码仓库演化而来的大型构建系统， 当前 Reindeer 版本还是适合 Monorepo 的组织方案。 如果使用 [Buck2](https://buck2.build) 进行构建， 建议使用 Monorepo / Monolithic 方式组织代码。 [Mega](https://github.com/web3infra-foundation/mega) 是使用 Rust 正在开发的 Monorepo / Monolithic Codebase Platform， 是 Google Piper 非官方的开源实现。 [Mega](https://github.com/web3infra-foundation/mega) 系统选择适配 Buck2 ， 支持用户使用 [Mega](https://github.com/web3infra-foundation/mega) + [Buck2](https://buck2.build) 构建一个完整的开发的工作流。