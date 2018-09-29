
#[derive(Clone, Debug, PartialEq)]
pub enum DataState<T> {
    NoData,
    DataLoading,
    DataLoaded(T),
    DataError,
}