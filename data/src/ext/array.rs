pub fn array_from_fallible_fn<T, E, const C: usize, F: FnMut(usize) -> Result<T, E>>(
    mut f: F,
) -> Result<[T; C], E> {
    Ok((0..C)
        .map(|i| f(i))
        .collect::<Result<Vec<T>, E>>()?
        .try_into()
        .or(Err(()))
        .unwrap())
}
