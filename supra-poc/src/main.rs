use aptos_language_e2e_tests::{
    account::Account,
    common_transactions::create_account_txn,
    execution_strategies::{basic_strategy::BasicExecutor, types::Executor},
};
use aptos_state_view::{in_memory_state_view::InMemoryStateView, TStateView};
use aptos_types::{transaction::SignedTransaction, vm_status::VMStatus};
use aptos_vm::{AptosVM, VMExecutor};

fn txn(seq_num: u64) -> SignedTransaction {
    let account = Account::new();
    let aptos_root = Account::new_aptos_root();
    create_account_txn(&aptos_root, &account, seq_num)
}

fn main() {
    // See `aptos-logger` crate for more on subscriber configuration.
    aptos_logger::Logger::init_for_testing();

    println!("Aptos executor test!");
    let big_block = (0..10).map(txn).collect();
    let mut exec = BasicExecutor::new();
    AptosVM::set_concurrency_level_once(6);
    println!("{:?}", exec.executor.data_store().get_usage().unwrap());

    exec.execute_block(big_block).unwrap();
    println!("{:?}", exec.executor.data_store().get_usage().unwrap());
}
