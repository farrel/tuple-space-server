use tuple_space::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum Command {
    Write(Tuple),
    Read(Tuple),
    Take(Tuple),
}
