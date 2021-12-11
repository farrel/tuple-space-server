use tuple_space::tuple::Tuple;

pub(crate) enum Command {
    Write(Tuple),
    Read(Tuple),
    Take(Tuple),
}
