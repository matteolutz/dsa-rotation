use std::hash::Hash;

use crate::constraint::PersonConstraint;

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

    pub constraints: Vec<PersonConstraint>,
}

impl Person {
    pub fn kl(course_id: CourseId, n: u8, name: Option<&str>) -> Self {
        Self {
            kind: PersonKind::KL { course_id, n },
            name: name.map(|name| name.to_string()),
            constraints: PersonConstraint::default_constraints(),
        }
    }

    pub fn is_pairing_forbidden(&self, other: &Self) -> bool {
        let forbidden_because_kl_collegue = match (self.kind, other.kind) {
            (
                PersonKind::KL { course_id, .. },
                PersonKind::KL {
                    course_id: other_course_id,
                    ..
                },
            ) if course_id == other_course_id => self
                .constraints
                .contains(&PersonConstraint::NotPairedWithCourseLeaderColleague),
            (_, _) => false,
        };

        let forbidden_because_constraint = self
            .constraints
            .contains(&PersonConstraint::NotPairedWith(other.kind));

        forbidden_because_kl_collegue || forbidden_because_constraint
    }

    pub fn is_course_forbidden(&self, course_id: CourseId) -> bool {
        let forbidden_because_own_course = self
            .constraints
            .contains(&PersonConstraint::CantVisitOwnCourse)
            && matches!(self.kind, PersonKind::KL { course_id: c, .. } if c == course_id);

        let forbidden_because_constraint = self
            .constraints
            .contains(&PersonConstraint::NotVisitingCourse(course_id));

        forbidden_because_own_course || forbidden_because_constraint
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
    fn test_pairing_forbidden_default_constraints() {
        let person1 = Person {
            kind: PersonKind::KL { course_id: 1, n: 1 },
            name: Some("Alice".to_string()),
            constraints: PersonConstraint::default_constraints(),
        };
        let person2 = Person {
            kind: PersonKind::KL { course_id: 1, n: 2 },
            name: Some("Bob".to_string()),
            constraints: PersonConstraint::default_constraints(),
        };
        assert!(person1.is_pairing_forbidden(&person2));
    }

    #[test]
    fn test_pairing_not_forbidden_default_constraints() {
        let person1 = Person {
            kind: PersonKind::KL { course_id: 1, n: 1 },
            name: Some("Alice".to_string()),
            constraints: PersonConstraint::default_constraints(),
        };
        let person2 = Person {
            kind: PersonKind::KL { course_id: 2, n: 1 },
            name: Some("Bob".to_string()),
            constraints: PersonConstraint::default_constraints(),
        };
        assert!(!person1.is_pairing_forbidden(&person2));
    }

    #[test]
    fn test_course_forbidden_default_constraints() {
        let kl_person = Person {
            kind: PersonKind::KL { course_id: 0, n: 0 },
            name: None,
            constraints: PersonConstraint::default_constraints(),
        };

        assert!(kl_person.is_course_forbidden(0));
    }

    #[test]
    fn test_course_not_forbidden_default_constraints() {
        let kl_person = Person {
            kind: PersonKind::KL { course_id: 1, n: 0 },
            name: None,
            constraints: PersonConstraint::default_constraints(),
        };

        assert!(!kl_person.is_course_forbidden(0));
    }

    #[test]
    fn test_pairing_forbidden_custom_constraints() {
        let al_person = Person {
            kind: PersonKind::AL,
            name: None,
            constraints: PersonConstraint::default_constraints_with([
                PersonConstraint::NotPairedWith(PersonKind::KuMu),
            ]),
        };

        let kumu_person = Person {
            kind: PersonKind::KuMu,
            name: None,
            constraints: PersonConstraint::default_constraints(),
        };

        assert!(al_person.is_pairing_forbidden(&kumu_person));
    }

    #[test]
    fn test_pairing_not_forbidden_custom_constraints() {
        let al_person = Person {
            kind: PersonKind::AL,
            name: None,
            constraints: PersonConstraint::default_constraints_with([
                PersonConstraint::NotPairedWith(PersonKind::KuMu),
            ]),
        };

        let ass_person = Person {
            kind: PersonKind::Ass { n: 0 },
            name: None,
            constraints: PersonConstraint::default_constraints(),
        };

        assert!(!al_person.is_pairing_forbidden(&ass_person));
    }

    #[test]
    fn test_course_forbidden_custom_constraints() {
        let al_person = Person {
            kind: PersonKind::AL,
            name: None,
            constraints: PersonConstraint::default_constraints_with([
                PersonConstraint::NotVisitingCourse(0),
            ]),
        };

        assert!(al_person.is_course_forbidden(0));
    }

    #[test]
    fn test_course_not_forbidden_custom_constraints() {
        let al_person = Person {
            kind: PersonKind::AL,
            name: None,
            constraints: PersonConstraint::default_constraints_with([
                PersonConstraint::NotVisitingCourse(0),
            ]),
        };

        assert!(!al_person.is_course_forbidden(1));
    }
}
