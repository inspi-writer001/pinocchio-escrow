// #[no_warn(unused_variables)]
use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    msg,
    pubkey::{self, log},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
// use pinocchio_log::log;
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;

use crate::state::Escrow;

//  let make_data = [
//             vec![0u8], // Discriminator for "Make" instruction
//             bump.to_le_bytes().to_vec(),
//             amount_to_receive.to_le_bytes().to_vec(),
//             amount_to_give.to_le_bytes().to_vec(),
//         ]

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct MakeData {
    pub make_amount: u64,
    pub take_amount: u64,
    // pub _padding: [u8; 7],
}

pub trait DataLen {
    const LEN: usize;
}

impl DataLen for MakeData {
    const LEN: usize = core::mem::size_of::<MakeData>();
}

// pub struct InitializeMyStateV2IxData {
//     pub owner: Pubkey,
//     pub data: [u8; 32],
// }

pub fn process_make_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    pinocchio_log::log!("Processing Make instruction");
    pinocchio_log::log!("Processing Make Ix again --");

    // let ix_data = bytemuck::from_bytes::<MakeData>(data);

    pinocchio_log::log!("did not fail at reading data with bytemuck");
    let [maker, mint_a, mint_b, escrow_account, maker_ata, escrow_ata, system_program, token_program, _associated_token_program, _rent_sysvar @ ..] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    let maker_ata_fot_tf = maker_ata.clone();
    let escrow_ata_for_tf = escrow_ata.clone();
    let maker_for_tf = maker.clone();

    pinocchio_log::log!("did not fail at accounts check --");

    let maker_ata_state = pinocchio_token::state::TokenAccount::from_account_info(&maker_ata)?;
    if maker_ata_state.owner() != maker.key() {
        return Err(pinocchio::program_error::ProgramError::IllegalOwner);
    }
    if maker_ata_state.mint() != mint_a.key() {
        return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
    }

    pinocchio_log::log!("did not fail at ata check --");

    // let bump = data[0];
    let seed = [b"escrow".as_ref(), maker.key().as_ref()];
    let (escrow_account_pda, bump) = pubkey::find_program_address(&seed, &crate::ID);
    // let seeds = &seed[..];

    // let escrow_account_pda = derive_address(&seed, None, &crate::ID);
    log(&escrow_account_pda);
    log(&escrow_account.key());
    assert_eq!(
        escrow_account_pda,
        *escrow_account.key(),
        "Error at Account Assertion {:#?} and {:#?}",
        escrow_account_pda,
        *escrow_account.key()
    );

    pinocchio_log::log!("did not fail at accounts assertion 👍️ --");

    let amount_to_receive = unsafe { *(data.as_ptr().add(1) as *const u64) };
    let amount_to_give = unsafe { *(data.as_ptr().add(9) as *const u64) };

    let bump = [bump.to_le()];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker.key()),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    if escrow_account.owner() != &crate::ID {
        CreateAccount {
            from: maker,
            to: escrow_account,
            lamports: Rent::get()?.minimum_balance(Escrow::LEN),
            space: Escrow::LEN as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&[seeds.clone()])?;
        msg!("did not fail at first invoke --");

        {
            let escrow_state = Escrow::from_account_info(escrow_account)?;

            escrow_state.set_maker(maker.key());
            escrow_state.set_mint_a(mint_a.key());
            escrow_state.set_mint_b(mint_b.key());
            escrow_state.set_amount_to_receive(amount_to_receive);
            escrow_state.set_amount_to_give(amount_to_give);
            escrow_state.bump = data[0];
        }
    } else {
        return Err(pinocchio::program_error::ProgramError::IllegalOwner);
    }

    {
        pinocchio_associated_token_account::instructions::Create {
            funding_account: maker,
            account: escrow_ata,
            wallet: escrow_account,
            mint: mint_a,
            token_program: token_program,
            system_program: system_program,
        }
        .invoke()?;
    }
    msg!("did not fail at create account invoke 🔥🔥--");

    Ok(())
}

pub fn do_transfer(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    pinocchio_log::log!("did not fail at reading data with bytemuck");
    let [maker, _mint_a, _mint_b, _escrow_account, maker_ata, escrow_ata, _system_program, _token_program, _associated_token_program, _rent_sysvar @ ..] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    msg!("Trying transfer now 👑👑--");
    let _amount_to_receive = unsafe { *(data.as_ptr().add(1) as *const u64) };
    let amount_to_give = unsafe { *(data.as_ptr().add(9) as *const u64) };

    pinocchio_log::log!("Amount to give {} ", amount_to_give);

    pinocchio_token::instructions::Transfer {
        from: maker_ata,
        to: escrow_ata,
        authority: maker,
        amount: amount_to_give,
    }
    .invoke()?;

    Ok(())
}
