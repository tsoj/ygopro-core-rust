-- Loaded right after utility.lua (see ygoai/core.py): faster versions of helpers
-- that give exactly the same results.

-- Auxiliary.SelectUnselectLoop searches for a group of minc..maxc cards that
-- satisfies rescon by adding one card at a time, trying every remaining card
-- at every step: it reaches a group of k cards in up to k! orders, and a search
-- that fails visits them all (e.g. Rainbow Dark Dragon checking for 7 DARK
-- monsters with different names in a big GY took seconds at every idle prompt).
-- Whether the cards still to add can complete the group depends only on which
-- cards are in it already (rescon sees the group and the card just added), so
-- that part is remembered per group for the duration of one top-level call.
local memo = nil

local function group_key(sg)
	local ids = {}
	for tc in sg:Iter() do
		ids[#ids + 1] = tc:GetCardID()
	end
	table.sort(ids)
	return table.concat(ids, ",")
end

local loop

-- (the original's mg:IsExists(Auxiliary.SelectUnselectLoop,1,sg,sg,mg,...))
local function extends(sg, mg, e, tp, minc, maxc, rescon)
	local key = group_key(sg)
	local res = memo[key]
	if res == nil then
		res = mg:IsExists(loop, 1, sg, sg, mg, e, tp, minc, maxc, rescon)
		memo[key] = res
	end
	return res
end

loop = function(c, sg, mg, e, tp, minc, maxc, rescon)
	local res = not rescon
	if #sg >= maxc then return false end
	sg:AddCard(c)
	if rescon then
		local stop
		res, stop = rescon(sg, e, tp, mg, c)
		if stop then
			sg:RemoveCard(c)
			return false
		end
	end
	if #sg < minc then
		res = extends(sg, mg, e, tp, minc, maxc, rescon)
	elseif #sg < maxc and not res then
		res = extends(sg, mg, e, tp, minc, maxc, rescon)
	end
	sg:RemoveCard(c)
	return res
end

function Auxiliary.SelectUnselectLoop(c, sg, mg, e, tp, minc, maxc, rescon)
	if memo ~= nil then
		return loop(c, sg, mg, e, tp, minc, maxc, rescon)
	end
	-- (a top-level call: mg, e, tp, minc, maxc and rescon stay the same below it)
	memo = {}
	local ok, res = pcall(loop, c, sg, mg, e, tp, minc, maxc, rescon)
	memo = nil
	if not ok then error(res, 0) end
	return res
end

-- Fusion materials (proc_fusion.lua, "N materials + X or more"): the search
-- tries every remaining card in the next step, so it reaches the same state in
-- every order, both when it extends the chosen materials with candidates
-- (Fusion.CheckSelectMixRep) and when it assigns materials already selected, e.g.
-- by the player one by one, to the material slots (Fusion.CheckMixRepSelectedCond).
-- Frightfur monsters with many Fluffals took seconds per check and ran into the
-- Lua budget. Each of the two answers depends only on the state reached: the
-- materials chosen, the cards that must still be used, the slots left, the counts
-- and whether a substitute was used. Every intermediate group is still checked
-- (Fusion.CheckAdditional, the Extra Monster Zone) exactly as before, along every
-- order that the original tries; only a state that was answered already is not
-- searched again. The answers are remembered for the duration of one
-- Fusion.SelectMixRep call (one first material: everything else passed down, e.g.
-- the candidate materials, stays the same below it).
do
	local select_mix_rep = Fusion.SelectMixRep
	local check_select_mix_rep = Fusion.CheckSelectMixRep
	local fmemo = nil

	function Fusion.SelectMixRep(...)
		if fmemo ~= nil then return select_mix_rep(...) end
		fmemo = {}
		local ok, res = pcall(select_mix_rep, ...)
		fmemo = nil
		if not ok then error(res, 0) end
		return res
	end

	-- (memoised: called with tp, mg, sg, mustg, g, fc, sub, sub2, contact, sumtype,
	-- chkf, fun1, minc, maxc and the slots left; everything but mustg, g, sub, the
	-- counts and the slots stays the same during one Fusion.SelectMixRep call)
	local function memoised(f, name)
		return function(tp, mg, sg, mustg, g, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, ...)
			if fmemo == nil then
				return f(tp, mg, sg, mustg, g, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, ...)
			end
			local parts = {name, group_key(g), group_key(mustg), tostring(sub), tostring(minc), tostring(maxc)}
			for _, slot in ipairs({...}) do
				parts[#parts + 1] = tostring(slot)
			end
			local key = table.concat(parts, "|")
			local hit = fmemo[key]
			if hit == nil then
				hit = {f(tp, mg, sg, mustg, g, fc, sub, sub2, contact, sumtype, chkf, fun1, minc, maxc, ...)}
				fmemo[key] = hit
			end
			return hit[1]
		end
	end
	-- extending the chosen materials with candidates
	Fusion.CheckSelectMixRep = memoised(check_select_mix_rep, "ext")
	-- assigning the materials already selected (e.g. by the player, one by one) to slots
	Fusion.CheckMixRepSelectedCond = memoised(Fusion.CheckMixRepSelectedCond, "sel")
end

-- Synchro Materials (proc_synchro.lua): Synchro.CheckP41 adds Tuners and
-- Synchro.CheckP42 non-Tuners one at a time, trying every remaining candidate at
-- every step, so the same materials are reached in every order (a few non-Tuners
-- among many candidates took the Lua budget). The answer depends only on the
-- state reached: the card added, the materials chosen so far (sg, and as Tuners
-- tsg and non-Tuners ntsg) and the candidates left (tg, ntg); the other arguments
-- stay the same during one top-level call, for which the answers are remembered.
do
	local check_p41 = Synchro.CheckP41
	local check_p42 = Synchro.CheckP42
	local smemo = nil

	local function memoised(f, name, ngroups)
		return function(c, ...)
			local top = smemo == nil
			if top then smemo = {} end
			local groups = {...}
			local parts = {name, tostring(c:GetCardID())}
			for i = 1, ngroups do
				parts[#parts + 1] = group_key(groups[i])
			end
			local key = table.concat(parts, "|")
			local hit = smemo[key]
			if hit == nil then
				local ok, res = pcall(f, c, ...)
				if not ok then
					if top then smemo = nil end
					error(res, 0)
				end
				hit = {res}
				smemo[key] = hit
			end
			if top then smemo = nil end
			return hit[1]
		end
	end
	-- (c, tg, ntg, tsg, ntsg, sg, ...) and (c, ntg, tsg, ntsg, sg, ...)
	Synchro.CheckP41 = memoised(check_p41, "t", 5)
	Synchro.CheckP42 = memoised(check_p42, "n", 4)
end
