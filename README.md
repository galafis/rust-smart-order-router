# Smart Order Router (SOR) em Rust

![Build Status](https://img.shields.io/github/actions/workflow/status/galafis/rust-smart-order-router/rust.yml?branch=main&style=for-the-badge) ![License](https://img.shields.io/github/license/galafis/rust-smart-order-router?style=for-the-badge) ![Crates.io](https://img.shields.io/crates/v/smart-order-router?style=for-the-badge)

---

## 🇧🇷 Visão Geral (Português)

Este repositório contém um **Smart Order Router (SOR)** de alta performance, desenvolvido em Rust. O objetivo deste projeto é fornecer um sistema robusto e eficiente para roteamento inteligente de ordens de compra e venda de criptoativos, buscando a melhor execução possível através de múltiplas exchanges.

O SOR analisa a liquidez em tempo real de diversas corretoras (Binance, Coinbase, Kraken, etc.) e utiliza algoritmos de otimização para dividir e rotear ordens, minimizando o *slippage* e os custos de transação. É uma ferramenta poderosa para traders algorítmicos, investidores institucionais e qualquer pessoa que busque otimizar a execução de suas estratégias de trading.

### Funcionalidades Principais

- **Conectividade Multi-Exchange:** Integração nativa com as principais exchanges de criptomoedas via APIs REST e WebSocket.
- **Algoritmos de Roteamento Inteligente:** Implementação de estratégias como VWAP (Volume Weighted Average Price) e TWAP (Time Weighted Average Price) para otimização da execução.
- **Análise de Liquidez em Tempo Real:** Monitoramento contínuo do order book de cada exchange para tomar decisões de roteamento baseadas em dados ao vivo.
- **Divisão de Ordens (Order Splitting):** Capacidade de dividir ordens maiores em fragmentos menores para minimizar o impacto no mercado.
- **Backtesting e Simulação:** Um framework completo para testar estratégias de roteamento com dados históricos e simular a execução em diferentes cenários de mercado.
- **Análise de Performance:** Métricas detalhadas sobre a qualidade da execução, incluindo *fill rate*, *slippage* e custos totais.

### Arquitetura do Sistema

O diagrama abaixo ilustra a arquitetura geral do Smart Order Router:

![Arquitetura do Smart Order Router](docs/architecture.png)

---

## 🇺🇸 Overview (English)

This repository contains a high-performance **Smart Order Router (SOR)**, developed in Rust. The goal of this project is to provide a robust and efficient system for intelligent routing of buy and sell orders for crypto assets, seeking the best possible execution across multiple exchanges.

The SOR analyzes real-time liquidity from various exchanges (Binance, Coinbase, Kraken, etc.) and uses optimization algorithms to split and route orders, minimizing slippage and transaction costs. It is a powerful tool for algorithmic traders, institutional investors, and anyone looking to optimize the execution of their trading strategies.

### Key Features

- **Multi-Exchange Connectivity:** Native integration with major cryptocurrency exchanges via REST and WebSocket APIs.
- **Intelligent Routing Algorithms:** Implementation of strategies such as VWAP (Volume Weighted Average Price) and TWAP (Time Weighted Average Price) for execution optimization.
- **Real-Time Liquidity Analysis:** Continuous monitoring of each exchange's order book to make routing decisions based on live data.
- **Order Splitting:** Ability to divide larger orders into smaller fragments to minimize market impact.
- **Backtesting and Simulation:** A complete framework for testing routing strategies with historical data and simulating execution in different market scenarios.
- **Performance Analytics:** Detailed metrics on execution quality, including fill rate, slippage, and total costs.

### System Architecture

The diagram below illustrates the general architecture of the Smart Order Router:

![Smart Order Router Architecture](docs/architecture.png)

---

## 🚀 Começando (Getting Started)

### Pré-requisitos (Prerequisites)

- Rust (>= 1.70): [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Git

### Instalação (Installation)

1. Clone o repositório:
   ```sh
   git clone https://github.com/galafis/rust-smart-order-router.git
   cd rust-smart-order-router
   ```

2. Compile o projeto:
   ```sh
   cargo build --release
   ```

### Exemplo de Uso (Usage Example)

Para executar o exemplo principal que demonstra o roteamento de uma ordem de compra:

```sh
cargo run --release --bin sor
```

Você também pode executar exemplos específicos:

```sh
cargo run --release --example basic_routing
```

---

## 🛠️ Desenvolvimento (Development)

### Testes

Para rodar os testes unitários e de integração:

```sh
cargo test
```

### Documentação

Para gerar a documentação do código:

```sh
cargo doc --open
```

---

## 📜 Licença (License)

Este projeto está licenciado sob a Licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## ✍️ Autor (Author)

**Gabriel Demetrios Lafis**

- [LinkedIn](https://www.linkedin.com/in/gabriel-demetrius/)
- [GitHub](https://github.com/galafis)

