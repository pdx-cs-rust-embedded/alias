#[cfg(feature = "safe")]
fn copy_ints(n: usize, a: &[i32], b: &mut [i32]) {
    for i in 0..n {
        b[i] = a[i];
    }
}

#[cfg(not(feature = "safe"))]
unsafe fn unsafe_copy_ints(n: usize, na: usize, a: *const i32, nb: usize, b: *mut i32) {
    assert!(n <= na);
    assert!(n <= nb);
    for i in 0..n as isize {
        unsafe {
            b.offset(i).write(*a.offset(i));
        }
    }
}

const N: usize = 10;

fn main() {
    let mut a: [i32; N] = [0; N];
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        a[i] = i as i32;
    }
    #[cfg(feature = "safe")]
    copy_ints(3, &a[1..], &mut a[0..]);
    #[cfg(not(feature = "safe"))]
    {
        let ap = &a as *const i32;
        let bp = &mut a as *mut i32;
        unsafe {
            unsafe_copy_ints(3, N - 1, ap.offset(1), N, bp);
        }
    }
    for x in a {
        println!("{}", x);
    }
}
