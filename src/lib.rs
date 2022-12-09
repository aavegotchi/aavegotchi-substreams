mod abi;
mod pb;

use pb::aavegotchi;
use substreams::errors::Error;
use substreams::hex;
use substreams::prelude::*;
use substreams::store;
use substreams::store::StoreGetProto;
use substreams::store::StoreSetProto;

use substreams::{log, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::Block;

// Aavegotchi Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("86935f11c86623dec8a25696e1c19a8659cbf95d");

substreams_ethereum::init!();

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
                    token_id: xingyun.token_id.to_u64(),
                    num_aavegotchis_to_purchase: xingyun.num_aavegotchis_to_purchase.to_u64(),
                    total_price: xingyun.total_price.to_u64(),
                    trx_hash: log.receipt.transaction.hash.clone(),
                    ordinal: log.block_index() as u64,
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
                    claimed: false,
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
                    token_id: portal_opened.token_id.to_u64(),
                    trx_hash: log.receipt.transaction.hash.clone(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

/// Store the open portals
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

// extract claim aavegotchi events from the contract
#[substreams::handlers::map]
fn map_claim_aavegotchi(
    blk: eth::Block,
) -> Result<aavegotchi::ClaimAavegotchis, substreams::errors::Error> {
    Ok(aavegotchi::ClaimAavegotchis {
        aavegotchis_claimed: blk
            .events::<abi::core_diamond::events::ClaimAavegotchi>(&[&TRACKED_CONTRACT])
            .map(|(claim_aavegotchi, log)| -> aavegotchi::ClaimAavegotchi {
                substreams::log::info!("Portal Open seen");
                aavegotchi::ClaimAavegotchi {
                    from: log.receipt.transaction.from.clone(),
                    token_id: claim_aavegotchi.token_id.to_u64(),
                    trx_hash: log.receipt.transaction.hash.clone(),
                    ordinal: log.block_index() as u64,
                    created_at_block_number: blk.number,
                    created_at_timestamp: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

/// Store the claimed aavegotchis
#[substreams::handlers::store]
fn store_aavegotchis(
    claimed_gotchis: aavegotchi::ClaimAavegotchis,
    open_portals: StoreGetProto<aavegotchi::Portal>,
    store: StoreSetProto<aavegotchi::Gotchi>,
) {
    let mut count = 0;
    for claimed_gotchi in claimed_gotchis.aavegotchis_claimed {
        if let Some(portal) = open_portals.get_last(claimed_gotchi.token_id.to_string()) {
            let new_gotchi = aavegotchi::Gotchi {
                created_at_block_number: claimed_gotchi.created_at_block_number,
                created_at_timestamp: claimed_gotchi.created_at_timestamp,
                status: 1,
                token_id: claimed_gotchi.token_id,
                owner: portal.owner,
            };

            store.set(count, (new_gotchi.token_id).to_string(), &new_gotchi);
            count += 1;
        } else {
            log::debug!(
                "portal {} does not exist",
                claimed_gotchi.token_id.to_string()
            );
            continue;
        }
    }
}

/// Upate the claimed portals
#[substreams::handlers::store]
fn store_claimed_portal(
    claimed_gotchis: aavegotchi::ClaimAavegotchis,
    open_portals: StoreGetProto<aavegotchi::Portal>,
    store: StoreSetProto<aavegotchi::Portal>,
) {
    let mut count = 0;
    for claimed_gotchi in claimed_gotchis.aavegotchis_claimed {
        if let Some(portal) = open_portals.get_last(claimed_gotchi.token_id.to_string()) {
            let mut new_portal = portal.clone();
            new_portal.claimed = true;

            store.set(count, (new_portal.token_id).to_string(), &new_portal);
            count += 1;
        } else {
            log::debug!(
                "portal {} does not exist",
                claimed_gotchi.token_id.to_string()
            );
            continue;
        }
    }
}

#[substreams::handlers::map]
pub fn map_portal_entities(
    block: Block,
    pool_count_deltas: store::Deltas<DeltaBigInt>,
    tx_count_deltas: store::Deltas<DeltaBigInt>,
    swaps_volume_deltas: store::Deltas<DeltaBigDecimal>,
    totals_deltas: store::Deltas<DeltaBigDecimal>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    // FIXME: Hard-coded start block, how could we pull that from the manifest?
    // if block.number == 12369621 {
    //     db::factory_created_factory_entity_change(&mut entity_changes)
    // }

    // db::pool_created_factory_entity_change(&mut entity_changes, pool_count_deltas);
    // db::tx_count_factory_entity_change(&mut entity_changes, tx_count_deltas);
    // db::swap_volume_factory_entity_change(&mut entity_changes, swaps_volume_deltas);
    // db::total_value_locked_factory_entity_change(&mut entity_changes, totals_deltas);

    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_aavegotchi_entities(
    block: Block,
    pool_count_deltas: store::Deltas<DeltaBigInt>,
    tx_count_deltas: store::Deltas<DeltaBigInt>,
    swaps_volume_deltas: store::Deltas<DeltaBigDecimal>,
    totals_deltas: store::Deltas<DeltaBigDecimal>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    // FIXME: Hard-coded start block, how could we pull that from the manifest?
    // if block.number == 12369621 {
    //     db::factory_created_factory_entity_change(&mut entity_changes)
    // }

    // db::pool_created_factory_entity_change(&mut entity_changes, pool_count_deltas);
    // db::tx_count_factory_entity_change(&mut entity_changes, tx_count_deltas);
    // db::swap_volume_factory_entity_change(&mut entity_changes, swaps_volume_deltas);
    // db::total_value_locked_factory_entity_change(&mut entity_changes, totals_deltas);

    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn graph_out(
    map_portal_entities: EntityChanges,
    map_aavegotchi_entities: EntityChanges,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    // FIXME: Hard-coded start block, how could we pull that from the manifest?
    // if block.number == 12369621 {
    //     db::factory_created_factory_entity_change(&mut entity_changes)
    // }

    // db::pool_created_factory_entity_change(&mut entity_changes, pool_count_deltas);
    // db::tx_count_factory_entity_change(&mut entity_changes, tx_count_deltas);
    // db::swap_volume_factory_entity_change(&mut entity_changes, swaps_volume_deltas);
    // db::total_value_locked_factory_entity_change(&mut entity_changes, totals_deltas);

    Ok(entity_changes)
}
