---
title: Build
description: Générer les exécutables de Heretic Launcher
---

Heretic Launcher se compile avec la CLI de Tauri. Elle produit un exécutable et des installeurs pour le système sur lequel tu lances la commande.

## Prérequis

- Node.js et npm
- Rust (via [rustup](https://rustup.rs))
- Les dépendances système de Tauri v2 pour ton OS :
  - **Windows** : Microsoft C++ Build Tools et WebView2 (déjà présent sur Windows 11)
  - **macOS** : Xcode Command Line Tools
  - **Linux** : WebKitGTK et les bibliothèques de développement associées (voir le guide de prérequis de Tauri pour ta distribution)

## Lancer le build

Depuis la racine du projet :

```bash
npm install
npm run build
```

Le build compile le frontend, compile le backend Rust en mode release, puis génère les installeurs.

## Résultats

Les fichiers sont créés dans `src-tauri/target/release/bundle/`, dans un sous-dossier par format :

| Système | Formats typiques |
| --- | --- |
| Windows | `.msi`, `.exe` (installeur NSIS) |
| macOS | `.app`, `.dmg` |
| Linux | `.deb`, `.rpm`, `.AppImage` |

L'exécutable seul se trouve dans `src-tauri/target/release/`.

## Un build par système

Tauri ne fait pas de compilation croisée fiable entre systèmes : pour obtenir l'exécutable Windows, il faut compiler sous Windows, et ainsi de suite pour macOS et Linux. Pour produire les trois, le plus pratique est une GitHub Action qui lance le build sur trois machines, avec `tauri-apps/tauri-action`.

## Builds de développement

- `npm run tauri dev` lance le launcher en mode développement, avec rechargement à chaud du frontend.
- `npm run tauri build -- --debug` produit un build avec les symboles de débogage, plus rapide à compiler.

## Signature

Les exécutables non signés déclenchent des avertissements chez le joueur (SmartScreen sous Windows, Gatekeeper sous macOS). La signature de code est optionnelle pour tester, mais recommandée avant de distribuer le launcher.