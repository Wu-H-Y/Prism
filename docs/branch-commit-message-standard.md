# 分支命名和提交消息规范

本文档定义了 Prism 项目的分支命名和提交消息规范，基于 Conventional Commits 规范。

---

## 🌿 分支命名规范

### 命名格式

```
<type>/<short-description>
```

### 类型说明

| 类型       | 说明         | 使用场景                            |
| ---------- | ------------ | ----------------------------------- |
| `feat`     | 新功能开发   | 添加新的 API、UI 功能、特性等       |
| `fix`      | Bug 修复     | 修复功能性的问题                    |
| `docs`     | 文档更新     | 修改项目文档（代码注释除外）        |
| `style`    | 代码格式调整 | 不影响代码逻辑的格式变更            |
| `refactor` | 重构         | 代码重构，既不修复 bug 也不添加功能 |
| `perf`     | 性能优化     | 性能改进、优化加载速度等            |
| `test`     | 测试相关     | 添加或修改测试代码                  |
| `build`    | 构建系统     | 修改构建工具、依赖、项目版本等      |
| `ci`       | CI 相关      | 修改 CI 配置文件、脚本等            |
| `chore`    | 其他杂项     | 其他不修改 src 或测试文件的提交     |
| `ops`      | 运维相关     | 基础设施、部署、备份、恢复等        |
| `revert`   | 回滚         | 回滚之前的提交                      |

### 示例

```bash
feat/user-authentication      # 用户认证功能
fix/login-validation            # 登录验证修复
docs/update-readme             # 更新 README
style/format-code              # 代码格式化
refactor/crawler-logic         # 重构爬虫逻辑
perf/database-query            # 数据库查询优化
test/add-unit-tests            # 添加单元测试
chore/update-dependencies      # 更新依赖
ci/add-workflow               # 添加 GitHub Actions workflow
ops/deployment-config          # 部署配置
revert/remove-feature          # 回滚功能移除
```

### 命名规则

- ✅ 使用小写字母
- ✅ 使用连字符分隔单词
- ✅ 描述简短但具有描述性（不超过 3 个单词）
- ⚠️ 避免使用数字，除非是版本号
- ❌ 不要使用下划线或驼峰命名

---

## 📝 提交消息规范

### 格式

提交消息必须遵循 Conventional Commits 规范：

```
<type>[optional scope]: <subject>

[optional body]

[optional footer(s)]
```

### 类型 (Type)

| 类型       | 说明     | 示例                                 |
| ---------- | -------- | ------------------------------------ |
| `feat`     | 新功能   | 添加新的 API 端点、UI 组件等         |
| `fix`      | Bug 修复 | 修复功能性问题                       |
| `docs`     | 文档     | 更新 README、API 文档等              |
| `style`    | 格式     | 代码格式化、缺失分号等（不影响逻辑） |
| `refactor` | 重构     | 代码重构，不改变行为                 |
| `perf`     | 性能     | 性能优化                             |
| `test`     | 测试     | 添加或修改测试                       |
| `build`    | 构建     | 修改构建工具、依赖等                 |
| `ci`       | CI       | 修改 CI 配置                         |
| `chore`    | 杂项     | 其他不修改 src 或测试的提交          |
| `revert`   | 回滚     | 回滚之前的提交                       |

### 范围 (Scope)

范围是可选的，用于标识提交影响的代码部分。常见的范围包括：

- `runtime` - 运行时相关
- `api` - API 层
- `ui` - 用户界面
- `core` - 核心逻辑
- `model` - 数据模型
- `crawler` - 爬虫模块
- `tauri` - Tauri 相关
- `docs` - 文档

### 破坏性变更 (Breaking Change)

有两种方式标记破坏性变更：

**方式 1：在 type/scope 后添加 `!`**

```
feat!: remove deprecated API
feat(api)!: remove deprecated endpoint
```

**方式 2：在 footer 中添加 `BREAKING CHANGE:`**

