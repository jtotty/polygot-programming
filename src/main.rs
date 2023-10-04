fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(7);
    }

    return Ok(());
}

fn main() -> Result<(), usize> {
    error_me(false)?;

    return Ok(());
}
