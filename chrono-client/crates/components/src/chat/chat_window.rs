use leptos::*;
use data_types::chat_list::{ChatInfo, ChatListReq, ChatListRes};
use data_types::friend::Friends;
use wasm_http::http_ctx::HttpCtx;
use crate::empty::Empty;
use super::textarea::Textarea;
use super::chat_list::{ChatList};

#[component]
pub fn ChatWindow(read_select_friend: ReadSignal<Friends>) -> impl IntoView {
    let (read_send_content, write_send_content) = create_signal::<Vec<ChatInfo>>(vec![]);
    let (chat_window, write_chat_window) = create_signal(view! { <Empty/> });
    let (read_friend, write_friend) = create_signal(Friends::default());
    create_resource(
        move || read_select_friend.get(),
        move |friend| async move {
            write_friend.set(friend.clone());
            write_send_content.set(vec![]);
            let http_ctx = HttpCtx::default();
            if let Ok(Some(data)) = http_ctx.post::<ChatListReq, ChatListRes>("/api/chat/list", &ChatListReq {
                receiver: friend.peer_id.to_string(),
                page_no: 1,
                page_size: 10,
            }).await {
                write_send_content.set(data.list);
            }
            let data = view! {
                <div class="chat-window">
                    <div class="chat-window-title">
                        <h3>{friend.nickname}</h3>
                    </div>
                    <div class="chat-record">
                        <ChatList read_send_content myself=read_select_friend friend=read_select_friend/>
                    </div>
                    <div class="chat-tool"></div>
                    <div class="chat-input">
                        <Textarea write_send_content read_friend/>
                    </div>
                </div>
            }.into_view();
            write_chat_window.set(data);
        },
    );
    view! {{move || chat_window.get() }}
}