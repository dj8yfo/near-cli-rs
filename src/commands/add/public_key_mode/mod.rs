use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod add_access_key;

#[derive(Debug, clap::Clap)]
pub enum CliPublicKeyMode {
    /// Предоставьте public key
    PublicKey(self::add_access_key::CliAddAccessKeyAction),
    /// Сгенерируйте ключи
    GenerateKeypair,
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum PublicKeyMode {
    #[strum_discriminants(strum(message = "Enter public key"))]
    PublicKey(self::add_access_key::AddAccessKeyAction),
    #[strum_discriminants(strum(message = "Generate key pair"))]
    GenerateKeypair,
}

impl From<CliPublicKeyMode> for PublicKeyMode {
    fn from(item: CliPublicKeyMode) -> Self {
        match item {
            CliPublicKeyMode::PublicKey(cli_add_access_key_action) => {
                PublicKeyMode::PublicKey(cli_add_access_key_action.into())
            }
            CliPublicKeyMode::GenerateKeypair => PublicKeyMode::GenerateKeypair
        }
    }
}

impl PublicKeyMode {
    pub fn choose_public_key_mode() -> Self {
        let variants = PublicKeyModeDiscriminants::iter().collect::<Vec<_>>();
        let modes = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a permission that you want to add to the access key:")
            .items(&modes)
            .default(0)
            .interact()
            .unwrap();
        match variants[select_mode] {
            PublicKeyModeDiscriminants::PublicKey => Self::from(CliPublicKeyMode::PublicKey(Default::default())),
            PublicKeyModeDiscriminants::GenerateKeypair => Self::from(CliPublicKeyMode::GenerateKeypair)//(Default::default()))
        }
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: Option<url::Url>,
    ) -> crate::CliResult {
        match self {
            PublicKeyMode::PublicKey(add_access_key_action) => {
                add_access_key_action
                    .process(prepopulated_unsigned_transaction, selected_server_url)
                    .await
            }
            PublicKeyMode::GenerateKeypair => {
                println!("This module is currently being developed.");
                Ok(())
            }
        }
    }
}
