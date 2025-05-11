use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

async fn paraphrase(req_body: web::Json<PromptRequest>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| "INVALID".into());
    if api_key == "INVALID" {
        return HttpResponse::InternalServerError().body("❌ API key not found");
    }

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [{
                "role": "user",
                "content": format!("Paraphrase this: {}", req_body.prompt)
            }],
            "max_tokens": 100
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<OpenAIResponse>().await {
                    Ok(parsed) => {
                        let result = &parsed.choices[0].message.content;
                        HttpResponse::Ok().json(result)
                    }
                    Err(err) => {
                        println!("❌ JSON parse error: {:?}", err);
                        HttpResponse::InternalServerError().body("❌ Failed to parse OpenAI response")
                    }
                }
            } else {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                println!("❌ API returned error {}: {}", status, text);
                HttpResponse::InternalServerError().body("❌ OpenAI API error")
            }
        }
        Err(err) => {
            println!("❌ HTTP request failed: {:?}", err);
            HttpResponse::InternalServerError().body("❌ Failed to contact OpenAI")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/api/chat", web::post().to(paraphrase))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
