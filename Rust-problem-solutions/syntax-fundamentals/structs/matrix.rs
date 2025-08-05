//18. Implement a `Matrix` struct with basic mathematical operations.
struct Matrix{
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix{
    fn new(rows: usize, cols: usize)-> Self{
        Self{rows, cols, data: vec![0.0; rows*cols]}
    }

    fn get(&self, row: usize, col: usize)-> f64{
        self.data[row*self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, value: f64){
        self.data[row*self.cols + col] = value;
    }

    fn add(&self, other: &Matrix)-> Matrix{
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows*self.cols{
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }

    fn multiply(&self, other: &Matrix)-> Matrix{
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows{
            for j in 0..other.cols{
                for k in 0..self.cols{}
            }
        }
    }

    fn main(){
        let mut matrix1 = Matrix::new(2, 3);
        matrix1.set(0, 0, 1.0);
        matrix1.set(0, 1, 2.0);
        matrix1.set(0, 2, 3.0);
        matrix1.set(1, 0, 4.0);
        matrix1.set(1, 1, 5.0);
        matrix1.set(1, 2, 6.0);

        let mut matrix2 = Matrix::new(3, 2);
        matrix2.set(0, 0, 7.0);
        matrix2.set(0, 1, 8.0);
        matrix2.set(1, 0, 9.0);
        matrix2.set(1, 1, 10.0);
        matrix2.set(2, 0, 11.0);
        matrix2.set(2, 1, 12.0);

        let result_add = matrix1.add(&matrix2);
        println!("Addition result:");
        for i in 0..result_add.rows{
            for j in 0..result_add.cols{
                print!("{} ", result_add.get(i, j));
            }
            println!();
        }

        let result_multiply = matrix1.multiply(&matrix2);
        println!("Multiplication result:");
        for i in 0..result_multiply.rows{
            for j in 0..result_multiply.cols{
                print!("{} ", result_multiply.get(i, j));
            }
            println!();
        }
    }
}   
}