use solana_program::{
  account_info::AccountInfo,
  entrypoint,
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  msg,
};

// solana-program-sdkの `entrypoint` を呼び出し。
entrypoint!(process_instruction);

fn process_instruction(
  _program_id: &Pubkey,
  _accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  msg!("Hello World!");
  Ok(())
}
