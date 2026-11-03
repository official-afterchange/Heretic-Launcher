---
title: Routes API Fabric (Loader)
description: Routes de l'API Fabric Meta pour récupérer le loader Fabric, son profil de lancement et ses bibliothèques pour une version de Minecraft.
---

[Fabric](https://fabricmc.net) est un mod loader. Pour lancer Minecraft avec Fabric, le launcher doit récupérer un **profil de lancement** qui vient se greffer sur la version vanilla : classe principale, arguments et bibliothèques supplémentaires. Ces informations sont fournies par l'API **Fabric Meta**.

:::note
Comme pour les routes Mojang, ces routes sont **publiques** et ne nécessitent aucune authentification.
:::

## Vue d'ensemble

```text
Liste des loaders pour la version du jeu
        │  (on choisit un loader stable)
        ▼
Profil de lancement (profile/json)
        ├── inheritsFrom  → version vanilla à fusionner
        ├── mainClass     → classe principale de Fabric
        ├── arguments     → arguments supplémentaires
        └── libraries[]   → bibliothèques Fabric (Maven)
```

## Domaines à autoriser

| Domaine | Rôle |
|---|---|
| `meta.fabricmc.net` | API Fabric Meta (versions de loader, profil de lancement) |
| `maven.fabricmc.net` | Téléchargement des bibliothèques Fabric (JAR) |

## 1. Lister les loaders disponibles

```text
https://meta.fabricmc.net/v2/versions/loader/{version_du_jeu}
```

Exemple pour Minecraft 26.2 :

```text
https://meta.fabricmc.net/v2/versions/loader/26.2
```

La réponse est un tableau, du plus récent au plus ancien. Chaque entrée contient un objet `loader` :

```json
"loader": {
    "separator": ".",
    "build": 1,
    "maven": "net.fabricmc:fabric-loader:0.20.1",
    "version": "0.20.1",
    "stable": true
}
```

| Champ | Description |
|---|---|
| `version` | Version du loader (ex : `0.20.1`), à réutiliser à l'étape suivante |
| `stable` | `true` pour une version stable, `false` pour une préversion |
| `maven` | Coordonnées Maven du loader |
| `build` | Numéro de build |
| `separator` | Séparateur entre le nom et le numéro de build |

### Choisir le bon loader

Pour obtenir la dernière version stable, il suffit de **parcourir le tableau et de prendre la première entrée où `loader.stable` vaut `true`**.

:::caution
Les valeurs `26.2` et `0.20.1` sont des exemples. Ne les codez pas en dur : la version du loader évolue régulièrement, il faut toujours l'interroger.
:::

Si le tableau est vide (ou si l'API renvoie une erreur), Fabric ne prend pas encore en charge cette version du jeu : le launcher doit afficher un message clair à l'utilisateur.

## 2. Récupérer le profil de lancement

```text
https://meta.fabricmc.net/v2/versions/loader/{version_du_jeu}/{version_du_loader}/profile/json
```

Exemple :

```text
https://meta.fabricmc.net/v2/versions/loader/26.2/0.20.1/profile/json
```

La réponse est un JSON au même format qu'un JSON de version Mojang, mais **incomplet** : il ne décrit que ce que Fabric ajoute à la version vanilla.

| Champ | Contenu |
|---|---|
| `id` | Identifiant du profil (ex : `fabric-loader-0.20.1-26.2`) |
| `inheritsFrom` | Version vanilla dont le profil hérite (ex : `26.2`) |
| `mainClass` | Classe principale de Fabric, qui **remplace** celle de Mojang |
| `arguments` | Arguments JVM et/ou de jeu supplémentaires |
| `libraries[]` | Bibliothèques Fabric à ajouter au classpath |

## 3. Fusionner avec la version vanilla

Le profil Fabric ne remplace pas le JSON de version Mojang, il le **complète**. Le launcher doit donc :

1. récupérer le JSON de version vanilla (voir [Routes et Domaines Mojang](/Heretic-Launcher/fr/api/mojang)) ;
2. récupérer le profil Fabric ;
3. fusionner les deux :
   * `libraries` : on **ajoute** les bibliothèques Fabric à celles de Mojang ;
   * `arguments` : on **ajoute** les arguments Fabric à ceux de Mojang ;
   * `mainClass` : on **utilise celle de Fabric**.

Le `client.jar`, les assets et la version de Java restent ceux de la version vanilla.

## 4. Télécharger les bibliothèques

Contrairement à Mojang, les entrées de `libraries[]` ne fournissent pas toujours une URL complète : elles donnent des **coordonnées Maven** et l'adresse du dépôt.

```json
{
  "name": "net.fabricmc:fabric-loader:0.20.1",
  "url": "https://maven.fabricmc.net/"
}
```

Il faut reconstruire l'URL du fichier à partir des coordonnées `groupe:artefact:version` :

```text
{url}{groupe avec des /}/{artefact}/{version}/{artefact}-{version}.jar
```

Ce qui donne pour l'exemple ci-dessus :

```text
https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.20.1/fabric-loader-0.20.1.jar
```

Enregistrez ensuite le fichier dans `libraries/` en suivant le même chemin (`net/fabricmc/fabric-loader/0.20.1/fabric-loader-0.20.1.jar`), comme pour les bibliothèques Mojang.

:::tip
Certaines entrées peuvent aussi fournir un `sha1` ou une taille. Quand ils sont présents, utilisez-les pour vérifier l'intégrité du fichier téléchargé.
:::

## Bonnes pratiques

* **Mettez en cache** la réponse de l'API pour éviter de l'interroger à chaque lancement.
* **Laissez l'utilisateur choisir** entre un loader stable (par défaut) et une préversion.
* **Gérez l'absence de loader** pour une version du jeu non prise en charge.
* **Parallélisez** le téléchargement des bibliothèques et réessayez en cas d'erreur réseau.

:::note
Cette page ne couvre que le **loader**. La **Fabric API**, qui est un mod et non le loader, se télécharge séparément, par exemple via Modrinth ou CurseForge.
:::