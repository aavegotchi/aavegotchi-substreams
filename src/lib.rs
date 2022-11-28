mod abi;
mod pb;
use hex_literal::hex;
use pb::aavegotchi;
use pb::erc721;
use substreams::prelude::*;
use substreams::store::StoreSetProto;
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};

// Bored Ape Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("86935f11c86623dec8a25696e1c19a8659cbf95d");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<erc721::Transfers, substreams::errors::Error> {
    Ok(erc721::Transfers {
        transfers: blk
            .events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT])
            .map(|(transfer, log)| {
                substreams::log::info!("NFT Transfer seen");

                erc721::Transfer {
                    trx_hash: log.receipt.transaction.hash.clone(),
                    from: transfer.from,
                    to: transfer.to,
                    token_id: transfer.token_id.low_u64(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
#[substreams::handlers::store]
fn store_transfers(transfers: erc721::Transfers, s: StoreAddInt64) {
    log::info!("NFT holders state builder");
    for transfer in transfers.transfers {
        if transfer.from != NULL_ADDRESS {
            log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != NULL_ADDRESS {
            log::info!("Found a transfer in {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}

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
fn store_portals(
    xingyuns: aavegotchi::Xingyuns,
    open_portal_events: aavegotchi::OpenPortals,
    store: StoreSetProto<aavegotchi::Portal>,
) {
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

    for open_portal in open_portal_events.portals {
        store.set(
            open_portal.ordinal,
            open_portal.token_id.to_string(),
            &aavegotchi::Portal {
                token_id: open_portal.token_id,
                owner: open_portal.from,
                opened: true,
            },
        )
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
