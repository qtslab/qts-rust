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
    pub best_solution: Solution,
    pub best_value: f64,
}

impl Record {
    pub fn new(file: String) -> Record {
        Record {
            file,
            iterations: Vec::new(),
            best_solution: Vec::new(),
            best_value: 0.0,
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

    pub fn add_global_best(&mut self, value: f64, best_solution: Solution) {
        self.best_value = value;
        self.best_solution = best_solution;
    }

    pub fn write_file_global_best(&self) -> std::io::Result<()> {
        let file = File::create(&self.file)?;
        let mut writer = std::io::BufWriter::new(file);
        writeln!(writer, "{}", self.best_value)?;
        for i in 0..self.best_solution.len() {
            write!(writer, "{} ", self.best_solution[i])?;
        }
        writeln!(writer)?;
        writer.flush()?;
        Ok(())
    }
}
