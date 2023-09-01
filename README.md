# Patina: Currency Exchange and Cryptocurrency Catalog API

Welcome to Patina, the Currency Exchange and Cryptocurrency Catalog API! Patina is a Rust-based API designed for efficiently managing and cataloging currency exchange rates and cryptocurrencies. Whether you are developing a financial application, conducting research, or require a reliable source for currency and cryptocurrency data, Patina has been crafted to meet your needs seamlessly.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
- [API Documentation](#api-documentation)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Managing currency exchange rates and cryptocurrencies can be a complex task. Patina makes it easy by offering Create, Read, Update, and Delete (CRUD) operations for both currency exchange rates and cryptocurrencies. This API is efficient, user-friendly, and highly customizable to cater to your specific requirements.

## Features

- **Currency Exchange Management:**
  - Create, read, update, and delete currency exchange rates.
  - Retrieve the latest exchange rate for a specific currency pair.
  - List all available currency pairs.

- **Cryptocurrency Catalog:**
  - Add, retrieve, update, and remove cryptocurrencies from the catalog.
  - Search for cryptocurrencies by name, symbol, or ID.
  - Get detailed information about a specific cryptocurrency.

- **Data Integrity with Diesel ORM:**
  - Utilizes the Diesel ORM for robust data storage, ensuring data integrity and reliability.
  - Supports data validation and error handling to prevent inconsistencies.

- **Security:**
  - Implements secure authentication and authorization mechanisms to safeguard your data.
  - Provides rate limiting and access control to mitigate abuse.

- **Extensible and Customizable:**
  - Easily extend the API with additional features or integrate it into your existing projects.
  - Customize response formats and data sources to align with your specific needs.

## Getting Started

### Prerequisites

Before you dive into using Patina, make sure you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/tools/install) - The programming language in which the API is written.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - The Rust package manager.
- A PostgreSQL database (using Diesel ORM) to store API data.

### Installation

1. Clone the Patina API repository from GitHub:

   ```bash
   git clone https://github.com/yourusername/patina-api.git
   cd patina-api
   ```

2. Create a `.env` file in the project root and configure your Diesel ORM database connection settings:

   ```env
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

   Replace `username`, `password`, `localhost`, and `database_name` with your actual database credentials.

3. Install the required dependencies:

   ```bash
   cargo install
   ```

4. Run the database migrations to set up the database schema:

   ```bash
   diesel migration run
   ```

### Usage

To start using Patina, follow these steps:

1. Start the Patina API server:

   ```bash
   cargo run --bin patina
   ```

2. Access the API at `http://localhost:8000` (or another specified address) to interact with the various endpoints.

## API Documentation

Comprehensive API documentation can be found [here](./API_DOCUMENTATION.md). It provides detailed information on available endpoints, request/response formats, and examples.

## Examples

Here are some examples demonstrating how to use the Patina API:

### Create a Currency Exchange Rate

```http
POST /exchange-rates
Content-Type: application/json

{
  "base_currency": "USD",
  "target_currency": "EUR",
  "rate": 0.85
}
```

### Get Cryptocurrency Information

```http
GET /cryptocurrencies/bitcoin
```

### Update a Currency Exchange Rate

```http
PUT /exchange-rates/1
Content-Type: application/json

{
  "rate": 0.86
}
```

### Delete a Cryptocurrency

```http
DELETE /cryptocurrencies/ethereum
```

## Contributing

We welcome contributions from the community! If you'd like to contribute to the Patina project, please read our [Contributing Guidelines](./CONTRIBUTING.md) for instructions on how to get started.

## License

Patina is open-source and available under the [MIT License](./LICENSE). You are free to use, modify, and distribute it in accordance with the terms of the license.

Thank you for choosing Patina, your solution for efficient currency and cryptocurrency data management. Should you have any questions or encounter any issues, please do not hesitate to [contact us](mailto:your@email.com).

Happy coding!
