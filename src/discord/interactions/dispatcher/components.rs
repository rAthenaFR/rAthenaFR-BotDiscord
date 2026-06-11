use super::*;

impl Handler {
    pub(super) async fn handle_component(
        &self,
        context: &Context,
        component: &ComponentInteraction,
    ) -> Result<()> {
        let Some(page_request) = parse_mvp_list_component_id(&component.data.custom_id) else {
            component
                .create_response(
                    &context.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .embed(embeds::error_embed_l10n(
                                self.locale_for_component(component),
                                &translate(
                                    self.locale_for_component(component),
                                    I18nKey::ErrorInvalidMvpPanel,
                                ),
                            ))
                            .ephemeral(true),
                    ),
                )
                .await?;

            return Ok(());
        };

        let max_page_size = mvp_list_max_page_size(self.state.config.display.max_limit);
        let page_size = page_request.page_size.clamp(1, max_page_size);
        let lines = self
            .state
            .database
            .mvp_list_lines(
                &self.state.config.commands.mob_table_name,
                MVP_LIST_FETCH_LIMIT,
            )
            .await?;

        self.update_mvp_list_panel(context, component, lines, page_request.page, page_size)
            .await
    }

    pub(super) async fn respond_mvp_list_panel(
        &self,
        context: &Context,
        command: &CommandInteraction,
        lines: Vec<String>,
        page: usize,
        page_size: usize,
    ) -> Result<()> {
        let max_page_size = mvp_list_max_page_size(self.state.config.display.max_limit);
        let page_size = page_size.clamp(1, max_page_size);
        let page = clamp_mvp_list_page(page, lines.len(), page_size);
        let components = mvp_list_components(
            self.locale_for_command(command),
            page,
            page_size,
            lines.len(),
        );

        command
            .create_response(
                &context.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(embeds::mvp_list_panel_embed_l10n(
                            self.locale_for_command(command),
                            &lines,
                            page,
                            page_size,
                        ))
                        .components(components),
                ),
            )
            .await?;

        Ok(())
    }

    pub(super) async fn update_mvp_list_panel(
        &self,
        context: &Context,
        component: &ComponentInteraction,
        lines: Vec<String>,
        page: usize,
        page_size: usize,
    ) -> Result<()> {
        let max_page_size = mvp_list_max_page_size(self.state.config.display.max_limit);
        let page_size = page_size.clamp(1, max_page_size);
        let page = clamp_mvp_list_page(page, lines.len(), page_size);
        let components = mvp_list_components(
            self.locale_for_component(component),
            page,
            page_size,
            lines.len(),
        );

        component
            .create_response(
                &context.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(embeds::mvp_list_panel_embed_l10n(
                            self.locale_for_component(component),
                            &lines,
                            page,
                            page_size,
                        ))
                        .components(components),
                ),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) struct MvpListPageRequest {
    pub(super) page: usize,
    pub(super) page_size: usize,
}

pub(super) fn parse_mvp_list_component_id(custom_id: &str) -> Option<MvpListPageRequest> {
    let payload = custom_id.strip_prefix(MVP_LIST_COMPONENT_PREFIX)?;
    let mut parts = payload.split(':');
    let page = parts.next()?.parse::<usize>().ok()?;
    let page_size = parts.next()?.parse::<usize>().ok()?;
    let action = parts.next()?;

    if parts.next().is_some()
        || page_size == 0
        || !matches!(action, "first" | "previous" | "next" | "last")
    {
        return None;
    }

    Some(MvpListPageRequest { page, page_size })
}

pub(super) fn mvp_list_component_id(action: &str, page: usize, page_size: usize) -> String {
    format!("{MVP_LIST_COMPONENT_PREFIX}{page}:{page_size}:{action}")
}

pub(super) fn mvp_list_page_count(total_count: usize, page_size: usize) -> usize {
    total_count.div_ceil(page_size.max(1)).max(1)
}

pub(super) fn mvp_list_max_page_size(configured_max: u32) -> usize {
    (configured_max as usize).clamp(1, MVP_LIST_PAGE_SIZE_LIMIT)
}

pub(super) fn clamp_mvp_list_page(page: usize, total_count: usize, page_size: usize) -> usize {
    page.min(mvp_list_page_count(total_count, page_size).saturating_sub(1))
}

pub(super) fn mvp_list_components(
    locale: BotLocale,
    page: usize,
    page_size: usize,
    total_count: usize,
) -> Vec<CreateActionRow> {
    let page_size = page_size.max(1);
    let page_count = mvp_list_page_count(total_count, page_size);

    if page_count <= 1 {
        return Vec::new();
    }

    let page = clamp_mvp_list_page(page, total_count, page_size);
    let last_page = page_count.saturating_sub(1);

    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(mvp_list_component_id("first", 0, page_size))
            .label(translate(locale, I18nKey::ButtonFirst))
            .style(ButtonStyle::Secondary)
            .disabled(page == 0),
        CreateButton::new(mvp_list_component_id(
            "previous",
            page.saturating_sub(1),
            page_size,
        ))
        .label(translate(locale, I18nKey::ButtonPrevious))
        .style(ButtonStyle::Primary)
        .disabled(page == 0),
        CreateButton::new(mvp_list_component_id(
            "next",
            (page + 1).min(last_page),
            page_size,
        ))
        .label(translate(locale, I18nKey::ButtonNext))
        .style(ButtonStyle::Primary)
        .disabled(page >= last_page),
        CreateButton::new(mvp_list_component_id("last", last_page, page_size))
            .label(translate(locale, I18nKey::ButtonLast))
            .style(ButtonStyle::Secondary)
            .disabled(page >= last_page),
    ])]
}
