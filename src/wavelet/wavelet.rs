pub fn wavelet(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let n = data.len();
    if n == 0 {
        return (Vec::new(), Vec::new())
    }
    let even: Vec<i32> = data.iter().step_by(2).copied().collect() ;
    let odd: Vec<i32> = data.iter().skip(1).step_by(2).copied().collect() ;

    let even_len = even.len();
    let odd_len = odd.len();

    let mut high = Vec::with_capacity(odd_len);

    for i in 0..odd_len {
        let left = even[i];
        let right = even[(i + 1).min(even_len - 1)];
        let prediction = (left + right) >> 1;
        high.push(odd[i] - prediction);
    }

    let mut low = Vec::with_capacity(even_len);

    for i in 0..even_len {
        let left = if i == 0 {
            high.first().copied().unwrap_or(0)
        } else {
            high[i - 1]
        };

        let right = high.get(i).copied()
            .unwrap_or_else(|| high.last().copied().unwrap_or(0));

        let update = (left + right + 2).div_euclid(4);

        low.push(even[i] + update);
    }
    (low, high)

}