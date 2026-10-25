local M = {}

-- {name = "Owls", wins = 2} -> wins becomes 3
function M.record_win(team)
  team.wins = team.wins + 1
end

-- a list of teams -> the name of the one with the most wins (the first one on a tie)
function M.most_wins(teams)
  local best = teams[1]
  for _, team in ipairs(teams) do
    if team.wins > best.wins then
      best = team
    end
  end
  return best.name
end

return M
