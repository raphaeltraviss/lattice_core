# lattice_core

![Build Status](https://img.shields.io/badge/build-passing-brightgreen) ![Version](https://img.shields.io/badge/version-1.0.0-blue)

**lattice_core** is a revolutionary Rust library that seamlessly combines the power of **Automerge**, **GraphQL**, and **SQLite**. It allows you to define your data schema, then insert, mutate and query your data as if it were a regular database table. All this while it automatically syncs with peers in the background and resolves merge conflicts without any user intervention.

Beneath its simple API, lattice_core handles complex concurrent edits and syncs with impressive performance. Whether you are building a collaborative application, a distributed system, or simply need a reliable local persistence layer, lattice_core has got you covered.

## Features

- **Schema Definition**: Define your data schema with ease and interact with it as you would with a regular database table.
- **CRUD Operations**: Insert, update, delete, and query data with the powerful and familiar SQL syntax.
- **Automatic Synchronization**: No need to worry about syncing data across different instances; lattice_core automatically handles it in the background.
- **Conflict-Free**: With Automerge's conflict-free data structures, forget about handling merge conflicts manually.

## Getting Started

### Prerequisites

Before you begin, ensure you have met the following requirements:

- You have installed the latest version of Rust and Cargo. If not, you can install them from [here](https://www.rust-lang.org/tools/install).

### Installation

Add `lattice_core` to your `Cargo.toml` file under `[dependencies]`:

```toml
[dependencies]
lattice_core = "1.0.0"
```

Then, run `cargo build` to download and compile `lattice_core`.

### Usage

Here's a quick example of how to use lattice_core:

```rust
use lattice_core::Lattice;

fn main() {
    // Initialize a new Lattice
    let mut lattice = Lattice::new();

    // Define a schema
    let schema = r#"
    type User {
        id: ID!
        name: String!
        email: String!
    }"#;

    // Apply the schema
    lattice.apply_schema(schema).unwrap();

    // Insert data
    let mutation = r#"
    mutation {
        createUser(input: { id: "1", name: "Alice", email: "alice@example.com" }) {
            user {
                id
                name
                email
            }
        }
    }"#;

    lattice.mutate(mutation).unwrap();

    // Query data
    let query = r#"
    {
        User(id: "1") {
            name
            email
        }
    }"#;

    let result = lattice.query(query).unwrap();
    println!("{}", result);
}
```

## Contributing

We'd love your help in making lattice_core better! Please take a look at our [Contribution Guide](CONTRIBUTING.md) for guidelines on how to proceed.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Contact

If you want to contact us, you can reach out at our [GitHub Discussions](https://github.com/raphaeltraviss/lattice_core/discussions).

Join us in shaping the future of data management with lattice_core!

