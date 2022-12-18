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

This package should show developers how they can make modifications to [cw721-base](https://crates.io/crates/cw721-base) to build custom tokens with idiosyncratic logic. This package is used by [librarians](https://github.com/drewstaylor/cw721-soulbound/tree/main/librarians) and is *not* used by [contractors](https://github.com/drewstaylor/cw721-soulbound/tree/main/contractors).


***

### Notes:
- When creating custom [cw721](https://crates.io/crates/cw721) tokens there are 2 strategies a developer must choose from
- Either 1) Import everything locally so that [cw721-base](https://crates.io/crates/cw721-base)'s internals can be modified directly (e.g. "Contractor" implementation); or, 2) import [cw721-base](https://crates.io/crates/cw721-base) into a separate project and expose it as a library (e.g. "Librarian" implementation)
- The included 2 projects ([contractors](https://github.com/drewstaylor/cw721-soulbound/tree/main/contractors) / [librarians](https://github.com/drewstaylor/cw721-soulbound/tree/main/librarians)) can be used to compare and contrast these 2 strategies
- Developers will be tempted to use the "Contractor" strategy, as it better resembles what they're used to seeing in CosmWasm contracts
- We can start with what they're used to seeing (e.g. "Contractor"), and take them through the process of trimming the code to something concise and easy to follow
- The "Librarian" strategy also allows developers to share their token specification as a Crate that can be hosted on [Crates.io](https://crates.io/)
- If a "Contractor" were to publish their project as a Crate it would create 2 issues: 1) Developers using the crate may not have access to all the required types (e.g. not public); and 2) the code is tightly coupled to the original project making it harder to modify or extend; also, the developer using the code is forced to implement wholesale, the custom logic and choices made by the original developer.
- To sum it up: the "Contractor" strategy is not extensible and the code base is bloated and difficult to follow
- By contrast, the "Librarian" strategy is generic and highly extensible; additionally, its code base is cleaner and easier to read.
- Understanding the differences between the 2 implementations should be helpful for understanding the relationship of [cw721](https://crates.io/crates/cw721) to [cw721-base](https://crates.io/crates/cw721-base), and the relationship of [cw721](https://crates.io/crates/cw721) and [cw721-base](https://crates.io/crates/cw721-base) to writing your own custom NFT code.
