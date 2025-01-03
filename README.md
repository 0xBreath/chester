# Chester
Chess GUI written in Rust using Yew.
The goal is for an LLM like Claude to be able to play chess with the user.

The opponent is an LLM agent, probably Claude, with an open source prompt.
The ultimate vision is for a custom Solana program to handle state management, multiplayer, and betting.

* The user visits the website.
* The user places a wager of SOL on a game, and the game starts.
* The SOL wager is transferred to escrow.
* If the user loses the game, the wager is awarded to the LLM agent pool which is effectively locked on-chain.
* If the user wins the game, some multiple beyond their wager is awarded to them.
* Liquidity providers can provide liquidity to the LLM agent pool and earn a portion of the earned wagers.
* The LPs are incentivized to make the LLM agent as strong as possible.



## Development
```shell
cargo install trunk

rustup override set 1.81.0

rustup target add wasm32-unknown-unknown

trunk serve
```
Open at `http://localhost:8080`