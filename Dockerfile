# 1. 基础镜像
FROM debian:bookworm-slim

# 2. 安装必要的运行时依赖 (OpenSSL, CA证书等)
# 这一步非常重要，保证你的 Rust 程序能发 HTTPS 请求和连接数据库
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 3. 设置工作目录
WORKDIR /app

# 4. 创建必要的目录结构
RUN mkdir -p /app/logs \
    && mkdir -p /app/uploads \
    && mkdir -p /app/config

# 5. 设置环境变量默认值
ENV APP_PORT=8081

# 6. 暴露端口
EXPOSE $APP_PORT

# 7. 启动命令 (核心修改)
# -------------------------------------------------------------
# 这里不再假设镜像里有 ./server 文件。
# 我们假设用户会在启动容器时，把可执行文件挂载到 /app/server。
#
# 逻辑说明：
# 1. chmod +x /app/server : 赋予挂载进来的文件执行权限 (防止宿主机权限不对)
# 2. ./server             : 运行程序
# 3. 2>&1 | tee ...       : 日志输出到屏幕并写入文件
# -------------------------------------------------------------

CMD chmod +x /app/server && ./server 2>&1 | tee -a /app/logs/app.log



# docker build -t rust-runtime:v1 .

# docker run -d --name  {appname} -p {port}:{port} -e APP_PORT={port}  -e RUN_MODE={dev}  -v {log_path}:/app/logs  -v {config_path}:/app/config  -v {uploads_path}:/app/uploads  -v {env_path}:/app/.env -v {regexes_path}:/app/regexexs.yaml  -v {app_path}:/app/server  rust-runtime:v1
