use collections::seq::{Seq, SeqError, ArraySeq, ListSeq};

//
// =======================
// Testes auxiliares
// =======================
//

fn fill_three<S: Seq<i32>>(mut s: S) -> S {
    s.append(1);
    s.append(2);
    s.append(3);
    s
}

//
// =======================
// Testes ARRAYSEQ
// =======================
//

#[test]
fn test_array_basic() {
    let mut a = ArraySeq::new();

    assert_eq!(a.len(), 0);

    a.append(10);
    a.append(20);
    a.append(30);

    assert_eq!(a.len(), 3);
    assert_eq!(a[0], 10);
    assert_eq!(a[1], 20);
    assert_eq!(a[2], 30);

    a.insert_at(1, 99).unwrap();
    assert_eq!(a[1], 99);

    let removed = a.remove_from(2);
    assert_eq!(removed, Some(20));
}

#[test]
fn test_array_iter() {
    let mut a = ArraySeq::new();
    a = fill_three(a);

    // iter
    let collected: Vec<_> = a.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3]);

    // iter_mut
    for x in a.iter_mut() {
        *x *= 2;
    }

    let collected2: Vec<_> = a.iter().copied().collect();
    assert_eq!(collected2, vec![2, 4, 6]);
}

#[test]
fn test_array_insert_out_of_bounds() {
    let mut a = ArraySeq::new();
    a.append(10);

    let err = a.insert_at(5, 99).unwrap_err();
    match err {
        SeqError::OutOfBounds { index, len } => {
            assert_eq!(index, 5);
            assert_eq!(len, 1);
        }
        _ => panic!("erro errado"),
    }
}

//
// =======================
// Testes LISTSEQ
// =======================
//

#[test]
fn test_list_basic() {
    let mut l = ListSeq::new();

    l.append(10);
    l.append(20);
    l.append(30);

    assert_eq!(l[0], 10);
    assert_eq!(l[1], 20);
    assert_eq!(l[2], 30);

    l.insert_at(1, 99).unwrap();
    assert_eq!(l[1], 99);

    let removed = l.remove_from(2);
    assert_eq!(removed, Some(20));
}

#[test]
fn test_list_iter() {
    let mut l = ListSeq::new();
    l = fill_three(l);

    let collected: Vec<_> = l.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3]);

    for x in l.iter_mut() {
        *x *= 2;
    }

    let collected2: Vec<_> = l.iter().copied().collect();
    assert_eq!(collected2, vec![2, 4, 6]);
}

#[test]
fn test_list_insert_out_of_bounds() {
    let mut l = ListSeq::new();
    l.append(10);

    let err = l.insert_at(5, 99).unwrap_err();
    match err {
        SeqError::OutOfBounds { index, len } => {
            assert_eq!(index, 5);
            assert_eq!(len, 1);
        }
        _ => panic!("erro errado"),
    }
}

//
// =======================
// Teste comum
// =======================
//

#[test]
fn test_remove_out_of_bounds() {
    let mut a = ArraySeq::new();
    let mut l = ListSeq::new();

    assert_eq!(a.remove_from(0), None);
    assert_eq!(l.remove_from(0), None);

    a.append(10);
    l.append(10);

    assert_eq!(a.remove_from(5), None);
    assert_eq!(l.remove_from(5), None);
}
