use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Validator {
    pub public_key: String,
    pub stake: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Workchain {
    pub id: u32,
    pub name: String,
    pub shard_prefix: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenesisConfig {
    pub balances: Vec<Balance>,
    pub validators: Vec<Validator>,
    pub workchains: Vec<Workchain>,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            balances: vec![Balance {
                address: "onx:genesis-account".to_string(),
                amount: 5_000_000_000_000_000_000,
            }],
            validators: vec![Validator {
                public_key: "validator-pubkey-00".to_string(),
                stake: 1_000_000,
            }],
            workchains: vec![Workchain {
                id: 0,
                name: "masterchain".to_string(),
                shard_prefix: "0x00".to_string(),
                enabled: true,
            }],
        }
    }
}

pub fn parse_config(path: impl AsRef<Path>) -> Result<GenesisConfig, String> {
    let raw = fs::read_to_string(path.as_ref())
        .map_err(|err| format!("failed to read genesis config {}: {err}", path.as_ref().display()))?;
    let config: GenesisConfig = toml::from_str(&raw).map_err(|err| err.to_string())?;
    Ok(config)
}

pub fn generate_genesis(config: &GenesisConfig, output_dir: PathBuf) -> Result<(), String> {
    fs::create_dir_all(&output_dir)
        .map_err(|err| format!("failed to create output dir {}: {err}", output_dir.display()))?;

    let mut balances = String::new();
    for balance in &config.balances {
        balances.push_str(&format!("{}:{};", balance.address, balance.amount));
    }

    let mut validators = String::new();
    for validator in &config.validators {
        validators.push_str(&format!("{}:{};", validator.public_key, validator.stake));
    }

    let mut workchains = String::new();
    for wc in &config.workchains {
        workchains.push_str(&format!("{}:{}:{}:{};", wc.id, wc.name, wc.shard_prefix, wc.enabled));
    }

    let gateway = format!(
        "ONX_GENESIS_BOC\nmasterchain_genesis#0\nvalidator_keys={validators}\ninitial_balances={balances}\nworkchains={workchains}\n"
    );

    let genesis_boc_path = output_dir.join("genesis.boc");
    fs::write(&genesis_boc_path, &gateway).map_err(|err| err.to_string())?;

    let shard_header = format!(
        "ONX_SHARD_HEADER_BOC\nshard=0x00\nworkchain=0\nparent=masterchain_genesis#0\n"
    );
    fs::write(output_dir.join("shard-header-0.boc"), shard_header).map_err(|err| err.to_string())?;

    for idx in 0..4 {
        let node_cfg = format!(
            "role = \"validator\"\nstorage_path = \"target/onxd-node-{}\"\nnetwork_enabled = true\nnetwork_bind = \"127.0.0.1:{}000\"\npeers = \"127.0.0.1:{}001\"\nbootstrap_genesis = \"{}\"\n",
            idx,
            idx + 1,
            idx + 1,
            genesis_boc_path.display()
        );
        fs::write(output_dir.join(format!("node-{}.toml", idx)), node_cfg).map_err(|err| err.to_string())?;
    }

    Ok(())
}

pub fn write_docs(output_dir: PathBuf) -> Result<(), String> {
    let doc = "# Launch Guide\n\nThis repository ships a deterministic `onx-genesis` bootstrap generator.\n\nUse `onx-genesis --config genesis.toml --out target/onx-genesis` to create `genesis.boc` and four `node-*.toml` files.\n\nThen launch four `onxd` boot nodes from the same generated `genesis.boc` by pointing each node at its generated configuration file, verifying that the first committed masterchain block is `#0` and that a shared shard header bootstrap path is emitted.\n";
    fs::write(output_dir.join("launch_guide.md"), doc).map_err(|err| err.to_string())?;
    Ok(())
}
