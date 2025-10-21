use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    msg,
    pubkey::{self, log},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

pub fn process_take_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [taker, maker, maker_ata_a, maker_ata_b, mint_a, mint_b, escrow_account, taker_ata_a, taker_ata_b, escrow_ata, system_program, token_program, _associated_token_program, _rent_sysvar @ ..] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    // check if the mint is correct

    // check that maker and taker mints are correct
    let taker_token_account_a =
        pinocchio_token::state::TokenAccount::from_account_info(taker_ata_a).unwrap();
    let maker_token_account_a =
        pinocchio_token::state::TokenAccount::from_account_info(maker_ata_a).unwrap();

    assert_eq!(
        taker_token_account_a.mint(),
        mint_a.key(),
        "Invalid taker_token_account_a"
    );
    assert_eq!(
        maker_token_account_a.mint(),
        mint_a.key(),
        "Invalid maker_token_account_a"
    );

    // check that maker and taker mints are correct
    let taker_token_account_b =
        pinocchio_token::state::TokenAccount::from_account_info(taker_ata_a).unwrap();
    let maker_token_account_b =
        pinocchio_token::state::TokenAccount::from_account_info(maker_ata_a).unwrap();

    assert_eq!(
        taker_token_account_b.mint(),
        mint_b.key(),
        "Invalid taker_token_account_a"
    );
    assert_eq!(
        maker_token_account_b.mint(),
        mint_b.key(),
        "Invalid maker_token_account_a"
    );
    // check that maker is the owner of maker_ata provided
    // check that maker is the creator of the make_ix [make state]

    // transfer token to maker_ata
    // transfer token to taker_ata
    // close escrow
    Ok(())
}
