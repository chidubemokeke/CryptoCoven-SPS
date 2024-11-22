mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const CRYPTOCOVENV1_TRACKED_CONTRACT: [u8; 20] = hex!("5180db8f5c931aae63c74266b211f580155ecac8");

fn map_cryptocovenv1_events(blk: &eth::Block, events: &mut contract::Events) {
    events.cryptocovenv1_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CRYPTOCOVENV1_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::cryptocovenv1_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::Cryptocovenv1Approval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            owner: event.owner,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.cryptocovenv1_approval_for_alls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CRYPTOCOVENV1_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::cryptocovenv1_contract::events::ApprovalForAll::match_and_decode(log) {
                        return Some(contract::Cryptocovenv1ApprovalForAll {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            operator: event.operator,
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.cryptocovenv1_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CRYPTOCOVENV1_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::cryptocovenv1_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::Cryptocovenv1OwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.cryptocovenv1_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CRYPTOCOVENV1_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::cryptocovenv1_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::Cryptocovenv1Transfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_cryptocovenv1_events(&blk, &mut events);
    Ok(events)
}

