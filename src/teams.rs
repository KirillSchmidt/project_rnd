use std::ops::Index;

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
                new_self.randomly_choose_captain();
            }
            Some(capt_name) => {
                if let Err(err_msg) = new_self.set_captain(capt_name) {
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
        return match self.find_captain() {
            Ok(c) => Some(&c.name),
            Err(_) => None,
        };
    }
    pub fn set_random_name(&mut self) {
        // todo!("Can't generate random names yet")
        self.name = generate_random_team_name();
    }
    pub fn set_captain(&mut self, new_capt_name: &str) -> Result<(), String> {
        if !self.name_exists(new_capt_name) {
            return Err(format!("There is no such member as {new_capt_name}"));
        }
        match self.find_captain() {
            Ok(c) => {
                if c.name == new_capt_name {
                    return Ok(()); // the captain is already set to the same person
                }
            }
            Err(_) => return Err(format!("There is no captain in team {}", &self.name)),
        }
        // at this point, some captain exists in a team, and a member with a new name exists too

        // retiring the old captain
        self.find_mut_captain()
            .expect("The captain should have existed")
            .set_role(TeamMemberRole::Regular);
        self.members
            .iter_mut()
            .find(|m| m.name == new_capt_name)
            .expect("A member with new captain's name should have existed")
            .set_role(TeamMemberRole::Captain);
        return Ok(());
    }
    pub fn randomly_choose_captain(&mut self) -> Result<(), String> {
        return if self.size() == 0 {
            Ok(())
        } else if self.size() == 1 {
            Ok(self.members[0].set_role(TeamMemberRole::Captain))
        } else {
            todo!();
            let rand_name = self.choose_random_name().unwrap();
            return self.set_captain(rand_name);
        };
    }
    fn find_captain(&self) -> Result<&TeamMember, ()> {
        let opt_capt = self
            .members
            .iter()
            .find(|m| m.role == TeamMemberRole::Captain);
        return match opt_capt {
            None => Err(()),
            Some(capt) => Ok(capt),
        };
    }
    fn find_mut_captain(&mut self) -> Result<&mut TeamMember, ()> {
        let opt_capt = self
            .members
            .iter_mut()
            .find(|m| m.role == TeamMemberRole::Captain);
        return match opt_capt {
            None => Err(()),
            Some(capt) => Ok(capt),
        };
    }
    fn name_exists(&self, m_name: &str) -> bool {
        return match self.members.iter().find(|m| m.name == m_name) {
            None => false,
            Some(_) => true,
        };
    }
    fn choose_random_name(&self) -> Option<&str> {
        if self.size() == 0 {
            return None;
        }
        let rand_index = crate::dice_generation::custom_dice(0, self.size() as i8 - 1)
            .expect("Should have had no errors") as usize;
        return Some(&self.members.index(rand_index).name);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_fail() {
        let mut correct_team = Team::new(
            Some("Test team"),
            Some(vec!["TM1", "TM2", "TM3"]),
            Some("TM1"),
        )
        .unwrap();
        correct_team.set_captain("TM2").unwrap();
        correct_team.set_captain("TM3").unwrap();
        correct_team.set_captain("TM1").unwrap();
        assert_eq!(correct_team.size(), 3);
        assert_eq!(correct_team.get_captain_name().unwrap(), "TM1");
    }
}
