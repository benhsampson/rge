#[macro_export]
macro_rules! impl_op_assign {
    ($Lhs:ty, $Rhs:ty, $($Op:ident)::+ { $(fn $method:ident $predicate:expr)+ }) => {
         impl $($Op)::+<$Rhs> for $Lhs {
            $(fn $method(&mut self, rhs: $Rhs) {
                $predicate(self, &rhs)
            })+
        }

        impl $($Op)::+<&$Rhs> for $Lhs {
            $(fn $method(&mut self, rhs: &$Rhs) {
                $predicate(self, rhs)
            })+
        }

        impl $($Op)::+<&mut $Rhs> for $Lhs {
            $(fn $method(&mut self, rhs: &mut $Rhs) {
                $predicate(self, rhs)
            })+
        }

        impl $($Op)::+<$Rhs> for &mut $Lhs {
            $(fn $method(&mut self, rhs: $Rhs) {
                $predicate(self, &rhs)
            })+
        }

        impl $($Op)::+<&$Rhs> for &mut $Lhs {
            $(fn $method(&mut self, rhs: &$Rhs) {
                $predicate(self, rhs)
            })+
        }

        impl $($Op)::+<&mut $Rhs> for &mut $Lhs {
            $(fn $method(&mut self, rhs: &mut $Rhs) {
                $predicate(self, rhs)
            })+
        }
    };
}

#[macro_export]
macro_rules! impl_op {
    ($Lhs:ty : $Rhs:ty => $Output:ty, $($Op:ident)::+ { $(fn $method:ident $predicate:expr)+ }) => {
        impl $($Op)::+<$Rhs> for $Lhs {
            type Output = $Output;

            $(fn $method(self, rhs: $Rhs) -> Self::Output {
                $predicate(&self, &rhs)
            })+
        }

        impl $($Op)::+<&$Rhs> for $Lhs {
            type Output = $Output;

            $(fn $method(self, rhs: &$Rhs) -> Self::Output {
                $predicate(&self, rhs)
            })+
        }

        impl $($Op)::+<&mut $Rhs> for $Lhs {
            type Output = $Output;

            $(fn $method(self, rhs: &mut $Rhs) -> Self::Output {
                $predicate(&self, rhs)
            })+
        }

        impl $($Op)::+<$Rhs> for &$Lhs {
            type Output = $Output;

            $(fn $method(self, rhs: $Rhs) -> Self::Output {
                $predicate(self, &rhs)
            })+
        }

        impl $($Op)::+<$Rhs> for &mut $Lhs {
            type Output = $Output;

            $(fn $method(self, rhs: $Rhs) -> Self::Output {
                $predicate(self, &rhs)
            })+
        }

        impl $($Op)::+<&$Rhs> for &$Lhs {
            type Output = $Output;

            $(fn $method(self, rhs: &$Rhs) -> Self::Output {
                $predicate(self, rhs)
            })+
        }

        impl $($Op)::+<&$Rhs> for &mut $Lhs {
            type Output = $Output;

            $(fn $method(self, rhs: &$Rhs) -> Self::Output {
                $predicate(self, rhs)
            })+
        }
    };

    ($Lhs:ty : $Rhs:ty, $($Op:ident)::+ { $(fn $method:ident $predicate:expr)+ }) => {
        $crate::impl_op!($Lhs : $Rhs => $Lhs, $($Op)::+ { $(fn $method $predicate)+ });
    };

    ($Container:ty, $($Op:ident)::+ { $(fn $method:ident $predicate:expr)+ }) => {
        impl $($Op)::+ for $Container {
            type Output = $Container;

            $(fn $method(self) -> Self::Output {
                $predicate(&self)
            })+
        }

        impl $($Op)::+ for &$Container {
            type Output = $Container;

            $(fn $method(self) -> Self::Output {
                $predicate(self)
            })+
        }
    };
}

#[macro_export]
macro_rules! impl_conversions {
    ($A:ty => $B:ty, $converter:expr) => {
        impl From<$A> for $B {
            fn from(a: $A) -> $B {
                $converter(&a)
            }
        }

        impl From<&$A> for $B {
            fn from(a: &$A) -> $B {
                $converter(a)
            }
        }

        impl From<&mut $A> for $B {
            fn from(a: &mut $A) -> $B {
                $converter(a)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_index_ops {
    ($A:ty { $($index:literal => $field:ident),+ } => $B:ty) => {
        impl ops::Index<usize> for $A {
            type Output = $B;

            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    $($index => &self.$field),+,
                    _ => panic!("Index out of bounds")
                }
            }
        }

        impl ops::IndexMut<usize> for $A {
            fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                match i {
                    $($index => &mut self.$field),+,
                    _ => panic!("Index out of bounds")
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_algebraic_ops {
    ($VecN:ident { $($field:ident),+ }, $n:expr) => {
        $crate::impl_op!($VecN, ops::Neg { fn neg |lhs: &$VecN| {
            *lhs * -1.
        }});

        $crate::impl_op!($VecN : $VecN, ops::Add { fn add |lhs: &$VecN, rhs: &$VecN| {
            $VecN { $($field: lhs.$field + rhs.$field),+ }
        }});

        $crate::impl_op_assign!($VecN, $VecN, ops::AddAssign { fn add_assign |lhs: &mut $VecN, rhs: &$VecN| {
            *lhs = *lhs + *rhs;
        }});

        $crate::impl_op!(
            $VecN : $VecN, ops::Sub { fn sub |lhs: &$VecN, rhs: &$VecN| {
                $VecN { $($field: lhs.$field - rhs.$field),+ }
            }}
        );

        $crate::impl_op_assign!(
            $VecN, $VecN, ops::SubAssign { fn sub_assign |lhs: &mut $VecN, rhs: &$VecN| {
                *lhs = *lhs - *rhs;
            }}
        );

        $crate::impl_op!(
            $VecN : f32, ops::Mul { fn mul |lhs: &$VecN, rhs: &f32| {
                $VecN { $($field: lhs.$field * rhs),+ }
            }}
        );

        $crate::impl_op_assign!(
            $VecN, f32, ops::MulAssign { fn mul_assign |lhs: &mut $VecN, rhs: &f32| {
                *lhs = *lhs * *rhs;
            }}
        );

        $crate::impl_op!(
            $VecN : f32, ops::Div { fn div |lhs: &$VecN, rhs: &f32| {
                let inv = 1. / rhs;
                $VecN { $($field: lhs.$field * inv),+ }
            }}
        );

        $crate::impl_op_assign!(
            $VecN, f32, ops::DivAssign { fn div_assign |lhs: &mut $VecN, rhs: &f32| {
                *lhs = *lhs / *rhs;
            }}
        );
    };
}
