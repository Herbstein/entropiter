use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    iter,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub trait Histogram<S>: Default {
    type FIter: Iterator<Item = (S, usize)> + 'static;

    fn update(&mut self, symbol: S);

    fn frequency_of(&self, symbol: S) -> Option<usize>;

    fn frequencies(self) -> Self::FIter;

    fn total(&self) -> usize;
}

pub struct ByteHistogram([usize; 256]);

impl Default for ByteHistogram {
    fn default() -> Self {
        Self([0; 256])
    }
}

impl Deref for ByteHistogram {
    type Target = [usize; 256];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteHistogram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type FilterTupleCountNotZero = Box<dyn Fn(&(usize, usize)) -> bool>;
type BoxedTupleSwapFunction = Box<dyn Fn((usize, usize)) -> (u8, usize)>;

impl Histogram<u8> for ByteHistogram {
    type FIter = iter::Map<
        iter::Filter<iter::Enumerate<std::array::IntoIter<usize, 256>>, FilterTupleCountNotZero>,
        BoxedTupleSwapFunction,
    >;

    fn update(&mut self, symbol: u8) {
        self[symbol as usize] += 1;
    }

    fn frequency_of(&self, symbol: u8) -> Option<usize> {
        Some(self[symbol as usize])
    }

    fn frequencies(self) -> Self::FIter {
        let filter: FilterTupleCountNotZero = Box::new(|(_, c)| *c > 0);
        let swap: BoxedTupleSwapFunction = Box::new(|(s, c)| (s as u8, c));

        self.into_iter().enumerate().filter(filter).map(swap)
    }

    fn total(&self) -> usize {
        self.into_iter().filter(|c| *c > 0).sum()
    }
}

pub struct HashMapHistogram<S>(HashMap<S, usize>);

impl<S> Default for HashMapHistogram<S> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<S> Deref for HashMapHistogram<S> {
    type Target = HashMap<S, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for HashMapHistogram<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S: Eq + Hash + 'static> Histogram<S> for HashMapHistogram<S> {
    type FIter = <HashMap<S, usize> as IntoIterator>::IntoIter;

    fn update(&mut self, symbol: S) {
        self.entry(symbol).and_modify(|s| *s += 1).or_insert(1);
    }

    fn frequency_of(&self, symbol: S) -> Option<usize> {
        self.get(&symbol).copied()
    }

    fn frequencies(self) -> Self::FIter {
        self.0.into_iter()
    }

    fn total(&self) -> usize {
        self.values().sum()
    }
}

pub struct BTreeMapHistogram<S>(BTreeMap<S, usize>);

impl<S> Default for BTreeMapHistogram<S> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl<S> Deref for BTreeMapHistogram<S> {
    type Target = BTreeMap<S, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for BTreeMapHistogram<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S: Ord + 'static> Histogram<S> for BTreeMapHistogram<S> {
    type FIter = <BTreeMap<S, usize> as IntoIterator>::IntoIter;

    fn update(&mut self, symbol: S) {
        self.entry(symbol).and_modify(|s| *s += 1).or_insert(1);
    }

    fn frequency_of(&self, symbol: S) -> Option<usize> {
        self.get(&symbol).copied()
    }

    fn frequencies(self) -> Self::FIter {
        self.0.into_iter()
    }

    fn total(&self) -> usize {
        self.values().sum()
    }
}

pub trait FrequencyIteratorExt<S, I> {
    fn shannon<T>(self) -> Frequencies<T, S>
    where
        T: Histogram<S>;
}

impl<S, I> FrequencyIteratorExt<S, I> for I
where
    I: Iterator<Item = S>,
{
    fn shannon<T>(self) -> Frequencies<T, S>
    where
        T: Histogram<S>,
    {
        let mut histogram = T::default();
        for s in self {
            histogram.update(s);
        }

        Frequencies {
            histogram,
            __phantom: PhantomData,
        }
    }
}

pub struct Frequencies<T, S>
where
    T: Histogram<S>,
{
    histogram: T,
    __phantom: PhantomData<S>,
}

impl<T, S> Frequencies<T, S>
where
    T: Histogram<S>,
{
    pub fn probabilities(self) -> impl Iterator<Item = (S, f64)> {
        let total = self.histogram.total();

        self.histogram
            .frequencies()
            .map(move |(symbol, frequency)| (symbol, frequency as f64 / total as f64))
    }

    pub fn entropy(self) -> f64 {
        let entropy = self
            .probabilities()
            .map(|(_, prob)| prob)
            .map(|p| p * p.log2())
            .sum::<f64>();

        if entropy == 0.0 {
            entropy
        } else {
            entropy * -1.0
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{BTreeMapHistogram, FrequencyIteratorExt};

    #[test]
    fn test_simple_string_histogram() {
        let freq = "Hello, World!".chars().shannon::<BTreeMapHistogram<_>>();
        assert_eq!(freq.histogram[&'l'], 3);
        assert_eq!(freq.histogram[&'o'], 2);
        assert_eq!(freq.histogram[&' '], 1);
        assert_eq!(freq.histogram.get(&'æ'), None);
    }
}
