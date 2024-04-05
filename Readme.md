Bing Copilot Async Client

## features

`default = ["rustls"]`

Using rustls by default makes it simpler to compile across platforms.

If you want to use native-tls, you can use `{------, features = ["native-tls"],default-features = false}`

## the cookie to build client

You can use one of the enum to build a client;
The cookie can be got by using `Cookie Editor` or other browser extensions.
(Also you can you javascript to get it by yourself)
```rust
pub enum Cookie {
    // Json means format like this:
    // [
    // {
    //     "domain": ".bing.com",
    //     "expirationDate": 1743827661.986849,
    //     "hostOnly": false,
    //     "httpOnly": false,
    //     "name": "SnrOvr",
    //     "path": "/",
    //     "sameSite": "no_restriction",
    //     "secure": true,
    //     "session": false,
    //     "storeId": null,
    //     "value": "X=rebateson"
    // },
    // ······
    // ]
    JsonPath(String),
    JsonStr(String),
    // Head means format like:
    // SnrOvr=X=rebateson;SRCHUSR=DOB=20240323&T=1712299341000&TPC=1711617907000&POEX=W; ······
    HeadPath(String),
    HeadStr(String),
}
```

## build a client without chats

```rust
use bing_client::BingClient;
#[tokio::main]
async fn main(){
    let client = BingClient::build_with_chats(&Cookie::JsonPath("path to cookie json".to_string()))
}
```

## build a client with chats

```rust
let client = BingClient::build(&Cookie::JsonPath("path to cookie json".to_string()))
    .await
    .unwrap();
client.chats.iter().for_each(|chat| {
    println!("{}", chat);
});
```

## Serialize and Deserialize a client

```rust
let client = BingClient::build(&Cookie::JsonPath("path to cookie json".to_string())).await.unwrap();
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
let client = BingClient::build(&Cookie::JsonPath("path to cookie json".to_string())).await.unwrap();
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

let (mut stream, stop_fn) = client
    .ask_stream_plain(&mut new_chat, user_input)
    .await
    .unwrap();
while let GeneratorState::Yielded(data) = stream.async_resume().await {
    print!("\x1b[2J\x1b[H");
    println!("{data}");
}
```

## Ask question in a chat, and get muti type reply

```rust
let client = BingClient::build(&Cookie::JsonPath("path to cookie json".to_string())).await.unwrap();
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
let (mut stream, stop_fn) = client
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
```

## Stop answering

```rust
let client = BingClient::build(&Cookie::JsonPath("path to cookie json".to_string()))
    .await
    .unwrap();
let mut new_chat = client.create_chat().await.unwrap();
let user_input = UserInput::build(
    "Write a science fiction story.".to_string(),
    None,
    crate::types::user_input_type::Tone::Creative,
    vec![],
    &new_chat,
    &client,
)
.await
.unwrap();
let (mut stream, stop_fn) = client
    .ask_stream_plain(&mut new_chat, user_input)
    .await
    .unwrap();
// For example, stop the client from answering after four answers

let mut times = 0;
while let GeneratorState::Yielded(data) = stream.async_resume().await {
    times += 1;
    if times == 4{
        println!("try stop");
        stop_fn();
    }
    print!("{} ",data.len());
}
```

## Draw images

```rust
let imgs = client.draw_image("a bird").await.unwrap();
```
