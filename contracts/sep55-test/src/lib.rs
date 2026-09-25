#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct Sep55Test;

#[contractimpl]
impl Sep55Test {
    pub fn ping(_env: Env) -> Symbol {
        symbol_short!("pong")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn ping_returns_pong() {
        let env = Env::default();
        let id = env.register(Sep55Test, ());
        let client = Sep55TestClient::new(&env, &id);

        assert_eq!(client.ping(), symbol_short!("pong"));
    }
}
