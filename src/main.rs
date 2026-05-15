// ... 其他逻辑保持不变 ...

    // 1. 极其严厉的指令：禁止废话，强制中文，限定场景
    let prompt_text = format!(
        "指令：将以下拼音转换为对应的中文短句，禁止输出英文，禁止解释。拼音：{}",
        query
    );

    let payload = json!({
        "model": "qwen2.5:0.5b",
        "prompt": prompt_text,
        "stream": false,
        "options": {
            "num_ctx": 128,
            "num_predict": 16, // 稍微多一点点空间，保证句子完整
            "temperature": 0.1, // 降低随机性，让它“死心眼”地转译
            "top_p": 0.9
        }
    });

// ... 后续处理逻辑 ...
    match request {
        Ok(response) => {
            if let Ok(json_body) = response.into_json::<serde_json::Value>() {
                if let Some(text) = json_body["response"].as_str() {
                    // 2. 这里的过滤逻辑非常重要：只取第一个换行符前的内容，并过滤掉非中文字符
                    let first_line = text.lines().next().unwrap_or("");
                    let clean_text: String = first_line.chars()
                        .filter(|c| (*c as u32) >= 0x4E00 && (*c as u32) <= 0x9FFF)
                        .collect();
                    print!("{}", clean_text.trim());
                }
            }
        }
// ...
