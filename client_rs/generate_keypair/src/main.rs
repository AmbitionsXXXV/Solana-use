use solana_sdk::{signature::Keypair, signer::Signer};
use std::{
    env::{args, current_dir},
    fs::File,
    io::{Error, Write},
    path::Path,
};

fn main() -> Result<(), Error> {
    let count: u16 = args()
        .nth(1)
        .expect("no count provided")
        .parse::<u16>()
        .unwrap();
    let default_path = current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .into_owned();
    let path = args().nth(2).unwrap_or(default_path);
    let path = Path::new(&path);

    for index in 0..count {
        let keypair = Keypair::new();
        let secret_bytes = keypair.to_bytes(); // 获取包含私钥的字节向量

        // 构造文件名和路径
        let filename = format!("keypair_{}.json", index);
        let single_path = path.join(filename);

        // 创建文件并写入 JSON 字符串
        let mut file = File::create(&single_path)?;
        let json_string = format!(
            "[{}]",
            secret_bytes
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        file.write_all(json_string.as_bytes())?;

        println!("Keypair {}: {:?}", index + 1, keypair.pubkey());
        println!("Secret key saved to {}", single_path.to_string_lossy());
    }

    println!("Saved {} keypairs.", count);

    Ok(())
}
