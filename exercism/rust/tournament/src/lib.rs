use std::fs::File;
use std::io::{BufRead, BufReader, Write, Result};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering::Equal;

#[derive(Default, Debug)]
struct Team {
  name: String,
  win: u32,
  loss: u32,
  draw: u32,
  games: u32,
  points: u32
}

enum GameResult {
  Win,
  Draw,
  Loss
}

pub fn tally(input: &Path, output: &Path) -> Result<u32> {
  let mut map: HashMap<String, Team> = HashMap::new();

  let f = try!(File::open(input));
  let file = BufReader::new(&f);
  let mut count = 0;
  for line in file.lines() {
    let line = line.unwrap();
    let parts: Vec<&str> = line.trim_right().split(";").collect();
    if parts.len() != 3 { continue; }

    let team1 = parts[0];
    let team2 = parts[1];

    match parts[2] {
      "win" => {
        update_team(&mut map, team1, GameResult::Win);
        update_team(&mut map, team2, GameResult::Loss);
        count += 1;
      },
      "loss" =>  {
        update_team(&mut map, team2, GameResult::Win);
        update_team(&mut map, team1, GameResult::Loss);
        count += 1;
      },
      "draw" =>  {
        update_team(&mut map, team1, GameResult::Draw);
        update_team(&mut map, team2, GameResult::Draw);
        count += 1;
      },
      _ => (),
    };
  }

  let mut scores: Vec<Team> = Vec::new();
  for (key, value) in &map {
    let games = value.win + value.draw + value.loss;
    let points = value.win*3 + value.draw;
    scores.push(Team{
      name: key.to_owned(),
      games: games,
      win: value.win,
      draw: value.draw,
      loss: value.loss,
      points: points,
    });
  }

  scores.sort_by(|a, b| {
    match a.points.cmp(&b.points).reverse() {
      Equal => match a.win.cmp(&b.win).reverse() {
        Equal => a.name.cmp(&b.name),
        other => other,
      },
      other => other
    }
  });

  let mut f = try!(File::create(output));
  try!(write!(&mut f, "{:30} | MP |  W |  D |  L |  P\r\n", "Team"));
  for score in scores {
    try!(write!(&mut f, "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}\r\n",
      score.name, score.games, score.win, score.draw, score.loss, score.points));
  }

  Ok(count)
}

fn update_team(map: &mut HashMap<String, Team>, team: &str, result: GameResult) {
  let entry = map.entry(team.to_owned()).or_insert(Team{..Default::default()});
  match result {
    GameResult::Win => (*entry).win += 1,
    GameResult::Draw => (*entry).draw += 1,
    GameResult::Loss => (*entry).loss += 1,
  };
}
