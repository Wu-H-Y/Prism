# Prism GitFlow 工作流指南

本文档说明如何使用 GitFlow 工作流进行版本管理和发布。

## 分支结构

```
main (稳定发布版)
  ↑ 合并 PR
develop (开发集成版)
  ↑ 合并 PR
feature/* (功能分支)
fix/* (修复分支)
release/* (发布准备分支)
hotfix/* (紧急修复分支)
```

## 分支说明

### main 分支

- **用途**: 稳定发布版本，对应生产环境
- **状态**: 始终可发布
- **保护规则**:
  - 禁止直接推送
  - 必须通过 PR 合并
  - PR 需要至少 1 个审查批准

### develop 分支

- **用途**: 开发集成版本，对应开发环境
- **状态**: 相对稳定，进行测试构建
- **保护规则**:
  - 禁止直接推送
  - 必须通过 PR 合并
  - 触发完整测试构建

### feature/\* 分支

- **用途**: 开发新功能
- **来源**: 从 develop 分支创建
- **目标**: 合并回 develop
- **命名**: `feature/功能描述`
- **触发**: CI 代码检查（不构建）

### fix/\* 分支

- **用途**: 修复 bug
- **来源**: 从 develop 分支创建
- **目标**: 合并回 develop
- **命名**: `fix/问题描述`
- **触发**: CI 代码检查（不构建）

### release/\* 分支

- **用途**: 准备发布版本
- **来源**: 从 develop 分支创建
- **目标**: 合并到 main 和 develop
- **命名**: `release/x.y.z`
- **触发**: CI 代码检查（不构建）

### hotfix/\* 分支

- **用途**: 紧急修复生产问题
- **来源**: 从 main 分支创建
- **目标**: 合并到 main 和 develop
- **命名**: `hotfix/x.y.z`
- **触发**: CI 代码检查（不构建）

## 工作流程

### 1. 开发新功能

```bash
# 从 develop 创建功能分支
git checkout develop
git pull origin develop
git checkout -b feature/add-user-profile

# 进行开发...
git add .
git commit -m "feat: 添加用户资料页面"

# 推送到远程
git push origin feature/add-user-profile

# 在 GitHub 上创建 PR: feature/add-user-profile → develop
# CI 会自动运行代码检查
# 合并后删除功能分支
```

### 2. 修复 bug

```bash
# 从 develop 创建修复分支
git checkout develop
git pull origin develop
git checkout -b fix/login-timeout

# 修复问题...
git add .
git commit -m "fix: 修复登录超时问题"

git push origin fix/login-timeout

# 创建 PR: fix/login-timeout → develop
```

### 3. 发布新版本

```bash
# 从 develop 创建发布分支
git checkout develop
git pull origin develop
git checkout -b release/0.2.0

# 在发布分支上进行最后调整
# 可以修复小 bug，但不能添加新功能

# 升级版本号（自动更新 package.json 和 Cargo.toml）
bun run release:minor  # 或 :major, :patch

# 提交版本更新
git add .
git commit -m "chore(release): 0.2.0"

# 推送到远程
git push origin release/0.2.0

# 创建两个 PR:
# 1. release/0.2.0 → main
# 2. release/0.2.0 → develop

# 等两个 PR 都合并后，切换到 main 创建发布 tag
git checkout main
git pull origin main
git tag v0.2.0
git push origin v0.2.0

# 这会触发 GitHub Actions 自动构建和发布
```

### 4. 紧急修复

```bash
# 从 main 创建紧急修复分支
git checkout main
git pull origin main
git checkout -b hotfix/0.1.8

# 修复问题并升级版本
bun run release:patch

git add .
git commit -m "hotfix: 修复安全漏洞"

git push origin hotfix/0.1.8

# 创建两个 PR:
# 1. hotfix/0.1.8 → main
# 2. hotfix/0.1.8 → develop

# 合并后创建 tag
git checkout main
git pull origin main
git tag v0.1.8
git push origin v0.1.8
```

## 版本管理命令

| 命令                    | 说明                         | 使用场景   |
| ----------------------- | ---------------------------- | ---------- |
| `bun run release:patch` | 升级补丁版本 (0.1.7 → 0.1.8) | bug 修复   |
| `bun run release:minor` | 升级次版本 (0.1.7 → 0.2.0)   | 新功能     |
| `bun run release:major` | 升级主版本 (0.1.7 → 1.0.0)   | 破坏性变更 |
| `bun run release:dry`   | 预览版本变更（不实际执行）   | 测试       |

## Commit 规范

使用 Conventional Commits 格式：

```bash
# 功能
git commit -m "feat: 添加用户登录功能"

# 修复
git commit -m "fix: 修复登录失败问题"

# 文档
git commit -m "docs: 更新 API 文档"

# 重构
git commit -m "refactor: 重构用户模块"

# 性能
git commit -m "perf: 优化加载速度"

# 测试
git commit -m "test: 添加用户测试用例"

# 构建/工具
git commit -m "chore: 更新依赖版本"
```

## CI/CD 流程

### Check 工作流 (`.github/workflows/check.yml`)

**触发**: `feature/*`, `fix/*`, `hotfix/*`, `release/*` 分支

**检查**:

- Lint (oxlint)
- Format check (oxfmt)
- Rust Clippy
- Rust Format check
- TypeScript Type check

### Test Build 工作流 (`.github/workflows/test-build.yml`)

**触发**: develop 分支

**构建**:

- macOS (Apple Silicon + Intel)
- Windows (x64)
- Linux (x64)

### Release 工作流 (`.github/workflows/release.yml`)

**触发**: 推送 `v*` tag

**流程**:

1. 检查版本一致性
2. 跨平台构建
3. 创建 GitHub Release

## 发布检查清单

在创建发布 tag 前确保：

- [ ] 所有相关的 PR 都已合并到 develop
- [ ] 版本号已正确更新
- [ ] CHANGELOG.md 已更新
- [ ] develop 分支的 CI 检查全部通过
- [ ] package.json 和 Cargo.toml 版本一致
- [ ] 在 release 分支上进行了必要的测试

## 常见问题

### Q: 如果构建失败怎么办？

删除失败的 tag 并修复问题后重新创建：

```bash
# 删除远程 tag
git push origin :refs/tags/v0.2.0

# 删除本地 tag
git tag -d v0.2.0

# 修复问题后重新创建
git tag v0.2.0
git push origin v0.2.0
```

### Q: 如何检查版本一致性？

```bash
# 检查 package.json
node -p "require('./package.json').version"

# 检查 Cargo.toml
node -e "import('./scripts/sync-cargo-version.js').then(m => console.log(m.readCargoVersion()))"
```

### Q: 发布后发现严重问题怎么办？

使用 hotfix 流程：

1. 从 main 创建 hotfix 分支
2. 修复问题并升级 patch 版本
3. 合并到 main 和 develop
4. 创建新的 tag 触发发布
