use crate::dice_generation::unchecked_generate_dice_result;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use std::ops::Index;
use std::slice::Iter;

#[derive(Clone, PartialEq)]
pub struct Team {
    name: String,
    members: Vec<TeamMember>,
}

#[derive(Clone, PartialEq)]
struct TeamMember {
    name: String,
    role: TeamMemberRole,
}

#[derive(PartialEq, Default, Clone)]
enum TeamMemberRole {
    Captain,
    #[default]
    Regular,
}

impl Team {
    pub fn new(
        team_name: Option<&str>,
        team_members: Option<Vec<&str>>,
        captain_name: Option<&str>,
    ) -> Result<Self, String> {
        let name = match team_name {
            None => generate_random_team_name(),
            Some(n) => n.into(),
        };
        let members: Vec<TeamMember> = match team_members {
            None => {
                vec![]
            }
            Some(members_vec) => members_vec
                .iter()
                .map(|name| TeamMember::new(name, false))
                .collect(),
        };
        let mut new_self = Self { name, members };
        match captain_name {
            None => {
                new_self
                    .randomly_choose_captain()
                    .expect("Failed to randomly choose a captain in team initialisation");
            }
            Some(capt_name) => {
                if let Err(err_msg) = new_self.change_captain(capt_name) {
                    return Err(err_msg);
                }
                // match new_self.set_captain(capt_name) {
                //     Ok(_) => {}
                //     Err(err_msg) => {return Err(err_msg)}
                // }
            }
        }
        return Ok(new_self);
    }
    pub fn size(&self) -> usize {
        return self.members.len();
    }
    pub fn get_captain_name(&self) -> Option<&str> {
        return match self.find_captain_index() {
            None => None,
            Some(i) => {
                let captain = self.members.index(i);
                Some(&captain.name)
            }
        };
    }
    pub fn set_random_name(&mut self) {
        // todo!("Can't generate random names yet")
        self.name = generate_random_team_name();
    }
    pub fn change_captain(&mut self, new_capt_name: &str) -> Result<(), String> {
        let new_capt_index = match self.find_member_index_by_name(new_capt_name) {
            None => {
                return Err(format!("There is no such member as {new_capt_name}"));
            }
            Some(i) => i,
        };
        // At this point, a person with this name for sure exists
        return match self.find_captain_index() {
            Some(i) => {
                if self.members.index(i).name == new_capt_name {
                    Ok(()) // the captain is already set to the same person
                } else {
                    // at this point, some captain exists in a team, and a member with a new name exists too

                    // retiring the old captain
                    self.members[i].set_role(TeamMemberRole::Regular);
                    // promoting a new member
                    self.members[new_capt_index].set_role(TeamMemberRole::Captain);
                    Ok(())
                }
            }
            None => {
                // There is no captain in this team (probably at initialisation)
                self.members[new_capt_index].set_role(TeamMemberRole::Captain);
                Ok(())
            }
        };
    }
    pub fn randomly_choose_captain(&mut self) -> Result<(), String> {
        let size = self.size();
        return if size == 0 {
            Ok(())
        } else if size == 1 {
            Ok(self.members[0].set_role(TeamMemberRole::Captain))
        } else {
            if size > i8::MAX as usize {
                todo!("Implement checks for extremely large teams")
            }
            let rand_index: usize = unchecked_generate_dice_result(0, (size - 1) as i8) as usize;
            /// Please, don't kill me for the following lines. I know this is insane, but!
            /// I can't fight the borrow checker. I have tried multiple things, but it just
            /// doesn't like to have multiple `self` being thrown around, and I don't know
            /// any better way to make it. At least this works
            let member_clone = self.members.clone();
            let rand_name = &member_clone[rand_index].name;
            self.change_captain(rand_name)
        };
    }
    pub fn add_member(&mut self, new_member: TeamMember) {
        self.members.push(new_member);
    }
    pub fn remove_member_by_name(&mut self, old_member_name: &str) -> Result<(), String> {
        for (i, el) in self.members.iter_mut().enumerate() {
            if el.name == old_member_name {
                self.members.remove(i);
                return Ok(());
            }
        }
        // here, the member is not found
        return Err("A member with this name does not exist to remove".into());
    }
    fn remove_member_by_index(&mut self, old_member_index: usize) -> Result<(), String> {
        let min_index_incl: usize = 0;
        let max_index_incl: usize = self.members.len() - 1;
        if old_member_index < min_index_incl || old_member_index > max_index_incl {
            return Err(format!(
                    "Index of the member to remove is out of range. (Expected [{}, {}], got {} instead)",
                    min_index_incl, max_index_incl, old_member_index)
            );
        }
        self.members.remove(old_member_index);
        return Ok(());
    }

