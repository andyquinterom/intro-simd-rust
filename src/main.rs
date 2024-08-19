use core::arch::x86_64::{
    __m256i, _mm256_cmpeq_epi32, _mm256_loadu_si256, _mm256_or_si256, _mm256_set1_epi32,
    _mm256_storeu_si256,
};

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Animal {
    Cow = 0,
    Dog = 1,
    Cat = 3,
    Bat = 4,
}

struct DataFrame {
    data: Vec<Vec<Animal>>,
    nrow: usize,
}

fn benchmark(title: &str, df: &DataFrame, value: Animal, f: impl Fn(&DataFrame, Animal)) {
    const BENCH_ITERS: u32 = 100;
    let mut time = std::time::Duration::from_secs(0);
    // WARM UP
    for _ in 0..BENCH_ITERS / 3 {
        f(df, value);
    }
    for _ in 0..BENCH_ITERS {
        let start = std::time::Instant::now();
        f(df, value);
        time += start.elapsed();
    }

    let average_time = time / BENCH_ITERS;
    println!("{title} | Average time: {:?}", average_time);
}
fn generate_mock_df() -> DataFrame {
    const REP: usize = 100_000;
    let col1 = [Animal::Cow, Animal::Dog, Animal::Cat, Animal::Bat].repeat(REP);
    let col2 = [Animal::Dog, Animal::Dog, Animal::Dog, Animal::Bat].repeat(REP);
    let col3 = [Animal::Bat, Animal::Dog, Animal::Dog, Animal::Bat].repeat(REP);
    let col4 = [Animal::Cat, Animal::Cat, Animal::Cat, Animal::Bat].repeat(REP);
    DataFrame {
        data: vec![col1, col2, col3, col4],
        nrow: 4 * REP,
    }
}

fn naive_filter_implementation(df: &DataFrame, value: Animal) {
    // create the output vector and set it to false
    let mut should_include = vec![false; df.nrow];
    for col in &df.data {
        for row in 0..df.nrow {
            should_include[row] |= col[row] == value;
        }
    }
}

#[inline(always)]
unsafe fn simd_filter_compare_and_store(
    should_include: &mut [i32],
    col: &[Animal],
    row: usize,
    value: __m256i,
) {
    let row_values = _mm256_loadu_si256(col[row..].as_ptr() as *const __m256i);
    let result = _mm256_or_si256(
        _mm256_loadu_si256(should_include[row..].as_ptr() as *const __m256i),
        _mm256_cmpeq_epi32(row_values, value),
    );
    _mm256_storeu_si256(should_include[row..].as_mut_ptr() as *mut __m256i, result)
}

fn simd_filter_implementation(df: &DataFrame, value: Animal) {
    // create the output vector and set it to false
    let value_x8 = unsafe { _mm256_set1_epi32(value as i32) };
    let mut should_include = vec![-1i32; df.nrow];
    let nrow = df.nrow;
    for col in &df.data {
        let mut row = 0;
        while row < nrow {
            unsafe {
                simd_filter_compare_and_store(&mut should_include, col, row, value_x8);
                simd_filter_compare_and_store(&mut should_include, col, row + 8, value_x8);
                simd_filter_compare_and_store(&mut should_include, col, row + 16, value_x8);
                simd_filter_compare_and_store(&mut should_include, col, row + 24, value_x8);
            }
            row += 32;
        }
        // This will remove what was added on the last iteration of the while loop
        row -= 32;
        while row < df.nrow {
            let row_value = col[row];
            let result = should_include[row] | -((row_value == value) as i32);
            should_include[row] = result;
            row += 1;
        }
    }
}

fn main() {
    let mock_df = generate_mock_df();
    benchmark("Naive", &mock_df, Animal::Bat, naive_filter_implementation);
    benchmark("SIMD", &mock_df, Animal::Bat, simd_filter_implementation);
}
