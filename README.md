#  Gaming - Solana Program

Programa en Solana desarrollado con Anchor Framework que permite gestionar videojuegos en la blockchain.

## ¿Qué hace?

Permite realizar operaciones CRUD sobre videojuegos almacenados en la blockchain de Solana usando PDAs (Program Derived Addresses).

## Operaciones

- **Crear** un videojuego con título, género y precio
- **Leer** la información de un videojuego
- **Actualizar** el título, género y precio
- **Eliminar** un videojuego de la blockchain

## Datos del Videojuego

| Campo   | Tipo   | Descripción              |
|---------|--------|--------------------------|
| owner   | Pubkey | Dueño del videojuego     |
| titulo  | String | Nombre del videojuego    |
| genero  | String | Género del videojuego    |
| precio  | u64    | Precio en lamports       |

## Red

Desplegado en **Solana Devnet**

## Tecnologías

- Rust
- Anchor Framework
- Solana Devnet

