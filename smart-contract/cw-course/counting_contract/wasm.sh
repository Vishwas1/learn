```
nibid tx wasm store ./artifacts/counting_contract.wasm --from validator --gas 100000000

nibid tx wasm instantiate 7 '{"counter": 0, "minimal_donation": { "amount": "10", "denom": "unibi"}}' --label "Activity" --from validator --gas 100000000 --no-admin

nibid q wasm list-code 

nibid q wasm list-contract-by-code 7

```

- nibi1utjx3594tlvfw4375esgu72wa4sdgf0q7x4ye27husf5kvuzp5rssq840y
- nibi1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0swj9x42

```
nibid tx wasm execute nibi1utjx3594tlvfw4375esgu72wa4sdgf0q7x4ye27husf5kvuzp5rssq840y '{"poke": {"proxy_contract_addr": "nibi1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0swj9x42"}}' --from validator --gas 100000000 --fees=200000unibi
```

nibid query wasm contract-state smart nibi1utjx3594tlvfw4375esgu72wa4sdgf0q7x4ye27husf5kvuzp5rssq840y '{"get_proxy_message":{}}'

nibid query wasm contract-state smart nibi1utjx3594tlvfw4375esgu72wa4sdgf0q7x4ye27husf5kvuzp5rssq840y '{"value":{}}'

nibid query wasm contract-state smart nibi1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0swj9x42 '{"value":{}}'

nibid query wasm contract-state smart nibi1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0swj9x42 '{"get_proxy_message":{}}'


```
nibid tx wasm execute nibi1utjx3594tlvfw4375esgu72wa4sdgf0q7x4ye27husf5kvuzp5rssq840y '{"donate": {}}' --from validator --amount 10000unibi --gas 100000000 --fees=200000unibi
```