use collections::set::{ListSet, BstSet, BstRBSet, BstAvlSet, HashChainingSet, HashProbingSet};
use collections::set::Set;

// =======================
// Helpers genéricos
// =======================

fn testa_set_basico<S: Set<i32>>(mut s: S) {
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);

    // inserção
    assert!(s.insert(10));
    assert!(s.insert(20));
    assert!(s.insert(30));

    assert_eq!(s.len(), 3);
    assert!(s.contains(&10));
    assert!(s.contains(&20));
    assert!(s.contains(&30));
    assert!(!s.contains(&99));

    // não insere repetido
    assert!(!s.insert(10));
    assert_eq!(s.len(), 3);

    // remoção
    assert!(s.remove(&20));
    assert!(!s.contains(&20));
    assert_eq!(s.len(), 2);

    // remover quem não existe
    assert!(!s.remove(&999));
    assert_eq!(s.len(), 2);
}

// =======================
// ListSet
// =======================

#[test]
fn listset_basico() {
    let s = ListSet::new();
    testa_set_basico(s);
}

// =======================
// BstSet
// =======================

#[test]
fn bstset_basico() {
    let s = BstSet::new();
    testa_set_basico(s);
}

// =======================
// HashSet (chaining)
// =======================

#[test]
fn hashset_chaining_basico() {
    let s: HashChainingSet<i32> = HashChainingSet::new();
    testa_set_basico(s);
}


// =======================
// HashSetProbing (linear probing)
// =======================

#[test]
fn hashset_probing_basico() {
    let s: HashProbingSet<i32> = HashProbingSet::new();
    testa_set_basico(s);
}

#[test]
fn bstavlset_basico() {
    let s = BstAvlSet::new();
    testa_set_basico(s);
}

#[test]
fn bstrbset_basico() {
    let s = BstRBSet::new();
    testa_set_basico(s);
}