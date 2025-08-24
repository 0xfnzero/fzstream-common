//! # FZ Stream Common Library
//! 
//! 这个库包含了 fz-stream-server 和 fz-stream-client 之间共享的数据结构和类型定义。

pub mod events;
pub mod auth;
pub mod config;
pub mod compression;
pub mod compression_stats;

// 定义 match_event! 宏
#[macro_export]
macro_rules! match_event {
    ($event:expr, {
        $($event_type:ident => |$e:ident: $event_struct:ty| $body:block,)*
        _ => $default:block
    }) => {
        $(
            if let Some($e) = $event.as_any().downcast_ref::<$event_struct>() {
                $body
                return;
            }
        )*
        $default
    };
    ($event:expr, {
        $($event_type:ident => |$e:ident: $event_struct:ty| $body:block,)*
    }) => {
        $(
            if let Some($e) = $event.as_any().downcast_ref::<$event_struct>() {
                $body
                return;
            }
        )*
    };
}

// Re-export main types
pub use events::*;
pub use auth::*;
pub use config::*;
pub use compression::*;
pub use compression_stats::*;
