use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_character_log_command(
        &self,
        context: &Context,
        command: &CommandInteraction,
        table_name: &str,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .character_log_lines(table_name, character, query_limit)
            .await?;
        self.respond_lines(context, command, table_name, lines, true)
            .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_variable_command(
        &self,
        context: &Context,
        command: &CommandInteraction,
        table_name: &str,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .variable_lines(table_name, character, query_limit)
            .await?;
        self.respond_lines(context, command, table_name, lines, true)
            .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_chat_search(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(text) = string_option(command, "text") else {
            return self.respond_missing_option(context, command, "text").await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .named_log_lines("chatlog", text, query_limit)
            .await?;
        self.respond_lines_key(context, command, I18nKey::TitleChatLogSearch, lines, true)
            .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_report_context(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let mut lines = Vec::new();
        if let Some(profile) = self.state.database.find_player(i32::MAX, character).await? {
            lines.push(format!(
                "Position: `{}` - online `{}`",
                profile.map, profile.online
            ));
        }
        lines.extend(
            self.state
                .database
                .character_log_lines("chatlog", character, query_limit)
                .await?,
        );
        self.respond_lines_key(context, command, I18nKey::TitleReportContext, lines, true)
            .await
    }
}
