<div align="center">

# ☁️Rust Web Template

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Salvo](https://img.shields.io/badge/Salvo-v0.68-blue?logo=rust)](https://salvo.rs/)
[![SQLx](https://img.shields.io/badge/SQLx-Postgres-green?logo=postgresql)](https://github.com/launchbadge/sqlx)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker)](https://www.docker.com/)
[![License](https://img.shields.io/badge/License-MIT-lightgrey.svg)](LICENSE)

**企业级模块化 Rust Web 服务端脚手架**

基于 **Salvo** + **SQLx** + **Tokio** 构建。
采用 **Workspace** 模式物理隔离核心框架与业务模块，集成 RBAC、审计日志与热更新部署方案。

[特性](#-核心特性) · [架构](#-架构设计) · [环境](#-开发环境准备) · [快速开始](#-快速开始) · [部署](#-docker-部署运维)

</div>

---

## ✨ 核心特性

- ⚡ **Salvo & Tokio**: 基于 Rust 异步生态，提供极高的并发处理能力，内存安全且高效。
- 🛡️ **SQLx Postgres**: 纯异步数据库驱动，支持**编译时 SQL 语法检查**，杜绝运行时错误。
- 📦 **Workspace 架构**: 采用 Cargo Workspace 管理多包项目，依赖清晰，编译缓存利用率高。
- 📝 **AOP 审计日志**: 基于 Middleware 实现无侵入式操作日志，自动捕获请求参数与响应。
- ⚙️ **多环境配置**: 支持 `dev.toml`, `release.toml`，配合 `RUN_MODE` 环境变量实现容器化配置切换。
- 🐳 **Docker 热更新**: 优化的 Dockerfile 方案，支持挂载二进制文件实现秒级发布。

---

## 🏗 架构设计

### 📂 目录结构

```text
.
├── app/               # 🚀 启动入口 (main.rs, 组装路由与中间件)
├── common/            # 🛠️ 公共工具库 (AppError, Result, Utils)
├── config/            # ⚙️ 环境配置文件
│   ├── dev.toml       # 开发环境配置
│   └── release.toml   # 生产环境配置
├── framework/         # 🧱 基础设施底座 (作为独立 Crate)
│   ├── db.rs          # SQLx 连接池管理
│   ├── jwt.rs         # JWT 认证封装
│   └── log.rs         # 日志与 AOP 逻辑
└── modules/           # 📦 业务领域层 (DDD)
    ├── monitor/       # 📊 系统监控 (login_info, operlog)
    └── system/        # 👤 系统管理 (user, role, menu, dict)
```

### 🧩 分层理念

1. **Framework (核心层)**: 基础设施底座。封装了与业务无关的基础能力，如 DB 连接、鉴权、日志。
2. **Modules (业务层)**: 具体的业务逻辑。拆分为 `system`, `monitor` 等独立 crate。包含各自的 `handler`, `service`, `model`。
3. **App (组装层)**: 二进制入口。负责读取配置、初始化 Framework、装配各个 Module 的路由，启动 Server。

---

## 🛠 开发环境准备

为了保证代码质量和开发效率，请预先安装以下 Rust 生态工具链。

### 1. 核心工具

| 工具 | 描述 | 安装命令 |
| :--- | :--- | :--- |
| **Rust** | 编译器 | `curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh` |
| **Cargo Generate** | 模板生成 | `cargo install cargo-generate` |
| **SQLx CLI** | 数据库迁移 | `cargo install sqlx-cli` |

### 2. 代码质量与测试

- **Pre-commit** (提交前检查):

    ```bash
    pipx install pre-commit && pre-commit install
    ```

- **Cargo Deny** (依赖安全检查):

    ```bash
    cargo install --locked cargo-deny
    ```

- **Typos** (拼写检查):

    ```bash
    cargo install typos-cli
    ```

- **Git Cliff** (自动生成 Changelog):

    ```bash
    cargo install git-cliff
    ```

- **Cargo Nextest** (高速测试运行器):

    ```bash
    cargo install cargo-nextest --locked
    ```

---

## 🚀 快速开始

### 1. 初始化项目

使用 `cargo generate` 从模板创建新项目：

```bash
cargo generate WindChim1/rust-web-template
cd {project_name}
```

### 2. 配置运行

复制配置文件并修改数据库连接信息：

```bash
cp config/dev.toml.example config/dev.toml

# 运行 app 模块 (Workspace 模式下需要指定 -p app)
# 确保本地 PostgreSQL 已启动
cargo run -p app
```

### 3. 业务代码示例

在 `modules/system/src/user/handler.rs` 中编写业务接口，只需一行代码即可记录操作日志：

```rust
#[handler]
pub async fn add_user(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    // 1. 获取 DB
    let pool = depot.obtain::<PgPool>().unwrap();
    // 2. 解析参数
    let dto = req.parse_json::<UserAddDTO>().await?;
    
    // 3. 执行业务
    let id = service::add(pool, dto).await?;

    // 4. 【关键】记录操作日志 (AOP)
    LogMeta::set(depot, "用户管理", 1, "新增用户");

    res.render(Json(AppResponse::success(id)));
    Ok(())
}
```

---

## 🐳 Docker 部署运维

### 推荐方案：挂载式热更新

我们推荐使用 **“基础运行时镜像 + 文件挂载”** 的方式。将编译好的二进制文件从宿主机挂载到容器中，**无需每次更新代码都重新构建 Docker 镜像**，实现秒级发布。

#### 启动命令 (生产环境)

```bash
docker run -d \
  --name app-server \
  -p 8081:8081 \
  -e APP_PORT=8081 \
  -e RUN_MODE=release \
  -v $(pwd)/config:/app/config \
  -v $(pwd)/uploads:/app/uploads \
  -v $(pwd)/logs:/app/logs \
  # 关键：挂载二进制文件
  -v $(pwd)/target/release/app-linux:/app/server \
  rust-runtime:v1
```

> **⚠️ 注意**: 如果你在 Windows 环境下开发，必须使用交叉编译工具生成 Linux 格式的可执行文件：
>
> ```bash
> cross build --target x86_64-unknown-linux-gnu --release
> ```

#### 更新流程

1. 本地交叉编译：`cross build --release`
2. 暂停容器：`docker stop app-server` (释放文件占用)
3. 替换文件：覆盖宿主机的 `app-linux` 文件
4. 启动容器：`docker start app-server`

---

<div align="center">
    © 2025 Rust Web Template Project. Made with ❤️ by Rustacean.
</div>
