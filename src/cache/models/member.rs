use std::sync::Arc;
use twilight::model::{
    id::RoleId,
    guild::Member,
    user::User
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CachedMember {
    pub roles: Vec<RoleId>,
    pub nick: Option<String>,
    pub user: Arc<User>,
}

impl PartialEq<Member> for CachedMember {
    fn eq(&self, other: &Member) -> bool {
        (
            &self.roles,
            &self.nick
        ) == (
            &other.roles,
            &other.nick
        )
    }
}