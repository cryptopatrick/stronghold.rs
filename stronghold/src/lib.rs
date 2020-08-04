//! stronghold.rs

#![warn(missing_docs, rust_2018_idioms)]
#![allow(unused_variables, dead_code)]

/// Stronghold Account Module
mod account;

/// Stronghold Storage Module
mod storage;

use account::{Account,AccountToCreate,AccountToImport};
use std::str;
//use account::{Account, AccountToCreate, AccountToImport};
use serde_json;

/// Stronghold doc com
struct Stronghold;

//{"id":"","external":true,"created":0,"lastDecryption":0,"decryptionCounter":0,"exportCounter":0,"bip39Mnemonic":"","bip39Passphrase":""}

impl Stronghold {

    fn find_record_id(&self, account_id_target: &str, snapshot_password: &str) -> storage::Id {
        let index = storage::get_index(snapshot_password);
        for (record_id,account_id) in index {
            if format!("{:?}",account_id) == account_id_target {
                return record_id;
            }
        };
        panic!("Unable to find record id with specified account id");
    }

    pub fn get_account(&self, account_id: &str, snapshot_password: &str) -> Account {
        let index = storage::get_index(snapshot_password);
        let account: Option<Account>;
        let record_id = self.find_record_id(account_id, snapshot_password);
        let decrypted = storage::read(record_id, snapshot_password);
        self.decode_record(&decrypted)
    }

    fn decode_record(&self, decrypted: &str) -> Account {
        let x: Account = serde_json::from_str(&decrypted).expect("Error reading record from snapshot");
        x
    }

    fn get_account_by_record_id(&self, record_id: storage::Id, snapshot_password: &str) -> Account {
        let decrypted = storage::read(record_id, snapshot_password);
        self.decode_record(&decrypted)
    }

    pub fn delete_account(&self, account_id: &str, snapshot_password: &str) -> Account {
        let record_id = self.find_record_id(account_id, snapshot_password);
        let account = self.get_account_by_record_id(record_id,snapshot_password);
        storage::revoke(record_id, snapshot_password);
        storage::garbage_collect_vault(snapshot_password);
        account
    }

    pub fn save_account(&self, account: Account, snapshot_password: &str) -> storage::Id {
        let account_serialized = serde_json::to_string(&account).expect("Error saving account in snapshot");
        storage::encrypt(&account.id, &account_serialized, snapshot_password)
    }

    // List ids of account
    pub fn get_account_index(&self, snapshot_password: &str, skip: usize, limit: usize) -> Vec< &str >  {
        let account_ids = Vec::new();
        for (i, (_ , account_id)) in storage::get_index(snapshot_password).into_iter().enumerate() {
            if i+1 <= skip {
                continue;
            }
            if i+1 > limit {
                break;
            }
            let id_ascii = format!("{:?}",account_id).as_str();
            account_ids.push(id_ascii);
        }
        account_ids
    }
    
    // List ids of account
    pub fn list_accounts(&self, snapshot_password: &str, skip: usize, limit: usize) -> Vec< storage::RecordHint >  {
        let account_ids = Vec::new();
        for (i, (_ , account_id)) in storage::get_index(snapshot_password).into_iter().enumerate() {
            if i+1 <= skip {
                continue;
            }
            if i+1 > limit {
                break;
            }
            account_ids.push(account_id);
        }
        account_ids
    }
    
    pub fn account_create(
        //bip39passphrase: Option<String>,
        snapshot_password: String,//for snapshot
        //password: String//for account encryption
    ) -> Result<Account, &'static str> {
        if snapshot_password.is_empty() {
            return Err("Invalid parameters: Password is missing");
        }

        if let Err(account) = Account::create(AccountToCreate{}) {
            Err(account)
        }else{

        }
    }
    /*
    pub fn account_import(
        created_at: u64,
        last_decryption: Option<u64>,
        decryption_counter: u32,
        export_counter: u32,
        bip39mnemonic: String,
        //bip39passphrase: Option<String>,
        snapshot_password: String,//snapshot
        //password: String//account
    ) -> Result<Account, &'static str> {
        if bip39mnemonic.is_empty() {
            return Err("Invalid parameters: bip39mnemonic is missing");
        }
        if snapshot_password.is_empty() {
            return Err("Invalid parameters: password is missing");
        }
        let account = Account::import(AccountToImport {
            created_at,
            last_decryption,
            decryption_counter,
            export_counter,
            bip39mnemonic,
            //bip39passphrase,
            //password,
        })?;

        // if is ok add to snapshot

        Ok(account)
    }
    */
    /*
    pub fn account_remove() {

    }

    pub fn transaction_sign() {

    }

    pub fn message_sign() {

    }

    pub fn message_decrypt() {

    }

    pub fn get_address() {

    }

    pub fn account_export() {

    }*/
}