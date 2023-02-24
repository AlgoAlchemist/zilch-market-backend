use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum zilchInstruction {
    Addzilchprogram {
        program_hash: String,
        outputs: u8,
        proof_account: String,
    },
}

#[derive(BorshDeserialize)]
struct zilchProgramPayload {
    program_hash: String,
    outputs: u8,
    proof_account: String,
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
                outputs: payload.outputs,
                proof_account: payload.proof_account,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
