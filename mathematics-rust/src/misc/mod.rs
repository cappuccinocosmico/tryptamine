use std::slice;

use const_for::const_for;

type SortType = u32;

const fn raw_bytes<T>(val: &T) -> &[u8] {
    let ptr_t = val as *const T;
    let ptr_u8 = ptr_t as *const u8;
    unsafe { slice::from_raw_parts(ptr_u8, size_of::<T>()) }
}

const _: () = assert!(raw_bytes(&32_u16)[0] == 32_u8);
const _: () = assert!(raw_bytes(&32_u16)[1] == 0_u8);
const _: () = assert!(raw_bytes(&0.0_f64)[0] == 0_u8);
const _: () = assert!(raw_bytes(&()).is_empty());

macro_rules! dedupe {
    ($x:ident) => {
        const {
            const ORIG_LEN: usize = $x.len();
            const SORTED_LIST: [SortType; ORIG_LEN] = quicksort($x);
            const DEDUPE_LEN: usize = sorted_deduped_len(&SORTED_LIST);
            const DEDUPED: [SortType; DEDUPE_LEN] = sorted_deduped(&SORTED_LIST);
            &DEDUPED
        }
    };
}

const ORIGINAL_LIST: [u32; 13] = [1, 1, 3, 3, 4, 1, 2, 2, 2, 3, 4, 10, 21];
const DEDUPED_LIST: &[u32] = dedupe!(ORIGINAL_LIST);

const fn sorted_deduped_len(list: &[SortType]) -> usize {
    if list.len() <= 1 {
        return list.len();
    }
    let mut count = 1;
    const_for!(i in 1..list.len() => {
        if list[i] != list[i-1] {
            count += 1;
        };
    });

    count
}

const fn sorted_deduped<const SORT_LEN: usize>(list: &[SortType]) -> [SortType; SORT_LEN] {
    let mut buff = [0; SORT_LEN];
    if !list.is_empty() {
        buff[0] = list[0]
    }
    if list.len() <= 1 {
        return buff;
    }
    let mut count = 1;
    const_for!(i in 1..list.len() => {
        if list[i] != list[i-1] {
            buff[count]=list[i];
            count += 1;
        };
    });

    buff
}

pub const fn quicksort<const N: usize>(mut list: [SortType; N]) -> [SortType; N] {
    quicksort_mut(&mut list);
    list
}
const fn quicksort_mut(list: &mut [SortType]) {
    if list.len() <= 1 {
        return;
    }
    // Pivot is the last element, current plan is to construct the list like this:
    // [lower_values] pivot [upper values]
    // So for ease of use quicksort the list so its in this form
    // pivot [lower_values] [upper_values]
    let pivot = list[0];
    // Then run list.swap(0,<last_index_of_lower_values>), to get the list into the right format.
    // And make a couple indexes to keep track of everything
    let mut working_index = 1;
    let mut greater_buffer_len = 0;
    // So if the greater_buffer_len is more then 1, that means that the pivot has been overwritten.
    while working_index + greater_buffer_len < list.len() {
        match list[working_index] <= pivot {
            true => working_index += 1,
            false => {
                list.swap(working_index, list.len() - greater_buffer_len - 1);
                greater_buffer_len += 1
            }
        }
    }
    list.swap(0, working_index - 1);

    quicksort_mut(slice_from_index(list, working_index, list.len()));
    quicksort_mut(slice_from_index(list, 0, working_index - 1));
}

/// Very annoying const compatible replacement for list[begin..end] which apparently doenst work in
/// const
const fn slice_from_index<T>(list: &mut [T], begin: usize, end: usize) -> &mut [T] {
    assert!(end >= begin);
    assert!(list.len() >= end);
    unsafe {
        let ptr = list.as_mut_ptr();
        let ptr_offset = ptr.add(begin);
        let sublen = end - begin;

        std::slice::from_raw_parts_mut(ptr_offset, sublen)
    }
}

const _: () = assert!(is_sorted(&quicksort([1, 5, 3, 1, 6, 6, 33, 31, 15, 10])));
const fn is_sorted(list: &[SortType]) -> bool {
    const_for!(i in 0..list.len()-1 => {
        if list[i]> list[i+1] {
            return false;
        }
    });
    true
}
