

```json
{
  "BlockNumber": "u32",
  "Status": {
      "_enum": {
        "WaitingToSign": "H256",
        "WaitingToConfirm": "(AccountId, AccountId)",
        "CanTransfer": null
      }
  },
  "Contract": {
      "digest": "H256",
      "status": "Status",
      "updated": "BlockNumber",
      "party_a": "Option<AccountId>",
      "party_b": "Option<AccountId>"
  }
}
```
