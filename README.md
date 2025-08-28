# 🔄 CLI Converter

> **Un outil de conversion d'images puissant et intuitif en ligne de commande**

## 📋 Description

**CLI Converter** est un outil de conversion d'images développé en Rust qui permet de convertir facilement vos fichiers entre différents formats populaires. Avec son interface de menu interactive et intuitive, convertissez vos images en quelques touches de clavier ! 🚀

## ✨ Fonctionnalités

- 🖼️ **JPG vers PNG** - Conversion sans perte de qualité
- 📸 **PNG vers JPG** - Optimisation de taille avec compression
- 🎯 **JPG vers ICO** - Création d'icônes à partir de JPEG
- 🔧 **PNG vers ICO** - Génération d'icônes depuis PNG
- ⚡ **Interface interactive** - Menu de navigation avec les flèches
- 🎨 **Interface colorée** - Affichage avec mise en évidence
- 🔒 **Sécurisé** - Gestion d'erreurs robuste
- ✈ **Connection** - Utilisable même en mode avion

## 🚀 Installation

### Prérequis

- [Rust](https://rustup.rs/) (version 1.70 ou supérieure)
- Cargo (inclus avec Rust)

### Compilation depuis les sources

```bash
# Cloner le repository
git clone https://github.com/votre-username/cli_converter.git
cd cli_converter

# Compiler le projet
cargo build --release

# L'exécutable sera disponible dans ./target/release/cli_converter
```

### Installation directe

```bash
# Installation via Cargo (si publié sur crates.io)
cargo install cli_converter
```

## 🎮 Utilisation

Lancez simplement l'outil depuis votre terminal :

```bash
./cli_converter
# ou si installé globalement
cli_converter
```

### Interface Interactive

```
Converter Management Tool
--------------------------
> jpg2png
  png2jpg
  jpg2ico
  png2ico
  Exit
```

### Contrôles

- **⬆️ Flèche Haut** : Naviguer vers le haut
- **⬇️ Flèche Bas** : Naviguer vers le bas  
- **⏎ Entrée** : Sélectionner l'option
- **Esc** ou **Ctrl+C** : Quitter l'application

## 📁 Structure du Projet

```
cli_converter/
├── src/
│   ├── main.rs          # Interface principale et menu
│   ├── jpg2ico.rs       # Conversion JPG → ICO
│   ├── jpg2png.rs       # Conversion JPG → PNG
│   ├── png2jpg.rs       # Conversion PNG → JPG
│   └── png2ico.rs       # Conversion PNG → ICO
├── Cargo.toml           # Configuration Cargo
├── LICENSE              # Licence MIT
└── README.md           # Ce fichier
```

## 🛠️ Dépendances

Le projet utilise les crates suivantes :

- **crossterm** - Interface terminal cross-platform
- **image** - Traitement d'images (probablement)
- **ico** - Support du format ICO (probablement)

## 🤝 Contribution

Les contributions sont les bienvenues ! 🎉

## 🐛 Signalement de Bugs

Si vous rencontrez un problème :

1. Vérifiez que vous utilisez la dernière version
2. Consultez les issues existantes ou mon serveur discord
3. Si le problème persiste, communiquez le sur mon serveur discord

## 📄 Licence

Ce projet est sous licence **MIT** - voir le fichier [LICENSE](LICENSE) pour plus de détails.

--------------------------

## 📞 Contact

-   **Discord** : https://discord.gg/GsARdNqSVh
- 🐙 **GitHub** : https://github.com/Psych3-N1x

## 🙏 Inspiration venu du site : 
https://convertio.co/fr/
---

⭐ **N'hésitez pas à donner une étoile si ce projet vous a été utile !** ⭐
