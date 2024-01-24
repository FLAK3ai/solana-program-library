module.exports = {
  docs: [
    "introduction",
    "token",
    {
      type: 'category',
      label: 'Token-2022',
      collapsed: true,
      items: [
        "token-2022",
        "token-2022/status",
        "token-2022/extensions",
        "token-2022/wallet",
        "token-2022/onchain",
        "token-2022/presentation",
      ],
    },
    "token-swap",
    "token-lending",
    "associated-token-account",
    "token-upgrade",
    "memo",
    "name-service",
    "shared-memory",
    {
      type: "category",
      label: "Stake Pool",
      collapsed: true,
      items: [
        "stake-pool",
        "stake-pool/quickstart",
        "stake-pool/overview",
        "stake-pool/fees",
        "stake-pool/cli",
      ],
    },
    "single-pool",
    "feature-proposal",
    {
      type: "category",
      label: "Confidential Token Extension",
      collapsed: true,
      items: [
        "confidential-token",
        "confidential-token/quickstart",
        {
          type: "category",
          label: "Protocol Deep Dive",
          collapsed: true,
          items: [
            "confidential-token/deep-dive/overview",
            "confidential-token/deep-dive/encryption",
            "confidential-token/deep-dive/zkps",
          ],
        },
      ],
    },
    {
      type: "category",
      label: "Account Compression",
      collapsed: true,
      items: [
        "account-compression",
        "account-compression/concepts",
        "account-compression/usage",
      ]
    },
    {
      type: "category",
      label: "Transfer Hook Interface",
      collapsed: true,
      items: [
        "transfer-hook-interface/introduction",
        "transfer-hook-interface/specification",
        "transfer-hook-interface/configuring-extra-accounts",
        "transfer-hook-interface/examples",
      ]
    },
  ],
};
