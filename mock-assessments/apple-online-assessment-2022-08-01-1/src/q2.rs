pub struct Solution;

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::sync::Arc;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut gens: Vec<HashSet<P>> = vec![];

        for g in 1..=n {
            assert_eq!(gens.len(), g - 1);

            if let Some(last) = gens.last() {
                // eprintln!("==== gen: {:?}", g);
                // eprintln!(
                //     "last: {:#?}",
                //     last.iter().map(P::render).collect::<Vec<_>>()
                // );

                let next = last
                    .into_iter()
                    .flat_map(|p| p.next())
                    .collect::<HashSet<_>>();

                // eprintln!(
                //     "next: {:#?}",
                //     next.iter().map(P::render).collect::<Vec<_>>()
                // );

                gens.push(next);
            } else {
                gens.push(HashSet::from([P(Arc::from(vec![P::default()]))]));
            }
        }

        gens.pop()
            .expect("At least one is there")
            .into_iter()
            .map(|p| p.render())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct P(Arc<[Self]>);

impl Default for P {
    fn default() -> Self {
        Self(Arc::<[Self]>::from(vec![]))
    }
}

impl P {
    fn render(&self) -> String {
        let mut out = String::new();
        self.render_into_string(&mut out, false);
        out
    }

    fn render_into_string(&self, out: &mut String, print_parens: bool) {
        if print_parens {
            out.push('(');
        }
        for each in self.0.iter() {
            each.render_into_string(out, true);
        }
        if print_parens {
            out.push(')');
        }
    }

    fn next(&self) -> impl Iterator<Item = Self> {
        let len = self.0.len();

        (0..=len).flat_map({
            let this = self.clone();
            move |skip| {
                // eprintln!("skip: {:?}", skip);

                (0..=(len - skip)).map({
                    let this = this.clone();
                    move |take| {
                        // eprintln!("skip: {:?}; take: {:?}", skip, take);

                        let mut out = vec![];

                        let mut items = this.0.iter().cloned();

                        for _ in 0..skip {
                            out.extend(items.next().into_iter());
                        }
                        let mut tmp = vec![];
                        for _ in 0..take {
                            tmp.extend(items.next().into_iter());
                        }
                        out.push(Self(Arc::from(tmp)));
                        out.extend(items);

                        let child = Self(Arc::from(out));

                        child
                    }
                })
            }
        })
        // .chain(std::iter::once({
        //     Self(Arc::from(vec![Default::default(), self.clone()]))
        // }))
        // .chain(std::iter::once({
        //     Self(Arc::from(vec![self.clone(), Default::default()]))
        // }))
    }
}
