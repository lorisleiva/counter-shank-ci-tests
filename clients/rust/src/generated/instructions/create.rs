//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Create {
    /// The program derived address of the counter account to create (seeds: ['counter', authority])
    pub counter: solana_program::pubkey::Pubkey,
    /// The authority of the counter
    pub authority: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees
    pub payer: solana_program::pubkey::Pubkey,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl Create {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.counter,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = CreateInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::COUNTER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateInstructionData {
    discriminator: u8,
}

impl CreateInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 0 }
    }
}

impl Default for CreateInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `Create`.
///
/// ### Accounts:
///
///   0. `[writable]` counter
///   1. `[signer]` authority
///   2. `[writable, signer]` payer
///   3. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct CreateBuilder {
    counter: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The program derived address of the counter account to create (seeds: ['counter', authority])
    #[inline(always)]
    pub fn counter(&mut self, counter: solana_program::pubkey::Pubkey) -> &mut Self {
        self.counter = Some(counter);
        self
    }
    /// The authority of the counter
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Create {
            counter: self.counter.expect("counter is not set"),
            authority: self.authority.expect("authority is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `create` CPI accounts.
pub struct CreateCpiAccounts<'a, 'b> {
    /// The program derived address of the counter account to create (seeds: ['counter', authority])
    pub counter: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the counter
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create` CPI instruction.
pub struct CreateCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The program derived address of the counter account to create (seeds: ['counter', authority])
    pub counter: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the counter
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> CreateCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            counter: accounts.counter,
            authority: accounts.authority,
            payer: accounts.payer,
            system_program: accounts.system_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.counter.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = CreateInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::COUNTER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.counter.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Create` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` counter
///   1. `[signer]` authority
///   2. `[writable, signer]` payer
///   3. `[]` system_program
#[derive(Clone, Debug)]
pub struct CreateCpiBuilder<'a, 'b> {
    instruction: Box<CreateCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateCpiBuilderInstruction {
            __program: program,
            counter: None,
            authority: None,
            payer: None,
            system_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The program derived address of the counter account to create (seeds: ['counter', authority])
    #[inline(always)]
    pub fn counter(
        &mut self,
        counter: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.counter = Some(counter);
        self
    }
    /// The authority of the counter
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = CreateCpi {
            __program: self.instruction.__program,

            counter: self.instruction.counter.expect("counter is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    counter: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
