local M = {}

-- stage 1: a new team record: the name, and 0 wins, draws and losses
function M.new_team(name)
  return {name = name, wins = 0, draws = 0, losses = 0}
end

-- stage 2: the team in the list whose name is name, or nil when there is none
function M.find(teams, name)
  for _, team in ipairs(teams) do
    if team.name == name then
      return team
    end
  end
  return nil
end

-- stage 3: 3 points for each win, 1 for each draw
function M.points(team)
  return team.wins * 3 + team.draws
end

-- stage 3: every game the team has played
function M.played(team)
  return team.wins + team.draws + team.losses
end

-- stage 4: find both teams, then add one win, draw or loss to each
-- more home goals: home wins, away loses; more away goals: the opposite; equal: both draw
function M.record_result(teams, home, away, home_goals, away_goals)
  local h = M.find(teams, home)
  local a = M.find(teams, away)
  if home_goals > away_goals then
    h.wins = h.wins + 1
    a.losses = a.losses + 1
  elseif away_goals > home_goals then
    a.wins = a.wins + 1
    h.losses = h.losses + 1
  else
    h.draws = h.draws + 1
    a.draws = a.draws + 1
  end
end

-- stage 5: the name of the team with the most points; on a tie, the earlier one
function M.leader(teams)
  local best = teams[1]
  for _, team in ipairs(teams) do
    if M.points(team) > M.points(best) then
      best = team
    end
  end
  return best.name
end

-- stage 6: a list with one line per team, in order, like "Owls 7 pts in 4 games"
function M.table_lines(teams)
  local lines = {}
  for _, team in ipairs(teams) do
    table.insert(lines, team.name .. " " .. M.points(team) .. " pts in " .. M.played(team) .. " games")
  end
  return lines
end

return M
