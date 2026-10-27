---
title: Frontend
description: Structure et fonctionnement de l'interface de Heretic Launcher
---

L'interface de Heretic Launcher est une application **React** écrite en **TypeScript**. Elle tourne dans la fenêtre web de Tauri et ne contient aucune logique sensible : tout ce qui touche au système passe par le backend Rust.

## Structure

```text
src/
├── pages/
│   ├── home.tsx
│   ├── account.tsx
│   ├── browser.tsx
│   ├── info.tsx
│   └── settings.tsx
├── components/      ← composants partagés
└── main.tsx         ← point d'entrée
```

## Les pages

| Page | Fichier | Rôle |
| --- | --- | --- |
| Home | `home.tsx` | Lancer le jeu et voir les informations du serveur auquel appartient le launcher |
| Account | `account.tsx` | Gérer le compte du joueur |
| Browser | `browser.tsx` | À CORRIGER : navigation dans le contenu du launcher |
| Info | `info.tsx` | Informations sur le développeur |
| Settings | `settings.tsx` | Paramètres du launcher |

## Navigation

Les pages sont affichées dans une seule fenêtre, sans rechargement. Une barre de navigation permet de passer de l'une à l'autre, et la page Home est affichée au démarrage.

## Communication avec le backend

Le frontend appelle les commandes Rust avec la fonction `invoke` de Tauri :

```ts
import { invoke } from '@tauri-apps/api/core';

async function launchGame() {
  try {
    await invoke('launch_game');
  } catch (error) {
    console.error('Échec du lancement :', error);
  }
}
```

Les commandes sont asynchrones : elles renvoient une promesse, et une erreur côté Rust devient un rejet côté TypeScript. Le backend peut aussi envoyer des événements au frontend (progression d'un téléchargement, logs du jeu) que l'interface écoute pour se mettre à jour.

## État de l'interface

L'état est géré avec les hooks de React (`useState`, `useEffect`). Les valeurs partagées entre plusieurs pages, comme le compte connecté ou la configuration, sont centralisées plutôt que dupliquées dans chaque page.