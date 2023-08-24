#[test_only]
module aptos_std::smart_vector_test {
    use aptos_std::smart_vector::{Self, SmartVector};

    #[test_only]
    public fun make_smart_vector(): SmartVector<u64> {
        let v = smart_vector::new<u64>();
        let i = 0u64;
        while (i < 100) {
            smart_vector::push_back(&mut v, i);
            i = i + 1;
        };
        v
    }

    #[test]
    public fun smart_vector_for_each_test() {
        let v = make_smart_vector();
        let i = 0;
        smart_vector::for_each(v, |x| {
            assert!(i == x, 0);
            i = i + 1;
        });
    }

    #[test]
    public fun smart_vector_for_each_reverse_test() {
        let v = make_smart_vector();
        let i = 0;
        smart_vector::for_each_reverse(v, |x| {
            assert!(i == 99 - x, 0);
            i = i + 1;
        });
    }

    #[test]
    public fun smart_vector_for_each_ref_test() {
        let v = make_smart_vector();
        let s = 0;
        smart_vector::for_each_ref(&v, |x| {
            s = s + *x;
        });
        assert!(s == 4950, 0);
        smart_vector::destroy(v);
    }

    #[test]
    public fun smart_vector_for_each_mut_test() {
        let v = make_smart_vector();
        let s = 0;
        smart_vector::for_each_mut(&mut v, |x| {
            let x: &mut u64 = x;
            *x = *x + 1;
        });
        smart_vector::for_each_ref(&v, |x| {
            s = s + *x;
        });
        assert!(s == 5050, 0);
        smart_vector::destroy(v);
    }

    #[test]
    public fun smart_vector_enumerate_ref_test() {
        let v = make_smart_vector();
        smart_vector::enumerate_ref(&v, |i, x| {
            assert!(i == *x, 0);
        });
        smart_vector::destroy(v);
    }

    #[test]
    public fun smart_vector_enumerate_mut_test() {
        let v = make_smart_vector();
        smart_vector::enumerate_mut(&mut v, |i, x| {
            let x: &mut u64 = x;
            assert!(i == *x, 0);
            *x = *x + 1;
        });
        assert!(smart_vector::fold(v, 0, |s, x| {
            s + x
        }) == 5050, 0);
    }

    #[test]
    public fun smart_vector_fold_test() {
        let v = make_smart_vector();
        let i = 0;
        let sum = smart_vector::fold(v, 0, |s, x| {
            assert!(i == x, 0);
            i = i + 1;
            s + x
        });
        assert!(sum == 4950, 0);
    }

    #[test]
    public fun smart_vector_for_foldr_test() {
        let v = make_smart_vector();
        let i = 0;
        let sum = smart_vector::foldr(v, 0, |x, s| {
            assert!(i == 99 - x, i);
            i = i + 1;
            s + x
        });
        assert!(sum == 4950, 0);
    }

    public fun smart_vector_map_test() {
        let v = make_smart_vector();
        let mapped_v = smart_vector::map(v, |x| { x * 2 });
        let sum = smart_vector::fold(mapped_v, 0, |s, x| {
            s + x
        });
        assert!(sum == 9900, 0);
    }

    #[test]
    public fun smart_vector_map_ref_test() {
        let v = make_smart_vector();
        let mapped_v = smart_vector::map_ref(&v, |x|  *x * 2);
        assert!(smart_vector::fold(v, 0, |s, x| {
            s + x
        }) == 4950, 0);
        assert!(smart_vector::fold(mapped_v, 0, |s, x| {
            s + x
        }) == 9900, 0);
    }

    #[test]
    public fun smart_vector_filter_test() {
        let v = make_smart_vector();
        let filtered_v = smart_vector::filter(v, |x| *x % 10 == 0);
        smart_vector::enumerate_ref(&filtered_v, |i, x| {
            assert!(i * 10 == *x, 0);
        });
        smart_vector::destroy(filtered_v);
    }
}
