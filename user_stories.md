# User Stories for Sandwich-Resistant AMM

## Liquidity Provider

1. As a liquidity provider, I want to add liquidity to the AMM pool so that I can earn fees from trades.
2. As a liquidity provider, I want to remove my liquidity from the AMM pool when I decide to exit my position.
3. As a liquidity provider, I want to view my current liquidity position and earned fees.

## Trader

1. As a trader, I want to swap tokens using the AMM so that I can exchange one token for another.
2. As a trader, I want to see the current exchange rate before making a trade.
3. As a trader, I want to be protected from sandwich attacks so that I get fair execution prices.
4. As a trader, I want to set a slippage tolerance for my trades to ensure I don't receive unexpectedly poor prices.

## AMM Administrator

1. As an AMM administrator, I want to initialize the AMM with initial liquidity and parameters.
2. As an AMM administrator, I want to set and adjust the minimum spread offered by the pool to balance LP and swapper welfare.

## Solana Validator

1. As a Solana validator, I want to process transactions for the AMM in a way that respects slot windows.
2. As a Solana validator, I want to reset the AMM state at the beginning of each slot window to maintain the sandwich-resistant properties.

## MEV Searcher (Potential Attacker)

1. As an MEV searcher, I want to identify profitable trading opportunities within the constraints of the sandwich-resistant AMM.
2. As an MEV searcher, I understand that I cannot execute atomic sandwich attacks within a single slot window.