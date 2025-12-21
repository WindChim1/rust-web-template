  Rust Web Template - 模块化 Rust 后端架构    body { font-family: 'Segoe UI', system-ui, sans-serif; } .code-font { font-family: 'JetBrains Mono', 'Fira Code', monospace; } /\* 优化代码块样式 \*/ pre\[class\*="language-"\] { border-radius: 0.5rem; margin: 0.5em 0; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); } /\* 卡片悬停特效 \*/ .hover-card { transition: all 0.3s ease; } .hover-card:hover { transform: translateY(-4px); box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); }

S

Sky Backend

[特性](#features) [架构](#architecture) [环境](#setup) [开发](#quickstart) [部署](#deployment)

Enterprise Rust Template

# 构建下一代  

高性能 Web 服务

基于 **Salvo** + **SQLx** + **Tokio**。 采用 Workspace 模式，集成 RBAC、审计日志与 Docker 方案，为企业级开发而生。

[开始安装](#setup) [查看架构](#architecture)

### Salvo & Tokio

基于 Rust 异步生态，提供极高的并发处理能力，内存安全且高效。

### SQLx Postgres

纯异步数据库驱动，支持编译时 SQL 语法检查，杜绝运行时错误。

### 审计与权限

内置 AOP 操作日志记录与 RBAC 权限控制，满足企业合规需求。

## Workspace 模块化架构

项目采用 Rust Workspace 模式，将核心框架与业务逻辑物理隔离，依赖清晰，编译速度更快。

// Project Structure

- app 启动入口
- common 公共工具
- config 环境配置
- 📄 dev.toml / release.toml
- framework 基座
- 🦀 db.rs (SQLx Pool)
- 🦀 jwt.rs (Auth)
- 🦀 log.rs (Middleware)
- modules 业务
- monitor login\_info, operlog
- system user, role, menu, dict

#### Framework 层

基础设施底座。封装了数据库连接池、JWT 认证、全局日志、统一错误处理。它作为依赖被业务模块引用。

#### Modules 层

业务领域层。按 DDD 思想拆分为独立的 Crate（如 system, monitor）。每个模块包含独立的 Handler、Service 和 Model。

#### App 层

组装层与入口。负责读取 Config 配置，初始化 Framework，挂载 Modules 的路由，最终启动 Web Server。

#### Config 层

支持多环境配置（dev/test/release）。配合 \`RUN\_MODE\` 环境变量，实现 Docker 环境下的配置无缝切换。

Environment

## 开发环境准备

为了保证最佳的开发体验和代码质量，请按顺序安装以下 Rust 生态工具。

### 核心工具

1\. 安装 Rust Compiler

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2\. 安装 Cargo Generate Scaffolding

```bash
cargo install cargo-generate
```

用于从模板快速初始化项目结构。

3\. 安装 SQLx CLI Database

```bash
cargo install sqlx-cli
```

用于数据库迁移 (Migrate) 和离线 SQL 检查。

### 代码质量 & 测试

4\. 代码检查工具集

Pre-commit (提交前检查):

```bash
pipx install pre-commit && pre-commit install
```

Cargo Deny (依赖安全):

```bash
cargo install --locked cargo-deny
```

Typos (拼写检查):

```bash
cargo install typos-cli
```

Git Cliff (更新日志):

```bash
cargo install git-cliff
```

5\. Cargo Nextest Testing

```bash
cargo install cargo-nextest --locked
```

比原生 \`cargo test\` 更快、更友好的测试运行器。

## 🚀 快速开始

1

### 生成项目

```bash
cargo generate WindChim1/rust-web-template
cd rust-web-template
```

2

### 配置运行

复制配置文件，并修改其中的数据库连接信息。

```bash
cp config/dev.toml.example config/dev.toml
# 运行 app 模块 (Workspace 模式下需要指定 -p app)
cargo run -p app
```

3

### 编写业务接口

示例：在 `modules/system/src/user/handler.rs` 中编写接口。

```rust
#[handler]
pub async fn add_user(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let pool = depot.obtain::().unwrap();
    let dto = req.parse_json::().await?;
    
    // 执行业务逻辑
    let id = service::add(pool, dto).await?;

    // 一行代码记录操作日志
    LogMeta::set(depot, "用户管理", 1, "新增用户");

    res.render(Json(AppResponse::success(id)));
    Ok(())
}
```

## Docker 部署运维

### 挂载式热更新方案

推荐使用 **“基础运行时镜像 + 文件挂载”** 的方式。 将编译好的二进制文件从宿主机挂载到容器中，无需每次更新代码都重新构建 Docker 镜像，实现秒级发布。

#### 更新流程

1. 本地交叉编译：`cross build --release`
2. 暂停容器：`docker stop sky-server`
3. 替换宿主机文件：覆盖 `app-linux`
4. 启动容器：`docker start sky-server`

启动命令 Linux / Docker

```bash
# 启动容器 (生产环境)
docker run -d \
  --name sky-server \
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

注意：在 Windows 环境下开发，必须使用 `cross build --target x86_64-unknown-linux-gnu` 编译出 Linux 格式的可执行文件。

Sky Backend

Enterprise Rust Web Template

[GitHub](#) [Documentation](#) [Report Issue](#)

© 2025 Made with ❤️ by Rustacean.``
