local function eval(fstring)
	local	expres = {}
	while string.find(fstring, "(%w+&&|%|%||=>|==%w+%)") do
		print("here")
		for expr in string.gmatch(fstring, "(%(%w+&&|%|%||=>|==%w+%)") do
			print("expr")
			table.insert(expres, expr)
		end
	end
	return expres
end

local function main()
	local usr_in = io.read("l")
	usr_in = string.gsub(usr_in, "%s+", "")
	local ret_val = eval(usr_in)
	print(ret_val[1])
end


main()
