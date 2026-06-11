use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) fn has_staff_access(
        &self,
        command: &CommandInteraction,
    ) -> bool {
        self.has_staff_role(command, StaffRole::Helper)
    }

    pub(in crate::discord::interactions::dispatcher) fn has_staff_role(
        &self,
        command: &CommandInteraction,
        minimum_role: StaffRole,
    ) -> bool {
        let config = &self.state.config.discord;
        let Some(member) = command.member.as_ref() else {
            return false;
        };
        let member_role_ids = member
            .roles
            .iter()
            .map(|role| role.get())
            .collect::<Vec<_>>();

        let configured_roles = ConfiguredRoles {
            helper: &config.helper_role_ids,
            moderator: &config.moderator_role_ids,
            gm: &config.gm_role_ids,
            legacy_staff: &config.staff_role_ids,
            admin: &config.admin_role_ids,
            owner: &config.owner_role_ids,
        };

        has_configured_role(&member_role_ids, minimum_role, configured_roles)
    }
}

pub(in crate::discord::interactions::dispatcher) struct ConfiguredRoles<'a> {
    pub(in crate::discord::interactions::dispatcher) helper: &'a [u64],
    pub(in crate::discord::interactions::dispatcher) moderator: &'a [u64],
    pub(in crate::discord::interactions::dispatcher) gm: &'a [u64],
    pub(in crate::discord::interactions::dispatcher) legacy_staff: &'a [u64],
    pub(in crate::discord::interactions::dispatcher) admin: &'a [u64],
    pub(in crate::discord::interactions::dispatcher) owner: &'a [u64],
}

pub(in crate::discord::interactions::dispatcher) fn has_configured_role(
    member_role_ids: &[u64],
    minimum_role: StaffRole,
    configured_roles: ConfiguredRoles<'_>,
) -> bool {
    let mut allowed_role_ids = Vec::new();

    if minimum_role <= StaffRole::Helper {
        allowed_role_ids.extend(configured_roles.helper.iter().copied());
        allowed_role_ids.extend(configured_roles.legacy_staff.iter().copied());
    }
    if minimum_role <= StaffRole::Moderator {
        allowed_role_ids.extend(configured_roles.moderator.iter().copied());
    }
    if minimum_role <= StaffRole::Gm {
        allowed_role_ids.extend(configured_roles.gm.iter().copied());
    }
    if minimum_role <= StaffRole::Admin {
        allowed_role_ids.extend(configured_roles.admin.iter().copied());
    }
    allowed_role_ids.extend(configured_roles.owner.iter().copied());

    !allowed_role_ids.is_empty()
        && member_role_ids
            .iter()
            .any(|role_id| allowed_role_ids.contains(role_id))
}
