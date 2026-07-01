pub trait GroupBy {
    type Item;
    fn group_by(self) -> Vec<Self::Item>;
}
