//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct ThawMintAccount {
    pub user: solana_program::pubkey::Pubkey,

    pub delegate_authority: solana_program::pubkey::Pubkey,

    pub mint: solana_program::pubkey::Pubkey,

    pub mint_token_account: solana_program::pubkey::Pubkey,

    pub manager: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl ThawMintAccount {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.delegate_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mint, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.manager,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ThawMintAccountInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::WEN_NEW_STANDARD_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ThawMintAccountInstructionData {
    discriminator: [u8; 8],
}

impl ThawMintAccountInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [27, 53, 61, 16, 162, 190, 27, 72],
        }
    }
}

impl Default for ThawMintAccountInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ThawMintAccount`.
///
/// ### Accounts:
///
///   0. `[]` user
///   1. `[writable, signer]` delegate_authority
///   2. `[]` mint
///   3. `[writable]` mint_token_account
///   4. `[]` manager
///   5. `[optional]` token_program (default to `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`)
#[derive(Clone, Debug, Default)]
pub struct ThawMintAccountBuilder {
    user: Option<solana_program::pubkey::Pubkey>,
    delegate_authority: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    mint_token_account: Option<solana_program::pubkey::Pubkey>,
    manager: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ThawMintAccountBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn delegate_authority(
        &mut self,
        delegate_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.delegate_authority = Some(delegate_authority);
        self
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn mint_token_account(
        &mut self,
        mint_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.mint_token_account = Some(mint_token_account);
        self
    }
    #[inline(always)]
    pub fn manager(&mut self, manager: solana_program::pubkey::Pubkey) -> &mut Self {
        self.manager = Some(manager);
        self
    }
    /// `[optional account, default to 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
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
        let accounts = ThawMintAccount {
            user: self.user.expect("user is not set"),
            delegate_authority: self
                .delegate_authority
                .expect("delegate_authority is not set"),
            mint: self.mint.expect("mint is not set"),
            mint_token_account: self
                .mint_token_account
                .expect("mint_token_account is not set"),
            manager: self.manager.expect("manager is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `thaw_mint_account` CPI accounts.
pub struct ThawMintAccountCpiAccounts<'a, 'b> {
    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub delegate_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub manager: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `thaw_mint_account` CPI instruction.
pub struct ThawMintAccountCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub delegate_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub mint_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub manager: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ThawMintAccountCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ThawMintAccountCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            user: accounts.user,
            delegate_authority: accounts.delegate_authority,
            mint: accounts.mint,
            mint_token_account: accounts.mint_token_account,
            manager: accounts.manager,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.delegate_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.manager.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ThawMintAccountInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WEN_NEW_STANDARD_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.delegate_authority.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.mint_token_account.clone());
        account_infos.push(self.manager.clone());
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `ThawMintAccount` via CPI.
///
/// ### Accounts:
///
///   0. `[]` user
///   1. `[writable, signer]` delegate_authority
///   2. `[]` mint
///   3. `[writable]` mint_token_account
///   4. `[]` manager
///   5. `[]` token_program
#[derive(Clone, Debug)]
pub struct ThawMintAccountCpiBuilder<'a, 'b> {
    instruction: Box<ThawMintAccountCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ThawMintAccountCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ThawMintAccountCpiBuilderInstruction {
            __program: program,
            user: None,
            delegate_authority: None,
            mint: None,
            mint_token_account: None,
            manager: None,
            token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn delegate_authority(
        &mut self,
        delegate_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.delegate_authority = Some(delegate_authority);
        self
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn mint_token_account(
        &mut self,
        mint_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mint_token_account = Some(mint_token_account);
        self
    }
    #[inline(always)]
    pub fn manager(
        &mut self,
        manager: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.manager = Some(manager);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
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
        let instruction = ThawMintAccountCpi {
            __program: self.instruction.__program,

            user: self.instruction.user.expect("user is not set"),

            delegate_authority: self
                .instruction
                .delegate_authority
                .expect("delegate_authority is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            mint_token_account: self
                .instruction
                .mint_token_account
                .expect("mint_token_account is not set"),

            manager: self.instruction.manager.expect("manager is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ThawMintAccountCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    delegate_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    manager: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
