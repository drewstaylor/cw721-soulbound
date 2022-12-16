# cw721-soulbound
Test implementation of Area-52 cw721-soulbound NFT blueprints for contractors and librarians

## Project Structure
- [contractors](https://github.com/drewstaylor/cw721-soulbound/tree/main/contractors) - Contract based implemenatation of cw721-soulbound NFTs
- [librarians](https://github.com/drewstaylor/cw721-soulbound/tree/main/librarians) - Library based implemenatation of cw721-soulbound NFTs
- [cw7210-soulbound](https://github.com/drewstaylor/cw721-soulbound/tree/main/cw721-soulbound) - Base cw721-soulbound package


## Goals
##### Contractors

This implementation should teach developers how to implement NFT contracts as a static CosmWasm contract.

##### Librarians

This implementation should teach developers how to implement NFT contracts as a library that can be imported and used in other projects.

##### cw721-soulbound

This package should show developers how they can make modifications to [cw721-base](https://crates.io/crates/cw721-base) to build customized tokens with idiosyncratic logic.


***

### TODOs:
- Contractor implementation should go a step further and create `execute.rs`, `query.rs`, `msg.rs`, `state.rs` and instance the execute and query functions in `contract.rs` to further highlight the differences between bins and libs
- The goal is start with what they're used to seeing and take them through the process of trimming down the very verbose code to something concise and sensible
- This will help show devs how that using libraries *should* generally lead to smaller and easier to read code bases
