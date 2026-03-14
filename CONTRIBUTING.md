# 贡献指南

感谢你对 CCRustStudy 项目的关注！我们欢迎各种形式的贡献。

## 🤝 如何贡献

### 报告问题

如果你发现了 bug 或有改进建议：

1. 检查 [Issues](https://github.com/your-repo/CCRustStudy/issues) 是否已有相关问题
2. 如果没有，创建新的 Issue，详细描述问题
3. 包含复现步骤、预期行为和实际行为

### 提交代码

1. **Fork 项目**
   ```bash
   git clone https://github.com/your-username/CCRustStudy.git
   cd CCRustStudy
   ```

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **编写代码**
   - 遵循项目的代码风格
   - 添加必要的测试
   - 更新相关文档

4. **运行测试**
   ```bash
   cargo test --workspace
   cargo clippy --workspace
   cargo fmt --all
   ```

5. **提交更改**
   ```bash
   git add .
   git commit -m "feat: 添加新功能描述"
   ```

6. **推送到 GitHub**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **创建 Pull Request**
   - 在 GitHub 上创建 PR
   - 详细描述你的更改
   - 关联相关的 Issue

## 📝 代码规范

### Rust 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### 提交信息规范

使用语义化的提交信息：

- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `style:` 代码格式调整
- `refactor:` 代码重构
- `test:` 添加或修改测试
- `chore:` 构建过程或辅助工具的变动

示例：
```
feat: 添加 WebSocket 支持到 Web 服务模块

- 实现 WebSocket 连接处理
- 添加消息广播功能
- 更新相关文档和测试
```

### 文档规范

- 所有公共 API 必须有文档注释
- 使用中文编写注释和文档
- 包含代码示例
- 更新 README 和模块文档

### 测试规范

- 每个新功能都要有对应的测试
- 测试覆盖率应保持在 80% 以上
- 测试命名清晰，描述测试目的

## 🎯 贡献方向

### 高优先级

- 修复 bug
- 改进文档
- 添加测试用例
- 性能优化

### 中优先级

- 添加新的示例项目
- 改进错误消息
- 添加练习题解答

### 低优先级

- 添加新模块
- 翻译文档
- 视频教程

## 🔍 代码审查

所有 PR 都需要经过代码审查：

- 至少一位维护者批准
- 所有 CI 检查通过
- 没有未解决的评论

## 📧 联系方式

如有问题，可以：

- 创建 Issue
- 在 PR 中讨论
- 发送邮件到项目维护者

## 📜 许可证

贡献的代码将采用与项目相同的 MIT 许可证。

---

再次感谢你的贡献！🎉
