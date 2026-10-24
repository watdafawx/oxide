local M = {}

-- 3 points for a win, 1 for a draw
function M.points(wins, draws)
  local from_wins = wins * 3
  local from_draws = draws * 3
  return from_wins + from_draws
end

-- every game played: wins, draws and losses
function M.played(wins, draws, losses)
  return wins + draws + draws
end

-- "Owls 7 pts in 4 games"
function M.table_line(name, wins, draws, losses)
  return name .. " " .. M.points(wins, draws) .. " pts in " .. M.played(wins, draws, losses) .. " games"
end

return M
