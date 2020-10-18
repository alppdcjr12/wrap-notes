pub fn make_ascii_titlecase(s: &mut str) {
  if let Some(r) = s.get_mut(0..1) {
    r.make_ascii_uppercase();
  }
}