use crate::{PokerAnalysis, screenshot};
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub async fn capture_and_analyze() -> Result<PokerAnalysis, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Starting poker analysis...");
    
    // Capture screenshot
    let base64_image = screenshot::capture_screen().await?;
    
    // Analyze with OpenAI
    let analysis = analyze_with_openai(&base64_image).await?;
    
    Ok(analysis)
}

async fn analyze_with_openai(base64_image: &str) -> Result<PokerAnalysis, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;
    
    let client = Client::new();
    
    let poker_prompt = r#"
You are an expert poker player and coach. Analyze this poker table screenshot and provide strategic advice.

Please analyze:
1. My hole cards (if visible)
2. Community cards on the board
3. Betting action and pot size
4. Number of players and their positions
5. Stack sizes (if visible)
6. Current betting round (preflop, flop, turn, river)

Based on your analysis, provide:
1. ACTION_RECOMMENDATION: What action should I take? (fold, call, raise, all-in)
2. REASONING: Detailed explanation of why this is the optimal play
3. HAND_STRENGTH: Assessment of my hand strength (weak, marginal, strong, very strong)
4. POT_ODDS: If relevant, calculate pot odds for calling
5. CONFIDENCE: Your confidence level in this recommendation (0.0 to 1.0)

Be specific about betting sizes if recommending a raise. Consider position, stack depths, and opponent tendencies if observable.

Format your response as JSON with these exact keys:
{
  "action_recommendation": "your recommendation",
  "reasoning": "detailed explanation",
  "hand_strength": "strength assessment", 
  "pot_odds": "odds calculation or null",
  "confidence": 0.85
}
"#;

    let payload = json!({
        "model": "gpt-4.1-nano-2025-04-14",
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": poker_prompt
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/jpeg;base64,{}", base64_image),
                            "detail": "high"
                        }
                    }
                ]
            }
        ],
        "max_tokens": 500,
        "temperature": 0.1
    });

    log::info!("Sending request to OpenAI API...");
    
    let response = client
        .post(OPENAI_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("OpenAI API error: {}", error_text).into());
    }

    let response_json: Value = response.json().await?;
    
    log::info!("Received response from OpenAI API");
    
    // Extract the content from the response
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content in OpenAI response")?;
    
    log::info!("OpenAI response content: {}", content);
    
    // Parse the JSON response from GPT-4
    let analysis: PokerAnalysis = serde_json::from_str(content)
        .map_err(|e| {
            log::error!("Failed to parse OpenAI response as JSON: {}. Content: {}", e, content);
            format!("Failed to parse OpenAI response: {}", e)
        })?;
    
    log::info!("Successfully parsed poker analysis");
    
    Ok(analysis)
}

// Helper function for testing without screenshots
#[allow(dead_code)]
pub async fn test_openai_connection() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;
    
    let client = Client::new();
    
    let payload = json!({
        "model": "gpt-4.1-nano-2025-04-14",
        "messages": [
            {
                "role": "user",
                "content": "Say 'Hello from Poker Analyzer!'"
            }
        ],
        "max_tokens": 10
    });

    let response = client
        .post(OPENAI_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let response_json: Value = response.json().await?;
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response");
    
    Ok(content.to_string())
} 