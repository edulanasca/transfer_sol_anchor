## How to build and deploy
```
anchor build \
anchor deploy -p undead_arena --program-keypair path/to/json --provider.cluster devnet \
anchor test --skip-deploy --skip-local-validator --skip-build
```

