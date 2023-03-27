#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Scope(pub(self) Vec<u64>);

impl Scope {
    pub fn push(&mut self, value: u64) {
        self.0.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn replace(&mut self, pattern: &Scope, mut replacement: Scope) {
        if pattern.is_empty() {
            replacement.0.extend(self.0.iter());
            self.0 = replacement.0;

            return;
        }

        let mut result = Vec::new();

        let mut index = 0;
        let mut pattern_index = 0;

        let mut no_matches = true;

        while index < self.0.len() {
            if self.0[index] == pattern.0[pattern_index] {
                if pattern_index == pattern.0.len() - 1 {
                    no_matches = false;
                    result.extend(replacement.0.clone());
                    pattern_index = 0;
                } else {
                    pattern_index += 1;
                }
            } else {
                result.push(self.0[index]);
                pattern_index = 0;
            }

            index += 1;
        }

        if no_matches {
            replacement.0.extend(self.0.iter());
            self.0 = replacement.0;
        } else {
            self.0 = result;
        }
    }

    pub fn common_ancestor(&self, other: &Self) -> Scope {
        let longest_length = self.0.len().max(other.0.len());

        if *self.0 == *other.0 {
            return self.clone();
        }

        for index in 0..longest_length {
            if self.0.get(index).is_none() {
                return self.clone();
            } else if other.0.get(index).is_none() {
                return other.clone();
            } else if self.0[index] != other.0[index] {
                return Scope(self.0[0..index].to_vec());
            }
        }

        Scope::default()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::Scope;

    #[test]
    fn common_ancestor_equal_vecs() {
        let ancestor = Scope(vec![1, 2, 3, 4, 5, 6]);

        let decendant = Scope(vec![1, 2, 3, 4, 5, 6]);

        let result = ancestor.common_ancestor(&decendant);

        assert_eq!(result, Scope(vec![1, 2, 3, 4, 5, 6]))
    }

    #[test]
    fn common_ancestor_equal_ancestor() {
        let ancestor = Scope(vec![1, 2, 3, 4]);

        let decendant = Scope(vec![1, 2, 3, 4, 5, 6]);

        let result = ancestor.common_ancestor(&decendant);

        assert_eq!(result, Scope(vec![1, 2, 3, 4]));
    }

    #[test]
    fn common_ancestor_not_subset() {
        let ancestor = Scope(vec![1, 2, 3, 4, 5]);

        let descendant = Scope(vec![1, 2, 3, 7, 8]);

        let result = ancestor.common_ancestor(&descendant);

        assert_eq!(result, Scope(vec![1, 2, 3]));
    }

    #[test]
    fn common_ancestor_not_found() {
        let ancestor = Scope(vec![1, 2, 3, 4, 5, 6]);

        let descendant = Scope(vec![4, 5, 6]);

        let result = ancestor.common_ancestor(&descendant);

        assert_eq!(result, Scope::default());
    }

    #[test]
    fn common_ancestor_no_shared_values() {
        let ancestor = Scope(vec![1, 2, 3]);

        let descendant = Scope(vec![4, 5, 6]);

        let result = ancestor.common_ancestor(&descendant);

        assert_eq!(result, Scope::default());
    }
}
