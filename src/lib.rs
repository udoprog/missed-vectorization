pub fn some_unsafe(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() % 8 == 0);

    unsafe {
        let mut acc0 = 0.0;
        let mut acc1 = 0.0;
        let mut acc2 = 0.0;
        let mut acc3 = 0.0;
        let mut acc4 = 0.0;
        let mut acc5 = 0.0;
        let mut acc6 = 0.0;
        let mut acc7 = 0.0;

        let mut idx = 0;

        for _ in 0..a.len() / 8 {
            acc0 += *a.get_unchecked(idx) * *b.get_unchecked(idx);
            acc1 += *a.get_unchecked(idx + 1) * *b.get_unchecked(idx + 1);
            acc2 += *a.get_unchecked(idx + 2) * *b.get_unchecked(idx + 2);
            acc3 += *a.get_unchecked(idx + 3) * *b.get_unchecked(idx + 3);
            acc4 += *a.get_unchecked(idx + 4) * *b.get_unchecked(idx + 4);
            acc5 += *a.get_unchecked(idx + 5) * *b.get_unchecked(idx + 5);
            acc6 += *a.get_unchecked(idx + 6) * *b.get_unchecked(idx + 6);
            acc7 += *a.get_unchecked(idx + 7) * *b.get_unchecked(idx + 7);
            idx += 8;
        }

        acc0 + acc1 + acc2 + acc3 + acc4 + acc5 + acc6 + acc7
    }
}

pub fn fully_safe(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() % 8 == 0);

    a.iter().zip(b).map(|(a, b)| *a * *b).sum()
}
