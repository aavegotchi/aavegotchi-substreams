mod abi;
mod pb;
use std::vec;

use hex_literal::hex;
use pb::aavegotchi;
use pb::erc721;
use substreams::prelude::*;
use substreams::store::StoreGetProto;
use substreams::store::StoreSetProto;
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};

// Bored Ape Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("86935f11c86623dec8a25696e1c19a8659cbf95d");

substreams_ethereum::init!();

fn generate_key(holder: &Vec<u8>) -> String {
    return format!("total:{}:{}", Hex(holder), Hex(TRACKED_CONTRACT));
}

/// Extracts mint events from the contract
#[substreams::handlers::map]
fn map_xingyuns(blk: eth::Block) -> Result<aavegotchi::Xingyuns, substreams::errors::Error> {
    Ok(aavegotchi::Xingyuns {
        xingyuns: blk
            .events::<abi::core_diamond::events::Xingyun>(&[&TRACKED_CONTRACT])
            .map(|(xingyun, log)| {
                substreams::log::info!("Xingyun seen");

                aavegotchi::Xingyun {
                    from: xingyun.from,
                    to: xingyun.to,
                    token_id: xingyun.token_id.low_u64(),
                    ordinal: log.block_index() as u64,
                    num_aavegotchis_to_purchase: xingyun.num_aavegotchis_to_purchase.low_u64(),
                    total_price: xingyun.total_price.low_u64(),
                    trx_hash: log.receipt.transaction.hash.clone(),
                }
            })
            .collect(),
    })
}

/// Store the minted closed portals
#[substreams::handlers::store]
fn store_closed_portals(xingyuns: aavegotchi::Xingyuns, store: StoreSetProto<aavegotchi::Portal>) {
    for xingyun in xingyuns.xingyuns {
        let mut count = 0;
        let from = xingyun.from;
        log::info!("Ordinal =  {}", xingyun.ordinal.to_string());
        loop {
            store.set(
                xingyun.ordinal + count,
                (&xingyun.token_id + count).to_string(),
                &aavegotchi::Portal {
                    token_id: &xingyun.token_id + count,
                    owner: from.clone(),
                    opened: false,
                },
            );
            count += 1;
            if count == xingyun.num_aavegotchis_to_purchase {
                break;
            }
        }
        log::info!("Done  {}", Hex(xingyun.trx_hash));
        // s.add(transfer.ordinal, generate_key(&transfer.to), 1);
    }
}

// extract open portal events from the contract
#[substreams::handlers::map]
fn map_open_portals(blk: eth::Block) -> Result<aavegotchi::OpenPortals, substreams::errors::Error> {
    Ok(aavegotchi::OpenPortals {
        portals: blk
            .events::<abi::core_diamond::events::PortalOpened>(&[&TRACKED_CONTRACT])
            .map(|(portal_opened, log)| -> aavegotchi::OpenPortal {
                substreams::log::info!("Portal Open seen");
                aavegotchi::OpenPortal {
                    from: log.receipt.transaction.from.clone(),
                    token_id: portal_opened.token_id.low_u64(),
                    trx_hash: log.receipt.transaction.hash.clone(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

/// Store the minted closed portals
#[substreams::handlers::store]
fn store_open_portals(
    closed_portals: StoreGetProto<aavegotchi::Portal>,
    portals: aavegotchi::OpenPortals,
    store: StoreSetProto<aavegotchi::Portal>,
) {
    let mut count = 0;
    for portal in portals.portals {
        if let Some(portal) = closed_portals.get_last(portal.token_id.to_string()) {
            let mut new_portal = portal.clone();
            new_portal.opened = true;

            store.set(count, (portal.token_id).to_string(), &new_portal);
            count += 1;
        } else {
            log::debug!("portal {} does not exist", portal.token_id.to_string());
            continue;
        }
    }
}
