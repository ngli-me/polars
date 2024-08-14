#![allow(clippy::nonstandard_macro_braces)] // Needed because clippy does not understand proc macro of PyO3
#![allow(clippy::transmute_undefined_repr)]
#![allow(non_local_definitions)]
#![allow(clippy::too_many_arguments)] // Python functions can have many arguments due to default arguments
#![allow(clippy::disallowed_types)]

pub mod allocator;
#[cfg(feature = "csv")]
pub mod batched_csv;
#[cfg(feature = "polars_cloud")]
pub mod cloud;
pub mod conversion;
pub mod dataframe;
pub mod datatypes;
pub mod error;
pub mod exceptions;
pub mod expr;
pub mod file;
pub mod gil_once_cell;
pub mod interop;
pub mod lazyframe;
pub mod lazygroupby;
pub mod map;
#[cfg(debug_assertions)]
pub mod memory;
#[cfg(feature = "object")]
pub mod object;
#[cfg(feature = "object")]
pub mod on_startup;
pub mod prelude;
pub mod py_modules;
pub mod series;
#[cfg(feature = "sql")]
pub mod sql;
pub mod utils;

pub use crate::conversion::Wrap;
pub use crate::dataframe::PyDataFrame;
pub use crate::expr::PyExpr;
pub use crate::lazyframe::PyLazyFrame;
pub use crate::lazygroupby::PyLazyGroupBy;
pub use crate::series::PySeries;
