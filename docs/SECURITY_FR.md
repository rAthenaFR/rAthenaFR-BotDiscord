# Sécurité

## Principes

- Toutes les commandes SQL de consultation sont en lecture seule.
- `/createaccount` est la seule commande conservée qui peut écrire en base, uniquement si elle est activée.
- Les commandes staff utilisent des réponses éphémères pour les données sensibles.
- Les mots de passe, hashes, e-mails privés et IP complètes ne doivent pas être affichés.
- `/gmmsg` ne passe pas par SQL et n’exécute jamais de shell.

!!! danger "Données sensibles"
    Ne publie jamais de mot de passe, hash, e-mail privé, IP complète, token Discord ou contenu réel de `.env` dans Discord, dans les logs publics ou dans Git.

## Rôles

Configure des rôles dédiés :

```env
RATHENAFR_HELPER_ROLE_IDS=
RATHENAFR_MODERATOR_ROLE_IDS=
RATHENAFR_GM_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Laisse une variable vide pour refuser ce niveau d’accès.

## SQL

Permission normale :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Permission optionnelle pour `/createaccount` :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

!!! warning "Droits à ne pas accorder"
    Ne donne pas `UPDATE`, `DELETE`, `DROP`, `ALTER` ou `CREATE` au bot pour cette version.

## `/gmmsg`

`/gmmsg` limite la longueur du message, nettoie les caractères de contrôle et neutralise `@everyone`/`@here` dans les logs Discord.

!!! tip "Journalisation staff"
    Configure `RATHENAFR_STAFF_LOG_CHANNEL_ID` si tu veux tracer les utilisations de `/gmmsg` dans un salon staff.
