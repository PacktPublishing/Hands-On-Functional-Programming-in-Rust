fn main() {
   let mut res: Vec<Vec<u64>> = vec![vec![0; 512]; 512];
   for _ in 0..50 {
      for i in 1..511 {
         for j in 1..511 {
            res[j][i] = 2;
            res[j][i] += res[j-1][i-1];
            res[j][i] += res[j][i-1];
            res[j][i] += res[j+1][i-1];
            res[j][i] += res[j-1][i];
            res[j][i] += res[j][i];
            res[j][i] += res[j+1][i];
            res[j][i] += res[j-1][i+1];
            res[j][i] += res[j][i+1];
            res[j][i] += res[j+1][i+1];
            res[j][i] /= 9;
         }
      }
   }
}
