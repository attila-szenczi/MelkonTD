//TODO: No Send + Sync after amethyst removal
pub trait MortalTrait: Send + Sync {
  fn dead(&self) -> bool;
}
