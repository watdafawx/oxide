-- Press Run once every objective is green: a whole tournament, played with your code.
local league = require("league")

local teams = {
  league.new_team("Owls"),
  league.new_team("Foxes"),
  league.new_team("Bees"),
  league.new_team("Moles"),
}

league.record_result(teams, "Owls", "Foxes", 2, 1)
league.record_result(teams, "Bees", "Moles", 0, 0)
league.record_result(teams, "Owls", "Bees", 1, 3)
league.record_result(teams, "Foxes", "Moles", 4, 2)
league.record_result(teams, "Owls", "Moles", 2, 2)
league.record_result(teams, "Foxes", "Bees", 1, 0)

print("GAMES NIGHT LEAGUE")
print("==================")
for _, line in ipairs(league.table_lines(teams)) do
  print(line)
end
print("")
print("Leader: " .. league.leader(teams))
