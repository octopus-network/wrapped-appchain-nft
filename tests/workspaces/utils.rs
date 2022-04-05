use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::TokenId;
use near_primitives::views::FinalExecutionStatus;
use near_units::parse_near;
use workspaces::prelude::DevAccountDeployer;
use workspaces::{Account, Contract, DevNetwork, Worker};

pub const TOKEN_ID: &str = "0";
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

pub async fn helper_mint(
    nft_contract: &Contract,
    worker: &Worker<impl DevNetwork>,
    token_id: TokenId,
    title: String,
    desc: String,
) -> anyhow::Result<()> {
    let token_metadata = TokenMetadata {
        title: Some(title),
        description: Some(desc),
        media: None,
        media_hash: None,
        copies: Some(1u64),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    };
    let res = nft_contract
        .call(&worker, "nft_mint")
        .args_json((token_id, nft_contract.id(), token_metadata))?
        .gas(300_000_000_000_000)
        .deposit(parse_near!("7 mN"))
        .transact()
        .await?;
    assert!(matches!(res.status, FinalExecutionStatus::SuccessValue(_)));

    Ok(())
}

/// Deploy and initialize contracts and return:
/// * nft_contract: the NFT contract, callable with `call!` and `view!`
/// * alice: a user account, does not yet own any tokens
/// * token_receiver_contract: a contract implementing `nft_on_transfer` for use with `transfer_and_call`
/// * approval_receiver_contract: a contract implementing `nft_on_approve` for use with `nft_approve`
pub async fn init(
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<(Contract, Account, Contract, Contract)> {
    let nft_contract = worker
        .dev_deploy(include_bytes!("../../res/wrapped_appchain_nft.wasm").to_vec())
        .await?;

    let res = nft_contract
        .call(&worker, "new")
        .args_json((
            nft_contract.id(),
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        ))?
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    assert!(matches!(res.status, FinalExecutionStatus::SuccessValue(_)));

    let token_metadata = TokenMetadata {
        title: Some("Olympus Mons".into()),
        description: Some("The tallest mountain in the charted solar system".into()),
        media: None,
        media_hash: None,
        copies: Some(1u64),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    };
    let res = nft_contract
        .call(&worker, "nft_mint")
        .args_json((TOKEN_ID, nft_contract.id(), token_metadata))?
        .gas(300_000_000_000_000)
        .deposit(parse_near!("7 mN"))
        .transact()
        .await?;
    assert!(matches!(res.status, FinalExecutionStatus::SuccessValue(_)));

    let res = nft_contract
        .as_account()
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?;
    assert!(matches!(
        res.details.status,
        FinalExecutionStatus::SuccessValue(_)
    ));
    let alice = res.result;

    let token_receiver_contract = worker
        .dev_deploy(include_bytes!("../../res/token_receiver.wasm").to_vec())
        .await?;
    let res = token_receiver_contract
        .call(&worker, "new")
        .args_json((nft_contract.id(),))?
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    assert!(matches!(res.status, FinalExecutionStatus::SuccessValue(_)));

    let approval_receiver_contract = worker
        .dev_deploy(include_bytes!("../../res/approval_receiver.wasm").to_vec())
        .await?;
    let res = approval_receiver_contract
        .call(&worker, "new")
        .args_json((nft_contract.id(),))?
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    assert!(matches!(res.status, FinalExecutionStatus::SuccessValue(_)));

    return Ok((
        nft_contract,
        alice,
        token_receiver_contract,
        approval_receiver_contract,
    ));
}
