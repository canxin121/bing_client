Bing Copilot Async Client

## build a client without chats

```rust
use bing_client::BingClient;
#[tokio::main]
async fn main(){
    let client = BingClient::build("path to cookie.json").await.unwrap();
}
```

## build a client with chats

```rust
let client = BingClient::build_with_chats("path to cookie.json")
    .await
    .unwrap();
client.chats.iter().for_each(|chat| {
    println!("{}", chat);
});
```

## Serialize and Deserialize a client

```rust
let client = BingClient::build_with_chats("_data/cookie.json").await.unwrap();
let client_str = serde_json::to_string(&client).unwrap();
let client = serde_json::from_str::<BingClient>(&client_str).unwrap();
```

## Get chat list

```rust
let chats = client.get_chat_list().await.unwrap();
```

## Create a new chat

```rust
let chat = client.create_chat().await.unwrap();
```

## Delete a chat

```rust
client.delete_chat(&mut chat).await.unwrap();
```

## Rename a chat

```rust
client..rename_chat(&mut chat, "new name".to_string()).await.unwrap();
```

## Get chat messages

```rust
let messages = client.get_chat_messages(&mut last_chat).await.unwrap();
```

## Ask question in a chat, and get only string(markdown) reply

```rust
#[tokio::main]
async fn main() {
    let client = BingClient::build("cookie.json").await.unwrap();
    let mut new_chat = client.create_chat().await.unwrap();
    let user_input = UserInput::build(
        // Text question
        "hello".to_string(),
        // Image attachment, uncomment this
        // Some(Image::Path(r"example_image.jpg".to_string())),
        None,
        // Chat Tone
        Tone::Balanced,
        // plugins to use
        vec![Plugin::search()],
        &new_chat,
        &client,
    )
    .await
    .unwrap();

    let mut stream = client
        .ask_stream_plain(&mut new_chat, user_input)
        .await
        .unwrap();
    while let GeneratorState::Yielded(data) = stream.async_resume().await {
        print!("\x1b[2J\x1b[H");
        println!("{data}");
    }
}
```

## Ask question in a chat, and get muti type reply

```rust
#[tokio::main]
async fn main() {
    let client = BingClient::build("cookie.json").await.unwrap();
    let mut new_chat = client.create_chat().await.unwrap();
    let user_input = UserInput::build(
        "hello".to_string(),
        None,
        crate::types::user_input_type::Tone::Balanced,
        vec![Plugin::search()],
        &new_chat,
        &client,
    )
    .await
    .unwrap();
    let mut stream = client
        .ask_stream(&mut new_chat, user_input)
        .await
        .unwrap();
    while let GeneratorState::Yielded(data) = stream.async_resume().await {
        print!("\x1b[2J\x1b[H");
        match data {
            crate::types::bot_easy_resp::BotResp::Text(text) => todo!(),
            crate::types::bot_easy_resp::BotResp::SuggestReply(suggest_replys) => todo!(),
            crate::types::bot_easy_resp::BotResp::Notice(notice) => todo!(),
            crate::types::bot_easy_resp::BotResp::Image(images) => todo!(),
            crate::types::bot_easy_resp::BotResp::Apology(apology) => todo!(),
            crate::types::bot_easy_resp::BotResp::SourceAttribution(sources) => todo!(),
            crate::types::bot_easy_resp::BotResp::Limit(limit) => todo!(),
        }
    }
}
```

## Draw images

```rust
let imgs = client.draw_image("a bird").await.unwrap();
```