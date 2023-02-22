use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum zilchInstruction {
    Addzilchprogram {
        program_hash: String,
        inputs: u8,
        program_code: String,
    },
}

#[derive(BorshDeserialize)]
struct zilchProgramPayload {
    program_hash: String,
    inputs: u8,
    program_code: String,
}

impl zilchInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = zilchProgramPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::Addzilchprogram {
                program_hash: payload.program_hash,
                inputs: payload.inputs,
                program_code: payload.program_code,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
