mod pb;
use pb::mydata::v1 as mydata;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";

#[substreams::handlers::map]
fn map_transfers(blk: Block) -> mydata::Transfers {
    let mut transfers: Vec<mydata::Transfer> = Vec::new();

    for instruction in blk.walk_instructions() {
        if bs58::encode(&instruction.program_id().0).into_string() != SYSTEM_PROGRAM {
            continue;
        }

        let data = instruction.data();
        if data.len() != 12 {
            continue;
        }
        let kind = u32::from_le_bytes(<[u8; 4]>::try_from(&data[0..4]).unwrap());
        if kind != 2 {
            continue; // 2 = System Program "Transfer"
        }
        let amount = u64::from_le_bytes(<[u8; 8]>::try_from(&data[4..12]).unwrap());

        let accounts = instruction.accounts();
        if accounts.len() < 2 {
            continue;
        }

        transfers.push(mydata::Transfer {
            from: bs58::encode(&accounts[0].0).into_string(),
            to: bs58::encode(&accounts[1].0).into_string(),
            amount,
        });
    }

    mydata::Transfers { transfers }
}