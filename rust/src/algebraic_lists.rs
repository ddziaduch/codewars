// https://www.codewars.com/kata/529a92d9aba78c356b000353/train/rust

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T>
    {
        let mut vector: Vec<T> = Vec::new();
        for x in it {
            vector.push(x);
        }
        let mut tail = Cons::Null;
        loop {
            tail = match vector.pop() {
                Some(x) => Cons::new(x, tail),
                None => break tail
            };
        }
    }

    pub fn filter<F>(&self, fun: F) -> Self
        where F: Fn(&T) -> bool
    {
        Cons::from_iter(self.to_vec().into_iter().filter(fun))
    }

    pub fn map<F,S>(&self, fun: F) -> Cons<S>
        where F: Fn(T) -> S, S: Clone
    {
        Cons::from_iter(self.to_vec().into_iter().map(fun))
    }
}
