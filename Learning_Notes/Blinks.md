# Blinks (Blockchain Links)

## Overview

Blinks, or Blockchain Links, are clients that detect Action URLs and unfurl them into full interactive experiences. They are part of the Actions protocol developed in collaboration with Solana to make Solana transactions sharable everywhere.

## Key Concepts

1. **Actions**: A protocol for creating & delivering Solana transactions through URLs, making Solana sharable everywhere.

2. **Blinks**: Clients that detect Action URLs & unfurl them into full experiences. They're similar to Link Previews but with interactive capabilities.

3. **Integration**: Blinks are integrated into leading Solana wallets like Phantom, Backpack, and soon Solflare.

## Action URL Scheme

Actions conform to a URL scheme denoted by `solana-action:`. The basic format is:

```
solana-action:<link>
```

Where `<link>` is a conditionally URL-encoded absolute HTTPS URL([1](https://docs.dialect.to/documentation/actions/actions/url-scheme)).

## Detecting Actions

Blinks can detect Actions in at least 3 ways:

1. Explicit Action URL: `solana-action://https://actions.alice.com/donate`
2. Website with `actions.json`: `https://alice.com/actions.json` maps website URLs to Action API URLs
3. Interstitial URL: `https://dial.to/?action=solana-action:https://actions.alice.com/donate`

## Rendering Blinks

Blinks should support a combination of multiple actions and at most one user input text field. The layout typically includes([3](https://docs.dialect.to/documentation/actions/blinks/rendering-blinks)):

- Icon
- Title
- Description
- Action buttons
- Optional input field

## Security

To ensure safety, Dialect maintains a public registry of verified non-malicious blockchain links. Only Actions registered in this registry will unfurl in supported platforms like Twitter([4](https://docs.dialect.to/documentation/actions/security)).

## Integration for Developers

Developers can integrate Blinks into their clients (e.g., wallets, web3 social dApps) using the Blinks SDK([5](https://docs.dialect.to/documentation/actions/guide-integrate-blinks-into-your-client)):

1. Install the SDK: `npm install @dialectlabs/blinks` or `yarn add @dialectlabs/blinks`
2. Import necessary components and hooks
3. Set up the Blink component in your application

## Additional Resources

- [Dialect Documentation](https://docs.dialect.to/documentation)
- [Actions Specification](https://docs.dialect.to/documentation/actions)
- [Blinks Public Registry](https://docs.dialect.to/documentation/actions/the-blinks-public-registry)

Remember to stay updated with the latest Dialect documentation, as the Blinks and Actions ecosystem is evolving.