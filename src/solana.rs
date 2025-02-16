pub fn exec() {}

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// BorshSerialize、BorshDeserialize 这2个派生宏是为了实现（反）序列化操作
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u32,
}

// 定义智能程序的入口点函数
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,    // 智能程序的公钥
    accounts: &[AccountInfo],   // 一个包含所有相关账户信息的数组
    instruction_data: &[u8],    // 包含指令数据的字节数组
) -> ProgramResult {
    // 账户迭代器
    let accounts_iter = &mut accounts.iter();

    // 获取调用者账户
    let account = next_account_info(accounts_iter)?;

    // 验证调用者身份
    // The account must be owned by the program in order to modify its data
    // 检查账户的 owner 是否与 program_id 匹配，如果不匹配则返回错误
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Debug output:");
    msg!("Account ID: {}", account.key);
    msg!("Executable?: {}", account.executable);
    msg!("Lamports: {:#?}", account.lamports);
    msg!("Debug output complete.");

    msg!("Adding 1 to sum...");

    // 读取 account 中的数据
    // 从 account.data.borrow() 返回的不可变引用中反序列化 CounterAccount 结构体。
    let mut counter = CounterAccount::try_from_slice(&account.data.borrow())?;
    counter.count += 1;
    // 然后将 counter 序列化并写入到 account.data 中
    counter.serialize(&mut *account.data.borrow_mut())?;

    msg!("Current sum is now: {}", counter.count);

    Ok(())
}
