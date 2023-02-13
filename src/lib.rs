/// struct getter creation macro
/// 
/// # Example
/// ```
/// use curmacro::*;
/// 
/// struct Point {
///     pub x: i32,
///     pub y: i32
/// }
/// 
/// impl Point {
///     getters!(
///         pub get_x(x) -> i32;
///         pub get_y(y) -> i32;    
///     );
/// }
/// 
/// let x = 13;
/// let y = 215;
/// let point = Point { x, y };
/// 
/// assert_eq!(point.get_x().clone(), x);
/// assert_eq!(point.get_y().clone(), y);
/// ```
#[macro_export]
macro_rules! getters {
    (
        $($vis:vis $fn:ident($field:ident) -> $type:ty;)*
    ) => {
        $(
            $vis fn $fn(&self) -> &$type {
                &self.$field
            }
        )*
    };
}

/// struct setter creation macro
/// 
/// # Example
/// ```
/// use curmacro::*;
/// 
/// struct Point {
///     pub x: i32,
///     pub y: i32
/// }
/// 
/// impl Point {
///     setters!(
///         pub set_x(i32) -> x;
///         pub set_y(i32) -> y;    
///     );
/// }
/// 
/// let mut point = Point { x: 0, y: 0 };
/// let x = 13;
/// let y = 215;
/// 
/// point.set_x(x);
/// point.set_y(y);
/// 
/// assert_eq!(point.x, x);
/// assert_eq!(point.y, y);
/// ```
#[macro_export]
macro_rules! setters {
    (
        $($vis:vis $fn:ident($type:ty) -> $field:ident;)*
    ) => {
        $(
            $vis fn $fn(&mut self, value: $type) {
                self.$field = value
            }
        )*
    };
}