    // fn find_captain(&self) -> Result<&TeamMember, ()> {
    //     let opt_capt = self
    //         .members
    //         .iter()
    //         .find(|m| m.role == TeamMemberRole::Captain);
    //     return match opt_capt {
    //         None => Err(()),
    //         Some(capt) => Ok(capt),
    //     };
    // }
    fn find_captain_index(&self) -> Option<usize> {
        for (i, member) in self.members.iter().enumerate() {
            match member.role {
                TeamMemberRole::Captain => return Some(i),
                TeamMemberRole::Regular => {}
            }
        }
        return None;
    }
    fn find_member_index_by_name(&self, member_name: &str) -> Option<usize> {
        for (i, member) in self.members.iter().enumerate() {
            if member.name == member_name {
                return Some(i);
            }
        }
        return None;
    }
    // fn find_mut_captain(&mut self) -> Result<&mut TeamMember, ()> {
    //     let opt_capt = self
    //         .members
    //         .iter_mut()
    //         .find(|m| m.role == TeamMemberRole::Captain);
    //     return match opt_capt {
    //         None => Err(()),
    //         Some(capt) => Ok(capt),
    //     };
    // }
    fn name_exists(&self, m_name: &str) -> bool {
        return match self.members.iter().find(|m| m.name == m_name) {
            None => false,
            Some(_) => true,
        };
    }
    // fn choose_random_name(&self) -> Option<&str> {
    //     if self.size() == 0 {
    //         return None;
    //     }
    //     let rand_index = crate::dice_generation::custom_dice(0, self.size() as i8 - 1)
    //         .expect("Should have had no errors") as usize;
    //     return Some(&self.members.index(rand_index).name);
    // }
}

impl TeamMember {
    pub fn new(name: &str, is_captain: bool) -> Self {
        return Self {
            name: String::from(name),
            role: match is_captain {
                true => TeamMemberRole::Captain,
                false => TeamMemberRole::Regular,
            },
        };
    }
    pub fn set_role(&mut self, new_role: TeamMemberRole) {
        self.role = new_role;
    }
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.into();
    }
}

fn generate_random_team_name() -> String {
    return String::from("Placeholder");
}

// fn get_shuffled_vec<T>(input: &Vec<T>) -> Vec<T> {
//     let mut new_vec: Vec<T> = input.iter().collect();
//     let mut rng = rand::rng();
//     new_vec.shuffle(&mut rng);
//     return new_vec;
// }

fn shuffle_vec<T>(mut input: Vec<T>) -> Vec<T> {
    let mut rng = rand::rng();
    input.shuffle(&mut rng);
    return input;
}

// NOTE: creates a copy of a Vec right now, should be a &mut Vec<String> in the future
pub fn create_new_pool_of_teams(mut all_names: Vec<String>, number_of_teams: usize) -> Vec<Team> {
    let mut all_teams: Vec<Team> = vec![]; // should make it an array in the future instead
    all_teams.reserve_exact(number_of_teams);
    for i in 0..number_of_teams {
        all_teams.push(Team::new(Some(&format!("Team {i}")), None, None)
            .expect("Failed to create placeholder team"))
    }
    all_names = shuffle_vec(all_names);

    /// if there are, let's say, 11 people for 3 teams, I don't want 5 + 5 + 1,
    /// I want 4 + 4 + 3
    // let avg = all_names.len() as f64 / number_of_teams as f64; // maybe implement it later (needs limit testing)
    let mut member_index = 0;
    while all_names.len() > 0 {
        let new_member = TeamMember::new(
            &all_names
                .pop()
                .expect("There should have been at least 1 person in the name list"),
            false,
        );
        let team_index = member_index % all_teams.len(); // can't be done inline (borrow checker is not happy)
        all_teams[team_index].add_member(new_member);
        member_index += 1;
    }
    for team in all_teams.iter_mut() {
        team.randomly_choose_captain().unwrap();
    }
    return all_teams;
}

fn choose_random_member_name(members: Iter<TeamMember>) -> Option<&str> {
    let len = members.len();
    return if len == 0 {
        None
    } else {
        let mut rng = rand::rng();
        // let rand_index = custom_dice(0, (len - 1) as i8).expect("Error while generating random index for name selection");
        let rand_member = members
            .choose(&mut rng)
            .expect("Failed to choose random member");
        Some(&rand_member.name)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captain_manipulations() {
        let mut correct_team = Team::new(
            Some("Test team"),
            Some(vec!["TM1", "TM2", "TM3"]),
            Some("TM1"),
        )
        .unwrap();
        correct_team.change_captain("TM2").unwrap();
        correct_team.change_captain("TM3").unwrap();
        correct_team.change_captain("TM1").unwrap();
        assert_eq!(correct_team.size(), 3);
        assert_eq!(correct_team.get_captain_name().unwrap(), "TM1");
    }

    #[test]
    fn pool_of_teams_creation() {
        let all_names: Vec<String> = (1..=11).map(|i| i.to_string()).collect();
        let all_teams = create_new_pool_of_teams(all_names, 3);
        assert_eq!(all_teams[0].size(), 4);
        assert_eq!(all_teams[1].size(), 4);
        assert_eq!(all_teams[2].size(), 3);
    }
}
