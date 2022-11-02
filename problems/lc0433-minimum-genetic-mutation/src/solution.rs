pub struct Solution;

use std::collections::{HashMap, VecDeque};
use std::convert::{TryFrom, TryInto};

const MAX_BANK_SIZE: usize = 10;
const GENE_LEN: usize = 8;
const WILDCARD: u8 = 'X' as u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Gene([u8; GENE_LEN]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Mask([u8; GENE_LEN]);

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        assert!(start.len() == GENE_LEN);
        assert!(end.len() == GENE_LEN);
        assert!(bank.len() <= MAX_BANK_SIZE);

        if !bank.iter().any(|a_valid_gene| end.as_str() == a_valid_gene) {
            return -1
        }

        let start_gene = Gene::try_from(start.as_str()).expect("invalid start");
        let end_gene = Gene::try_from(end.as_str()).expect("invalid end");

        let vertices = bank
            .into_iter()
            .filter(|s| s.as_str() != start.as_str())
            .filter(|s| s.as_str() != end.as_str())
            .enumerate()
            .map(|(vertex_id, gene)| {
                let gene = Gene::try_from(gene.as_str())
                    .expect(&format!("Bad gene {:?}->{:?}", vertex_id, gene));
                gene.masks()
            })
            .collect::<Vec<_>>();

        let edges = vertices.iter().enumerate().fold(
            HashMap::<Mask, Vec<usize>>::new(),
            |mut acc, (vertex_id, masks)| {
                for &mask in masks {
                    acc.entry(mask).or_default().push(vertex_id);
                }
                acc
            },
        );

        let mut memo = vertices.iter().map(|_| usize::MAX).collect::<Vec<_>>();

        let mut next = start_gene
            .masks()
            // .into_iter()
            .iter()
            .copied()
            .map(|mask| (mask, 0usize))
            .collect::<VecDeque<_>>();

        while let Some((mask, distance)) = next.pop_front() {
            if end_gene.matches(mask) {
                return (distance + 1) as i32
            }
            if let Some(vertex_ids) = edges.get(&mask) {
                for &vertex_id in vertex_ids {
                    if memo[vertex_id] <= distance {
                        continue
                    }

                    memo[vertex_id] = distance;

                    next.extend(
                        vertices[vertex_id]
                            //.into_iter()
                            .iter()
                            .copied()
                            .map(|m| (m, distance + 1)),
                    );
                }
            }
        }

        return -1
    }
}

impl TryFrom<&str> for Gene {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into().map(Self).map_err(|e| e.to_string())
    }
}

impl Gene {
    pub fn matches(&self, mask: Mask) -> bool {
        self.0
            // .into_iter()
            .iter()
            .copied()
            .zip(
                mask.0
                    //.into_iter()
                    .iter()
                    .copied(),
            )
            .all(|(g, m)| g == m || m == WILDCARD)
    }
    pub fn masks(&self) -> [Mask; GENE_LEN] {
        // // Leetcode, would you please update the Rust version already?!

        // core::array::from_fn(|idx| {
        //     let mut mask = self.0;
        //     mask[idx] = WILDCARD;
        //     Mask(mask)
        // })

        let mut blanks: [[u8; GENE_LEN]; GENE_LEN] = [self.0; GENE_LEN];

        for (idx, blank) in blanks.iter_mut().enumerate() {
            blank[idx] = WILDCARD;
        }

        blanks.map(Mask)
    }
}
