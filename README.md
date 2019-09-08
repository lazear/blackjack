# Blackjack

"Provably fair" Blackjack, implemented in Rust

- Blackjack pays out 3:2

The blackjack engine is set up as a simple state machine that operates as follows:

Initialize a `Player` and then a `Game`. The state is now `Ready`.

Set an initial bet, and then cards are dealt and an initial view of the table is returned. If the player was dealt a blackjack, the game state is set to `Final`.

While the game state is set to the player's turn, the player may take an action, returning an updated view.
Once the game state is set to `Dealer`, the dealer will play until winning or losing. The game state will then be set to `Final`.

Once the game state is set to `Final`, any winnings will be returned to the player