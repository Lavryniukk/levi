# Levi: Rust Web App Bootstrapper

Levi is a CLI tool designed to streamline the process of starting new web projects in Rust. It provides a variety of templates that combine popular Rust frameworks and ORMs, with the added capability of GraphQL via Juniper.

## Installation

Install Levi with the following command:
cargo install levi-cli

## Usage

To start a new project, run Levi and follow the interactive prompts to choose your preferred template based on your project requirements.

## Features

- **Interactive Template Selection**: Choose from a variety of pre-configured templates through an easy-to-use interactive menu.

## Available Template Combinations with Juniper GraphQL

Levi supports a range of Rust web frameworks with Juniper for creating GraphQL APIs. Here are some of the combinations you can start with:

- **Actix + Sea-ORM + Juniper**: Combine the speed of Actix-web with the flexibility of Sea-ORM and the power of Juniper for GraphQL APIs.✅
- **Axum + Diesel + Juniper**: Build powerful GraphQL APIs using Axum, with Diesel for ORM and Juniper for GraphQL.❌
- **Axum + Sea-ORM + Juniper**: A setup that combines Axum and Sea-ORM with Juniper for developers who prefer a lighter ORM option.❌
- **Actix + Diesel + Juniper**: Leverage Actix-web’s performance with Diesel’s robust data handling and Juniper’s GraphQL capabilities.❌
- **Yew + Axum + Diesel + Juniper**: A full-stack solution offering Yew for the frontend, Axum and Diesel for the backend, enhanced with Juniper for GraphQL interactions.❌
- **Yew + Actix + Sea-ORM + Juniper**: Integrate Yew for frontend development with a backend powered by Actix-web, Sea-ORM, and Juniper for comprehensive GraphQL support.❌

## Getting Started

Simply run Levi, and let the interactive menu guide you through selecting and setting up your preferred template.

For more detailed documentation and examples, please visit our official repository or documentation site.
