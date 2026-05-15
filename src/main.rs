use serde_json::json;
use std::env;
use std::time::Duration;

fn main() {
    // 1. 获取 Lua 脚本传入的拼音或上下文参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let query = &args[1];

    // 2. 配置 N100 Debian 的 API 地址
    let api_url = "http://192.168.66.2:11434/api/generate";

    // 3. 构建 0.5B 专用 Prompt：填空模式，强制输出中文
    let prompt_text = format!(
        "<|im_start|>system\n你是一个中文输入法。直接把拼音转换成中文短句，严禁输出英文，严禁解释。<|im_end|>\n<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
        query
    );

    let payload = json!({
        "model": "qwen2.5:0.5b",
        "prompt": prompt_text,
        "stream": false,
        "options": {
            "num_ctx": 128,
            "stop": ["<|im_end|>", "\n"], 
            "temperature": 0.1,
            "num_predict": 12
        }
    });

    // 4. 设置 3 秒超时，适配 N100 推理耗时
    let request = ureq::post(api_url)
        .timeout(Duration::from_millis(3000))
        .send_json(payload);

    // 5. 解析并输出结果
    match request {
        Ok(response) => {
            if let Ok(json_body) = response.into_json::<serde_json::Value>() {
                if let Some(text) = json_body["response"].as_str() {
                    // 只提取字符串中的中文字符，彻底过滤掉 AI 可能夹带的英文废话
                    let clean_text: String = text.chars()
                        .filter(|c| (*c as u32) >= 0x4E00 && (*c as u32) <= 0x9FFF)
                        .collect();
                    
                    if !clean_text.is_empty() {
                        print!("{}", clean_text.trim());
                    }
                }
            }
        }
        Err(_) => {
            // 失败时静默退出，不卡住输入法
        }
    }
}
