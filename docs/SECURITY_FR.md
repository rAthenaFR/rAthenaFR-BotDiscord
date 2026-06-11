# Sécurité

## Modèle de confiance

Le bot reçoit des interactions Discord, lit une base rAthena et peut effectuer trois familles d’écriture explicitement activées :

- création de compte ;
- gestion staff de compte ;
- insertion GMMSG dans une file SQL.

Tout le reste doit rester en lecture seule.

> [!CAUTION]
> Ne publie jamais token Discord, mot de passe SQL, hash, PIN, e-mail privé, IP complète ou contenu réel de `.env`.

## Secrets

- Garde `.env` hors Git.
- Utilise un fichier protégé ou un gestionnaire de secrets en production.
- Renouvelle immédiatement un token ou mot de passe exposé.
- N’inclus pas de secrets dans `RUST_LOG`.
- Évite de passer les mots de passe en argument de commande, car ils peuvent apparaître dans l’historique ou la liste des processus.

## Rôles Discord

Configure des rôles dédiés :

```env
RATHENAFR_HELPER_ROLE_IDS=
RATHENAFR_MODERATOR_ROLE_IDS=
RATHENAFR_GM_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Les listes vides ne donnent aucun accès. Les rôles supérieurs sont cumulativement autorisés.

> [!IMPORTANT]
> La permission est contrôlée dans le handler à chaque interaction. Le nom d’une commande ou sa visibilité dans Discord ne constitue pas une protection.

## Réponses et données privées

- Les commandes staff sensibles répondent en éphémère.
- Les IP issues des logs sont masquées.
- Les comptes n’exposent pas `user_pass`, hash, PIN ou e-mail privé.
- Les mentions `@everyone` et `@here` sont neutralisées dans les logs GMMSG.
- Les messages d’erreur SQL visibles sont simplifiés.

## Droits SQL

Base recommandée :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Ajoute seulement les droits nécessaires :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'%';
```

> [!WARNING]
> Les scripts combinés peuvent accorder plusieurs droits optionnels. Relis-les et retire les droits correspondant aux fonctions désactivées.

Interdictions recommandées pour le compte du bot :

- `DELETE`
- `DROP`
- `ALTER`
- `CREATE`
- privilèges globaux

## Gestion de compte

`/staff account-manage delete` est une désactivation forte, pas un `DELETE SQL`. Elle exige un rôle Owner par défaut, une option dédiée et la confirmation `SUPPRIMER`.

`/createaccount` fait transiter le mot de passe dans une interaction Discord.

> [!CAUTION]
> N’active pas la création publique sans informer les utilisateurs du canal de transmission et sans vérifier les règles de conservation de Discord applicables à ton serveur.

## GMMSG

Les messages peuvent être conservés dans :

- l’interaction Discord ;
- le salon de logs staff ;
- `discord_gmmsg_queue` ;
- les logs rAthena ;
- le client en jeu.

N’utilise jamais GMMSG pour des secrets ou données personnelles.

## Docker et réseau

- Le conteneur s’exécute sans root.
- Le système de fichiers est en lecture seule.
- Aucun port entrant n’est publié.
- MariaDB doit rester sur un réseau privé.
- Les images et dépendances doivent être régulièrement reconstruites.

> [!TIP]
> Vérifie périodiquement les droits avec `SHOW GRANTS FOR 'rathenafr_bot'@'%';` et compare-les aux fonctionnalités réellement activées.

## Signalement

Pour une vulnérabilité, suis la procédure de `SECURITY.md` à la racine plutôt que d’ouvrir une issue publique avec des détails exploitables.
