use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        
        // Update team_1's stats
        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;
        
        // Update team_2's stats
        let team_2 = scores.entry(team_2_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores
}