#[derive(Clone, Copy, Debug)]
pub enum ComplexSph<T> {
    Infinity,
    Number(Complex<T>),
}

impl<T: Num + Add + Clone> Add for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn + rhsn),
            (_, _) => Self::Infinity,
        }
    }
}

impl<T: Num + Sub + Clone> Sub for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn - rhsn),
            (_, _) => Self::Infinity,
        }
    }
}

impl<T: Num + Mul + Clone + Zero> Mul for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn * rhsn),
            (_, _) => Self::Infinity,
        }
    }
}

impl<T: Num + Div + Clone + Zero> Div for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn / rhsn),
            (Self::Number(_), Self::Infinity) => Self::Number(Complex::<T>::zero()),
            (_, _) => Self::Infinity,
        }
    }
}
