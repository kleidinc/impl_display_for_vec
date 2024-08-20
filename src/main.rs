use std::fmt;

fn main() {
    let mut unprintable_vec: Vec<String> = vec!["Dmitry Mendeleyev".to_string()];
    unprintable_vec.push("Yuri Gagarin".to_string());

    let mut number_vec: Vec<i32> = vec![1, 2, 3];
    number_vec.push(15);

    struct PrintableVec(Vec<String>);
    struct GenericPrintableVec<T>(Vec<T>);

    impl fmt::Display for PrintableVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let v = &self.0;
            write!(f, "[")?;
            for (index, element) in v.iter().enumerate() {
                if index != 0 {
                    write!(f, " ,")?;
                }
                write!(f, " {},", element)?;
            }
            write!(f, "]")
        }
    }

    // You need to constrain the T
    impl<T: fmt::Display> fmt::Display for GenericPrintableVec<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let v = &self.0;
            write!(f, "[")?;
            for (index, element) in v.iter().enumerate() {
                if index != 0 {
                    write!(f, " ,")?;
                }
                write!(f, "{}", element)?;
            }
            write!(f, "]")
        }
    }

    println!(
        "the printable vector is {}",
        GenericPrintableVec(unprintable_vec)
    );
    println!(
        "The printable vector is {}",
        GenericPrintableVec(number_vec)
    );
}
