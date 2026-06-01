# Gestion de comptes

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

Cette page couvre les commandes qui écrivent dans la base rAthena.

> [!WARNING]
> Ces commandes sortent du modèle lecture seule historique du bot. Active-les uniquement avec un utilisateur SQL dédié, des rôles Discord maîtrisés et des sauvegardes récentes.

## Commandes concernées

| Commande | Écriture SQL | Accès Discord |
|---|---:|---|
| `/createaccount` | `INSERT` dans `login` | Publique, mais désactivée par défaut |
| `/accountmanage action:edit` | `UPDATE` dans `login` | Staff/admin/owner configurés |
| `/accountmanage action:delete` | `DELETE` sur les tables liées au compte | Staff/admin/owner configurés |

## `/createaccount`

La commande est toujours déclarée dans Discord, mais elle refuse la création tant que la variable suivante reste à `false` :

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=false
```

Le mode de mot de passe doit suivre la configuration du serveur login rAthena :

```env
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
```

Valeurs supportées :

- `plain`
- `md5`

> [!IMPORTANT]
> Le mot de passe transite par Discord au moment de la commande, mais le bot ne le réaffiche jamais dans ses réponses.

## `/accountmanage action:edit`

L’action `edit` permet aux GM de modifier les champs de compte suivants :

- login (`username`) ;
- mot de passe (`password`) ;
- sexe (`sex`) ;
- date de naissance (`birthdate`) ;
- email (`email`) ;
- groupe (`group_id`) ;
- état (`state`) ;
- timestamp de fin de bannissement (`unban_time`) ;
- timestamp d’expiration (`expiration_time`) ;
- nombre de slots personnages (`character_slots`).

Au moins un champ d’édition doit être renseigné.

> [!IMPORTANT]
> Le mot de passe et les valeurs sensibles ne sont pas réaffichés dans la réponse du bot.

> [!WARNING]
> Modifier `group_id`, `state`, `unban_time` ou `expiration_time` a un impact direct sur les accès du compte. Réserve cette action aux rôles GM/staff de confiance.

## `/accountmanage action:delete`

La suppression complète exige :

- un rôle présent dans `RATHENAFR_STAFF_ROLE_IDS`, `RATHENAFR_ADMIN_ROLE_IDS` ou `RATHENAFR_OWNER_ROLE_IDS` ;
- l’action `delete` ;
- une confirmation exacte au format `DELETE-ALL-ID`.

Exemple pour le compte `2000001` :

```text
DELETE-ALL-2000001
```

La suppression est exécutée dans une transaction. Le bot nettoie les lignes liées au compte et aux personnages en s’appuyant sur les colonnes `account_id`, `char_id` et certaines relations connues comme les boutiques et les mails.

> [!CAUTION]
> La suppression est refusée si un personnage du compte possède une guilde. Transfère ou dissous la guilde avant de supprimer le compte.

## Permissions SQL

Pour les commandes de compte, utilise le script dédié :

```bash
mysql -u root -p < sql/create-account-management-user.sql
```

Il ajoute notamment :

```sql
GRANT INSERT, UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT DELETE ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

> [!WARNING]
> Ne donne pas `DROP`, `ALTER` ou `CREATE` au bot. `INSERT` sert à `/createaccount`, `UPDATE` à `/accountmanage action:edit`, et `DELETE` à `/accountmanage action:delete`.

## Checklist avant activation

- Sauvegarde de la base rAthena disponible.
- Rôles staff/admin/owner configurés avec des IDs Discord exacts.
- Utilisateur SQL dédié au bot.
- Permissions SQL vérifiées.
- Commandes Discord redéployées après changement d’options.

> [!TIP]
> Pour une installation strictement lecture seule, laisse `RATHENAFR_ACCOUNT_CREATION_ENABLED=false` et n’applique pas le script `create-account-management-user.sql`.
