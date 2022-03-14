use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    marker::PhantomData,
    ops::Neg,
};

pub trait FrequencyContainer<S>: Default {
    type FIter: Iterator<Item = (S, usize)> + 'static;

    fn update(&mut self, symbol: S);

    fn frequency_of(&self, symbol: S) -> Option<usize>;

    fn frequencies(self) -> Self::FIter;

    fn total(&self) -> usize;
}

impl<S: Eq + Hash + 'static> FrequencyContainer<S> for HashMap<S, usize> {
    type FIter = <Self as IntoIterator>::IntoIter;

    fn update(&mut self, symbol: S) {
        self.entry(symbol).and_modify(|s| *s += 1).or_insert(1);
    }

    fn frequency_of(&self, symbol: S) -> Option<usize> {
        self.get(&symbol).copied()
    }

    fn frequencies(self) -> Self::FIter {
        self.into_iter()
    }

    fn total(&self) -> usize {
        self.values().sum()
    }
}

impl<S: Ord + 'static> FrequencyContainer<S> for BTreeMap<S, usize> {
    type FIter = <Self as IntoIterator>::IntoIter;

    fn update(&mut self, symbol: S) {
        self.entry(symbol).and_modify(|s| *s += 1).or_insert(1);
    }

    fn frequency_of(&self, symbol: S) -> Option<usize> {
        self.get(&symbol).copied()
    }

    fn total(&self) -> usize {
        self.values().sum()
    }

    fn frequencies(self) -> Self::FIter {
        self.into_iter()
    }
}

pub trait FrequencyIteratorExt<S, I> {
    fn shannon<T>(self) -> Frequencies<T, S>
    where
        T: FrequencyContainer<S>;
}

impl<S, I> FrequencyIteratorExt<S, I> for I
where
    I: Iterator<Item = S>,
{
    fn shannon<T>(self) -> Frequencies<T, S>
    where
        T: FrequencyContainer<S>,
    {
        let mut frequencies = T::default();
        for s in self {
            frequencies.update(s);
        }

        Frequencies {
            counts: frequencies,
            __phantom: PhantomData,
        }
    }
}

pub struct Frequencies<T, S>
where
    T: FrequencyContainer<S>,
{
    counts: T,
    __phantom: PhantomData<S>,
}

impl<T, S> Frequencies<T, S>
where
    T: FrequencyContainer<S>,
{
    pub fn probabilities(self) -> impl Iterator<Item = (S, f64)> {
        let total = self.counts.total();

        self.counts
            .frequencies()
            .map(move |(symbol, frequency)| (symbol, freq(frequency, total)))
    }

    pub fn entropy(self) -> f64 {
        self.probabilities()
            .map(|(_, prob)| prob)
            .map(log2)
            .sum::<f64>()
            .neg()
    }
}

fn freq(f: usize, tot: usize) -> f64 {
    f as f64 / tot as f64
}

fn log2(p: f64) -> f64 {
    p * f64::log2(p)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::FrequencyIteratorExt;

    #[test]
    fn test_simple_string() {
        let freq = "Hello, World!".chars().shannon::<BTreeMap<_, _>>();
        assert_eq!(freq.counts[&'l'], 3);
        assert_eq!(freq.counts[&'o'], 2);
        assert_eq!(freq.counts[&' '], 1);
        assert_eq!(freq.counts.get(&'æ'), None);
    }
}
