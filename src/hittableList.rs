use crate::hittable::Hittable;

struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}
