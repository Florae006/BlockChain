use crate::hash_function::sha256_hash;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TxIn {
    pub previous_output: [u8; 32], // 交易输入的哈希值
    pub script_sig: Vec<u8>,       // 解锁脚本
    pub sequence: u32,             // 序列号
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOut {
    pub value: u64,             // 交易输出金额
    pub script_pubkey: Vec<u8>, // 锁定脚本
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn sign(&mut self, key_pair: &Ed25519KeyPair) {
        let message = serde_json::to_vec(self).unwrap();
        let signature = key_pair.sign(&message);
        // 将签名和公钥存储在 script_sig 中
        let mut script_sig = Vec::new();
        script_sig.extend_from_slice(signature.as_ref()); // 添加签名
        script_sig.extend_from_slice(key_pair.public_key().as_ref()); // 添加公钥
        self.inputs[0].script_sig = script_sig; // 假设只有一个输入
    }

    // 验证交易签名
    pub fn verify_signature(&self) -> bool {
        if self.inputs.is_empty() {
            return false; // 没有输入，无法验证
        }

        let script_sig = &self.inputs[0].script_sig;
        if script_sig.len() < 64 {
            return false; // 签名和公钥至少需要 64 字节（签名 64 字节，公钥 32 字节）
        }

        let signature = &script_sig[..64]; // 前 64 字节是签名
        let public_key = &script_sig[64..]; // 后面的字节是公钥

        let message = serde_json::to_vec(self).unwrap();
        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        public_key.verify(&message, signature.as_ref()).is_ok()
    }
}
