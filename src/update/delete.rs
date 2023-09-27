use std::num::NonZeroU32;

use crate::{Avltriee, AvltrieeNode};

impl<T> Avltriee<T> {
    #[inline(always)]
    fn delete_same(&mut self, delete_node: &AvltrieeNode<T>) {
        let new_node = unsafe { self.offset_mut(delete_node.same) };

        new_node.parent = delete_node.parent;
        new_node.height = delete_node.height;

        new_node.left = delete_node.left;
        self.set_parent(new_node.left, delete_node.same);

        new_node.right = delete_node.right;
        self.set_parent(new_node.right, delete_node.same);
    }

    #[inline(always)]
    unsafe fn delete_intermediate(
        &mut self,
        delete_node: &mut AvltrieeNode<T>,
    ) -> (NonZeroU32, NonZeroU32) {
        let left_max_row = self.max(delete_node.left);
        let left_max = self.offset_mut(left_max_row);

        left_max.right = delete_node.right;
        self.offset_mut(left_max.right).parent = left_max_row;

        if delete_node.left == left_max_row {
            left_max.parent = delete_node.parent;
            self.calc_height_node(left_max);
            let left_max_row = NonZeroU32::new_unchecked(left_max_row);
            (left_max_row, left_max_row)
        } else {
            left_max.height = delete_node.height;

            let left_max_parent_row = left_max.parent;
            let left_max_parent = self.offset_mut(left_max_parent_row);

            left_max_parent.right = left_max.left;
            self.set_parent(left_max_parent.right, left_max_parent_row);

            left_max.left = delete_node.left;
            self.offset_mut(left_max.left).parent = left_max_row;

            (
                NonZeroU32::new_unchecked(left_max_row),
                NonZeroU32::new_unchecked(left_max_parent_row),
            )
        }
    }

    #[inline(always)]
    pub unsafe fn delete(&mut self, target_row: u32) {
        assert!(target_row > 0);
        let delete_node = self.offset_mut(target_row);
        if delete_node.height > 0 {
            let row_parent = delete_node.parent;
            if row_parent == 0 {
                if delete_node.same != 0 {
                    self.set_root(delete_node.same);
                    self.delete_same(delete_node);
                } else if delete_node.left == 0 {
                    self.set_root(delete_node.right);
                    self.set_parent(delete_node.right, 0);
                } else if delete_node.right == 0 {
                    self.set_root(delete_node.left);
                    self.offset_mut(delete_node.left).parent = 0;
                } else {
                    let (new_row, balance_row) = self.delete_intermediate(delete_node);
                    self.set_root(new_row.get());
                    let node = self.offset_mut(new_row.get());
                    node.parent = 0;
                    self.calc_height(balance_row);
                    self.balance(balance_row);
                }
            } else {
                let mut parent = self.offset_mut(row_parent);
                if parent.same == target_row {
                    parent.same = delete_node.same;
                    if delete_node.same != 0 {
                        self.delete_same(delete_node);
                    }
                } else if delete_node.same != 0 {
                    Self::join_intermediate(
                        parent,
                        NonZeroU32::new_unchecked(target_row),
                        delete_node.same,
                    );
                    self.delete_same(delete_node);
                } else if delete_node.left == 0 {
                    Self::join_intermediate(
                        &mut parent,
                        NonZeroU32::new_unchecked(target_row),
                        delete_node.right,
                    );
                    self.set_parent(delete_node.right, row_parent);
                    self.balance(NonZeroU32::new_unchecked(row_parent));
                } else if delete_node.right == 0 {
                    Self::join_intermediate(
                        parent,
                        NonZeroU32::new_unchecked(target_row),
                        delete_node.left,
                    );
                    self.offset_mut(delete_node.left).parent = row_parent;
                    self.balance(NonZeroU32::new_unchecked(row_parent));
                } else {
                    let (new_row, balance_row) = self.delete_intermediate(delete_node);
                    Self::join_intermediate(
                        parent,
                        NonZeroU32::new_unchecked(target_row),
                        new_row.get(),
                    );
                    let node = self.offset_mut(new_row.get());
                    node.height = delete_node.height;
                    node.parent = row_parent;
                    self.calc_height(balance_row);
                    self.balance(balance_row);
                }
            }
            delete_node.height = 0;
        }
    }
}
