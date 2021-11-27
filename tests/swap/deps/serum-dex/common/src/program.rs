use safecoin_sdk::account_info::AccountInfo;
use safecoin_sdk::program_error::ProgramError;
use safe_token::instruction as token_instruction;

pub fn invoke_token_transfer<'a, 'b>(
    from_acc_info: &'a AccountInfo<'b>,
    to_acc_info: &'a AccountInfo<'b>,
    authority_acc_info: &'a AccountInfo<'b>,
    tok_program_acc_info: &'a AccountInfo<'b>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = token_instruction::transfer(
        &safe_token::ID,
        from_acc_info.key,
        to_acc_info.key,
        authority_acc_info.key,
        &[],
        amount,
    )?;
    safecoin_sdk::program::invoke_signed(
        &ix,
        &[
            from_acc_info.clone(),
            to_acc_info.clone(),
            authority_acc_info.clone(),
            tok_program_acc_info.clone(),
        ],
        signer_seeds,
    )
}

pub fn invoke_mint_tokens<'a, 'b>(
    mint: &'a AccountInfo<'b>,
    to: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
    tok_program: &'a AccountInfo<'b>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<(), ProgramError> {
    let ix =
        token_instruction::mint_to(&safe_token::ID, mint.key, to.key, authority.key, &[], amount)?;
    safecoin_sdk::program::invoke_signed(
        &ix,
        &[
            mint.clone(),
            to.clone(),
            authority.clone(),
            tok_program.clone(),
        ],
        signer_seeds,
    )
}

pub fn invoke_burn_tokens<'a, 'b>(
    token: &'a AccountInfo<'b>,
    mint: &'a AccountInfo<'b>,
    authority: &'a AccountInfo<'b>,
    tok_program: &'a AccountInfo<'b>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = token_instruction::burn(
        &safe_token::ID,
        token.key,
        mint.key,
        authority.key,
        &[],
        amount,
    )?;
    safecoin_sdk::program::invoke_signed(
        &ix,
        &[
            token.clone(),
            mint.clone(),
            authority.clone(),
            tok_program.clone(),
        ],
        signer_seeds,
    )
}
