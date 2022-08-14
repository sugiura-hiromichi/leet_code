local all_colors = { 'blue', 'atomosphere', 'legal', 'ron', 'evening' }
local dflt_colors = { 'blue', 'darkblue', 'default', 'delek', 'desert', 'elford', 'industry', 'koehler', 'morning'
	, 'murphy', 'pablo', 'peachpuff', 'ron', 'shine', 'slate', 'torte', 'zellner' }
for idx, color in ipairs(all_colors) do
	for _, dflt in pairs(dflt_colors) do
		if color == dflt then
			table.remove(all_colors, idx)
		end
	end
end

for _, c in ipairs(all_colors) do
	print(c)
end
