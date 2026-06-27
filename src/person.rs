use std::hash::Hash;

pub type CourseId = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PersonKind {
    AL,
    Ass { n: u8 },
    KuMu,
    KL { course_id: CourseId, n: u8 },
}

impl std::fmt::Display for PersonKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonKind::AL => write!(f, "AL"),
            PersonKind::Ass { n } => write!(f, "{}. Ass", n + 1),
            PersonKind::KuMu => write!(f, "KuMu"),
            PersonKind::KL { course_id, n } => write!(f, "{}. KL (K{})", n + 1, course_id + 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Person {
    pub kind: PersonKind,
    pub name: Option<String>,
}

impl Person {
    pub fn kl(course_id: CourseId, n: u8, name: Option<&str>) -> Self {
        Self {
            kind: PersonKind::KL { course_id, n },
            name: name.map(|name| name.to_string()),
        }
    }

    pub fn is_forbidden_with(&self, other: &Self) -> bool {
        match (self.kind, other.kind) {
            (
                PersonKind::KL { course_id, .. },
                PersonKind::KL {
                    course_id: other_course_id,
                    ..
                },
            ) if course_id == other_course_id => true,
            (_, _) => false,
        }
    }

    pub fn is_in_course(&self, course_id: CourseId) -> bool {
        matches!(self.kind, PersonKind::KL { course_id: c, .. } if c == course_id)
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name.as_ref() {
            write!(f, "{} ({})", name, self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Person {}

impl Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_forbidden_with() {
        let person1 = Person {
            kind: PersonKind::KL { course_id: 1, n: 1 },
            name: Some("Alice".to_string()),
        };
        let person2 = Person {
            kind: PersonKind::KL { course_id: 1, n: 2 },
            name: Some("Bob".to_string()),
        };
        assert!(person1.is_forbidden_with(&person2));
    }

    #[test]
    fn test_is_not_forbidden_with() {
        let person1 = Person {
            kind: PersonKind::KL { course_id: 1, n: 1 },
            name: Some("Alice".to_string()),
        };
        let person2 = Person {
            kind: PersonKind::KL { course_id: 2, n: 1 },
            name: Some("Bob".to_string()),
        };
        assert!(!person1.is_forbidden_with(&person2));
    }
}
