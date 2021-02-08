use proconio::input;

fn main() {
    input!{
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut ans = "No";

    for i in 0..n-2 {
        for j in i+1..n-1{
            for k in j+1..n{
                let x0 = xy[i].0;
                let y0 = xy[i].1;
                let x1 = xy[j].0;
                let y1 = xy[j].1;
                let x2 = xy[k].0;
                let y2 = xy[k].1;

                let v1x = x1 - x0;
                let v1y = y1 - y0;
                let v2x = x2 - x0;
                let v2y = y2 - y0;

                if (v1x * v2x + v1y * v2y).pow(2) == (v1x.pow(2) + v1y.pow(2)) * (v2x.pow(2) + v2y.pow(2)) {
                    ans = "Yes";
                }
            }
        }
    }

    println!("{}", ans)
}
