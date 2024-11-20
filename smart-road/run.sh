#!/bin/bash

# Met à jour le système et installe les dépendances nécessaires
sudo apt update
sudo apt install -y libsdl2-dev libsdl2-image-dev

# Installe Rust si ce n'est pas déjà fait
if ! command -v cargo &> /dev/null
then
    echo "Rust n'est pas installé. Installation de Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Vérifie la présence de Cargo.toml
if [ ! -f "Cargo.toml" ]; then
    echo "Cargo.toml introuvable dans le répertoire actuel."
    exit 1
fi

# Installe les dépendances du projet
cargo build

# Lance le programme
cargo run
