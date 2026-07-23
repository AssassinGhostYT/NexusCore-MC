use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;
use crate::macros::helpers::write_string;

pub const ID_TEXT: u32 = 9; // 0x09

pub const TYPE_RAW: u8 = 0;
pub const TYPE_CHAT: u8 = 1;
pub const TYPE_TRANSLATION: u8 = 2;
pub const TYPE_POPUP: u8 = 3;
pub const TYPE_JUKEBOX_POPUP: u8 = 4;
pub const TYPE_TIP: u8 = 5;
pub const TYPE_SYSTEM: u8 = 6;
pub const TYPE_WHISPER: u8 = 7;
pub const TYPE_ANNOUNCEMENT: u8 = 8;
pub const TYPE_JSON_WHISPER: u8 = 9;
pub const TYPE_JSON: u8 = 10;
pub const TYPE_JSON_ANNOUNCEMENT: u8 = 11;

pub const CATEGORY_MESSAGE_ONLY: u8 = 0;
pub const CATEGORY_AUTHORED_MESSAGE: u8 = 1;
pub const CATEGORY_MESSAGE_WITH_PARAMETERS: u8 = 2;

/// Text packet (ID = 9 / 0x09).
/// Used for chat messages, system messages, popups, and tips.
pub struct Text {
    pub type_id: u8,
    pub needs_translation: bool,
    pub source_name: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub xbox_user_id: String,
    pub platform_chat_id: String,
    pub filtered_message: Option<String>,
}

impl Text {
    pub fn raw(message: String) -> Self {
        Self {
            type_id: TYPE_RAW,
            needs_translation: false,
            source_name: String::new(),
            message,
            parameters: Vec::new(),
            xbox_user_id: String::new(),
            platform_chat_id: String::new(),
            filtered_message: None,
        }
    }

    pub fn chat(source_name: String, message: String) -> Self {
        Self {
            type_id: TYPE_CHAT,
            needs_translation: false,
            source_name,
            message,
            parameters: Vec::new(),
            xbox_user_id: String::new(),
            platform_chat_id: String::new(),
            filtered_message: None,
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        buf.push(if self.needs_translation { 1 } else { 0 });

        let category = match self.type_id {
            TYPE_RAW | TYPE_TIP | TYPE_SYSTEM | TYPE_JSON_WHISPER | TYPE_JSON | TYPE_JSON_ANNOUNCEMENT => CATEGORY_MESSAGE_ONLY,
            TYPE_CHAT | TYPE_WHISPER | TYPE_ANNOUNCEMENT => CATEGORY_AUTHORED_MESSAGE,
            TYPE_TRANSLATION | TYPE_POPUP | TYPE_JUKEBOX_POPUP => CATEGORY_MESSAGE_WITH_PARAMETERS,
            _ => CATEGORY_MESSAGE_ONLY,
        };

        buf.push(category);
        buf.push(self.type_id);

        match self.type_id {
            TYPE_CHAT | TYPE_WHISPER | TYPE_ANNOUNCEMENT => {
                write_string(&mut buf, &self.source_name);
                write_string(&mut buf, &self.message);
            }
            TYPE_RAW | TYPE_TIP | TYPE_SYSTEM | TYPE_JSON_WHISPER | TYPE_JSON | TYPE_JSON_ANNOUNCEMENT => {
                write_string(&mut buf, &self.message);
            }
            TYPE_TRANSLATION | TYPE_POPUP | TYPE_JUKEBOX_POPUP => {
                write_string(&mut buf, &self.message);
                write_varu32(&mut buf, self.parameters.len() as u32);
                for param in &self.parameters {
                    write_string(&mut buf, param);
                }
            }
            _ => {
                write_string(&mut buf, &self.message);
            }
        }

        write_string(&mut buf, &self.xbox_user_id);
        write_string(&mut buf, &self.platform_chat_id);

        match &self.filtered_message {
            Some(msg) => {
                buf.push(1);
                write_string(&mut buf, msg);
            }
            None => {
                buf.push(0);
            }
        }

        Ok(buf)
    }
}
