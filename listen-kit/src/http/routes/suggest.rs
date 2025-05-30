use crate::http::serde::deserialize_messages;
use crate::{agents::suggester, http::middleware::verify_auth};
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use rig::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Clone)]
pub struct SuggestRequest {
    #[serde(deserialize_with = "deserialize_messages")]
    chat_history: Vec<Message>,
    locale: Option<String>,
    context: Option<String>,
}

#[post("/suggest")]
async fn suggest(
    req: HttpRequest,
    body: web::Json<SuggestRequest>,
) -> Result<HttpResponse, Error> {
    let chat_history = body.chat_history.clone();
    let _ = match verify_auth(&req).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Error: unauthorized: {}", e);
            return Ok(HttpResponse::Unauthorized().json(json!({
                "error": "unauthorized"
            })));
        }
    };
    let suggestions = match suggester::suggest(
        &chat_history,
        body.locale.as_deref().unwrap_or("en"),
        body.context.as_deref(),
    )
    .await
    {
        Ok(suggestions) => suggestions,
        Err(e) => {
            tracing::error!("Error: failed to suggest: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "error": format!("failed to suggest: {}", e)
            })));
        }
    };
    Ok(HttpResponse::Ok().json(json!({
        "suggestions": suggestions
    })))
}
