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

### Notes:
- When creating custom [cw721](https://crates.io/crates/cw721-base) tokens there are 2 strategies a developer must choose from
- Either 1) Import everything locally so that [cw721](https://crates.io/crates/cw721-base)'s internals can be modified directly (e.g. "Contractor" implementation); or, 2) import [cw721](https://crates.io/crates/cw721-base) into a separate project and expose it as a library (e.g. "Librarian" implementation)
- The included 2 projects ([contractors](https://github.com/drewstaylor/cw721-soulbound/tree/main/contractors) / [librarians](https://github.com/drewstaylor/cw721-soulbound/tree/main/librarians)) can be used to compare and contrast these 2 strategies
- Developers will be tempted to use the "Contractor" strategy, as it better resembles what they're used to seeing in CosmWasm contracts
- The goal is start with what they're used to seeing and take them through the process of trimming down the very verbose code to something concise and sensible
- This will help show devs how that using libraries *should* generally lead to smaller and easier to read code bases
- The "Librarian" strategy is also useful because it allows developers to share their token specification as a Crate that can be hosted on [Crates.io](https://crates.io/)
- If a "Contractor" were to publish their project as a Crate it would create 2 issues: 1) Developers inheriting and using the crate in their own code probably won't have access to all the required types (e.g. not public); and, 2) the Crate code is tightly coupled to the original developer's project—not only would it be perhaps impossible to modify or extend, the developer importing the code will be forced to implement all the custom logic, choices, and modifications made by the original developer.
- To sum it up: the "Contractor" strategy is not extensible and the code base is bloated and difficult to follow
- By contrast, the "Librarian" strategy is generic and highly extensible; additionally, its code base is cleaner and easier to read.
