use crate::bounds::Bounded;
use crate::bounds::BoundingBox;
use crate::intersection::Intersectable;
use crate::intersection::Intersections;
use crate::matrix::Mat4;

use crate::matrix::MatBase;
use crate::object::Shape;
use crate::ray::Ray;
use crate::tuple::Tuple;
use lazy_static::lazy_static;

use parking_lot::RwLock;
use slotmap::new_key_type;
use slotmap::Key;
use slotmap::SlotMap;

lazy_static! {
    static ref ALL_GROUPS: RwLock<SlotMap<Group, GroupData>> = RwLock::new(SlotMap::with_key());
}

new_key_type! {
    pub struct Group;
}
struct GroupData {
    transform: Mat4,
    shapes: Vec<Shape>,
    children: Vec<Group>,
    parent: Group,
    bounds: Option<BoundingBox>,
}

impl Intersectable for Group {
    fn intersects(&self, r: Ray) -> Intersections {
        self.local_intersect(r.transform(self.transform().inverse()))
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        unimplemented!()
    }
}

// pub struct Group(slotmap::KeyData);

// impl slotmap::Key for Group {}

impl Group {
    pub fn len(&self) -> usize {
        let groups = ALL_GROUPS.read_recursive();
        let group = groups.get(*self).unwrap();
        group.children.len() + group.shapes.len()
    }

    pub fn transform(&self) -> Mat4 {
        let groups = ALL_GROUPS.read_recursive();
        groups.get(*self).unwrap().transform
    }

    pub fn with_transform(self, transform: Mat4) -> Self {
        let mut groups = ALL_GROUPS.write();
        let group = groups.get_mut(self).unwrap();
        group.transform = transform;
        self
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        let mut groups = ALL_GROUPS.write();
        let group = groups.get_mut(*self).unwrap();
        group.transform = transform;
    }

    pub fn shapes(&self) -> Vec<Shape> {
        let groups = ALL_GROUPS.read_recursive();
        groups.get(*self).unwrap().shapes.clone()
    }

    pub fn is_empty(&self) -> bool {
        let groups = ALL_GROUPS.read_recursive();
        let group = groups.get(*self).unwrap();
        group.shapes.is_empty() && group.children.is_empty()
    }

    pub fn add_shape(&self, mut shape: Shape) -> Shape {
        let mut groups = ALL_GROUPS.write();
        let group = groups.get_mut(*self).unwrap();
        shape.parent = *self;
        group.shapes.push(shape);
        shape
    }
    pub fn add_child(&self, child: Group) {
        let mut groups = ALL_GROUPS.write();
        let group = groups.get_mut(*self).unwrap();
        group.children.push(child);
        let child = groups.get_mut(child).unwrap();
        child.parent = *self
    }

    pub fn get_child(&self, index: usize) -> Group {
        let groups = ALL_GROUPS.read_recursive();
        groups.get(*self).unwrap().children[index]
    }

    pub fn parent(&self) -> Group {
        ALL_GROUPS.read_recursive().get(*self).unwrap().parent
    }

    pub fn new() -> Self {
        let group = GroupData {
            transform: Mat4::identity(),
            children: vec![],
            parent: Group::null(),
            shapes: vec![],
            bounds: None,
        };

        let handle = ALL_GROUPS.write().insert(group);
        handle
    }

    pub fn local_intersect(&self, r: Ray) -> Intersections {
        if self.is_empty() {
            return Intersections::new_none();
        }
        let groups = ALL_GROUPS.read_recursive();
        let group = groups.get(*self).unwrap();
        let mut xs = Intersections::new_none();
        for shape in &group.shapes {
            xs.extend(shape.intersects(r));
        }
        xs
    }

    pub fn world_to_object(&self, point: Tuple) -> Tuple {
        let parent = self.parent();
        let point = if !parent.is_null() {
            parent.world_to_object(point)
        } else {
            point
        };
        self.transform().inverse() * point
    }

    pub fn normal_to_world(&self, normal: Tuple) -> Tuple {
        let inverse = self.transform().inverse().transpose();
        let normal = ((inverse * normal).into_vector()).norm();
        let parent = self.parent();
        if !parent.is_null() {
            parent.normal_to_world(normal)
        } else {
            normal
        }
    }

