use std::{ptr, collections::LinkedList};

use crate::allocator::{Mem, Alloc};

pub struct NaiveAllocator {
    memory: Mem,
    free_list: LinkedList<Block>,
    alloc_list : LinkedList<Block>,
}

impl NaiveAllocator {
    pub fn new(bytes: usize) -> Self {
        let memory = Mem::new(bytes);

        let mut list = LinkedList::new();
        list.push_back(Block { address: memory.data, size: bytes });

        Self {
            memory,
            free_list: list,
            alloc_list: LinkedList::new(),
        }
    }

    fn allocate_in_block(&mut self, bytes: usize, index: usize) -> *mut u8 {
        let mut free_list_after_index = self.free_list.split_off(index);

        let old_block_address = if free_list_after_index.front().unwrap().size - bytes > 0 {
            let old_block = free_list_after_index.front_mut().unwrap();

            unsafe { old_block.address = old_block.address.add(bytes); }
            old_block.size -= bytes;

            old_block.address
        } else {
            free_list_after_index.pop_front().unwrap().address
        };

        let new_block = Block {
            address: old_block_address,
            size: bytes,
        };

        self.alloc_list.push_back(new_block);

        self.free_list.append(&mut free_list_after_index);

        old_block_address
    }

    fn search_free_block(&self, bytes: usize) -> Option<usize> {
        for (i, block) in self.free_list.iter().enumerate() {
            if block.size >= bytes {
                return Some(i);
            }
        }

        None
    }
}

impl Alloc for NaiveAllocator {
    fn malloc(&mut self, bytes: usize) -> Option<*mut u8> {
        if bytes == 0 {
            return None;
        }

        let index = self.search_free_block(bytes);

        if let Some(index) = index {
            let address = self.allocate_in_block(bytes, index);

            return Some(address);
        }

        None
    }

    fn free(&mut self, address: *mut u8) {
        if address == ptr::null_mut() {
            return;
        }

        for block in &self.alloc_list {

        }
    }
}

struct Block {
    address: *mut u8,
    size: usize,
}
