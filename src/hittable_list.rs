use crate::hittable::Hittable;

use crate::hittable::Hit;
use crate::ray::Ray;

type HittableElement = Box<dyn Hittable>;
struct HittableList(Vec<HittableElement>);

impl HittableList {
    fn clear(&mut self) {
        self.0.clear();
    }
    fn add(&mut self, obj: HittableElement) {
        self.0.push(obj);
    }
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_hit = None;
        for hittable in self.0.iter() {
            let closest_so_far = if let Some(Hit { ray_t, .. }) = closest_hit {
                ray_t
            } else {
                t_max
            };
            if let Some(hit) = hittable.hit(&ray, t_min, closest_so_far) {
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
