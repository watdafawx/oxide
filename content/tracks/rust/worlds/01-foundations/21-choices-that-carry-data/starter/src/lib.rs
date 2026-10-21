pub enum Shipping {
    Pickup,
    Courier(u32),
    Post(u32),
}

/// Days until it arrives: Pickup 0, Courier(days) days, Post(days) days + 2 for the sorting office.
pub fn days_to_arrive(shipping: Shipping) -> u32 {
    // TODO: match shipping, naming the data in the Courier and Post arms
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn courier_takes_its_days() {
        assert_eq!(days_to_arrive(Shipping::Courier(3)), 3);
        assert_eq!(days_to_arrive(Shipping::Courier(1)), 1);
    }

    #[test]
    fn post_adds_two_days_and_pickup_is_instant() {
        assert_eq!(days_to_arrive(Shipping::Post(3)), 5);
        assert_eq!(days_to_arrive(Shipping::Pickup), 0);
    }
}
