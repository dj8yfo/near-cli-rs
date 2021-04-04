use async_recursion::async_recursion;
use dialoguer::Input;
use clap::Clap;

use super::super::receiver::{CliSkipNextAction, CliNextAction, NextAction};

#[derive(Debug)]
pub struct DeleteAccessKeyAction {
    pub public_key: near_crypto::PublicKey,
    pub next_action: Box<NextAction>,
}

#[derive(Debug, Default, Clap)]
pub struct CliDeleteAccessKeyAction {
    #[clap(long)]
    public_key: Option<near_crypto::PublicKey>,
    #[clap(subcommand)]
    next_action: Option<CliSkipNextAction>,
}

impl From<CliDeleteAccessKeyAction> for DeleteAccessKeyAction {
    fn from(item: CliDeleteAccessKeyAction) -> Self {
        let public_key: near_crypto::PublicKey = match item.public_key {
            Some(cli_public_key) => cli_public_key,
            None => DeleteAccessKeyAction::input_public_key(),
        };
        let cli_skip_next_action: CliNextAction = match item.next_action {
            Some(cli_skip_action) => CliNextAction::from(cli_skip_action),
            None => NextAction::input_next_action(),
        };
        DeleteAccessKeyAction {
            public_key,
            next_action: Box::new(NextAction::from(cli_skip_next_action)),
        }
    }
}

impl DeleteAccessKeyAction {
    #[async_recursion(?Send)]
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: Option<url::Url>,
    ) -> crate::CliResult {
        let action = near_primitives::transaction::Action::DeleteKey(
            near_primitives::transaction::DeleteKeyAction { public_key: self.public_key },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        match *self.next_action {
            NextAction::AddAction(select_action) => {
                select_action
                    .process(unsigned_transaction, selected_server_url)
                    .await
            }
            NextAction::Skip(skip_action) => {
                skip_action
                    .process(unsigned_transaction, selected_server_url)
                    .await
            }
        }
    }
    pub fn input_public_key() -> near_crypto::PublicKey {
        Input::new()
            .with_prompt("Enter the access key to remove it")
            .interact_text()
            .unwrap()
    }
}
