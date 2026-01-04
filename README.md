# ADK-Rust Agent

基于 [adk-rust](https://github.com/zavora-ai/adk-rust) 框架实现的智能代理，这是对原始 react-loop 项目的重构版本。

## 功能特性

- **基于 ADK-Rust**: 使用生产级的 Rust Agent 开发框架
- **Gemini 集成**: 支持 Google Gemini 2.0 Flash 模型
- **流式响应**: 支持实时流式输出
- **会话管理**: 基于 ADK-Rust 的会话存储系统
- **类型安全**: 利用 Rust 的类型系统确保代码安全
- **模块化架构**: 清晰的分层架构，易于扩展

## 快速开始

### 1. 环境配置

复制环境变量模板：
```bash
cp .env.example .env
```

编辑 `.env` 文件，设置你的 Google API 密钥：
```bash
GOOGLE_API_KEY=your-google-api-key-here
```

### 2. 获取 Google API Key

1. 访问 [Google AI Studio](https://aistudio.google.com/)
2. 创建新的 API 密钥
3. 将密钥添加到 `.env` 文件中

### 3. 运行

```bash
cargo run
```

## 与原版对比

### 原版 (react-loop)
- 从零实现的简单 ReAct 循环
- 手动解析 LLM 响应
- 基础的工具调用机制
- 使用 OpenAI/DeepSeek API

### ADK-Rust 版本
- 基于成熟的 ADK 框架
- 内置的流式处理和事件系统
- 标准化的工具接口
- 更好的错误处理和日志记录
- 生产级的会话管理

## 示例对话

```
问题: 你好！
AI: 你好！我是一个友好的智能助手，很高兴为您服务。有什么我可以帮助您的吗？

问题: 什么是人工智能？
AI: 人工智能（AI）是计算机科学的一个分支，致力于创建能够执行通常需要人类智能的任务的系统...

问题: 请计算 15 + 27
AI: 15 + 27 = 42
```

## 架构说明

本项目基于 ADK-Rust 框架构建，主要组件包括：

- **Agent**: 使用 `LlmAgentBuilder` 创建的智能代理
- **Model**: Gemini 2.0 Flash 模型集成
- **Runner**: 负责执行代理和管理会话
- **Session Service**: 内存会话存储
- **Event Stream**: 处理实时响应流

## 扩展开发

### 添加工具支持

要添加工具功能（如搜索、计算等），可以参考以下模式：

```rust
// 创建工具函数
async fn my_tool_function(_ctx: Arc<dyn ToolContext>, args: Value) -> adk_rust::Result<Value> {
    // 工具实现逻辑
    Ok(json!({ "result": "工具执行结果" }))
}

// 在 agent 构建时添加工具
let agent = LlmAgentBuilder::new("agent-with-tools")
    .model(model)
    .tool(Arc::new(FunctionTool::new(
        "my_tool",
        "工具描述",
        my_tool_function,
    )))
    .build()?;
```

### 切换模型

当前版本主要支持 Gemini 模型。要使用其他模型，需要：

1. 检查 adk-rust 的模型支持
2. 添加相应的特性标志
3. 修改模型创建逻辑

## 依赖项

- `adk-rust`: ADK-Rust 核心框架
- `tokio`: 异步运行时
- `serde/serde_json`: 序列化支持
- `tracing`: 日志记录
- `futures`: 流处理
- `dotenvy`: 环境变量加载

## 许可证

Apache 2.0 License

## 相关项目

- [原版 react-loop](https://github.com/holmofy/react-loop/): 从零实现的简单版本
- [adk-rust](https://github.com/zavora-ai/adk-rust): 底层框架
- [Google ADK](https://github.com/google/adk-python): 官方 Python 版本
