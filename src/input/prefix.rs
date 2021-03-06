use super::{Bytes, BytesLength, String};

pub unsafe trait Prefix<I>: BytesLength {
    fn is_prefix_of(self, input: &I) -> bool;
}

unsafe impl<'i, T, I> Prefix<I> for &T
where
    T: Prefix<I>,
{
    #[inline(always)]
    fn is_prefix_of(self, input: &I) -> bool {
        (*self).is_prefix_of(input)
    }
}

unsafe impl<'i> Prefix<Bytes<'i>> for u8 {
    #[inline(always)]
    fn is_prefix_of(self, input: &Bytes<'i>) -> bool {
        input.as_dangerous().starts_with(&[self])
    }
}

unsafe impl<'i> Prefix<String<'i>> for char {
    #[inline(always)]
    fn is_prefix_of(self, input: &String<'i>) -> bool {
        match input.as_dangerous().chars().next() {
            Some(c) => c == self,
            None => false,
        }
    }
}

unsafe impl<'i> Prefix<Bytes<'i>> for char {
    #[inline(always)]
    fn is_prefix_of(self, input: &Bytes<'i>) -> bool {
        let mut arr = [0_u8; 4];
        let prefix = self.encode_utf8(&mut arr);
        input.as_dangerous().starts_with(prefix.as_bytes())
    }
}

unsafe impl<'i> Prefix<Bytes<'i>> for &[u8] {
    #[inline(always)]
    fn is_prefix_of(self, input: &Bytes<'i>) -> bool {
        input.as_dangerous().starts_with(self)
    }
}

unsafe impl<'i> Prefix<String<'i>> for &str {
    #[inline(always)]
    fn is_prefix_of(self, input: &String<'i>) -> bool {
        input.as_dangerous().starts_with(self)
    }
}

unsafe impl<'i> Prefix<Bytes<'i>> for &str {
    #[inline(always)]
    fn is_prefix_of(self, input: &Bytes<'i>) -> bool {
        input.as_dangerous().starts_with(self.as_bytes())
    }
}

#[cfg(feature = "unstable-const-generics")]
unsafe impl<'i, const N: usize> Prefix<Bytes<'i>> for &[u8; N] {
    #[inline(always)]
    fn is_prefix_of(self, input: &Bytes<'i>) -> bool {
        input.as_dangerous().starts_with(&self[..])
    }
}

#[cfg(not(feature = "unstable-const-generics"))]
macro_rules! impl_array_prefix {
    ($($n:expr),*) => {
        $(
            unsafe impl<'i> Prefix<Bytes<'i>> for &[u8; $n] {
                #[inline(always)]
                fn is_prefix_of(self, input: &Bytes<'i>) -> bool {
                    input.as_dangerous().starts_with(&self[..])
                }
            }
        )*
    };
}

#[cfg(not(feature = "unstable-const-generics"))]
for_common_array_sizes!(impl_array_prefix);
