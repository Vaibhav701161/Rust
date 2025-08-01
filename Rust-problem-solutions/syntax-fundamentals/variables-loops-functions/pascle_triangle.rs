fn pascals_triangle(n: usize) {
    let mut triangle = vec![vec![1]];

    for i in 1..n {
        let mut row = vec![1];
        for j in 1..i {
            let val = triangle[i - 1][j - 1] + triangle[i - 1][j];
            row.push(val);
        }
        row.push(1);
        triangle.push(row);
    }

    for row in triangle {
        println!("{:?}", row);
    }
}

fn main() {
    pascals_triangle(5);
}
