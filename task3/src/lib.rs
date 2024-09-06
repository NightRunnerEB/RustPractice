use std::sync::{Arc, Mutex};
use std::thread;

pub fn multiply_matrices(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>, num_threads: usize) -> Vec<Vec<i32>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    // Создаем результирующую матрицу, заполненную нулями
    let result = Arc::new(Mutex::new(vec![vec![0; cols_b]; rows_a]));

    let mut handles = vec![];

    // Определяем диапазоны строк, которые будет обрабатывать каждый поток
    let chunk_size = (rows_a + num_threads - 1) / num_threads; // округление вверх

    for thread_id in 0..num_threads {
        let a = a.clone();
        let b = b.clone();
        let result = Arc::clone(&result);

        let start_row = thread_id * chunk_size;
        let end_row = usize::min(start_row + chunk_size, rows_a);

        // Создаем поток для обработки строк
        let handle = thread::spawn(move || {
            for i in start_row..end_row {
                for j in 0..cols_b {
                    let mut sum = 0;
                    for k in 0..cols_a {
                        sum += a[i][k] * b[k][j];
                    }

                    let mut result = result.lock().unwrap();
                    result[i][j] = sum;
                }
            }
        });

        handles.push(handle);
    }

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}
