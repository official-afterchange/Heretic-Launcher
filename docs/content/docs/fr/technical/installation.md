---
title: Installation
description: Mettre en place l'environnement de développement de Heretic Launcher
---

Cette page explique comment cloner le projet et le lancer en mode développement. Pour générer les exécutables, voir la page [Build](/fr/technique/build/).

## Prérequis

- **Node.js** et npm
- **Rust**, installé avec [rustup](https://rustup.rs)
- Les **dépendances système de Tauri v2** pour ton OS (voir la page [Build](/fr/technique/build/) pour le détail)

Vérifie les installations :

```bash
node --version
npm --version
rustc --version
cargo --version
```

## Récupérer le projet

```bash
git clone https://github.com/official-afterchange/heretic-launcher.git
cd heretic-launcher
npm install
```

La première compilation de Rust télécharge et compile toutes les dépendances du backend : elle peut prendre plusieurs minutes. Les suivantes sont beaucoup plus rapides.

## Lancer le launcher en développement

```bash
npm run dev
```

Cette commande démarre le serveur de développement du frontend (Vite) et ouvre le launcher dans sa fenêtre Tauri. Les modifications du frontend sont appliquées à chaud. Les modifications du code Rust déclenchent une recompilation et un redémarrage de l'application.

## Lancer la documentation

La documentation est un site Starlight dans le dossier `docs/`. Pour la visualiser en local :

```bash
npm run docs:dev
```

Elle est alors disponible sur `http://localhost:4321`.

## Problèmes courants

- **La fenêtre ne s'ouvre pas ou Rust ne compile pas** : vérifie que les dépendances système de Tauri v2 sont installées pour ton OS.
- **`cargo` introuvable** : ferme et rouvre le terminal après l'installation de Rust pour que le `PATH` soit mis à jour.
- **Le port du serveur de développement est déjà utilisé** : ferme l'autre instance du launcher ou de Vite qui tourne encore.