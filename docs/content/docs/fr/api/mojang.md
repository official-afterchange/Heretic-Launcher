---
title: Routes et Domaines Mojang (Assets)
description: Domaines et routes utilisés par un launcher pour télécharger les versions, les bibliothèques, le client et les assets de Minecraft.
---

Un launcher personnalisé a besoin d'accéder aux serveurs de contenu officiels de Mojang pour télécharger les fichiers nécessaires au lancement du jeu : `client.jar`, bibliothèques Java, index et ressources (sons, textures, langues).

:::note
Le téléchargement de ces fichiers ne nécessite **aucune authentification**. Ces routes sont publiques et accessibles même si l'authentification Microsoft est ignorée ou simulée dans votre launcher.
:::

## Vue d'ensemble

Le téléchargement d'une version se fait en cascade : chaque étape fournit les URL de la suivante.

```text
Manifeste des versions
        │  (url de la version choisie)
        ▼
JSON de la version
        ├── downloads.client.url  → client.jar
        ├── libraries[]           → bibliothèques Java
        ├── assetIndex.url        → index des assets
        │                               │  (hash de chaque fichier)
        │                               ▼
        │                          fichiers d'assets
        └── javaVersion           → version de Java requise
```

## Domaines à autoriser

Si vous filtrez le réseau (pare-feu, proxy, CSP de votre application), autorisez ces domaines :

| Domaine | Rôle |
|---|---|
| `piston-meta.mojang.com` | Manifeste des versions, JSON de version, index des assets |
| `piston-data.mojang.com` | `client.jar` et autres fichiers de la version |
| `libraries.minecraft.net` | Bibliothèques Java (`libraries`) |
| `resources.download.minecraft.net` | Fichiers d'assets (sons, textures, langues) |
| `launchermeta.mojang.com` | Manifeste des runtimes Java fournis par Mojang (optionnel) |

## 1. Manifeste des versions (Version Manifest)

```text
https://piston-meta.mojang.com/mc/game/version_manifest_v2.json
```

Ce fichier liste toutes les versions disponibles dans le tableau `versions`. Cherchez l'entrée dont l'`id` correspond à la version voulue (ex : `26.2`) et récupérez son champ `url`.

```json
{
  "id": "26.2",
  "type": "release",
  "url": "https://piston-meta.mojang.com/v1/packages/<sha1>/26.2.json",
  "sha1": "<sha1>"
}
```

## 2. JSON de la version

C'est l'URL récupérée à l'étape précédente, par exemple :

```text
https://piston-meta.mojang.com/v1/packages/ea228cdd1379853393fd73ad0a562c3f5eb7fd9d/26.2.json
```

:::caution
Ne codez pas cette URL en dur : le hash change si Mojang republie le fichier. Récupérez-la toujours depuis le manifeste.
:::

Ce JSON est le fichier central. Il contient tout ce qu'il faut pour télécharger et lancer le jeu :

| Champ | Contenu |
|---|---|
| `downloads.client` | URL, `sha1` et taille du `client.jar` |
| `libraries[]` | Liste des bibliothèques à télécharger |
| `assetIndex` | `id`, `sha1` et `url` de l'index des assets |
| `javaVersion` | Version majeure de Java requise (`component` et `majorVersion`) |
| `arguments` | Arguments JVM et arguments du jeu |
| `mainClass` | Classe principale à lancer |
| `logging` | Configuration des logs (optionnel) |

## 3. Client du jeu (`client.jar`)

L'URL se trouve dans `downloads.client.url` du JSON de version :

```text
https://piston-data.mojang.com/v1/objects/<sha1>/client.jar
```

## 4. Bibliothèques (`libraries`)

Chaque entrée de `libraries[]` fournit directement son URL :

```json
{
  "name": "group:artifact:version",
  "downloads": {
    "artifact": {
      "path": "group/artifact/version/artifact-version.jar",
      "url": "https://libraries.minecraft.net/group/artifact/version/artifact-version.jar",
      "sha1": "<sha1>",
      "size": 12345
    }
  },
  "rules": [
    { "action": "allow", "os": { "name": "windows" } }
  ]
}
```

* **Règles (`rules`)** : certaines bibliothèques ne concernent qu'un OS ou une architecture. Évaluez ces règles et ne téléchargez que celles qui s'appliquent à la machine de l'utilisateur.
* **Natives** : dans les versions récentes, les bibliothèques natives sont des entrées normales de `libraries[]`, filtrées par `rules`.
* **Stockage** : enregistrez chaque fichier selon son `path`, dans `libraries/`.

## 5. Assets (sons, textures, langues)

### Index des assets

L'URL est dans `assetIndex.url` du JSON de version. Le fichier contient un objet `objects` qui associe chaque nom de ressource à son hash :

```json
{
  "objects": {
    "minecraft/lang/fr_fr.json": {
      "hash": "ab12cd34...",
      "size": 123456
    }
  }
}
```

### Téléchargement d'un asset

```text
https://resources.download.minecraft.net/{2 premiers caractères du hash}/{hash}
```

Exemple pour le hash `ab12cd34...` :

```text
https://resources.download.minecraft.net/ab/ab12cd34...
```

### Organisation sur le disque

Reproduisez la structure du launcher officiel :

```text
assets/
├── indexes/
│   └── {assetIndex.id}.json
└── objects/
    └── ab/
        └── ab12cd34...
```

## 6. Java (optionnel)

Le champ `javaVersion.majorVersion` indique la version de Java requise (Java 25 pour Minecraft 26.2). Deux options :

* **Adoptium (Temurin)** : voir la page [Routes et Paramètres de l'API Adoptium](/Heretic-Launcher/en/api/adoptium).
* **Runtime fourni par Mojang** : via le manifeste des runtimes.

```text
https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json
```

## Bonnes pratiques

* **Vérifiez l'intégrité** : comparez le `sha1` de chaque fichier téléchargé avec celui fourni par l'API.
* **Évitez les retéléchargements** : ne téléchargez que les fichiers absents ou dont le hash diffère.
* **Parallélisez** : un index d'assets contient plusieurs milliers de fichiers, un téléchargement séquentiel serait très lent.
* **Gérez les erreurs réseau** : prévoyez des tentatives multiples (retry) pour les assets.