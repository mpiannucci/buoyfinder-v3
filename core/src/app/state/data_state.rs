
#[derive(Clone, Debug)]
pub enum DataState<T> {
    NoData,
    DataLoading,
    DataLoaded(T),
    DataError,
}