```
feat: remove deprecated API

BREAKING CHANGE: The old API has been removed. All clients must migrate to the new API.
```

### 完整示例

```bash
# 简单提交
feat: add user authentication

# 带范围的提交
feat(api): add user endpoint

添加获取用户信息的 REST API 端点。

支持以下字段：
- id
- username
- email

# 破坏性变更
feat(api)!: remove legacy user endpoint

BREAKING CHANGE: /api/v1/users 已被移除，请使用 /api/v2/users

# 多行提交
fix(runtime): prevent racing of requests

引入请求 ID 和对最新请求的引用。丢弃来自其他请求的响应。

移除用于缓解竞争问题的超时机制，这些超时现在已经过时。

Closes #123

# 回滚提交
revert: feat: remove experimental feature

This reverts commit 676104e.
```

### 提交消息长度限制

- **标题行**（第一行）：不超过 72 个字符
- **正文每行**：不超过 100 个字符
- 使用**祈使句**：例如 "add" 而不是 "added" 或 "adds"

---

## 🔧 常用命令

### 提交和推送

```bash
# 提交变更（会自动检查提交消息格式）
git add .
git commit -m "feat: add new feature"
git push origin feat/new-feature
```

### 发布命令

```bash
# 自动发布（根据提交类型自动计算版本）
bun run release

# 指定版本类型
bun run release:patch   # 1.0.0 → 1.0.1
bun run release:minor   # 1.0.0 → 1.1.0
bun run release:major   # 1.0.0 → 2.0.0

# 预发布
bun run release:prerelease alpha   # 1.0.1-alpha.0

# 干运行（查看会做什么，但不实际执行）
bun run release:dry
```

### 发布流程

```bash
# 1. 确保在主分支上
git checkout main
git pull origin main

# 2. 运行发布（会自动：
#    - 计算新版本号
#    - 更新 package.json 和 Cargo.toml
#    - 生成 CHANGELOG.md
#    - 创建 git commit
#    - 创建 git tag
bun run release

# 3. 推送 tag
git push --follow-tags origin main
```

---

## 📋 快速参考

### 语义化版本控制映射

| 提交类型          | 版本变更                  | 示例       |
| ----------------- | ------------------------- | ---------- |
| `feat`            | **MINOR** (1.0.0 → 1.1.0) | 添加新功能 |
| `fix`             | **PATCH** (1.0.0 → 1.0.1) | Bug 修复   |
| `BREAKING CHANGE` | **MAJOR** (1.0.0 → 2.0.0) | 破坏性变更 |
| `feat!` 或 `fix!` | **MAJOR** (1.0.0 → 2.0.0) | 破坏性变更 |

### 检查清单

在提交前，请检查：

- [ ] 分支名称遵循命名规范
- [ ] 提交消息符合 Conventional Commits 格式
- [ ] 提交内容单一，只包含相关更改
- [ ] 代码通过所有测试和 lint 检查
- [ ] 如有破坏性变更，已在提交消息中明确标注

### 自动化说明

**本项目的提交消息验证和版本管理已自动化**：

- ✅ **commitlint** + **husky** - 自动检查提交消息格式（提交时）
- ✅ **commit-and-tag-version** - 自动计算版本号、更新文件、生成 CHANGELOG、创建 tag
- ✅ **GitHub Actions** - 自动构建和测试（.github/workflows/build.yml）

### 相关文件

- `.commitlintrc.json` - 提交消息验证配置
- `.versionrc.json` - 版本管理配置（支持 package.json 和 Cargo.toml）
- `.husky/commit-msg` - Git hook 脚本
- `package.json` - 发布命令

---

## 📚 参考资料

- [Conventional Commits 规范](https://www.conventionalcommits.org/)
- [commitlint 文档](https://commitlint.js.org/)
- [commit-and-tag-version 文档](https://github.com/absolute-version/commit-and-tag-version)
- [Semantic Versioning](https://semver.org/)
