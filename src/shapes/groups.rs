use crate::intersection::Intersectable;
use crate::intersection::Intersections;
use crate::matrix::Mat4;
use crate::object::LocalIntersect;
use crate::object::Shape;
use crate::ray::Ray;
use crate::tuple::Tuple;
use lazy_static::lazy_static;
use rayon::vec;
use slotmap::new_key_type;
use slotmap::Key;
use slotmap::SlotMap;
// use std::sync::RwLock;
use parking_lot::RwLock;

lazy_static! {
    static ref GROUPS: RwLock<SlotMap<Group, GroupData>> = RwLock::new(SlotMap::with_key());
}

new_key_type! {
    pub struct Group;
}
struct GroupData {
    transform: Mat4,
    shapes: Vec<Shape>,
    children: Vec<Group>,
    parent: Group,
}

impl Intersectable for Group {
    fn intersects(&self, r: Ray) -> Intersections {
        if self.is_empty() {
            return Intersections::new_none();
        }
        let groups = GROUPS.read_recursive();
        let group = groups.get(*self).unwrap();
        let mut xs = Intersections::new_none();
        for shape in &group.shapes {
            xs.extend(shape.intersects(r));
        }
        xs
    }
}

// pub struct Group(slotmap::KeyData);

// impl slotmap::Key for Group {}

impl Group {
    pub fn transform(&self) -> Mat4 {
        let groups = GROUPS.read_recursive();
        groups.get(*self).unwrap().transform
    }

    pub fn is_empty(&self) -> bool {
        let groups = GROUPS.read_recursive();
        let group = groups.get(*self).unwrap();
        group.shapes.is_empty() && group.children.is_empty()
    }

    pub fn add_child(&self, child: Group) {
        let mut groups = GROUPS.write();
        let group = groups.get_mut(*self).unwrap();
        group.children.push(child);
        let child = groups.get_mut(child).unwrap();
        child.parent = *self
    }

    pub fn get_child(&self, index: usize) -> Group {
        let groups = GROUPS.read_recursive();
        groups.get(*self).unwrap().children[index]
    }

    pub fn parent(&self) -> Group {
        GROUPS.read_recursive().get(*self).unwrap().parent
    }

    pub fn new() -> Self {
        let group = GroupData {
            transform: Mat4::identity(),
            children: vec![],
            parent: Group::null(),
            shapes: vec![],
        };

        let handle = GROUPS.write().insert(group);
        handle
    }
}

#[cfg(test)]
mod tests {
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
    fn add_child_to_group() {}
}
