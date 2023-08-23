use crate::DbResult;

pub(crate) fn encode<S: ?Sized + serde::Serialize>(item: &S) -> DbResult<Vec<u8>> {
    Ok(postcard::to_stdvec(item)?)
}

pub(crate) fn decode<'a, T: serde::Deserialize<'a>>(bytes: &'a [u8]) -> DbResult<T> {
    Ok(postcard::from_bytes(bytes)?)
}
