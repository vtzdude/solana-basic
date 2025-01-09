use anchor_lang::prelude::*;

declare_id!("3DjU6DvkEz11tXN3fFhBA7ULEYVzowzTz3cZ1RQt84QM");

#[program]
pub mod my_program {
    use super::*;

    pub fn say_hello(_ctx:Context<SayHello>)->Result<()>{
        msg!("Hello, I am here");
        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct SayHello {}
