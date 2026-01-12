#!/usr/bin/env node

/**
 * 同步 package.json 的版本到 Cargo.toml
 * 这个脚本会在 commit-and-tag-version 运行后自动执行
 * 并将 Cargo.toml 的修改 amend 到上一个 commit
 */

import { readFileSync, writeFileSync } from 'fs'
import { fileURLToPath } from 'url'
import { dirname, join } from 'path'
import { parse, stringify } from 'smol-toml'
import { execSync } from 'child_process'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

const rootDir = join(__dirname, '..')
const packageJsonPath = join(rootDir, 'package.json')
const cargoTomlPath = join(rootDir, 'src-tauri', 'Cargo.toml')

// 读取 package.json 获取版本
const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'))
const version = packageJson.version

console.log(`📦 当前版本: ${version}`)

// 读取 Cargo.toml
const cargoToml = readFileSync(cargoTomlPath, 'utf-8')

// 使用 smol-toml 解析
const cargo = parse(cargoToml)

// 更新版本
cargo.package.version = version

// 写回 Cargo.toml
writeFileSync(cargoTomlPath, stringify(cargo), 'utf-8')

console.log(`✅ 已更新 src-tauri/Cargo.toml 版本为 ${version}`)

// 将 Cargo.toml 的修改 amend 到上一个 commit
try {
  execSync('git add src-tauri/Cargo.toml', { cwd: rootDir })
  execSync('git commit --amend --no-edit --no-verify', { cwd: rootDir })
  console.log(`✅ 已将 Cargo.toml 添加到 release commit`)
} catch (error) {
  console.error(`❌ amend commit 失败:`, error.message)
  process.exit(1)
}
