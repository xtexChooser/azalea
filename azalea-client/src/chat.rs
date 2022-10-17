use std::time::{SystemTime, UNIX_EPOCH};

use azalea_crypto::MessageSignature;
use azalea_protocol::packets::game::{
    clientbound_player_chat_packet::LastSeenMessagesUpdate,
    serverbound_chat_command_packet::ServerboundChatCommandPacket,
    serverbound_chat_packet::ServerboundChatPacket,
};

use crate::Client;

impl Client {
    /// Sends chat message to the server. This only sends the chat packet and
    /// not the command packet. The `chat` function handles checking whether
    /// the message is a command and using the proper packet for you.
    pub async fn send_chat_packet(&self, message: &str) -> Result<(), std::io::Error> {
        // TODO: chat signing
        let signature = sign_message();
        let packet = ServerboundChatPacket {
            message: message.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time shouldn't be before epoch")
                .as_millis()
                .try_into()
                .expect("Instant should fit into a u64"),
            salt: azalea_crypto::make_salt(),
            signature,
            signed_preview: false,
            last_seen_messages: LastSeenMessagesUpdate::default(),
        }
        .get();
        self.write_packet(packet).await
    }

    /// Send a command packet to the server. The `command` argument should not
    /// include the slash at the front.
    pub async fn send_command_packet(&self, command: &str) -> Result<(), std::io::Error> {
        // TODO: chat signing
        let packet = ServerboundChatCommandPacket {
            command: command.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time shouldn't be before epoch")
                .as_millis()
                .try_into()
                .expect("Instant should fit into a u64"),
            salt: azalea_crypto::make_salt(),
            argument_signatures: vec![],
            signed_preview: false,
            last_seen_messages: LastSeenMessagesUpdate::default(),
        }
        .get();
        self.write_packet(packet).await
    }

    pub async fn chat(&self, message: &str) -> Result<(), std::io::Error> {
        if message.chars().next() == Some('/') {
            self.send_command_packet(&message[1..]).await
        } else {
            self.send_chat_packet(message).await
        }
    }

    // will be used for when the server tells the client about a chat preview
    // with custom formatting
    // pub fn acknowledge_preview(&self, message: &str) {}
}

// TODO
// MessageSigner, ChatMessageContent, LastSeenMessages
fn sign_message() -> MessageSignature {
    MessageSignature::default()
}