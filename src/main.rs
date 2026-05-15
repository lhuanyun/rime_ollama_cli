use serde_json::json;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return; }
    let query = &args[1];

    let api_url = "http://192.168.66.2:11434/api/generate";

    // 1. 强化 Prompt，防止 AI 像刚才那样胡言乱语（刚才它把明天解释成了金塔...）
    let prompt_text = format!("请根据拼音续写一个4到8字的中文短句，直接输出中文结果，不要解释。输入:{}", query);

    let payload = json!({
        "model": "qwen2.5:0.5b",
        "prompt": prompt_text,
        "stream": false,
        "options": {
            "num_ctx": 128,
            "num_predict": 10, // 限制字数提高速度
            "temperature": 0.2
        }
    });

    // 2. 将超时时间改为 3000ms (3秒)
    // 这是为了适配 N100 的响应速度，确保在卡死和结果之间取得平衡
    let request = ureq::post(api_url)
        .timeout(Duration::from_millis(3000)) 
        .send_json(payload);

    match request {
        Ok(response) => {
            if let Ok(json_body) = response.into_json::<serde_json::Value>() {
                if let Some(text) = json_body["response"].as_str() {
                    // 3. 只打印中文，过滤掉 AI 可能带出的英文
                    let clean_text: String = text.chars()
                        .filter(|c| (*c as u32) >= 0x4E00 && (*c as u32) <= 0x9FFF)
                        .collect();
                    print!("{}", clean_text.trim());
                }
            }
        }
        Err(_) => {
            // 失败不输出，保证 Rime 干净
        }
    }
}
