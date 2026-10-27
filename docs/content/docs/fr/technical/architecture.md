---
title: Architecture
description: Vue d'ensemble technique de Heretic Launcher
---

Heretic Launcher est un launcher Minecraft personnalisé, construit avec **Tauri v2**. Chaque launcher est relié à **un seul serveur**. Il n'y a pas de gestion multi-instance.

## Vue d'ensemble

```text
┌──────────────────────────────────────────────┐
│               Heretic Launcher               │
│                                              │
│  ┌────────────────┐      ┌────────────────┐  │
│  │ Frontend       │      │ Backend        │  │
│  │ React + TS     │◄────►│ Rust (Tauri)   │  │
│  │                │ IPC  │                │  │
│  └────────────────┘      └───────┬────────┘  │
└──────────────────────────────────┼───────────┘
                                   │ lance
                                   ▼
                           ┌──────────────┐      ┌─────────────┐
                           │ Processus    │─────►│ Serveur     │
                           │ Java         │      │ Minecraft   │
                           │ (Minecraft)  │      └─────────────┘
                           └──────────────┘
```

## Les couches

### Frontend (React + TypeScript)

L'interface affiche quatre pages :

| Page | Rôle |
| --- | --- |
| Home | Lancer le jeu et voir les informations du serveur auquel appartient le launcher |
| Account | Gérer le compte du joueur |
| Info | Informations sur le développeur |
| Settings | Paramètres du launcher |

Le frontend ne contient aucune logique sensible. Il appelle le backend par les commandes Tauri et affiche l'état renvoyé (progression, erreurs, statut du jeu).

### Backend (Rust)

Le backend s'occupe de tout ce qui touche au système :

- la **vérification et le téléchargement** des fichiers du jeu (client, bibliothèques, ressources) ;
- la **détection de Java** et le choix de la version adaptée ;
- la **construction de la commande Java** et l'injection des arguments JVM ;
- le **lancement et le suivi du processus** Minecraft, avec transmission des logs au frontend ;
- la **lecture et l'écriture de la configuration** du launcher (mémoire allouée, chemin de Java, langue).

## Cycle de lancement du jeu

1. Le joueur clique sur **Jouer** dans la page Home.
2. Le frontend appelle la commande Tauri de lancement.
3. Le backend vérifie que les fichiers du jeu sont présents et à jour, et télécharge ceux qui manquent.
4. Il localise Java et assemble la commande de lancement avec les arguments JVM.
5. Il démarre le processus Minecraft et envoie son état au frontend.
6. À la fermeture du jeu, le backend libère le processus et remet l'interface à l'état initial.

## Structure du dépôt

```text
heretic-launcher/
├── src/                  ← frontend React
│   ├── pages/            ← Home, Account, Info, Settings
│   └── components/       ← composants partagés
├── src-tauri/            ← backend Rust
│   ├── src/
│   │   ├── main.rs       ← point d'entrée
│   │   ├── commands.rs   ← commandes exposées au frontend
│   │   ├── launcher.rs   ← construction de la commande et lancement
│   │   ├── download.rs   ← téléchargement des fichiers du jeu
│   │   └── config.rs     ← configuration du launcher
│   ├── Cargo.toml
│   └── tauri.conf.json
└── docs/                 ← cette documentation
```

## Choix techniques

- **Tauri v2 plutôt qu'Electron** : exécutable beaucoup plus léger, puisqu'il utilise le moteur web du système au lieu d'embarquer Chromium, et backend en Rust.
- **Rust pour le backend** : gestion fiable des processus et des fichiers, sans runtime supplémentaire à installer chez le joueur.
- **React + TypeScript pour le frontend** : composants réutilisables et typage des échanges avec le backend.
- **Un seul serveur par launcher** : pas de gestion de profils ou d'instances, ce qui simplifie l'interface et la configuration.