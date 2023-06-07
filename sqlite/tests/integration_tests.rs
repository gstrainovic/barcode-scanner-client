use sqlite::{create_history, establish_connection};

#[test]
fn test_create_history() {
  let mut conn = establish_connection();
  for i in 0..2000 {
    let status = format!("status{}", i);
    let barcode = format!("barcode{}", i);
    let timestamp = format!("timestamp{}", i);
    create_history(&mut conn, &status, &barcode, &timestamp);
  }
}


