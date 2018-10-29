pub fn select<M>() -> Select<M> {
    //...
}

impl<M> Select<M> {
    pub fn from<R>(self) -> SelectFrom<R>
        where R: Relation<Model = M> {
        //...
    }
}