use dormmatch_common::models::{profile::StudentProfile, room::Room};

pub struct MatchingService;

impl MatchingService {
  pub fn is_compatible(room: &Room, profile: &StudentProfile) -> bool {
    if room.status != "available" {
      return false;
    }

    if room.current_occupants >= room.capacity {
      return false;
    }

    if room.sex_restriction != "any" && room.sex_restriction != profile.gender {
      return false;
    }

    if let Some(faculty) = &room.faculty_restriction {
      if faculty != &profile.faculty {
        return false;
      }
    }

    if let Some(course) = room.course_restriction {
      if course != profile.course {
        return false;
      }
    }

    true
  }

  pub fn find_best_room<'a>(profile: &StudentProfile, rooms: &'a [Room]) -> Option<&'a Room> {
    rooms.iter().find(|room| Self::is_compatible(room, profile))
  }
}
