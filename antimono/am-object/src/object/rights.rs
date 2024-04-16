use bitflags::bitflags;

bitflags! {
  /// TODO
  pub struct Rights:u64 {
    
    const TRANSFER = 1 << 1;
  }
}
