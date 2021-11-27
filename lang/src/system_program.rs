use crate::*;
use safecoin_program::program_error::ProgramError;
use safecoin_program::pubkey::Pubkey;

pub use safecoin_program::system_program::ID;

#[derive(Clone)]
pub struct System;

impl anchor_lang::AccountDeserialize for System {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        System::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(System)
    }
}

impl anchor_lang::Id for System {
    fn id() -> Pubkey {
        ID
    }
}
