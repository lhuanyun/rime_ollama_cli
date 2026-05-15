use serde_json::json;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return; }
    let query = &args[1];

    let api_url = "http://192.168.66.2:11434/api/generate";

    // 提示词精简，减少模型思考时间
    let prompt_text = format!("续写文字:{}", query);

    let payload = json!({
        "model": "qwen2.5:0.5b",
        "prompt": prompt_text,
        "stream": false,
        "options": {
            "num_ctx": 64,  // 进一步缩小上下文
            "num_predict": 10 // 限制生成的字数，防止 AI 话痨
        }
    });

    // 150ms 是生死线，超过这个时间直接掐断，保证不卡字
    let request = ureq::post(api_url)
        .timeout(Duration::from_millis(150)) 
        .send_json(payload);

    if let Ok(response) = request {
        if let Ok(json_body) = response.into_json::<serde_json::Value>() {
            if let Some(text) = json_body["response"].as_str() {
                print!("{}", text.trim());
            }
        }
    }
}
