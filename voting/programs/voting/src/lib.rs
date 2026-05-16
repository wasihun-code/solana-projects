use anchor_lang::prelude::*;

declare_id!("AGvUE1HbzuDutZRQQS3QWJ1PQQ36wZMyqBCzE9G9WDW5");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>, 
        _poll_id: u64,
        name: String, 
        description: String, 
        start_date: i64, 
        end_date: i64, 
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.name = name;
        poll.description = description;
        poll.start_date = start_date;
        poll.end_date = end_date;

        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>, _poll_id: u64, name: String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account; 
        candidate.name = name;
        candidate.votes = 0;
        ctx.accounts.poll.option_index += 1; 
        Ok(())
    }

    pub fn vote_candidate(ctx: Context<VoteCandidate>, _poll_id: u64, _name: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let candidate = &mut ctx.accounts.candidate;

        let current_time = Clock::get()?.unix_timestamp;

        if current_time > poll.end_date as i64 {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time < (poll.start_date as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate.votes += 1;

        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct Poll {
    #[max_len(32)]
    pub name: String,

    #[max_len(32)]
    pub description: String,

    pub start_date: i64,
    pub end_date: i64,

    pub option_index: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub name: String,

    pub votes: u64,
} #[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), name.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, Candidate>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, name: String)]
pub struct VoteCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), name.as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has ended")]
    VotingEnded,

    #[msg("Voting hasnt started yet")]
    VotingNotStarted,
}