    pub fn make_bounds(&self, group: &GroupData) -> BoundingBox {
        let mut bb = BoundingBox::default();
        for shape in &group.shapes {
            bb.add_box(shape.bounds())
        }
        for child in &group.children {
            if group.parent.is_null() {
                bb = bb.transform(group.transform);
            }
            bb.add_box(child.bounds());
        }

        bb
    }
}

impl From<Shape> for Group {
    fn from(value: Shape) -> Self {
        Group::new().with_transform(value.transform)
    }
}

impl Bounded for Group {
    fn bounds(&self) -> crate::bounds::BoundingBox {
        let groups = ALL_GROUPS.read_recursive();
        let group = groups.get(*self).unwrap();
        if let Some(bb) = group.bounds {
            bb
        } else {
            self.make_bounds(group)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        transformations::{rot_y, scaling, translation},
        tuple::{point, vector},
    };

    use super::*;

    #[test]
    fn create_new_group() {
        let group = Group::new();
        assert_eq!(group.transform(), Mat4::identity());
        assert!(group.is_empty());
    }

    #[test]
    fn group_has_parent() {
        let group = Group::new();
        assert_eq!(group.parent(), Group::null())
    }

    #[test]
    fn add_child_to_group() {
        let group = Group::new();
        let s = Shape::default_shape();
        group.add_shape(s);
        assert_eq!(group.len(), 1);
        assert_eq!(group.shapes()[0].parent, group);
    }
    #[test]
    fn local_intersect_group() {
        let g = Group::new();
        let r = Ray::new(point(0., 0., 0.), vector(0.0, 0.0, 1.0));
        let xs = g.local_intersect(r);
        assert_eq!(xs.len(), 0)
    }

    #[test]
    fn local_intersect_non_empty_group() {
        let g = Group::new();
        let s1 = Shape::sphere();
        let s2 = Shape::sphere().with_transform(translation(0., 0., -3.));
        let s3 = Shape::sphere().with_transform(translation(5., 0., 0.));
        g.add_shape(s1);
        g.add_shape(s2);
        g.add_shape(s3);
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = g.local_intersect(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, s2);
        assert_eq!(xs[1].object, s2);
        assert_eq!(xs[2].object, s1);
        assert_eq!(xs[3].object, s1);
    }

    #[test]
    fn intersect_transformed_group() {
        let g = Group::new().with_transform(scaling(2.0, 2.0, 2.0));
        let s = Shape::sphere().with_transform(translation(5.0, 0.0, 0.0));
        g.add_shape(s);
        let r = Ray::new(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersects(r);
        assert_eq!(xs.len(), 2)
    }

    #[test]
    fn point_from_world_to_object_space() {
        let g1 = Group::new().with_transform(rot_y(PI / 2.));
        let g2 = Group::new().with_transform(scaling(2., 2., 2.));
        let s = Shape::sphere().with_transform(translation(5., 0., 0.));
        let s = g2.add_shape(s.into());
        g1.add_child(g2);
        let p = point(-2., 0., -10.);
        assert_eq!(s.world_to_object(p), (point(0., 0., -1.)));
    }

    #[test]
    fn object_to_world_space() {
        let g1 = Group::new().with_transform(rot_y(PI / 2.));
        let g2 = Group::new().with_transform(scaling(1., 2., 3.));
        let s = Shape::sphere().with_transform(translation(5., 0., 0.));
        let s = g2.add_shape(s.into());
        g1.add_child(g2);
        let n = vector(f64::sqrt(3.) / 3., f64::sqrt(3.) / 3., f64::sqrt(3.) / 3.);
        assert_eq!(s.normal_to_world(n), vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    fn normal_group() {
        let g1 = Group::new().with_transform(rot_y(PI / 2.));
        let g2 = Group::new().with_transform(scaling(1., 2., 3.));
        let s = Shape::sphere().with_transform(translation(5., 0., 0.));
        let s = g2.add_shape(s.into());
        g1.add_child(g2);
        let p = point(1.7321, 1.1547, -5.5774);
        assert_eq!(s.normal_at(&p), vector(0.2857, 0.4286, -0.8571));
    }
}
