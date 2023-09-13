🦑 ink-examples
Curso 2023 Introduccion a Ink !

Clase 1:
Introducción a Polkadot, Substrate e ink!
Configuración de entorno para Rust e ink!
(Flipper)

Clase 2:
Herramientas de Rust para reutilizar
Tipos y funciones nativas de ink!
Mensajes (query/tx)
Introducción a Storage
Implementación de contratos
(voting) 

Clase 3:
Storage avanzado
Eventos
OpenBrush & Swanky
(voting_mapping) y (voting_mappingII)

Clase 4:
Llamadas Cross-Contract
Traits con OpenBrush
(CrossContract ,  openbrush::psp34 (MInteable))


Clase 5:
Chain Extensions

Clase 6:
Tests unitarios
Tests de integración
Tests E2E

Clase 7:
Métodos de Actualización de contratos
Verificación de contratos on-chain

Clase 8:
Repaso de Librería Polkadot.js
Useink (React hooks)


Rust & Cargo
Web: https://www.rust-lang.org/tools/install
Acción: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Ink!
Webs:
https://use.ink/getting-started/setup/
https://github.com/paritytech/cargo-contract
Acciones:
rustup install 1.72
rustup default 1.72
rustup component add rust-src --toolchain 1.72
rustup target add wasm32-unknown-unknown --toolchain 1.72
cargo install --force --version 3.2.0 cargo-contract
Node
Webs:
https://github.com/paritytech/substrate-contracts-node
https://github.com/paritytech/substrate-contracts-node/releases
Acciones:
Descargar substrate-contracts-node-linux.tar.gz
Colocar substrate-contracts-node en una carpeta del PATH del sistema
Proyecto
Software utilizado:
Software	Versión
rustup	1.26.0 (5af9b9484 2023-04-05)
rustc	1.72.0 (5680fa18f 2023-08-23)
cargo	1.72.0 (103a7ff2e 2023-08-15)
cargo-contract	3.2.0-unknown-x86_64-unknown-linux-gnu
substrate-contracts-node	0.31.0-c8863fe08b7

