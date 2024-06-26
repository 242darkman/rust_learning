# Entrainement

Vous êtes chargé de créer un programme CLI en Rust pour gérer les employés d'une entreprise. Le programme devrait permettre à l'utilisateur d'ajouter de nouveaux employés, de les afficher, de les mettre à jour et même de les licencier. Utilisez un vecteur pour stocker les employés, et utilisez des structures de contrôle comme match et des boucles pour gérer les interactions.

## **Fonctionnalités**

- Créer une struct Employee avec les champs suivants :
    - id : l'identifiant unique de l'employé.
    - name : le nom complet de l'employé.
    - status : le statut de l'employé (actif, en congé, licencié, etc.).
- Créer une enum Status pour représenter les différents statuts possibles de l'employé (actif, en congé, licencié, etc.).
- Implémenter des fonctions pour :
    - Ajouter un nouvel employé à la “base de données”.
    - Afficher tous les employés existants.
    - Mettre à jour les détails d'un employé existant.
    - Licencier un employé de l'entreprise.
- Créer un menu CLI interactif permettant à l'utilisateur d'exécuter les différentes fonctionnalités du programme.