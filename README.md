<div align="center">

# ☁️ Sky Backend Framework

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
