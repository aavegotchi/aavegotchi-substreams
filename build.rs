use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ERC721", "abi/erc721.json")?
        .generate()?
        .write_to_file("src/abi/erc721.rs")?;

    Abigen::new("CoreDiamond", "abi/core-diamond.json")?
        .generate()?
        .write_to_file("src/abi/core_diamond.rs")?;

    Abigen::new("RealmDiamond", "abi/realm-diamond.json")?
        .generate()?
        .write_to_file("src/abi/realm_diamond.rs")?;

    Abigen::new("InstallationDiamond", "abi/installation-diamond.json")?
        .generate()?
        .write_to_file("src/abi/installation_diamond.rs")?;

    Abigen::new("TileDiamond", "abi/tile-diamond.json")?
        .generate()?
        .write_to_file("src/abi/tile_diamond.rs")?;

    Ok(())
}
