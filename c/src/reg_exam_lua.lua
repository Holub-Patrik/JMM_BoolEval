local usr_in = io.read("l")
local _, _, day, month, year = string.find(usr_in, "(%d+)[%./:](%d+)[%./:](%d+)")
print(day, month, year)
