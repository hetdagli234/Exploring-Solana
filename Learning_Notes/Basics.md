# Solana Basics

## 1. Accounts

Everything is an account in Solana. Data and Programs are stored as accounts.

### Key Points:
- **Creation**: Only the System Program can create new accounts.
- **Account Flags**:
  1. **Writable**: Allows state changes, but only serial access.
  2. **Read-only**: No state changes allowed, enabling parallel access.
  3. **Signer**: Indicates if the account is a transaction signer.
  4. **Executable**: Determines if the account is a program.

## 2. Programs

Programs in Solana are similar to smart contracts in other blockchains.

### Characteristics:
- Can own non-executable accounts.
- Owned by the BPF Upgradeable Loader.

## 3. Rent

Rent is a mechanism to prevent blockchain bloat and incentivize efficient resource usage.

### Important Notes:
- Accounts need 2 years of rent to become rent-exempt.
- Resizing an account adjusts the rent cost/return accordingly.

## 4. Compute Units

Compute units measure the computational resources used by transactions.

### Limits:
- There's a maximum compute unit per block.
- Default: 200,000 CU per transaction.

## 5. Transactions

Transactions are the primary way to interact with the Solana blockchain.

### Structure:
1. **Messages**:
   - Instructions array
   - Recent blockhash
   - Fee payer
2. **Signers**: Array of signing accounts
3. **Note**: A transaction is essentially a type of message.

## 6. Program Derived Addresses (PDAs)

PDAs are special accounts that don't have private keys.

### Characteristics:
- Composed of seeds and bumps.
- Deterministic with the same set of seeds.
- Accounts without private keys.

## 7. SPL-Token

The SPL-Token program is Solana's native token standard.

### Key Features:
- Native program on Solana.
- Creating a new token requires minting first.
- Non-deterministic by default.
- Associated Token Account: Makes tokens deterministic using mint address and wallet address.

---

## Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)