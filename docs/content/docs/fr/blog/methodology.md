---
title: Méthodologie
date: 2026-10-26
tags:
  - Methodology
---

# DevOps

Cette page décrit la façon dont nous organisons le développement, la livraison et l'exploitation du projet. L'objectif du DevOps ici est simple : **livrer souvent, en petites étapes, avec un minimum de risques**, en automatisant tout ce qui est répétitif.

## Principes

* **Tout est versionné** : code, documentation, configuration et scripts de build vivent dans le dépôt Git.
* **Tout ce qui est répétitif est automatisé** : tests, builds, publication et déploiement ne se font pas à la main.
* **Petites modifications, livrées souvent** : une petite modification est plus facile à relire, à tester et à annuler.
* **La `main` est toujours saine** : elle doit pouvoir être compilée et publiée à tout moment.
* **Les secrets n'entrent jamais dans le dépôt** : clés, tokens et mots de passe sont gérés via les secrets du CI/CD ou des variables d'environnement.

## Cycle de développement

```text
Idée / ticket
     │
     ▼
Branche de travail ──► Pull Request ──► Revue + CI ──► Merge dans main
                                                            │
                                                            ▼
                                              Build des exécutables ──► Release
```

### 1. Planification

Chaque tâche est décrite dans une *issue* : contexte, comportement attendu et critères de validation. Une issue = un objectif clair et réalisable en peu de temps.

### 2. Gestion des branches

Nous utilisons un flux simple basé sur des branches courtes :

| Branche | Rôle |
|---|---|
| `main` | Code stable, prêt à être publié |
| `feat/<nom>` | Nouvelle fonctionnalité |
| `fix/<nom>` | Correction de bug |
| `docs/<nom>` | Modification de la documentation |
| `chore/<nom>` | Maintenance, dépendances, outillage |

:::tip
On ne pousse jamais directement sur `main` : tout passe par une Pull Request.
:::

### 3. Convention de commits

Les messages suivent la convention [Conventional Commits](https://www.conventionalcommits.org/fr/) :

```text
feat: ajout de la gestion multi-instance
fix: correction du chemin des assets sous Windows
docs: ajout de la page Méthodologie
chore: mise à jour des dépendances
```

Cela rend l'historique lisible et permet de générer automatiquement les notes de version.

### 4. Pull Requests et revue de code

Une Pull Request est mergée uniquement si :

* la CI est au vert (compilation, lint, tests) ;
* au moins une personne l'a relue ;
* la description explique **ce qui change et pourquoi** ;
* la documentation est mise à jour si le comportement change.

## Intégration continue (CI)

À chaque Pull Request et à chaque push sur `main`, la CI exécute automatiquement :

1. **Installation** des dépendances (avec cache pour gagner du temps).
2. **Lint et formatage** du code (TypeScript, Rust).
3. **Tests** unitaires.
4. **Build** de l'application.
5. **Build de la documentation** pour détecter les liens cassés.

Comme l'application est multiplateforme, le build est exécuté sur une **matrice d'OS** :

| Plateforme | Cible |
|---|---|
| Windows | `x64` |
| Linux | `x64` |
| macOS | `x64` (Intel) et `aarch64` (Apple Silicon) |

## Livraison continue (CD)

La publication d'une version est déclenchée par un **tag Git** suivant le versionnage sémantique (`vMAJEUR.MINEUR.CORRECTIF`).

```text
git tag v0.2.0
git push origin v0.2.0
```

Le pipeline de release :

1. compile les exécutables pour chaque plateforme ;
2. calcule les empreintes (`sha256`) des fichiers produits ;
3. crée une *release* GitHub avec les binaires et les notes de version générées depuis les commits.

### Versionnage sémantique

| Incrément | Quand ? | Exemple |
|---|---|---|
| `MAJEUR` | Changement incompatible | `1.0.0` → `2.0.0` |
| `MINEUR` | Nouvelle fonctionnalité compatible | `1.0.0` → `1.1.0` |
| `CORRECTIF` | Correction de bug | `1.0.0` → `1.0.1` |

Tant que le projet est en `0.x`, l'API et le comportement peuvent encore évoluer.

## Gestion de la configuration et des secrets

* La configuration par environnement (développement, production) passe par des **variables d'environnement**.
* Les secrets (tokens, clés de signature, identifiants de base de données) sont stockés dans les **secrets du dépôt** et ne sont jamais écrits en clair dans le code, les logs ou les issues.
* Un fichier `.env.example` documente les variables attendues, sans leurs valeurs.

## Qualité et sécurité

* **Dépendances** : mises à jour régulières, avec une alerte automatique en cas de vulnérabilité connue (Dependabot ou équivalent).
* **Vérification d'intégrité** : les fichiers téléchargés par l'application (bibliothèques, assets, Java) sont toujours vérifiés par leur `sha1`.
* **Principe du moindre privilège** : chaque service et chaque jeton n'a accès qu'à ce dont il a besoin.

## Observabilité

* **Logs** structurés et lisibles, sans données sensibles (jamais de mots de passe ni de tokens).
* **Remontée d'erreurs** claire côté utilisateur, avec un identifiant ou un fichier de log facile à joindre à une issue.
* **Surveillance** des services en ligne (disponibilité et temps de réponse) lorsqu'ils sont en production.

## Documentation

La documentation est traitée **comme du code** :

* elle vit dans le même dépôt et suit le même processus de revue ;
* elle est publiée automatiquement à chaque merge sur `main` ;
* elle est maintenue en **français et en anglais**.

:::note
Une fonctionnalité n'est considérée comme terminée que lorsqu'elle est testée, documentée et mergée.
:::

## Définition de « terminé »

Une tâche est terminée lorsque :

- [ ] le code est relu et mergé sur `main` ;
- [ ] la CI est au vert sur toutes les plateformes ;
- [ ] la documentation est à jour ;
- [ ] aucun secret ni donnée sensible n'a été ajouté au dépôt ;
- [ ] la fonctionnalité a été testée manuellement dans un cas réel.