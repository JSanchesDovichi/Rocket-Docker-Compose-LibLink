use dlib_face_recognition::FaceEncoding;

fn main() {
    println!("Hello from the Crate 2!");

    let encoding = FaceEncoding::new_from_scalar(7.0);

    println!("{encoding:?}");
}