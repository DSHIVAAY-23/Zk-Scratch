#[derive(Debug, Clone)]
struct PolynomialVec(Vec<u64>);

impl PolynomialVec {
    fn new(data: Vec<u64>) -> Self {
        Self(data)
    }
}

impl FromIterator<u64> for PolynomialVec {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        PolynomialVec(iter.into_iter().collect())
    }
}

impl IntoIterator for PolynomialVec {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl IntoIterator for PolynomialVec {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> IntoIterator for &'a PolynomialVec {
    type Item = &'a u64;
    type IntoIter = std::slice::Iter<'a, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Extend<u64> for PolynomialVec {
    fn extend<I: IntoIterator<Item = u64>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}