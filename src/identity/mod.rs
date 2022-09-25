use crate::mosquitto::defs::{
    PLUGIN_AUDIENCE_KEY, PLUGIN_CLIENT_ID_KEY, PLUGIN_GRANT_TYPE_KEY, PLUGIN_REALM_KEY,
    PLUGIN_SCOPE_KEY, PLUGIN_SERVER_OAUTH_PATH_KEY,
};

use super::mosquitto::defs::PLUGIN_SERVER_ADDRESS_KEY;
use std::collections::HashMap;

pub(crate) struct Configs<'c> {
    server_address: &'c str,
    server_oauth_path: &'c str,
    client_id: &'c str,
    realm: &'c str,
    grant_type: &'c str,
    scope: &'c str,
    audience: &'c str,
}

pub(crate) struct IdentityPlugin<'i> {
    cfg: Option<Configs<'i>>,
}

impl<'i> IdentityPlugin<'i> {
    pub fn new() -> IdentityPlugin<'i> {
        IdentityPlugin { cfg: None }
    }

    pub fn configs(&mut self, opts: HashMap<&str, &'i str>) -> Result<(), &str> {
        let server_address = opts.get(PLUGIN_SERVER_ADDRESS_KEY);
        if server_address.is_none() {
            return Err("missing server_address config");
        }
        let &server_address = server_address.unwrap();

        let server_oauth_path = opts.get(PLUGIN_SERVER_OAUTH_PATH_KEY);
        if server_oauth_path.is_none() {
            return Err("missing server_oauth_path config");
        }
        let &server_oauth_path = server_oauth_path.unwrap();

        let client_id = opts.get(PLUGIN_CLIENT_ID_KEY);
        if client_id.is_none() {
            return Err("missing client_id config");
        }
        let &client_id = client_id.unwrap();

        let realm = opts.get(PLUGIN_REALM_KEY);
        if realm.is_none() {
            return Err("missing relam config");
        }
        let &realm = realm.unwrap();

        let grant_type = opts.get(PLUGIN_GRANT_TYPE_KEY);
        if grant_type.is_none() {
            return Err("missing grant_type config");
        }
        let &grant_type = grant_type.unwrap();

        let scope = opts.get(PLUGIN_SCOPE_KEY);
        if scope.is_none() {
            return Err("missing scope config");
        }
        let &scope = scope.unwrap();

        let audience = opts.get(PLUGIN_AUDIENCE_KEY);
        if audience.is_none() {
            return Err("missing audience config");
        }
        let &audience = audience.unwrap();

        self.cfg = Some(Configs {
            server_address,
            server_oauth_path,
            client_id,
            realm,
            grant_type,
            scope,
            audience,
        });

        Ok(())
    }
}
