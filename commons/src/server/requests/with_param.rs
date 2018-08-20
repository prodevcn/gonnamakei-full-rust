pub struct RequestWithParam<P, T> {
    pub param: P,
    pub body: T,
}

impl<P, T> RequestWithParam<P, T> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(param: P, body: T) -> Self {
        RequestWithParam { param, body }
    }
}
