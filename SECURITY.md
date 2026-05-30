# Politique de sécurité

## Versions maintenues

Seule la dernière version publiée est maintenue activement.

## Signaler une vulnérabilité

N’ouvre pas d’issue publique pour un secret, une fuite de token, une exposition de compte ou un problème de permissions SQL.

Utilise un canal privé de maintenance ou une alerte de sécurité GitHub.

## Modèle de sécurité

- Le bot est conçu pour un accès SQL `SELECT` uniquement.
- Les commandes staff nécessitent des IDs de rôles Discord.
- Les réponses staff sont éphémères quand Discord le permet.
- Les champs sensibles comme `user_pass`, `email`, `last_ip`, `pincode` et `web_auth_token` ne doivent jamais être affichés.
- L’image Docker s’exécute avec un utilisateur non-root.

## Secrets à ne jamais committer

```text
.env
token Discord
mots de passe SQL
IDs de rôles Discord privés
```

Utilise `.env.example` et `.env.docker.example` uniquement comme modèles.
