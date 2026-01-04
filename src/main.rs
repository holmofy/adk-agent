use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tracing::{info, warn};
use anyhow::{anyhow, Result};

use adk_rust::prelude::*;

async fn create_simple_agent() -> Result<Arc<dyn Agent>> {
    // 加载环境变量
    match dotenv() {
        Ok(_) => info!("已加载.env文件"),
        Err(_) => info!("未找到.env文件，使用系统环境变量"),
    }

    // 选择模型 - 优先使用 Gemini
    let model: Arc<dyn Llm> = if let Ok(key) = env::var("GOOGLE_API_KEY") {
        info!("使用Google Gemini API");
        Arc::new(GeminiModel::new(&key, "gemini-2.0-flash")?)
    } else {
        return Err(anyhow!(
            "未设置API密钥。请设置 GOOGLE_API_KEY 环境变量"
        ));
    };

    // 创建简单的LLM Agent（不使用工具）
    let agent = LlmAgentBuilder::new("simple-agent")
        .description("一个简单的智能助手")
        .instruction(
            "你是一个友好的智能助手。请简洁地回答用户的问题。"
        )
        .model(model)
        .build()?;

    Ok(Arc::new(agent))
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    info!("启动简单的ADK-Rust Agent...");

    // 创建agent
    let agent = create_simple_agent().await?;

    // 创建会话服务
    let session_service = Arc::new(InMemorySessionService::new());

    // 创建Runner
    let runner = Runner::new(RunnerConfig {
        app_name: "simple-adk-agent".to_string(),
        agent: agent.clone(),
        session_service,
        artifact_service: None,
        memory_service: None,
    })?;

    // 测试问题
    let questions = vec![
        "你好！",
        "今天天气怎么样？",
        "什么是人工智能？",
        "请计算 15 + 27",
    ];

    for question in questions {
        println!("{}", "=".repeat(60));
        println!("问题: {}", question);
        println!("{}", "-".repeat(40));

        // 创建输入内容
        let input = Content::new("user").with_text(question);

        // 运行agent - 需要提供用户ID和会话ID
        match runner.run("user123".to_string(), "session456".to_string(), input).await {
            Ok(mut stream) => {
                let mut response_parts = Vec::new();
                
                // 使用 futures::StreamExt 来处理流
                use futures::StreamExt;
                while let Some(event_result) = stream.next().await {
                    match event_result {
                        Ok(event) => {
                            if let Some(content) = event.content() {
                                for part in &content.parts {
                                    if let Part::Text { text } = part {
                                        print!("{}", text);
                                        response_parts.push(text.clone());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            warn!("流事件错误: {}", e);
                        }
                    }
                }
                
                let full_response = response_parts.join("");
                if !full_response.is_empty() {
                    println!("\n");
                } else {
                    println!("未收到响应");
                }
            }
            Err(e) => {
                warn!("执行失败: {}", e);
            }
        }
        
        println!();
    }

    info!("ADK-Rust Agent 运行完成");
    Ok(())
}