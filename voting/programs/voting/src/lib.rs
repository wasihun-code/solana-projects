use anchor_lang::prelude::*;

declare_id!("2VXz1SuH2W5U5S851b6LHtFdS8KAzSQmkHWvy3xkb2D1");

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

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>, name: String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate; 
        candidate.name = name;
        candidate.votes = 0;
        ctx.accounts.poll.option_index += 1; 
        Ok(())
    }

    pub fn vote_candidate(ctx: Context<VoteCandidate>) -> Result<()> {
         
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

}

#[derive(Accounts)]
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
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub candidate: Account<'info, Candidate>,

    #[account(mut)]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}
