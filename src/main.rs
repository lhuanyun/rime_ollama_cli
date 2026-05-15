use serde_json::json;
use std::env;
use std::time::Duration;

fn main() {
    // 1. 获取 Lua 脚本传入的拼音或上下文参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return; // 若无参数，静默退出
    }
    let query = &args[1];

    // 2. 配置 Ollama 接口地址 (若在 Windows 测试本地运行则用 127.0.0.1，部署后改为 Debian 的 IP)
    let api_url = "http://192.168.66.2:11434/api/generate";

    // 3. 构建请求负载 (Payload)
    // 约束 Prompt，强制 AI 仅输出预测词，不带标点和废话
    let prompt_text = format!(
        "你是一个中文输入法预测引擎。根据用户输入，直接给出最可能的后续词语或短语。不要任何解释，不要标点符号。输入：{}",
        query
    );

    let payload = json!({
        "model": "qwen2.5:0.5b",
        "prompt": prompt_text,
        "stream": false,
        "options": {
            "num_ctx": 128 // 限制上下文，降低 N100 算力开销，提高首字输出速度
        }
    });

    // 4. 发送 HTTP POST 请求并设置严格的超时时间
    // 输入法场景下，如果 400 毫秒内没返回结果，应直接放弃，防止卡住后续打字
    let request = ureq::post(api_url)
        .timeout(Duration::from_millis(400)) 
        .send_json(payload);

    // 5. 解析并输出结果
    match request {
        Ok(response) => {
            if let Ok(json_body) = response.into_json::<serde_json::Value>() {
                if let Some(text) = json_body["response"].as_str() {
                    // 将纯净的文本输出到 stdout，Lua 的 io.popen 会捕获这里
                    print!("{}", text.trim());
                }
            }
        }
        Err(_) => {
            // 网络错误或超时，静默失败，确保输入法主程序不崩溃
        }
    }
}
