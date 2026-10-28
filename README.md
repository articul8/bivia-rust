# Bivia Rust SDK

[![CI](https://github.com/articul8/bivia-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/articul8/bivia-rust/actions/workflows/ci.yml)

Rust client for [Bivia](https://articul8.ai/bivia) by [Articul8 AI](https://articul8.ai). See [bivia](https://github.com/articul8/bivia) for full documentation and examples.

## Installation

```toml
[dependencies]
bivia = "1.0"
tokio = { version = "1", features = ["full"] }
```

## Authentication

Set the `A8_API_KEY` environment variable:

```bash
export A8_API_KEY="your-api-key"
```

## Usage

### Boolean decision

```rust,no_run
use bivia::{Client, types::BooleanParams};

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::new()?;

    let result = client.decisions().boolean(BooleanParams {
        question: "Is the sky blue?".into(),
        state: Some("The sky is clear today.".into()),
        ..Default::default()
    }).await?;

    println!("{}: p={:?}", result.status, result.probability);
    Ok(())
}
```

### Choice decision

```rust,no_run
use bivia::{Client, types::ChoiceParams};

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::new()?;

    let result = client.decisions().choice(ChoiceParams {
        question: "What is the primary color?".into(),
        choices: vec!["red".into(), "green".into(), "blue".into()],
        state: Some("The object is a fire truck.".into()),
        ..Default::default()
    }).await?;

    println!("choice={:?} p={:?}", result.value, result.probability);
    Ok(())
}
```

### Score

```rust,no_run
use bivia::{Client, types::ScoreParams};

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::new()?;

    let result = client.decisions().score(ScoreParams {
        question: "Rate the sentiment".into(),
        state: Some("I love this product!".into()),
        range: Some([1.0, 5.0]),
        ..Default::default()
    }).await?;

    println!("score={:?}", result.value);
    Ok(())
}
```

### Multi-select

```rust,no_run
use bivia::{Client, types::SetParams};

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::new()?;

    let result = client.decisions().multi_select(SetParams {
        question: "Which topics are discussed?".into(),
        options: vec!["sports".into(), "politics".into(), "weather".into()],
        state: Some("The game was rained out today.".into()),
        ..Default::default()
    }).await?;

    println!("selected={:?}", result.value);
    Ok(())
}
```

### Image grounding

```rust,no_run
use bivia::{Client, types::{GroundParams, ImageRef}};

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::new()?;

    let result = client.grounding().image(GroundParams {
        question: Some("Find all cars".into()),
        images: Some(vec![ImageRef {
            url: Some("https://example.com/photo.jpg".into()),
            ..Default::default()
        }]),
        ..Default::default()
    }).await?;

    for region in result.value {
        println!("{}: {:?}", region.label, region.box_coords);
    }
    Ok(())
}
```

### Batch (SystemOne)

```rust,no_run
use bivia::{Client, types::{BatchParams, BatchQuestion}};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::new()?;

    let mut questions = HashMap::new();
    questions.insert("sentiment".to_string(), BatchQuestion {
        question_type: Some("boolean".into()),
        question: Some("Is this positive?".into()),
        ..Default::default()
    });

    let result = client.batch().create(BatchParams {
        state: Some("I love Rust!".into()),
        questions: Some(questions),
        ..Default::default()
    }).await?;

    println!("answers={:?}", result.answers);
    Ok(())
}
```

### List models

```rust,no_run
use bivia::Client;

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::new()?;
    let models = client.models().list().await?;

    for m in models.models {
        println!("{}: {:?}", m.name, m.description);
    }
    Ok(())
}
```

### Custom configuration

```rust,no_run
use bivia::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), bivia::BiviaError> {
    let client = Client::builder()
        .api_key("your-api-key")
        .base_url("https://custom.api.example.com/bivia")
        .timeout(Duration::from_secs(60))
        .max_retries(3)
        .build();

    let caps = client.models().capabilities().await?;
    println!("{:?}", caps);
    Ok(())
}
```

## License

MIT
