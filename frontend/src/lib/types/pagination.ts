export interface Page<T> {
    items: T[],
    hasPrevious: boolean,
    hasNext: boolean,
}
