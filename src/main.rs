use ndarray::{ArrayViewMut, ShapeBuilder};
fn main() {
    let size = 4;
    let mut new_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let nd = ArrayViewMut::from_shape((2, 2).strides((1, 2)), &mut new_data[..=size]).unwrap();
    let nd = nd.as_standard_layout();
    let nd_falt = nd.into_shape(4).unwrap();
    let mut nd_vec = nd_falt.to_owned().into_raw_vec();
    nd_vec.extend(&new_data[size..]);
    dbg!(nd_vec);
}
