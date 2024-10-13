use std::fs::File;
use std::io::Write;

use crate::types::Qubits;
use crate::types::Solution;

pub struct Iteration {
    pub iter: i32,
    pub value: f64,
    pub weight: f64,
    pub best_solution: Solution,
    pub qubits: Qubits,
}

pub struct Record {
    pub file: String,
    pub iterations: Vec<Iteration>,
}

impl Record {
    pub fn new(file: String) -> Record {
        Record {
            file,
            iterations: Vec::new(),
        }
    }

    pub fn add_iteration(
        &mut self,
        iter: i32,
        value: f64,
        weight: f64,
        best_solution: Solution,
        qubits: Qubits,
    ) {
        self.iterations.push(Iteration {
            iter,
            value,
            weight,
            best_solution,
            qubits,
        });
    }

    pub fn write_file(&self) -> std::io::Result<()> {
        let file = File::create(&self.file)?;
        let mut writer = std::io::BufWriter::new(file);
        for i in 0..self.iterations.len() {
            writeln!(
                writer,
                "{} {} {} {}",
                self.iterations[i].iter,
                self.iterations[i].value,
                self.iterations[i].weight,
                self.iterations[i].best_solution.len()
            )?;
            for j in 0..self.iterations[i].best_solution.len() {
                write!(writer, "{} ", self.iterations[i].best_solution[j])?;
            }
            writeln!(writer)?;
        }
        writer.flush()?;
        Ok(())
    }
}
