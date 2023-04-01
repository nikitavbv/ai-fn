use {
    std::fs::read_to_string,
    proc_macro::TokenStream,
    async_openai::{
        Client,
        Chat,
        types::{CreateChatCompletionRequest, ChatCompletionRequestMessage, Role},
    },
};

#[proc_macro_attribute]
pub fn ai_fn(item: TokenStream, item2: TokenStream) -> TokenStream {
    let token = read_to_string(".openai_api_token");
    
    let client = Client::new().with_api_key(token.unwrap());
    let chat = Chat::new(&client);

    let mut request = CreateChatCompletionRequest::default();
    request.model = "gpt-3.5-turbo".to_owned();
    request.messages = vec![
        ChatCompletionRequestMessage {
            role: Role::System,
            content: "You are an assistant that provides code snippest in Rust for each user query. Your whole reply should be code snippet only. Do not include any description or explanation. Include \"use\" statements.".to_owned(),
            name: None,
        },
        ChatCompletionRequestMessage {
            role: Role::User,
            content: format!("{}. Use the following function signature: {}", item, item2),
            name: None,
        }
    ];

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let res = rt.block_on(chat.create(request)).unwrap();
    let mut res = res.choices[0].message.content.to_owned();

    if let Some(rust_start) = res.find("```rust") {
        res = res[rust_start + 7..].to_owned();
    }
    
    if let Some(end) = res.rfind("```") {
        res = res[..end].to_owned();
    }

    if let Some(rust_start) = res.find("```") {
        res = res[rust_start + 3..].to_owned();
    }

    res.parse().unwrap()
}