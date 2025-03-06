use crate::hash_function::sha256_hash;
use reqwest;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxIn {
    pub previous_output: [u8; 32], // 交易输入的哈希值
    pub script_sig: Vec<u8>,       // 解锁脚本
    pub sequence: u32,             // 序列号
}

impl TxIn {
    pub fn new(previous_output: [u8; 32], script_sig: Vec<u8>, sequence: u32) -> Self {
        TxIn {
            previous_output,
            script_sig,
            sequence,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOut {
    pub value: u64,             // 交易输出金额
    pub script_pubkey: Vec<u8>, // 锁定脚本
}
impl TxOut {
    pub fn new(value: u64, script_pubkey: Vec<u8>) -> Self {
        TxOut {
            value,
            script_pubkey,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub version: u32,        // 版本号
    pub inputs: Vec<TxIn>,   // 交易输入
    pub outputs: Vec<TxOut>, // 交易输出
    pub lock_time: u32,      // 锁定时间
}

impl Transaction {
    // 创建一个新的交易
    pub fn new(version: u32, inputs: Vec<TxIn>, outputs: Vec<TxOut>, lock_time: u32) -> Self {
        Transaction {
            version,
            inputs,
            outputs,
            lock_time,
        }
    }

    // 计算交易的哈希值
    pub fn hash(&self) -> [u8; 32] {
        let serialized = serde_json::to_vec(self).unwrap();
        let hash = sha256_hash(&serialized);
        hash.as_ref().try_into().unwrap()
    }

    // 反序列化交易
    pub fn deserialize(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }

    // 签名交易
    pub fn sign(&mut self, key_pair: &Ed25519KeyPair, input_index: usize) {
        if input_index >= self.inputs.len() {
            panic!("Input index out of bounds");
        }

        // 在签名前，清空 script_sig
        let original_script_sig = self.inputs[input_index].script_sig.clone();
        self.inputs[input_index].script_sig = Vec::new();

        // 使用交易的序列化数据作为签名消息
        let message = serde_json::to_vec(self).unwrap();
        let signature = key_pair.sign(&message);

        // 恢复 script_sig
        self.inputs[input_index].script_sig = original_script_sig;

        // 将签名和公钥存储在 script_sig 中
        let mut script_sig = Vec::new();
        script_sig.extend_from_slice(signature.as_ref()); // 添加签名
        script_sig.extend_from_slice(key_pair.public_key().as_ref()); // 添加公钥
        self.inputs[input_index].script_sig = script_sig;
    }

    // 验证交易签名
    pub fn verify_signature(&self, input_index: usize) -> bool {
        if input_index >= self.inputs.len() {
            return false; // 输入索引超出范围
        }

        let script_sig = &self.inputs[input_index].script_sig;
        if script_sig.len() < 96 {
            return false; // 签名和公钥至少需要 96 字节（签名 64 字节，公钥 32 字节）
        }

        let signature = &script_sig[..64]; // 前 64 字节是签名
        let public_key = &script_sig[64..96]; // 后面的 32 字节是公钥
                                              // 克隆交易并清空 script_sig
        let mut tx_clone = self.clone();
        tx_clone.inputs[input_index].script_sig = Vec::new();

        // 使用克隆后的交易序列化数据作为验证消息
        let message = serde_json::to_vec(&tx_clone).unwrap();
        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        public_key.verify(&message, signature.as_ref()).is_ok()
    }

    // 广播交易到其他节点
    pub async fn broadcast_transaction(&self, node_url: &str) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/transactions", node_url))
            .json(&self)
            .send()
            .await?;
        res.error_for_status()?;
        Ok(())
    }
}
