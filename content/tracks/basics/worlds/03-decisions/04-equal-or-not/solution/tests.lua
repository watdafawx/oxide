local t = require("oxide.test")
local door = require("staff_door")

t.test("door_opens_for_the_right_code", function()
  t.eq(door.door_opens(4721), true, "the right code")
  t.eq(door.door_opens(1234), false, "a wrong code")
  t.eq(door.door_opens(4712), false, "close, but wrong")
end)

t.test("alarm_for_wrong_codes", function()
  t.eq(door.sound_alarm(1234), true, "a wrong code sounds the alarm")
  t.eq(door.sound_alarm(4721), false, "the right code is quiet")
end)
