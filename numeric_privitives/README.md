# Calcul de la Moyenne
## Description

Ce programme en Rust calcule la moyenne de quatre valeurs, deux entiers (`i32`) et deux nombres à virgule flottante (`f64`). Il montre comment gérer les conversions entre différents types de données et comment effectuer des opérations arithmétiques simples en Rust.

## Structure du Programme
Le programme est composé de deux fonctions principales :

1. `calculate_average` : Cette fonction prend quatre arguments (deux i32 et deux f64), calcule leur somme, et divise par le nombre de valeurs pour obtenir la moyenne. Elle gère également la conversion des types de données (i32 en f64) pour assurer une précision dans le calcul.

2. `main` : Cette fonction déclare les variables, appelle la fonction calculate_average pour calculer la moyenne des valeurs, et affiche le résultat.

## Fonctionnement
Déclaration des Variables : Dans la fonction main, quatre variables sont déclarées : deux entiers (`a` et `c`) et deux nombres à virgule flottante (`b` et `d`).

1. **Calcul de la Moyenne** : La fonction `calculate_average` est appelée avec les quatre variables en tant qu'arguments. Cette fonction :
Convertit les valeurs i32 en f64.

2. **Calcule la somme des valeurs**
Divise la somme par le nombre de valeurs pour obtenir la moyenne.
Gère le cas de division par zéro (bien que ce cas ne se produise pas dans ce programme spécifique).

3. **Affichage du Résultat** : La moyenne calculée est affichée à l'écran.


## Compilation et Exécution
Pour compiler le programme, exécutez la commande suivante :
```bash
cargo run
```