enum Behave {
    Better,
    Good,
    Bad,
    Worst
}

impl Behave {
    fn is_good(&self) {
        match &self {
            Behave::Good | Behave::Better => println!("{}", true),
            _ => println!("{}", false)
        }
    }

    fn is_bad(&self) {
        match &self {
            Behave::Bad | Behave::Worst => println!("{}", true),
            _ => println!("{}", false)
        }
    }
}

fn main() {
    Behave::Better.is_good();
    Behave::Bad.is_good();
    Behave::Worst.is_bad();
    Behave::Good.is_bad();
}
