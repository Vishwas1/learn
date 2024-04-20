## 1. Issuer IDentity
```
nibid keys add issuer --keyring-backend test

issuer: {
    address: "nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg",
    name: "issuer",
    mnemonic: "fossil away enjoy victory conduct position window torch middle grab maple head scheme kick idle shoe width monkey village spawn goddess ankle parrot knife"
}

nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg 
```

## 2. User Identity
```
nibid keys add user --keyring-backend test

user: {
    address: "nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5",
    name: "user",
    mnemonic: "mind rate breeze party huge brain solar upon budget find opinion sketch submit awkward evil throw phrase umbrella night person improve ribbon siren cute"
}

nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 1000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 

balances:
- amount: "100000"
  denom: unibi
pagination:
  next_key: null
  total: "0"
```

## 3. Admin deploys kyc contract

```
nibid tx wasm store ./artifacts/counting_contract.wasm --from validator --gas 100000000

nibid q wasm list-code 

nibid tx wasm instantiate 11 '{"counter": 0, "minimal_donation": { "amount": "10", "denom": "unibi"}}' --label "Activity" --from validator --gas 100000000 --no-admin

nibid q wasm list-contract-by-code 11

```
kyc_contract_addr: 
nibi13s9jpl2cqy8g3ggxhps40up3cys7ru6rgek5ad06zraf6xacsg3slecw8s

```
nibid query wasm contract-state smart nibi13s9jpl2cqy8g3ggxhps40up3cys7ru6rgek5ad06zraf6xacsg3slecw8s '{"value":{}}'
```

## 4. Issuer deploys the NFT contract with Kyc_contract_address as an admin

```
nibid tx wasm store ./artifacts/cw721_base.wasm --from issuer --gas 100000000 --keyring-backend test

nibid q wasm list-code 

nibid tx wasm instantiate 12 '{"name": "Kyc SBT", "symbol": "kycsbt", "minter": "nibi13s9jpl2cqy8g3ggxhps40up3cys7ru6rgek5ad06zraf6xacsg3slecw8s"}' --label "Activit" --from issuer --gas 100000000 --no-admin --keyring-backend test

nibid q wasm list-contract-by-code 12

```
issuer contract address: nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y

### Check the minter - it should be kyc contract address
```
nibid query wasm contract-state smart nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y '{"minter":{}}'
```



## 5. Admin whitelist nft_contract_address in KYC contract through reg_issuer_contract()

```
nibid tx wasm execute nibi13s9jpl2cqy8g3ggxhps40up3cys7ru6rgek5ad06zraf6xacsg3slecw8s '{"poke": {"proxy_contract_addr": "nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y"}}' --from validator --gas 100000000 --fees=200000unibi

nibid query wasm contract-state smart nibi13s9jpl2cqy8g3ggxhps40up3cys7ru6rgek5ad06zraf6xacsg3slecw8s '{"get_proxy_message":{}}'
```

## 6. User calls mintNFT() - donate of the KYC contract

```
nibid tx wasm execute nibi13s9jpl2cqy8g3ggxhps40up3cys7ru6rgek5ad06zraf6xacsg3slecw8s '{"donate": {}}' --from user --amount 10unibi --gas 100000000 --fees=200000unibi --keyring-backend test
```

## 7. Check in NFT contract is user is the owner of the NFT (token - id 2)

```
nibid query wasm contract-state smart nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y '{"owner_of":{"token_id": "4"}}'
```

this is user
```
data:
  approvals: []
  owner: nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5
```


## Return total number of tokens issued
```
nibid query wasm contract-state smart nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y '{"num_tokens":{}}'
```

## Check metadata of the nft 

```
nibid query wasm contract-state smart nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y '{"nft_info":{"token_id": "4"}}'
```

## Check metadata of the nft (all)
```
nibid query wasm contract-state smart nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y '{"all_nft_info":{"token_id": "4"}}'
```

## Check nos tokens owned by a user

```
nibid query wasm contract-state smart nibi1twqmj52wms2hhpxvwc4rulrfezy9yprpdv7tannvr5w0u7e05wmqs3j26y '{"tokens":{"owner": "nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5"}}'
```
