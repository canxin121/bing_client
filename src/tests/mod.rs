#[cfg(test)]
mod test {


    use genawaiter::GeneratorState;
    use tokio::io::AsyncWriteExt;
    
    use crate::{
        client::BingClient,
        types::{cookie_type::Cookie, plugin_type::Plugin, user_input_type::UserInput},
    };

    #[tokio::test]
    async fn test_build_client_with_path() {
        let client =
            BingClient::build_with_chats(&Cookie::JsonPath("_data/cookie.json".to_string()))
                .await
                .unwrap();
        println!("Client cookie: {}", client.cookie_str);
        println!("Client id: {}", client.client_id);
        println!("Total {} chats got.", client.chats.len());
        for (index, chat) in client.chats.iter().enumerate() {
            println!("{index}: {chat}");
        }
    }

    #[tokio::test]
    async fn test_build_client_with_str() {
        let client = BingClient::build_with_chats(&Cookie::HeadStr(
            include_str!(r#"D:\Git\bing_client\_data\cookie"#).to_string(),
        ))
        .await
        .unwrap();
        println!("Client cookie: {}", client.cookie_str);
        println!("Client id: {}", client.client_id);
        println!("Total {} chats got.", client.chats.len());
        for (index, chat) in client.chats.iter().enumerate() {
            println!("{index}: {chat}");
        }
    }

    #[tokio::test]
    async fn test_create_chat() {
        let client = BingClient::build(&Cookie::JsonPath("_data/cookie.json".to_string()))
            .await
            .unwrap();
        let chat = client.create_chat().await.unwrap();
        println!("{:?}", chat)
    }

    #[tokio::test]
    async fn test_del_chat() {
        let client =
            BingClient::build_with_chats(&Cookie::JsonPath("_data/cookie.json".to_string()))
                .await
                .unwrap();
        let last_chat = client.chats.last().unwrap();
        match client.delete_chat(& last_chat).await {
            Ok(_) => {
                println!("删除成功")
            }
            Err(e) => {
                println!("删除失败!\n{e}")
            }
        }
    }
    
    #[tokio::test]
    async fn test_del_chats() {
        let client =
            BingClient::build_with_chats(&Cookie::JsonPath("_data/cookie.json".to_string()))
                .await
                .unwrap();
        let last_chat = client.chats.last().unwrap();
        match client.delete_chats(crate::TodelChats::Chats(vec![& last_chat])).await {
            Ok(_) => {
                println!("删除成功")
            }
            Err(e) => {
                println!("删除失败!\n{e}")
            }
        }
    }

    #[tokio::test]
    async fn test_get_chat_msgs() {
        let client = 
            // BingClient::build_with_chats(&Cookie::HeadPath("_data/cookie".to_string()))
            BingClient::build_with_chats(&Cookie::JsonPath("_data/cookie.json".to_string()))
            .await
            .unwrap();
        // let last_chat = client.chats.first().unwrap();
        let last_chat = client.chats.iter().find(|chat|{chat.chat_name.as_ref().unwrap().contains("画")}).unwrap();
        match client.get_chat_messages(& last_chat).await {
            Ok(value) => {
                println!("成功获取 chat 的messages: {:#?}", value);
                let mut file = tokio::fs::File::create("./_data/msgs.json").await.unwrap();
                file.write_all(serde_json::to_string(&value).unwrap().as_bytes())
                    .await
                    .unwrap();
            }
            Err(e) => {
                println!("获取失败!\n{e}")
            }
        }
    }

    #[tokio::test]
    async fn test_reanme_chat() {
        let client =
            BingClient::build_with_chats(&Cookie::JsonPath("_data/cookie.json".to_string()))
                .await
                .unwrap();
        let last_chat = client.chats.first().unwrap();
        client
            .rename_chat(& last_chat, "1234".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_gen_image_id() {
        let client = BingClient::build(&Cookie::JsonPath("_data/cookie.json".to_string()))
            .await
            .unwrap();
        let new_chat = client.create_chat().await.unwrap();
        let image_url = client
            .gen_upload_image_url(
                crate::utils::image_base64::Image::Path("example.jpg".to_string()),
                &new_chat,
            )
            .await
            .unwrap();
        println!("image_url: {image_url}")
    }

    #[tokio::test]
    async fn test_plain_chat() {
        let client = BingClient::build_with_chats(&Cookie::HeadPath("_data/cookie".to_string()))
            .await
            .unwrap();
        // let chat =  client.chats.iter().find(|chat|{
        //     chat.chat_name.as_ref().unwrap().contains("绘制猫咪")
        // }).unwrap();
        // let chat = client.chats.last().unwrap();
        let chat = client.create_chat().await.unwrap();
        let user_input = UserInput::build(
            "画一只猫".to_string(),
            // Some(Image::Path(
            //     r"D:\Git\bing_client\_data\{0AF8F716-2078-47e8-8842-01C8EC62D911}.png".to_string(),
            // )),
            None,
            crate::types::user_input_type::Tone::Creative,
            vec![
            Plugin::search()
            ],
            &chat,
            &client,
        )
        .await
        .unwrap();
        let (mut stream, _stop_fn) = client
            .ask_stream_plain(& chat, user_input)
            .await
            .unwrap();
        while let GeneratorState::Yielded(data) = stream.async_resume().await {
            println!("{data}");
        }
        let msgs = client.get_chat_messages(&chat).await.unwrap();
        println!("{:#?}",msgs);
    }

    #[tokio::test]
    async fn test_stop_chat() {
        let client = BingClient::build(&Cookie::JsonPath("_data/cookie.json".to_string()))
            .await
            .unwrap();
        let new_chat = client.create_chat().await.unwrap();
        let user_input = UserInput::build(
            "写一个科幻小说".to_string(),
            // Some(Image::Path(
            //     r"D:\Git\bing_client\_data\{0AF8F716-2078-47e8-8842-01C8EC62D911}.png".to_string(),
            // )),
            None,
            crate::types::user_input_type::Tone::Creative,
            vec![
            // Plugin::search()
            ],
            &new_chat,
            &client,
        )
        .await
        .unwrap();
        let (mut stream, stop_fn) = client
            .ask_stream_plain(& new_chat, user_input)
            .await
            .unwrap();
        let mut times = 0;

        while let GeneratorState::Yielded(data) = stream.async_resume().await {
            times += 1;
            if times == 4 {
                println!("try stop");
                stop_fn();
            }
            print!("{} ", data.len());
        }
    }

    #[tokio::test]
    #[allow(unused)]
    async fn test_chat() {
        let client = BingClient::build(&Cookie::JsonPath("_data/cookie.json".to_string()))
            .await
            .unwrap();
        let mut new_chat = client.create_chat().await.unwrap();
        let user_input = UserInput::build(
            "在吗".to_string(),
            // Some(Image::Path(r"D:\Git\bing_client\aaa.jpg".to_string())),
            None,
            crate::types::user_input_type::Tone::Balanced,
            vec![Plugin::search()],
            &new_chat,
            &client,
        )
        .await
        .unwrap();
        let (mut stream, stop_fn) = client.ask_stream(& new_chat, user_input).await.unwrap();
        while let GeneratorState::Yielded(data) = stream.async_resume().await {
            print!("\x1b[2J\x1b[H");
            match data {
                crate::types::bot_easy_resp_type::BotResp::Text(text) => todo!(),
                crate::types::bot_easy_resp_type::BotResp::SuggestReply(suggest_replys) => todo!(),
                crate::types::bot_easy_resp_type::BotResp::Notice(notice) => todo!(),
                crate::types::bot_easy_resp_type::BotResp::Image(images) => todo!(),
                crate::types::bot_easy_resp_type::BotResp::Apology(apology) => todo!(),
                crate::types::bot_easy_resp_type::BotResp::SourceAttribution(sources) => todo!(),
                crate::types::bot_easy_resp_type::BotResp::Limit(limit) => todo!(),
            }
        }
    }

    #[tokio::test]
    async fn test_draw() {
        let client = BingClient::build(&Cookie::JsonPath("_data/cookie.json".to_string()))
            .await
            .unwrap();
        let imgs = client.draw_image("a cat").await.unwrap();
        println!("{:?}", imgs);
    }

    #[tokio::test]
    async fn test_se_de() {
        let client =
            BingClient::build_with_chats(&Cookie::JsonPath("_data/cookie.json".to_string()))
                .await
                .unwrap();
        let client_str = serde_json::to_string(&client).unwrap();
        let client = serde_json::from_str::<BingClient>(&client_str).unwrap();
        let last_chat = client.chats.last().unwrap();
        println!("{:?}", last_chat);
        client.delete_chat(& last_chat).await.unwrap();
    }
    
    #[tokio::test]
    async fn build_chat_msg() {
        let client =
            BingClient::build_with_chats(&Cookie::JsonPath("_data/cookie.json".to_string()))
                .await
                .unwrap();
        // let chat =  client.chats.iter().find(|chat|{
        //     chat.chat_name.as_ref().unwrap().contains("绘制猫咪")
        // }).unwrap();
        // println!("{}",chat);
        let chat = client.create_chat().await.unwrap();
        let userinput = UserInput::build("在吗".to_string(), None, crate::Tone::Creative, vec![Plugin::search(),Plugin::suno()], &chat, &client).await.unwrap();
        let json = serde_json::to_string(&userinput).unwrap();
        println!("{}",json);
    }
}
