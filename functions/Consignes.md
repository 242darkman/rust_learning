# Entrainement

Créer une application en ligne de commande en Rust qui permet à l'utilisateur d'effectuer des opérations arithmétiques simples : addition, soustraction, multiplication et division.

## **Fonctionnalités**

- L'utilisateur doit être invité à choisir une opération : addition (+), soustraction (-), multiplication (*) ou division (/).
- Ensuite, l'utilisateur doit entrer deux nombres sur lesquels l'opération sera effectuée, l'un après l'autre.
- L'application doit vérifier la validité de l'entrée de l'utilisateur.
- Le calcul est affiché ainsi que son résultat.
- L'application ne s'éteint pas, sauf si l'utilisateur le décide
- Vous devez créer et utiliser des fonctions quand c’est possible

## Aide

### Récoltez un input utilisateur

```rust
let mut input = String::new();
std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
```

### Convertir un String en i32

```rust
Let number: i32 = mon_string.parse().expect("Conversion failed");
```