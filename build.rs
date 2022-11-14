use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ERC721", "abi/erc721.json")?
        .generate()?
        .write_to_file("src/abi/erc721.rs")?;

    Abigen::new("CoreDiamond", "abi/core-diamond.json")?
        .generate()?
        .write_to_file("src/abi/core-diamond.rs")?;

    Abigen::new("RealmDiamond", "abi/realm-diamond.json")?
        .generate()?
        .write_to_file("src/abi/realm-diamond.rs")?;

    Abigen::new("InstallationDiamond", "abi/installation-diamond.json")?
        .generate()?
        .write_to_file("src/abi/installation-diamond.rs")?;

    Abigen::new("TileDiamond", "abi/tile-diamond.json")?
        .generate()?
        .write_to_file("src/abi/tile-diamond.rs")?;

    Ok(())
}
