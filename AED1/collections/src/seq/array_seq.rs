use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::ptr;

use super::{Seq, SeqError};

/// Implementação de Seq<T> usando um "arranjo dinâmico" feito na mão.
///
/// Invariantes:
/// - `ptr` aponta para um bloco de memória válido de tamanho `cap` elementos do tipo T
/// - Para todo i < len, `ptr.add(i)` contém um T inicializado
/// - Para todo i >= len, `ptr.add(i)` contém lixo (não inicializado) e nunca é lido / droppado
#[derive(Debug)]
pub struct ArraySeq<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> ArraySeq<T> {
    const INITIAL_CAPACITY: usize = 4;

    /// Cria um array vazio com capacidade inicial padrão.
    pub fn new() -> Self {
        Self::with_capacity(Self::INITIAL_CAPACITY)
    }

    /// Cria um array vazio com capacidade inicial `cap`.
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0);

        // Calcula o layout de um array de `cap` elementos T
        let layout = Layout::array::<T>(cap).expect("layout inválido");

        // Aloca memória bruta (não inicializada) para `cap` elementos T
        let ptr = unsafe { alloc(layout) as *mut T };

        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        Self { ptr, len: 0, cap }
    }

    fn ensure_capacity_for_insert(&mut self) {
        if self.len == self.cap {
            self.grow();
        }
    }

    /// Dobra a capacidade do array (como em C: aloca novo bloco, copia, libera o antigo).
    fn grow(&mut self) {
        let old_cap = self.cap;
        let new_cap = old_cap * 2;

        let new_layout = Layout::array::<T>(new_cap).expect("layout inválido");
        let new_ptr = unsafe {
            let p = alloc(new_layout) as *mut T;
            if p.is_null() {
                handle_alloc_error(new_layout);
            }

            // Copia bit a bit os `len` elementos atuais para o novo bloco.
            // Não chamamos drop nos antigos; vamos só liberar a memória bruta depois.
            ptr::copy_nonoverlapping(self.ptr, p, self.len);
            p
        };

        // Libera a memória antiga sem chamar drop nos elementos (já "movidos").
        let old_layout = Layout::array::<T>(old_cap).expect("layout inválido");
        unsafe {
            dealloc(self.ptr as *mut u8, old_layout);
        }

        self.ptr = new_ptr;
        self.cap = new_cap;
    }

     /// Encolhe o array se ele estiver "folgado demais":
    /// se len > 0 e len < cap / 4, reduz cap pela metade.
    fn shrink_if_necessary(&mut self) {
        if self.len > 0 && self.len < self.cap / 4 {
            self.shrink();
        }
    }

    /// Diminui a capacidade do array (pela metade), mantendo os `len` elementos.
    fn shrink(&mut self) {
        let old_cap = self.cap;
        let mut new_cap = old_cap / 2;

        if new_cap < Self::INITIAL_CAPACITY {
            new_cap = Self::INITIAL_CAPACITY;
        }
        if new_cap < self.len {
            new_cap = self.len;
        }

        if new_cap >= old_cap {
            return;
        }

        let new_layout = Layout::array::<T>(new_cap).expect("layout inválido");
        let new_ptr = unsafe {
            let p = alloc(new_layout) as *mut T;
            if p.is_null() {
                handle_alloc_error(new_layout);
            }
            ptr::copy_nonoverlapping(self.ptr, p, self.len);
            p
        };

        let old_layout = Layout::array::<T>(old_cap).expect("layout inválido");
        unsafe {
            dealloc(self.ptr as *mut u8, old_layout);
        }

        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}

// Precisamos liberar os elementos e a memória quando Array cair fora de escopo.
impl<T> Drop for ArraySeq<T> {
    fn drop(&mut self) {
        unsafe {
            // Chama drop para todos os elementos válidos (0..len)
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }

            // Libera o bloco de memória (sem chamar drop nos "slots vazios").
            if self.cap > 0 {
                let layout = Layout::array::<T>(self.cap).expect("layout inválido");
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

// ========================
// impl Seq<T> for Array<T>
// ========================

impl<T> Seq<T> for ArraySeq<T> {
    fn len(&self) -> usize {
        self.len
    }

    fn insert_at(&mut self, index: usize, value: T) -> Result<(), SeqError> {
        let len = self.len;
        if index > len {
            return Err(SeqError::OutOfBounds { index, len });
        }

        self.ensure_capacity_for_insert();

        unsafe {
            // Shift para a direita: [index..len-1] → [index+1..len]
            //
            // Ex: len = 4, index = 1
            // antes: [0,1,2,3]
            // depois: [0,_,1,2,3], e no '_' colocamos o novo valor
            //
            // Fazemos de trás pra frente para não sobrescrever dados ainda não movidos.
            for i in (index..len).rev() {
                let src = self.ptr.add(i);
                let dst = self.ptr.add(i + 1);
                // move: lê de src (tornando-o "não inicializado") e escreve em dst
                dst.write(src.read());
            }

            // Escreve o novo valor na posição index
            self.ptr.add(index).write(value);
        }

        self.len += 1;
        Ok(())
    }

    fn remove_from(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        let len = self.len;
        let removed = unsafe {
            // Lê o elemento a ser removido (move, não copia)
            let removed = self.ptr.add(index).read();

            // Shift para a esquerda: [index+1..len-1] → [index..len-2]
            for i in index + 1..len {
                let src = self.ptr.add(i);
                let dst = self.ptr.add(i - 1);
                dst.write(src.read());
            }

            // O último elemento (len-1) agora é considerado não inicializado.
            removed
        };

        self.len -= 1;

        // checa se vale a pena encolher
        self.shrink_if_necessary();

        Some(removed)
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(&*self.ptr.add(index)) }
        }
    }

    // acesso seguro mutável
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(&mut *self.ptr.add(index)) }
        }
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        // Itera sobre [0..len), cada posição é um T válido.
        Box::new((0..self.len).map(move |i| unsafe { &*self.ptr.add(i) }))
    }

    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut T> + 'a> {
        Box::new(ArraySeqIterMut {
            ptr: self.ptr,
            remaining: self.len,
            _marker: PhantomData,
        })
    }

}

// ========================
// Index / IndexMut
// ========================

impl<T> Index<usize> for ArraySeq<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .expect("Array::index: índice fora dos limites")
    }
}

impl<T> IndexMut<usize> for ArraySeq<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("Array::index_mut: índice fora dos limites")
    }
}

// ========================
// Iterador mutável
// ========================

pub struct ArraySeqIterMut<'a, T> {
    ptr: *mut T,
    remaining: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for ArraySeqIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        unsafe {
            let current = &mut *self.ptr;
            self.ptr = self.ptr.add(1);
            self.remaining -= 1;
            Some(current)
        }
    }
}
