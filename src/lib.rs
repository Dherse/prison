#![feature(generic_const_exprs)]

use bitvec::array::BitArray;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boxes<const N: usize>
where
    [Cell; N]: Sized,
{
    cells: [Cell; N],
    rng: SmallRng,
}

impl<const N: usize> Boxes<N>
where
    [Cell; N]: Sized,
    [u64; (N + 63) / 64]: Sized,
{
    pub fn new_random(seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);

        let mut nums = (0..N).collect::<Vec<_>>();
        nums.shuffle(&mut rng);

        let mut cells = [Cell::empty(); N];
        for (i, n) in nums.into_iter().enumerate() {
            cells[i] = Cell::new(n);
        }

        Self { cells, rng }
    }

    /// Tries to solve the boxes problem using the random method.
    pub fn solve_dumb(&mut self) -> BitArray<[u64; (N + 63) / 64]> {
        let mut found_their_box = BitArray::ZERO;

        for prisoner in 0..N {
            let mut visited: BitArray<[u64; (N + 63) / 64]> = BitArray::ZERO;

            let mut i = 0;
            let found = loop {
                if i >= N / 2 {
                    break false;
                }

                let box_idx = self.rng.next_u32() as usize % N;
                if visited[box_idx] {
                    continue;
                }

                visited.set(box_idx, true);
                i += 1;

                if self.cells[box_idx].value == prisoner {
                    break true;
                }
            };

            found_their_box.set(prisoner as usize, found);
        }

        found_their_box
    }

    pub fn solve_dumb_no_list(&mut self) -> bool {
        for prisoner in 0..N {
            let mut visited: BitArray<[u64; (N + 63) / 64]> = BitArray::ZERO;

            let mut i = 0;
            loop {
                if i >= N / 2 {
                    return false;
                }

                let box_idx = self.rng.next_u32() as usize % N;
                if visited[box_idx] {
                    continue;
                }

                visited.set(box_idx, true);
                i += 1;

                if self.cells[box_idx].value == prisoner {
                    break;
                }
            }
        }

        true
    }

    pub fn solve_dumb_shuffle(&mut self) -> bool {
        let mut list = (0..N).collect::<Vec<_>>();
        for prisoner in 0..N {
            list.shuffle(&mut self.rng);

            if !list
                .iter()
                .take(N / 2)
                .any(|i| self.cells[*i].value == prisoner)
            {
                return false;
            }
        }

        true
    }

    pub fn solve_smart(&mut self) -> BitArray<[u64; (N + 63) / 64]> {
        let mut found_their_box = BitArray::ZERO;

        for prisoner in 0..N {
            let mut i = 0;
            let mut box_idx = prisoner as usize;
            let found = loop {
                if i >= N / 2 {
                    break false;
                }

                if self.cells[box_idx].value == prisoner {
                    break true;
                }

                i += 1;
                box_idx = self.cells[box_idx].value as usize;
            };

            found_their_box.set(prisoner as usize, found);
        }

        found_their_box
    }

    pub fn solve_smart_cycle_detect(&self) -> bool {
        let mut visited: BitArray<[u64; (N + 63) / 64]> = BitArray::ZERO;

        let mut longest_cycle = 0;
        let mut sum_of_cycle_lengths = 0;

        let mut n = 0;
        while n < N {
            if visited[n] {
                n += 1;
                continue;
            }

            let mut len = 0;
            let mut idx = n;
            'inner: loop {
                visited.set(idx, true);
                idx = self.cells[idx].value as usize;

                len += 1;

                if len > N / 2 {
                    return false;
                }

                if idx == n {
                    break 'inner;
                }
            }

            sum_of_cycle_lengths += len;

            if len > longest_cycle {
                longest_cycle = len;
            }

            if sum_of_cycle_lengths >= N / 2 {
                return longest_cycle <= N / 2;
            }

            n += 1;
        }

        longest_cycle <= N / 2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    value: usize,
}

impl Cell {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn empty() -> Self {
        Self { value: 0 }
    }
}
