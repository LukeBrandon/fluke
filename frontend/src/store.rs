use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::api::types::User;


#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct Store {
    pub auth_user: Option<User>,
    pub page_loading: bool,
}

pub fn set_page_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.page_loading = loading;
    })
}

pub fn set_auth_user(user: Option<User>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
    })
}
