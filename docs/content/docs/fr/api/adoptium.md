---
title: Routes et Paramètres de l'API Adoptium (Java)
description: Guide détaillé de l'API Eclipse Adoptium pour télécharger automatiquement les runtimes Java (JRE) nécessaires à Minecraft.
---

Pour faire fonctionner Minecraft, ton launcher a besoin d'un environnement d'exécution Java adapté (souvent un JRE). L'API officielle d'**Eclipse Adoptium (Temurin)** permet de récupérer dynamiquement les liens de téléchargement des binaires à jour.

L'endpoint principal utilisé est une requête `GET` sur les versions majeures stabilisées (`ga`).

## Structure de l'URL de base

```text
https://api.adoptium.net/v3/assets/feature_releases/{version}/ga?{parametres}
```

* `{version}` : Le numéro de la version majeur de Java (ex: `25` pour Java 25)

* `/ga` : Signfie *General Availibility* (versions stables prêtes pour la production).

| Paramètre | Valeur example | Description |
|---|---|---|
| architecture | x64  |  L'architecture du processeur cible (`x64` pour les PC standards 64-bits, `aarch64` pour ARM/Apple Silicon). |
| image_type | jre | Le type d'image. On privilégie un JRE (Java Runtime Environment) plutôt qu'un JDK, car il est plus léger et contient uniquement ce qu'il faut pour exécuter le jeu. |
| os | `windows`, `linux`, `max` | Le système d'exploitation cible pour le binaire. |
| page | 0 | La page de pagination des résultats (commence à 0) |
| page_size | 1 | Le nombre de résultats renvoyés par page. On met `1` pour récupérer uniquement le résultat le plus récent correspondant aux critères. |
| project | jdk | Le projet source de la release chez Adoptium (laissé à `jdk` même pour obtenir un JRE). |
| sort_method | DEFAULT | La méthode de tri des résultats. |
| sort_order | DESC | L'ordre de tri (`DESC` pour trier du plus récent au plus ancien, garantissant d'obtenir la dernière mise à jour en première position). |
| vendor | eclipse | Le fournisseur des binaires (ici `eclipse` pour Adoptium Temurin). |

## Exemples par Système d'Exploitation

Voici les URL complètes prêtes à l'emploi selon l'OS de l'utilisateur :

**Windows (64-bits)**
```text
https://api.adoptium.net/v3/assets/feature_releases/25/ga?architecture=x64&image_type=jre&os=windows&page=0&page_size=1&project=jdk&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse
```

**Linux (64-bits)**
```text
https://api.adoptium.net/v3/assets/feature_releases/25/ga?architecture=x64&image_type=jre&os=linux&page=0&page_size=1&project=jdk&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse
```

**macOS (Intel / Apple Silicon selon l'archi)**
```text
https://api.adoptium.net/v3/assets/feature_releases/25/ga?architecture=x64&image_type=jre&os=mac&page=0&page_size=1&project=jdk&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse
```
> **Note pour macOS** : *Pense à adapter le paramètre `architecture` (`aarch64` pour les puces M1/M2/M3 et `x64` pour les anciens Mac Intel)*