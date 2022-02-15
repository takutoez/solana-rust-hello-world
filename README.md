# solana-tutorial

元記事: https://note.com/cml_2010/n/n3b0895215b64

## Solana セットアップ
```
solana config set --url https://api.devnet.solana.com
solana-keygen new
solana-keygen pubkey #確認
solana airdrop 1
solana balance #確認
```

## ビルド
```
cargo build-bpf
```
To deploy this program: 以下のコマンドを実行
