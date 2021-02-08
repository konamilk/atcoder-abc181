use proconio::input;

fn main() {
    input! {
        n: i32,
        ab: [(i64, i64); n]
    }

    let mut ans = 0i64;

    for (a, b) in ab{
        ans += (a + b) * (b - a + 1) /2
    }

    println!("{}", ans)
}
