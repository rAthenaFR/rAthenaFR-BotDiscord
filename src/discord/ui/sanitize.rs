#![allow(dead_code)]

pub fn sanitize_embed_mentions(value: &str) -> String {
    value
        .replace('@', "@\u{200B}")
        .replace("discord.gg/", "discord.gg/\u{200B}")
}
