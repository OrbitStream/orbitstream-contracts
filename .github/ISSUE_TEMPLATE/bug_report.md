---
name: "Bug Report"
about: "Report a bug in the OrbitStream Soroban Contract"
title: "[BUG] "
labels: ["bug", "needs-triage"]
assignees: ""
---

# Bug Report

## 🔍 Is this a regression?

<!-- Did this work before and now it's broken? If so, which version last worked? -->

## 📝 Description

<!-- A clear and concise description of what the bug is. -->

## 🔄 Steps to Reproduce

1. 
2. 
3. 

## ✅ Expected Behavior

<!-- What you expected to happen. -->

## ❌ Actual Behavior

<!-- What actually happened. Include error messages or transaction hashes. -->

## 🌍 Environment

- **OS**: [e.g., Ubuntu 22.04, macOS 14]
- **Rust version**: [e.g., 1.75.0]
- **Soroban SDK version**: [e.g., 21.0.0]
- **Stellar network**: [testnet / mainnet]
- **Contract deployment**: [deployed / local only]

## 📋 Contract Details

- **Contract name**: [e.g., OrbitStream]
- **Contract ID**: [e.g., CABC...1234 (if deployed)]
- **Function called**: [e.g., create_escrow]
- **Transaction hash**: [e.g., abc123... (if applicable)]

## 📋 Arguments Passed

<!-- Paste the exact arguments used to call the contract function. -->

```rust
// Example
client.create_escrow(&buyer, &seller, &token, &1000, &3600);
```

## 🔍 Error Output

<!-- Paste the full error message or panic output. -->

```
[Paste error here]
```

## 🧪 Test Case

<!-- If possible, provide a minimal Rust test that reproduces the issue. -->

```rust
#[test]
fn test_reproduce_bug() {
    let env = Env::default();
    // ...
}
```

## 📋 Storage State

<!-- If the bug involves contract state, describe the relevant storage entries. -->

## 📎 Additional Context

<!-- Any other context about the problem. -->

## ✅ Checklist

- [ ] I have searched existing issues and this is not a duplicate
- [ ] I am using the latest version of Soroban SDK
- [ ] I have included contract function and arguments
- [ ] I have included error output
