mod insertion;
mod binary;
mod bubble;
mod merge;
mod selection;
mod heap;
pub use self::insertion::sort as insertion_sort;
pub use self::binary::sort as binary_sort;
pub use self::bubble::sort as bubble_sort;
pub use self::selection::sort as selection_sort;
pub use self::merge::sort as merge_sort;
pub use self::heap::max_sort as heap_max_sort;
pub use self::heap::min_sort as heap_min_sort;

