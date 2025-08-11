# ogito 项目分析

## 项目概览

ogito 是一个用 Rust 和 TypeScript 编写的命令行工具，旨在简化 Git 仓库的克隆和管理。它允许用户快速创建干净的项目副本，而无需保留源仓库的历史记录。该项目同时提供 Rust (Cargo) 和 Node.js (npm) 的安装方式。

## 技术栈

- **核心语言**: Rust (用于主要的 CLI 功能)
- **辅助语言**: TypeScript (用于 npm 包的安装和运行脚本)
- **构建工具**: 
  - Rust: `cargo`
  - Node.js: `tsdown`, `tsx`
- **包管理器**: `pnpm`
- **CLI 框架**: `clap` (Rust)
- **HTTP 客户端**: `reqwest` (Rust)
- **异步运行时**: `tokio` (Rust)

## 当前功能 (`new` 命令)

根据现有代码 (`src/cli.rs`, `src/cmd/new.rs`)，ogito 目前的核心功能是通过 `new` 子命令实现的：

1.  **克隆仓库**: 从给定的 URL 克隆 Git 仓库。
2.  **指定目录**: 使用 `-d` 或 `--dir` 参数指定目标目录名称。
3.  **选择分支**: 使用 `-b` 或 `--branch` 参数指定要克隆的分支。
4.  **操作模式**: 使用 `-m` 或 `--mode` 参数指定操作模式（当前默认且似乎仅支持 `git`）。
5.  **强制覆盖**: 使用 `-f` 或 `--force` 参数强制覆盖已存在的目录。
6.  **保留历史**: 使用 `-H` 或 `--keep-history` 参数决定是否保留仓库的 Git 历史记录。

## 用户需求与愿景

用户希望将 ogito 打造成一个**个人模板代码管理工具**，而非传统的项目脚手架（如 Vite 或 Create React App）。其核心愿景是提供**更高的自由度**来管理和使用模板。

### 拟定的子命令

用户计划开发以下六个子命令来管理模板：

1.  **`add`**: 添加一个新的模板。
2.  **`new`**: 基于现有模板创建一个新项目（此功能部分存在）。
3.  **`update`**: 更新本地缓存的模板。
4.  **`remove` / `delete`**: 删除一个本地模板。
5.  **`list`**: 列出所有可用的本地模板。
6.  **`clear`**: 清除所有本地缓存的模板。

## 代码结构分析

```
ogito/
├── Cargo.toml / Cargo.lock     # Rust 项目配置和依赖
├── package.json / pnpm-lock.yaml # Node.js/npm 项目配置和依赖
├── src/                        # Rust 源代码
│   ├── main.rs                 # 程序入口
│   ├── cli.rs                  # CLI 参数解析和分发 (使用 clap)
│   ├── cmd/                    # 各个子命令的实现
│   │   ├── new.rs              # `new` 命令逻辑
│   │   └── add.rs              # `add` 命令逻辑 (初步)
│   └── ...                     # 其他模块 (clone, fetch, file, git, models, regex)
├── npm/                        # npm 包相关文件
│   ├── run.ts                  # npm 全局命令入口，调用编译后的 Rust 二进制文件
│   └── install.ts              # npm 安装后脚本，根据平台解压对应的 Rust 二进制文件
├── packages/                   # npm 发布用的平台特定包模板
│   └── _template.json
├── script/                     # 开发/维护脚本
├── dist/                       # TypeScript 编译输出目录 (由 tsdown 生成)
├── target/                     # Rust 编译输出目录 (由 cargo 生成)
├── tsdown.config.ts            # TypeScript 构建配置
└── README.md, LICENSE, ...     # 文档和配置文件
```

## 总结与建议

ogito 项目已经具备了基础的 CLI 结构和 `new` 命令的部分功能。要实现用户将其作为个人模板管理工具的目标，需要进行以下扩展：

1.  **完善模板存储机制**: 需要设计一个本地模板存储方案，例如在用户主目录下创建 `ogito/cache` 文件夹来存放克隆或添加的模板。
2.  **实现 `add` 命令**: 允许用户将一个 Git 仓库或本地目录添加为模板，并存储其元数据（如名称、描述、源 URL）。
3.  **实现 `list`, `remove/delete`, `update`, `clear` 命令**: 基于模板存储机制，开发这些管理命令。
4.  **调整 `new` 命令**: 修改 `new` 命令，使其能够从本地模板库中选择模板来创建新项目，而不仅仅是直接克隆远程仓库。
5.  **文档更新**: 更新 `README.md` 和可能的 CLI 帮助信息，以反映新的定位和命令。

这个方向将使 ogito 成为一个更加强大和灵活的个人开发工具。