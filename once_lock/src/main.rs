use std::sync::OnceLock;

static X: OnceLock<u64> = OnceLock::new();

fn calculate_x() -> u64 {
    // Burada uzun müddət tələb edən hesablamalar və ya IO əməliyyatları ola bilər
    println!("Hesablanır...");
    42 // Məsələn, hesablanmış nəticə
}

fn get_x() -> u64 {
    *X.get_or_init(|| calculate_x()) // İlk dəfə `calculate_x()` icra olunur, sonra saxlanılır
}

fn main() {
    let x1 = get_x();
    println!("x1: {}", x1);

    let x2 = get_x(); // Artıq `calculate_x()` icra olunmur, əvvəlki dəyər qaytarılır
    println!("x2: {}", x2);
}
