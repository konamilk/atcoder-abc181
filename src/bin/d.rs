use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        mut s: Chars
    }

    let mut ans = "No";

    let eight_baisu =
        (0..125)
            .collect::<Vec<i32>>()
            .into_iter()
            .map(|x| x * 8 + 100000)
            .collect::<Vec<i32>>();

    let mut eight_set = vec![];

    for i in eight_baisu {
        let num1 = i % 10;
        let num2 = (i / 10) % 10;
        let num3 = (i / 100) % 10;
        eight_set.push((num1, num2, num3))
    }

    if s.len() == 1 {
        if s[0] == '8'{
            ans = "Yes"
        }
    }

    if s.len() == 2 {
        if ((s[0] as i32 - 48) * 10 + (s[1] as i32 - 48)) % 8 == 0{
            ans = "Yes"
        }
        if ((s[1] as i32 - 48) * 10 + (s[0] as i32 - 48)) % 8 == 0{
            ans = "Yes"
        }
    }

    let s_as_nums = s.into_iter()
        .map(|x| x as i32 - 48)
        .collect::<Vec<i32>>();

    let mut count = vec![0;10];
    for num in s_as_nums {
        count[num as usize] += 1
    }

    for (num1, num2, num3) in eight_set {
        let mut required = vec![0;10];
        required[num1 as usize] += 1;
        required[num2 as usize] += 1;
        required[num3 as usize] += 1;

        let mut flg = true;

        for j in 0..10{
            if count[j] < required[j] {
                flg = false;
            }
        }

        if flg{
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans)
}
