use cosmwasm_std::{DepsMut, MessageInfo, Response};
use cw721_base::{state::TokenInfo};

use crate::{msg::TokenData, Cw2981Contract, error::ContractError};

pub fn batch_mint(
    deps: DepsMut,
    info: MessageInfo,
    tokens: Vec<TokenData>
) -> Result<Response, ContractError> {
    let contract = Cw2981Contract::default();
    // cw_ownable::assert_owner(deps.storage, &info.sender)?;

    for t in tokens.iter().cloned() {
        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&t.owner)?,
            approvals: vec![],
            token_uri: t.token_uri,
            extension: Some(t.extension),
        };
        contract.tokens
            .update(deps.storage, &t.token_id, |old| match old {
                Some(_) => Err(ContractError::Base(cw721_base::ContractError::Claimed {  })),
                None => Ok(token),
            })?;

        contract.increment_tokens(deps.storage)?;
    }

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("minted", tokens.len().to_string())
    )
}
