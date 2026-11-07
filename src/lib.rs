mod pb;

use std::collections::HashMap;

use pb::{
    mydata::v1 as mydata,
    sf::substreams::solana::v1::Transactions,
};

const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";

const TOKEN_PROGRAM: &str =
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

const TOKEN_2022_PROGRAM: &str =
    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

fn account_address(keys: &[Vec<u8>], index: usize) -> Option<String> {
    keys.get(index)
        .map(|key| bs58::encode(key).into_string())
}

fn add_instruction_transfer(
    program_id_index: u32,
    accounts: &[u8],
    data: &[u8],
    keys: &[Vec<u8>],
    token_owners: &HashMap<usize, String>,
    transfers: &mut Vec<mydata::Transfer>,
) {
    let Some(program_id) = account_address(keys, program_id_index as usize) else {
        return;
    };

    // Native SOL transfer:
    // System Program instruction:
    // 4-byte discriminator (2) + 8-byte lamports amount.
    if program_id == SYSTEM_PROGRAM {
        if data.len() != 12 || accounts.len() < 2 {
            return;
        }

        let discriminator =
            u32::from_le_bytes(data[0..4].try_into().unwrap());

        if discriminator != 2 {
            return;
        }

        let amount =
            u64::from_le_bytes(data[4..12].try_into().unwrap());

        let Some(from) = account_address(keys, accounts[0] as usize) else {
            return;
        };

        let Some(to) = account_address(keys, accounts[1] as usize) else {
            return;
        };

        transfers.push(mydata::Transfer {
            from,
            to,
            amount,
        });

        return;
    }

    // SPL Token and Token-2022 transfers.
    if program_id != TOKEN_PROGRAM && program_id != TOKEN_2022_PROGRAM {
        return;
    }

    if data.is_empty() {
        return;
    }

    let instruction_type = data[0];

    let (source_index, destination_index, amount) = match instruction_type {
        // Transfer: source, destination, authority
        // Data: discriminator (3) + amount (u64)
        3 if data.len() >= 9 && accounts.len() >= 2 => (
            accounts[0] as usize,
            accounts[1] as usize,
            u64::from_le_bytes(data[1..9].try_into().unwrap()),
        ),

        // TransferChecked: source, mint, destination, authority
        // Data: discriminator (12) + amount (u64) + decimals (u8)
        12 if data.len() >= 10 && accounts.len() >= 3 => (
            accounts[0] as usize,
            accounts[2] as usize,
            u64::from_le_bytes(data[1..9].try_into().unwrap()),
        ),

        _ => return,
    };

    // Token instruction accounts are token accounts.
    // Resolve their actual wallet owners from transaction metadata.
    let Some(from) = token_owners.get(&source_index).cloned() else {
        return;
    };

    let Some(to) = token_owners.get(&destination_index).cloned() else {
        return;
    };

    transfers.push(mydata::Transfer {
        from,
        to,
        amount,
    });
}

#[substreams::handlers::map]
fn map_my_data(transactions: Transactions) -> mydata::MyData {
    let mut output = mydata::MyData::default();

    for confirmed in transactions.transactions {
        // Ignore transactions without transaction or metadata.
        let Some(transaction) = confirmed.transaction.as_option() else {
            continue;
        };

        let Some(message) = transaction.message.as_option() else {
            continue;
        };

        let Some(meta) = confirmed.meta.as_option() else {
            continue;
        };

        // Ignore failed transactions.
        if meta.err.is_set() {
            continue;
        }

        // Resolve account keys, including addresses loaded through
        // versioned transaction address lookup tables.
        let mut account_keys = message.account_keys.clone();

        account_keys.extend(meta.loaded_writable_addresses.iter().cloned());
        account_keys.extend(meta.loaded_readonly_addresses.iter().cloned());

        // Build token-account-index -> wallet-owner mapping.
        let mut token_owners: HashMap<usize, String> = HashMap::new();

        for balance in meta
            .pre_token_balances
            .iter()
            .chain(meta.post_token_balances.iter())
        {
            if !balance.owner.is_empty() {
                token_owners.insert(
                    balance.account_index as usize,
                    balance.owner.clone(),
                );
            }
        }

        // Process top-level instructions.
        for instruction in &message.instructions {
            add_instruction_transfer(
                instruction.program_id_index,
                &instruction.accounts,
                &instruction.data,
                &account_keys,
                &token_owners,
                &mut output.transfers,
            );
        }

        // Process inner instructions (CPI transfers).
        for inner_group in &meta.inner_instructions {
            for instruction in &inner_group.instructions {
                add_instruction_transfer(
                    instruction.program_id_index,
                    &instruction.accounts,
                    &instruction.data,
                    &account_keys,
                    &token_owners,
                    &mut output.transfers,
                );
            }
        }
    }

    output
}