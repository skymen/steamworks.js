use napi_derive::napi;

#[napi]
pub mod friends {
    use napi::bindgen_prelude::BigInt;
    use steamworks::SteamId;

    #[napi]
    pub fn get_friend_name(steam_id64: BigInt) -> String {
        let client = crate::client::get_client();
        client
            .friends()
            .get_friend(SteamId::from_raw(steam_id64.get_u64().1))
            .name()
    }
}
