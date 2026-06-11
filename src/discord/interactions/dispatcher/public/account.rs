use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_createaccount(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.state.config.account_commands.creation_enabled {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::account_creation_disabled_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        let Some(username) = string_option(command, "username") else {
            return self
                .respond_missing_option(context, command, "username")
                .await;
        };
        let Some(password) = string_option(command, "password") else {
            return self
                .respond_missing_option(context, command, "password")
                .await;
        };
        let Some(sex) = string_option(command, "sex") else {
            return self.respond_missing_option(context, command, "sex").await;
        };
        let Some(birthdate) = string_option(command, "birthdate") else {
            return self
                .respond_missing_option(context, command, "birthdate")
                .await;
        };

        let username =
            match validate_account_username_l10n(self.locale_for_command(command), username) {
                Ok(username) => username,
                Err(message) => return self.respond_error(context, command, &message).await,
            };
        let password =
            match validate_account_password_l10n(self.locale_for_command(command), password) {
                Ok(password) => password,
                Err(message) => return self.respond_error(context, command, &message).await,
            };
        let sex = match validate_account_sex_l10n(self.locale_for_command(command), sex) {
            Ok(sex) => sex,
            Err(message) => return self.respond_error(context, command, &message).await,
        };

        let birthdate =
            match validate_account_birthdate_l10n(self.locale_for_command(command), birthdate) {
                Ok(birthdate) => birthdate,
                Err(message) => return self.respond_error(context, command, &message).await,
            };

        let email = match validate_account_email_l10n(
            self.locale_for_command(command),
            string_option(command, "email"),
        ) {
            Ok(email) => email,
            Err(message) => return self.respond_error(context, command, &message).await,
        };

        if self.state.database.account_userid_exists(&username).await? {
            return self
                .respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorAccountExists,
                    &[TranslationArg::new("account", &username)],
                )
                .await;
        }

        let account = self
            .state
            .database
            .create_account(
                &username,
                &password,
                self.state.config.account_commands.password_mode,
                &sex,
                &birthdate,
                &email,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::account_created_embed_l10n(self.locale_for_command(command), &account),
            true,
        )
        .await
    }
}
