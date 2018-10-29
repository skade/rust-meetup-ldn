pub trait Repository {
    type Gateway: Gateway;

    fn gateway(&self) -> &Self::Gateway;
}

pub trait Stores<Rel> : Repository {

}