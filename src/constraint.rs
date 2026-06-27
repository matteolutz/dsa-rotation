use crate::person::{CourseId, PersonKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonConstraint {
    NotPairedWithCourseLeaderColleague,
    CantVisitOwnCourse,

    NotPairedWith(PersonKind),
    NotVisitingCourse(CourseId),
}

impl PersonConstraint {
    pub fn default_constraints() -> Vec<Self> {
        vec![
            PersonConstraint::NotPairedWithCourseLeaderColleague,
            PersonConstraint::CantVisitOwnCourse,
        ]
    }

    pub fn default_constraints_with(other: impl IntoIterator<Item = Self>) -> Vec<Self> {
        let mut constraints = Self::default_constraints();
        constraints.extend(other);
        constraints
    }
}
