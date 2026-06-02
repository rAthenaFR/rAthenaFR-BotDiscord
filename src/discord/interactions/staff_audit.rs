use crate::discord::embeds;
use serenity::all::{ChannelId, CommandInteraction, Context, CreateAllowedMentions, CreateMessage};
use tracing::{error, info};

pub(super) struct StaffAuditLogger<'a> {
    context: &'a Context,
    command: &'a CommandInteraction,
    channel_id: Option<u64>,
}

pub(super) struct GmmsgAuditEntry<'a> {
    pub status: embeds::GmmsgLogStatus,
    pub action: &'a str,
    pub message: &'a str,
    pub result: &'a str,
}

pub(super) struct AccountManageAuditEntry<'a> {
    pub status: embeds::AccountManageLogStatus,
    pub action: &'a str,
    pub account: &'a str,
    pub result: &'a str,
    pub reason: Option<&'a str>,
}

impl<'a> StaffAuditLogger<'a> {
    pub(super) fn new(
        context: &'a Context,
        command: &'a CommandInteraction,
        channel_id: Option<u64>,
    ) -> Self {
        Self {
            context,
            command,
            channel_id,
        }
    }

    pub(super) async fn log_gmmsg(&self, entry: GmmsgAuditEntry<'_>) {
        let Some(channel_id) = self.channel_id else {
            info!(
                user_id = self.command.user.id.get(),
                action = entry.action,
                message = entry.message,
                result = entry.result,
                "Action staff GMMSG traitee."
            );
            return;
        };

        let embed = embeds::gmmsg_staff_log_embed(
            entry.status,
            self.command.user.id.get(),
            entry.action,
            entry.message,
            entry.result,
        );
        self.send_embed(channel_id, embed).await;
    }

    pub(super) async fn log_account_manage(&self, entry: AccountManageAuditEntry<'_>) {
        let Some(channel_id) = self.channel_id else {
            info!(
                user_id = self.command.user.id.get(),
                action = entry.action,
                account = entry.account,
                result = entry.result,
                "Action staff account-manage traitee."
            );
            return;
        };

        let embed = embeds::account_manage_staff_log_embed(
            entry.status,
            self.command.user.id.get(),
            entry.action,
            entry.account,
            entry.result,
            entry.reason,
        );
        self.send_embed(channel_id, embed).await;
    }

    async fn send_embed(&self, channel_id: u64, embed: serenity::all::CreateEmbed) {
        if let Err(error) = ChannelId::new(channel_id)
            .send_message(
                &self.context.http,
                CreateMessage::new()
                    .embed(embed)
                    .allowed_mentions(CreateAllowedMentions::new()),
            )
            .await
        {
            error!(error = %error, "Impossible d'envoyer le log staff Discord.");
        }
    }
}
