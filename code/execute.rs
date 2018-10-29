pub trait Execute<Repos, R, ReturnType>
    where R: Relation,
          Repos: Repository,
          Self: Query<ReturnType=ReturnType> {
    
    type FutureType;

    fn execute(&self, repos: &Repos) -> Self::FutureType
        where Repos: Stores<R>;
}