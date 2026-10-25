use std::rc::{Rc, Weak};

/// A machine's link to its pole, which does not keep the pole standing.
pub fn link_to(pole: &Rc<String>) -> Weak<String> {
    // TODO: a Weak to the pole, made with Rc::downgrade
    todo!()
}

/// The pole's name while it still stands, or None once it is removed.
pub fn pole_still_there(link: &Weak<String>) -> Option<String> {
    // TODO: upgrade the link; Some(pole) gives Some(copy of its name), None gives None
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linking_does_not_add_an_owner() {
        let pole = Rc::new(String::from("pole-north"));
        let link = link_to(&pole);
        assert_eq!(Rc::strong_count(&pole), 1, "a Weak is not an owner");
        assert_eq!(Rc::weak_count(&pole), 1);
        assert!(link.upgrade().is_some(), "the link must point at the pole");
    }

    #[test]
    fn pole_still_there_is_none_once_removed() {
        let pole = Rc::new(String::from("pole-north"));
        let link = Rc::downgrade(&pole);
        assert_eq!(pole_still_there(&link), Some(String::from("pole-north")));
        drop(pole);
        assert_eq!(pole_still_there(&link), None, "the pole was removed, upgrade() gives None");
    }
}
