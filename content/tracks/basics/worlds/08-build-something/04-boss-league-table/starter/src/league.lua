local M = {}

-- TODO (stage 1): a new team record: the name, and 0 wins, draws and losses
function M.new_team(name)
  return nil
end

-- TODO (stage 2): the team in the list whose name is name, or nil when there is none
function M.find(teams, name)
  return nil
end

-- TODO (stage 3): 3 points for each win, 1 for each draw
function M.points(team)
  return nil
end

-- TODO (stage 3): every game the team has played
function M.played(team)
  return nil
end

-- TODO (stage 4): find both teams, then add one win, draw or loss to each
-- more home goals: home wins, away loses; more away goals: the opposite; equal: both draw
function M.record_result(teams, home, away, home_goals, away_goals)
end

-- TODO (stage 5): the name of the team with the most points; on a tie, the earlier one
function M.leader(teams)
  return nil
end

-- TODO (stage 6): a list with one line per team, in order, like "Owls 7 pts in 4 games"
function M.table_lines(teams)
  return {}
end

return M
