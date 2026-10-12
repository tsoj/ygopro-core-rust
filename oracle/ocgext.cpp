/*
 * Extensions to ygopro-core used by the yugioh_AI_2 project, see ocgext.h.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
#include <algorithm>
#include <deque>
#include <unordered_map>
#include <vector>
#include "ocgext.h"
#include "ocgapi_types.h"
#include <set>
#include <tuple>
#include <string>
#include <lauxlib.h>
#include <lua.h>
#include "card.h"
#include "duel.h"
#include "effect.h"
#include "field.h"

#if defined(_WIN32)
#define OCGEXT_API extern "C" __declspec(dllexport)
#else
#define OCGEXT_API extern "C" __attribute__ ((visibility ("default")))
#endif

namespace {

struct ForcedLoc {
	uint8_t con;
	uint8_t loc;
	uint32_t seq;
	uint32_t overlay_seq;
};

struct State {
	std::deque<int32_t> forced_ints[ocgext::FORCED_KIND_COUNT];
	// [player][0 = deck, 1 = hand, 2 = extra]
	// wanted order by card id (may be empty) and by code (the fallback)
	std::deque<std::pair<std::vector<uint32_t>, std::vector<uint32_t>>> forced_shuffles[2][3];
	std::deque<std::vector<ForcedLoc>> forced_random_selects;
	std::deque<std::pair<std::vector<uint32_t>, std::vector<uint32_t>>> forced_set_card_shuffles;  // (ids, codes)
	// (player, card code, effect description) whose cost/target checks are skipped
	std::set<std::tuple<uint8_t, uint32_t, uint64_t>> forced_activatable;
	// (card code, effect description) of mandatory triggers only the forcing made activatable
	std::set<std::pair<uint32_t, uint64_t>> forced_used;
	// card ids that count as matching every card filter
	std::set<uint32_t> forced_match;
	std::set<uint32_t> forced_nomatch;
	// the read log: filters evaluated on cards the viewer cannot see
	int read_viewer = -1;
	std::vector<std::pair<uint32_t, uint8_t>> read_log;  // (card id, result)
	uint32_t errors = 0;
	uint32_t consumed = 0;
};

std::unordered_map<duel*, State> states;

State* find_state(duel* pduel) {
	auto it = states.find(pduel);
	return it == states.end() ? nullptr : &it->second;
}

int shuffle_index(uint32_t location) {
	switch(location) {
	case LOCATION_DECK: return 0;
	case LOCATION_HAND: return 1;
	case LOCATION_EXTRA: return 2;
	default: return -1;
	}
}

card_vector* location_list(duel* pduel, uint8_t playerid, uint32_t location) {
	if(playerid > 1)
		return nullptr;
	auto& player = pduel->game_field->player[playerid];
	switch(location) {
	case LOCATION_DECK: return &player.list_main;
	case LOCATION_HAND: return &player.list_hand;
	case LOCATION_EXTRA: return &player.list_extra;
	case LOCATION_GRAVE: return &player.list_grave;
	case LOCATION_REMOVED: return &player.list_remove;
	case LOCATION_MZONE: return &player.list_mzone;
	case LOCATION_SZONE: return &player.list_szone;
	default: return nullptr;
	}
}

// Permutes the first `count` entries of `list` so that entry i has data.code
// == wanted[i]. Among cards with equal codes the current relative order is
// kept. Returns false (and leaves `list` untouched) if the multisets differ.
bool reorder_by_code(card_vector& list, uint32_t count, const std::vector<uint32_t>& wanted) {
	if(wanted.size() != count || count > list.size())
		return false;
	std::vector<card*> result;
	result.reserve(count);
	std::vector<bool> used(count, false);
	for(auto code : wanted) {
		bool found = false;
		for(uint32_t i = 0; i < count; ++i) {
			if(!used[i] && list[i]->data.code == code) {
				used[i] = true;
				result.push_back(list[i]);
				found = true;
				break;
			}
		}
		if(!found)
			return false;
	}
	std::copy(result.begin(), result.end(), list.begin());
	return true;
}

bool reorder_by_id(card_vector& list, uint32_t count, const std::vector<uint32_t>& wanted) {
	if(wanted.size() != count || count > list.size())
		return false;
	std::vector<card*> result;
	result.reserve(count);
	for(auto id : wanted) {
		auto it = std::find_if(list.begin(), list.begin() + count, [id](card* c) { return c->cardid == id; });
		if(it == list.begin() + count || std::find(result.begin(), result.end(), *it) != result.end())
			return false;
		result.push_back(*it);
	}
	std::copy(result.begin(), result.end(), list.begin());
	return true;
}

bool is_sequenced_list(uint32_t location) {
	return location & (LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE | LOCATION_REMOVED);
}

uint32_t reorderable_count(duel* pduel, uint8_t playerid, uint32_t location, const card_vector& list) {
	if(location == LOCATION_EXTRA)
		return static_cast<uint32_t>(list.size() - pduel->game_field->player[playerid].extra_p_count);
	return static_cast<uint32_t>(list.size());
}

} // namespace

namespace ocgext {

int32_t forced_int(duel* pduel, int kind, int32_t fallback) {
	auto* st = find_state(pduel);
	if(!st || kind < 0 || kind >= FORCED_KIND_COUNT || st->forced_ints[kind].empty())
		return fallback;
	auto value = st->forced_ints[kind].front();
	st->forced_ints[kind].pop_front();
	++st->consumed;
	return value;
}

void on_shuffle(duel* pduel, uint8_t playerid, uint8_t location, card_vector& list, uint32_t count) {
	auto* st = find_state(pduel);
	int idx = shuffle_index(location);
	if(!st || idx < 0 || playerid > 1)
		return;
	auto& queue = st->forced_shuffles[playerid][idx];
	if(queue.empty())
		return;
	auto wanted = std::move(queue.front());
	queue.pop_front();
	++st->consumed;
	// by card id if these are the cards in the pile, else by code
	if(!(wanted.first.size() && reorder_by_id(list, count, wanted.first)) && !reorder_by_code(list, count, wanted.second)) {
		++st->errors;
		return;
	}
	pduel->game_field->reset_sequence(playerid, location);
}

void on_random_select(duel* pduel, const card_set& from, card_set& selected) {
	auto* st = find_state(pduel);
	if(!st || st->forced_random_selects.empty())
		return;
	auto wanted = std::move(st->forced_random_selects.front());
	st->forced_random_selects.pop_front();
	++st->consumed;
	card_set result;
	for(const auto& w : wanted) {
		card* match = nullptr;
		for(auto* pcard : from) {
			auto info = pcard->get_info_location();
			if(info.controler != w.con || info.location != w.loc || info.sequence != w.seq)
				continue;
			if((w.loc & LOCATION_OVERLAY) && info.position != w.overlay_seq)
				continue;
			match = pcard;
			break;
		}
		if(!match) {
			++st->errors;
			return;
		}
		result.insert(match);
	}
	if(result.size() != selected.size()) {
		++st->errors;
		return;
	}
	selected = std::move(result);
}

void on_shuffle_set_card(duel* pduel, card** ms, const uint8_t* seq, int32_t ct) {
	auto* st = find_state(pduel);
	if(!st || st->forced_set_card_shuffles.empty())
		return;
	auto wanted = std::move(st->forced_set_card_shuffles.front());
	st->forced_set_card_shuffles.pop_front();
	++st->consumed;
	if(static_cast<int32_t>(wanted.second.size()) != ct) {
		++st->errors;
		return;
	}
	// wanted[k] is the card (by id if these are the cards, else by code) that must
	// end up at the k-th smallest sequence
	std::vector<uint8_t> sorted_seq(seq, seq + ct);
	std::sort(sorted_seq.begin(), sorted_seq.end());
	for(bool by_id : {true, false}) {
		if(by_id && static_cast<int32_t>(wanted.first.size()) != ct)
			continue;
		std::vector<card*> pool(ms, ms + ct);
		std::vector<card*> result(ct, nullptr);
		bool ok = true;
		for(int32_t i = 0; i < ct && ok; ++i) {
			auto rank = std::distance(sorted_seq.begin(), std::find(sorted_seq.begin(), sorted_seq.end(), seq[i]));
			auto it = std::find_if(pool.begin(), pool.end(), [&](card* c) {
				return c && (by_id ? c->cardid == wanted.first[rank] : c->data.code == wanted.second[rank]);
			});
			if(it == pool.end()) {
				ok = false;
				break;
			}
			result[i] = *it;
			*it = nullptr;
		}
		if(ok) {
			std::copy(result.begin(), result.end(), ms);
			return;
		}
	}
	++st->errors;
}

bool is_forced_activatable(duel* pduel, effect* peffect, uint8_t playerid) {
	auto* st = find_state(pduel);
	if(!st || st->forced_activatable.empty())
		return false;
	card* handler = peffect->get_handler();
	if(!handler)
		return false;
	return st->forced_activatable.count({ playerid, handler->data.code, peffect->description }) > 0;
}

void note_forced_used(duel* pduel, effect* peffect) {
	auto* st = find_state(pduel);
	card* handler = peffect ? peffect->get_handler() : nullptr;
	if(st && handler && st->forced_used.size() < 1024)
		st->forced_used.emplace(handler->data.code, peffect->description);
}

bool is_forced_match(duel* pduel, card* pcard) {
	auto* st = find_state(pduel);
	return st && !st->forced_match.empty() && pcard && st->forced_match.count(pcard->cardid) > 0;
}

bool is_forced_nomatch(duel* pduel, card* pcard) {
	auto* st = find_state(pduel);
	return st && !st->forced_nomatch.empty() && pcard && st->forced_nomatch.count(pcard->cardid) > 0;
}

// Whether `viewer` cannot see which card `pcard` is (by what the game shows:
// piles, face-down cards, the other player's private hand cards). The order of
// the decks is unknown to both players, so deck cards are hidden from both.
static bool hidden_from(card* pcard, uint8_t viewer) {
	const auto loc = pcard->current.location;
	if(loc & LOCATION_OVERLAY)
		return false;
	const bool own = pcard->current.controler == viewer;
	const bool facedown = (pcard->current.position & POS_FACEDOWN) != 0;
	switch(loc) {
	case LOCATION_DECK: return true;
	case LOCATION_HAND: return !own && !(pcard->current.position & POS_FACEUP);
	case LOCATION_EXTRA:
	case LOCATION_MZONE:
	case LOCATION_SZONE:
	case LOCATION_REMOVED: return !own && facedown;
	default: return false;
	}
}

bool read_log_on(duel* pduel) {
	auto* st = find_state(pduel);
	return st && st->read_viewer >= 0;
}

void log_filter_read(duel* pduel, card* pcard, bool result) {
	auto* st = find_state(pduel);
	if(!st || st->read_viewer < 0 || !pcard || st->read_log.size() >= 4096)
		return;
	if(hidden_from(pcard, static_cast<uint8_t>(st->read_viewer)))
		st->read_log.emplace_back(pcard->cardid, result ? 1 : 0);
}

void on_destroy(duel* pduel) {
	states.erase(pduel);
}

} // namespace ocgext

/*** Extension API ***/

OCGEXT_API void OCG_ExtPushForcedInts(OCG_Duel ocg_duel, int kind, const int32_t* values, uint32_t count) {
	if(kind < 0 || kind >= ocgext::FORCED_KIND_COUNT)
		return;
	auto& st = states[static_cast<duel*>(ocg_duel)];
	for(uint32_t i = 0; i < count; ++i)
		st.forced_ints[kind].push_back(values[i]);
}

OCGEXT_API void OCG_ExtPushForcedShuffle(OCG_Duel ocg_duel, uint8_t playerid, uint32_t location, const uint32_t* codes, uint32_t count) {
	int idx = shuffle_index(location);
	if(idx < 0 || playerid > 1)
		return;
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.forced_shuffles[playerid][idx].emplace_back(std::vector<uint32_t>(), std::vector<uint32_t>(codes, codes + count));
}

// Like OCG_ExtPushForcedShuffle, but the result is also given as unique card ids
// (which physical copy goes where), bottom first. If the pile does not hold
// exactly these cards, the codes are used.
OCGEXT_API void OCG_ExtPushForcedShuffleById(OCG_Duel ocg_duel, uint8_t playerid, uint32_t location, const uint32_t* ids, const uint32_t* codes, uint32_t count) {
	int idx = shuffle_index(location);
	if(idx < 0 || playerid > 1)
		return;
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.forced_shuffles[playerid][idx].emplace_back(std::vector<uint32_t>(ids, ids + count), std::vector<uint32_t>(codes, codes + count));
}

// `locs` holds 4 uint32 per entry: controller, location, sequence, overlay sequence
OCGEXT_API void OCG_ExtPushForcedRandomSelect(OCG_Duel ocg_duel, const uint32_t* locs, uint32_t count) {
	auto& st = states[static_cast<duel*>(ocg_duel)];
	std::vector<ForcedLoc> entry;
	for(uint32_t i = 0; i < count; ++i) {
		entry.push_back(ForcedLoc{
			static_cast<uint8_t>(locs[i * 4]), static_cast<uint8_t>(locs[i * 4 + 1]),
			locs[i * 4 + 2], locs[i * 4 + 3] });
	}
	st.forced_random_selects.push_back(std::move(entry));
}

OCGEXT_API void OCG_ExtPushForcedSetCardShuffle(OCG_Duel ocg_duel, const uint32_t* codes, uint32_t count) {
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.forced_set_card_shuffles.emplace_back(std::vector<uint32_t>(), std::vector<uint32_t>(codes, codes + count));
}

// Like OCG_ExtPushForcedSetCardShuffle, but also with the unique card ids (used
// if these are the cards shuffled, else the codes).
OCGEXT_API void OCG_ExtPushForcedSetCardShuffleById(OCG_Duel ocg_duel, const uint32_t* ids, const uint32_t* codes, uint32_t count) {
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.forced_set_card_shuffles.emplace_back(std::vector<uint32_t>(ids, ids + count), std::vector<uint32_t>(codes, codes + count));
}

// Effects of cards with `code` and description `desc` count as activatable for
// `playerid` even if their cost or target check fails. Used when the observed
// player activated an effect whose requirements depend on hidden cards.
OCGEXT_API void OCG_ExtForceActivatable(OCG_Duel ocg_duel, uint8_t playerid, uint32_t code, uint64_t desc) {
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.forced_activatable.insert({ playerid, code, desc });
}

// Removes all entries added with OCG_ExtForceActivatable.
OCGEXT_API void OCG_ExtClearForceActivatable(OCG_Duel ocg_duel) {
	auto* st = find_state(static_cast<duel*>(ocg_duel));
	if(st)
		st->forced_activatable.clear();
}

// Until changed again, the cards with these unique ids count as matching every
// card filter (Duel.GetMatchingGroup, Group.Filter, ...). Used for a single
// processing step, when the observed player took a hidden card for an effect
// that the hypothesis' card at that position could not satisfy.
OCGEXT_API void OCG_ExtSetForcedMatch(OCG_Duel ocg_duel, const uint32_t* ids, uint32_t count) {
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.forced_match.clear();
	for(uint32_t i = 0; i < count; ++i)
		st.forced_match.insert(ids[i]);
}

// Until changed again, the cards with these unique ids match no card filter.
// Used for a single span of the stream in which the observed player's effect
// found no fitting card although the hypothesis' hidden cards would fit.
OCGEXT_API void OCG_ExtSetForcedNoMatch(OCG_Duel ocg_duel, const uint32_t* ids, uint32_t count) {
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.forced_nomatch.clear();
	for(uint32_t i = 0; i < count; ++i)
		st.forced_nomatch.insert(ids[i]);
}

// Swaps the places of two cards (by unique card id) in the same pile (deck,
// hand, face-down extra, grave, banished) of the same player: the two
// physical cards trade places and keep their own history (reason, turn id,
// ...). Used to exchange copies of the same card. Returns 1 on success.
// Turns the read log on for `viewer` (or off with -1).
OCGEXT_API void OCG_ExtSetReadLog(OCG_Duel ocg_duel, int viewer) {
	auto& st = states[static_cast<duel*>(ocg_duel)];
	st.read_viewer = viewer;
	st.read_log.clear();
}

// Copies out and clears the mandatory triggers only the forcing made activatable
// since the last call: pairs (card code, effect description).
OCGEXT_API uint32_t OCG_ExtTakeForcedUsed(OCG_Duel ocg_duel, uint64_t* out, uint32_t cap) {
	auto* st = find_state(static_cast<duel*>(ocg_duel));
	if(!st)
		return 0;
	uint32_t n = 0;
	for(const auto& [code, desc] : st->forced_used) {
		if(n >= cap)
			break;
		out[2 * n] = code;
		out[2 * n + 1] = desc;
		++n;
	}
	st->forced_used.clear();
	return n;
}

// Copies out and clears the read log: card ids and results (1 = matched).
OCGEXT_API uint32_t OCG_ExtTakeReadLog(OCG_Duel ocg_duel, uint32_t* ids, uint8_t* results, uint32_t cap) {
	auto* st = find_state(static_cast<duel*>(ocg_duel));
	if(!st)
		return 0;
	uint32_t n = static_cast<uint32_t>(std::min<size_t>(cap, st->read_log.size()));
	for(uint32_t i = 0; i < n; ++i) {
		ids[i] = st->read_log[i].first;
		results[i] = st->read_log[i].second;
	}
	st->read_log.clear();
	return n;
}

OCGEXT_API int OCG_ExtSwapCards(OCG_Duel ocg_duel, uint32_t id_a, uint32_t id_b) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	card* a = nullptr;
	card* b = nullptr;
	for(auto* pcard : pduel->cards) {
		if(pcard->cardid == id_a)
			a = pcard;
		else if(pcard->cardid == id_b)
			b = pcard;
	}
	if(!a || !b || a == b)
		return 0;
	if(a->overlay_target || b->overlay_target)
		return 0;
	if(a->owner != b->owner || a->current.controler != b->current.controler || a->current.location != b->current.location)
		return 0;
	auto loc = a->current.location;
	if(loc & (LOCATION_MZONE | LOCATION_SZONE)) {
		// two face-down cards in their zones (e.g. after Duel.ShuffleSetCard): they
		// trade zones, as the shuffle could have placed them
		if(!a->is_position(POS_FACEDOWN) || !b->is_position(POS_FACEDOWN)
		   || a->equiping_target || b->equiping_target || !a->equiping_cards.empty() || !b->equiping_cards.empty())
			return 0;
		auto& p = pduel->game_field->player[a->current.controler];
		auto& zones = (loc & LOCATION_MZONE) ? p.list_mzone : p.list_szone;
		auto sa = a->current.sequence, sb = b->current.sequence;
		if(sa >= zones.size() || sb >= zones.size() || zones[sa] != a || zones[sb] != b)
			return 0;
		std::swap(zones[sa], zones[sb]);
		std::swap(a->current.sequence, b->current.sequence);
		return 1;
	}
	if(!is_sequenced_list(loc))
		return 0;
	auto* list = location_list(pduel, a->current.controler, loc);
	if(!list || a->current.sequence >= list->size() || b->current.sequence >= list->size())
		return 0;
	if((*list)[a->current.sequence] != a || (*list)[b->current.sequence] != b)
		return 0;
	std::swap((*list)[a->current.sequence], (*list)[b->current.sequence]);
	std::swap(a->current.sequence, b->current.sequence);
	return 1;
}

OCGEXT_API void OCG_ExtClearForced(OCG_Duel ocg_duel) {
	states.erase(static_cast<duel*>(ocg_duel));
}

// Number of queued forced entries that were not consumed yet.
OCGEXT_API uint32_t OCG_ExtPendingForced(OCG_Duel ocg_duel) {
	auto* st = find_state(static_cast<duel*>(ocg_duel));
	if(!st)
		return 0;
	size_t n = st->forced_random_selects.size() + st->forced_set_card_shuffles.size();
	for(auto& q : st->forced_ints)
		n += q.size();
	for(auto& p : st->forced_shuffles)
		for(auto& q : p)
			n += q.size();
	return static_cast<uint32_t>(n);
}

// Number of forced entries that could not be applied (e.g. multiset mismatch).
OCGEXT_API uint32_t OCG_ExtGetErrors(OCG_Duel ocg_duel) {
	auto* st = find_state(static_cast<duel*>(ocg_duel));
	return st ? st->errors : 0;
}

// Reorders a sequenced location (deck, hand, extra (face-down part), grave,
// banished) so that the card at sequence i has code codes[i]. For the deck,
// sequence 0 is the bottom. Emits no messages. Returns 1 on success.
OCGEXT_API int OCG_ExtSetLocationOrder(OCG_Duel ocg_duel, uint8_t playerid, uint32_t location, const uint32_t* codes, uint32_t count) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	if(!is_sequenced_list(location))
		return 0;
	auto* list = location_list(pduel, playerid, location);
	if(!list)
		return 0;
	std::vector<uint32_t> wanted(codes, codes + count);
	if(!reorder_by_code(*list, reorderable_count(pduel, playerid, location, *list), wanted))
		return 0;
	pduel->game_field->reset_sequence(playerid, static_cast<uint8_t>(location));
	return 1;
}

// Same as OCG_ExtSetLocationOrder but identifies cards by their unique card id.
OCGEXT_API int OCG_ExtSetLocationOrderById(OCG_Duel ocg_duel, uint8_t playerid, uint32_t location, const uint32_t* ids, uint32_t count) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	if(!is_sequenced_list(location))
		return 0;
	auto* list = location_list(pduel, playerid, location);
	if(!list)
		return 0;
	std::vector<uint32_t> wanted(ids, ids + count);
	if(!reorder_by_id(*list, reorderable_count(pduel, playerid, location, *list), wanted))
		return 0;
	pduel->game_field->reset_sequence(playerid, static_cast<uint8_t>(location));
	return 1;
}

// Writes code and unique card id of every slot of a location (0 for empty
// field zones). Returns the number of slots; writes at most `cap` entries.
OCGEXT_API uint32_t OCG_ExtGetLocationCards(OCG_Duel ocg_duel, uint8_t playerid, uint32_t location, uint32_t* codes, uint32_t* ids, uint32_t cap) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	auto* list = location_list(pduel, playerid, location);
	if(!list)
		return 0;
	uint32_t n = static_cast<uint32_t>(list->size());
	for(uint32_t i = 0; i < n && i < cap; ++i) {
		card* pcard = (*list)[i];
		codes[i] = pcard ? pcard->data.code : 0;
		ids[i] = pcard ? pcard->cardid : 0;
	}
	return n;
}

// Same for the xyz materials of the card at a monster/spell zone.
OCGEXT_API uint32_t OCG_ExtGetOverlayCards(OCG_Duel ocg_duel, uint8_t playerid, uint32_t location, uint32_t sequence, uint32_t* codes, uint32_t* ids, uint32_t cap) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	auto* list = location_list(pduel, playerid, location);
	if(!list || !(location & (LOCATION_MZONE | LOCATION_SZONE)) || sequence >= list->size() || !(*list)[sequence])
		return 0;
	auto& mats = (*list)[sequence]->xyz_materials;
	uint32_t n = static_cast<uint32_t>(mats.size());
	for(uint32_t i = 0; i < n && i < cap; ++i) {
		codes[i] = mats[i]->data.code;
		ids[i] = mats[i]->cardid;
	}
	return n;
}

// Every slot of both players at once (one call instead of one per location and
// per material): 6 uint32 per entry: player, location, sequence, material index
// + 1 (0 for the slot itself; its materials follow it), code, card id (0, 0 for
// an empty field zone). Locations in the order deck, hand, monster zones, spell
// zones, GY, banished, Extra Deck. Returns the entry count (writes at most `cap`).
OCGEXT_API uint32_t OCG_ExtGetAllCards(OCG_Duel ocg_duel, uint32_t* out, uint32_t cap) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	static const uint32_t locations[] = {LOCATION_DECK, LOCATION_HAND, LOCATION_MZONE, LOCATION_SZONE,
	                                     LOCATION_GRAVE, LOCATION_REMOVED, LOCATION_EXTRA};
	uint32_t n = 0;
	auto put = [&](uint32_t p, uint32_t loc, uint32_t seq, uint32_t mat, card* pcard) {
		if(n < cap) {
			uint32_t* e = out + n * 6;
			e[0] = p; e[1] = loc; e[2] = seq; e[3] = mat;
			e[4] = pcard ? pcard->data.code : 0;
			e[5] = pcard ? pcard->cardid : 0;
		}
		++n;
	};
	for(uint32_t p = 0; p < 2; ++p) {
		for(uint32_t loc : locations) {
			auto* list = location_list(pduel, static_cast<uint8_t>(p), loc);
			uint32_t size = static_cast<uint32_t>(list->size());
			for(uint32_t i = 0; i < size; ++i) {
				card* pcard = (*list)[i];
				put(p, loc, i, 0, pcard);
				if(pcard && (loc & (LOCATION_MZONE | LOCATION_SZONE))) {
					uint32_t m = 0;
					for(card* mat : pcard->xyz_materials)
						put(p, loc, i, ++m, mat);
				}
			}
		}
	}
	return n;
}

// For each link of the current chain, whether its card is still the object that
// was activated (it keeps its relation to the link until it leaves its place, e.g.
// to the deck): after a trip through a hidden pile, which physical copy it is
// cannot be known. Writes one byte per link; returns the link count.
OCGEXT_API uint32_t OCG_ExtChainRelated(OCG_Duel ocg_duel, uint8_t* out, uint32_t cap) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	auto& chain_list = pduel->game_field->core.current_chain;
	uint32_t n = 0;
	for(const auto& ch : chain_list) {
		if(n < cap) {
			card* handler = ch.triggering_effect ? ch.triggering_effect->get_handler() : nullptr;
			out[n] = handler && handler->is_has_relation(ch) ? 1 : 0;
		}
		++n;
	}
	return n;
}

// Debugging: the core's shuffle bookkeeping. Writes check_level, shuffle_check_disabled,
// shuffle_hand_check[0..1], shuffle_deck_check[0..1].
OCGEXT_API void OCG_ExtDebugShuffleState(OCG_Duel ocg_duel, int32_t* out) {
	auto& core = static_cast<duel*>(ocg_duel)->game_field->core;
	out[0] = static_cast<int32_t>(core.check_level);
	out[1] = core.shuffle_check_disabled;
	out[2] = core.shuffle_hand_check[0];
	out[3] = core.shuffle_hand_check[1];
	out[4] = core.shuffle_deck_check[0];
	out[5] = core.shuffle_deck_check[1];
}

// Debugging: the pending trigger chains (core.new_fchain and core.new_ochain).
// Writes 6 uint64 per entry: forced, handler code, description, handler
// controller, handler location, handler sequence. Returns the entry count.
OCGEXT_API uint32_t OCG_ExtDebugPendingTriggers(OCG_Duel ocg_duel, uint64_t* out, uint32_t cap) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	auto& core = pduel->game_field->core;
	uint32_t n = 0;
	auto dump = [&](const auto& list, uint64_t forced) {
		for(const auto& ch : list) {
			if(n < cap) {
				effect* peffect = ch.triggering_effect;
				card* handler = peffect ? peffect->get_handler() : nullptr;
				out[n * 6 + 0] = forced;
				out[n * 6 + 1] = handler ? handler->data.code : 0;
				out[n * 6 + 2] = peffect ? peffect->description : 0;
				out[n * 6 + 3] = handler ? handler->current.controler : 0;
				out[n * 6 + 4] = handler ? handler->current.location : 0;
				out[n * 6 + 5] = handler ? handler->current.sequence : 0;
			}
			++n;
		}
	};
	dump(core.new_fchain, 1);
	dump(core.new_ochain, 0);
	return n;
}

// Basic duel information: turn, turn player, phase, lp0, lp1.
// (code, description) of the effects the current selection is about
// (core.select_chains), e.g. the optional trigger of a yes/no prompt.
OCGEXT_API uint32_t OCG_ExtSelectChains(OCG_Duel ocg_duel, uint64_t* out, uint32_t cap) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	uint32_t n = 0;
	for(const auto& ch : pduel->game_field->core.select_chains) {
		if(n >= cap)
			break;
		effect* peffect = ch.triggering_effect;
		card* handler = peffect ? peffect->get_handler() : nullptr;
		out[n * 2] = handler ? handler->data.code : 0;
		out[n * 2 + 1] = peffect ? peffect->description : 0;
		++n;
	}
	return n;
}

// Re-synchronizing an approximate reconstruction with what its viewer observed
// (see belief/sync.py): these change the state silently, without messages or
// events, at a decision with an empty chain.

// Moves the card with this id to (con, location, sequence) in `position`, as if it
// had always been there: it leaves its equip/Xyz relations, its effects of the old
// place reset, its Xyz materials go to their owner's GY. Sequence for the deck:
// 0 top, 1 bottom. Returns 0 if the card is unknown or the zone is taken.
OCGEXT_API int OCG_ExtPlaceCard(OCG_Duel ocg_duel, uint32_t id, uint8_t con, uint32_t location, uint32_t sequence, uint32_t position) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	auto& f = *pduel->game_field;
	card* pcard = nullptr;
	for(auto* c : pduel->cards) {
		if(c->cardid == id) {
			pcard = c;
			break;
		}
	}
	if(!pcard || con > 1 || (location == 0 && !pcard->current.location && !pcard->overlay_target))
		return 0;
	if(location == LOCATION_MZONE || location == LOCATION_SZONE) {
		auto& zones = location == LOCATION_MZONE ? f.player[con].list_mzone : f.player[con].list_szone;
		if(sequence >= zones.size() || (zones[sequence] && zones[sequence] != pcard))
			return 0;
	}
	if(pcard->current.location == location && pcard->current.controler == con
	   && (location & LOCATION_ONFIELD) && pcard->current.sequence == sequence) {
		pcard->current.position = position;  // (only its position)
		pcard->refresh_disable_status();
		return 1;
	}
	// (no shuffle of its own: a forced shuffle order is meant for the real game's next one)
	auto& core = f.core;
	bool deck_check[2] = {core.shuffle_deck_check[0], core.shuffle_deck_check[1]};
	bool hand_check[2] = {core.shuffle_hand_check[0], core.shuffle_hand_check[1]};
	struct Restore {
		decltype(core)& c;
		bool* d;
		bool* h;
		~Restore() {
			c.shuffle_deck_check[0] = d[0]; c.shuffle_deck_check[1] = d[1];
			c.shuffle_hand_check[0] = h[0]; c.shuffle_hand_check[1] = h[1];
		}
	} restore{core, deck_check, hand_check};
	if(pcard->overlay_target) {
		pcard->overlay_target->xyz_remove(pcard);
	} else if(pcard->current.location) {
		pcard->enable_field_effect(false);
		if(pcard->equiping_target)
			pcard->unequip();
		for(auto* eq : card_set(pcard->equiping_cards))
			eq->unequip();
		for(auto* mat : std::vector<card*>(pcard->xyz_materials.begin(), pcard->xyz_materials.end())) {
			pcard->xyz_remove(mat);
			f.add_card(mat->owner, mat, LOCATION_GRAVE, 0);
			mat->current.position = POS_FACEUP;
		}
		if(pcard->current.location & LOCATION_ONFIELD)
			pcard->reset(RESET_LEAVE, RESET_EVENT);
		pcard->clear_card_target();
		f.remove_card(pcard);
	}
	if(location == 0)
		return 1;  // (out of play: a token the viewer did not see)
	pcard->sendto_param.position = static_cast<uint8_t>(position);
	bool pzone = location == LOCATION_SZONE && (pcard->data.type & TYPE_PENDULUM) && (position & POS_FACEUP)
		&& (sequence == 0 || sequence == 4);
	f.add_card(con, pcard, static_cast<uint8_t>(location), static_cast<uint8_t>(sequence), pzone);
	if(pcard->current.location == 0)
		return 0;
	pcard->current.position = (location & (LOCATION_DECK | LOCATION_HAND)) ? POS_FACEDOWN_DEFENSE
		: (location == LOCATION_EXTRA && !(position & POS_FACEUP)) ? POS_FACEDOWN_DEFENSE : position;
	uint32_t reset = location == LOCATION_HAND ? RESET_TOHAND : location == LOCATION_DECK || location == LOCATION_EXTRA
		? RESET_TODECK : location == LOCATION_GRAVE ? RESET_TOGRAVE : location == LOCATION_REMOVED ? RESET_REMOVE
		: RESET_TOFIELD;
	pcard->reset(reset, RESET_EVENT);
	pcard->enable_field_effect(true);
	pcard->refresh_disable_status();
	return 1;
}

static card* card_by_id(duel* pduel, uint32_t id) {
	for(auto* c : pduel->cards) {
		if(c->cardid == id)
			return c;
	}
	return nullptr;
}

// A new card object out of play (e.g. a token to place); returns its id.
OCGEXT_API uint32_t OCG_ExtCreateCard(OCG_Duel ocg_duel, uint8_t owner, uint32_t code) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	card* pcard = pduel->new_card(code);
	pcard->owner = owner;
	pcard->current.location = 0;
	pcard->current.controler = owner;
	return pcard->cardid;
}

// Sets what the card shows of its history: its reason (unless ~0) and the status
// bits in `mask`.
OCGEXT_API int OCG_ExtSetCardInfo(OCG_Duel ocg_duel, uint32_t id, uint32_t reason, uint32_t status, uint32_t mask) {
	card* pcard = card_by_id(static_cast<duel*>(ocg_duel), id);
	if(!pcard)
		return 0;
	if(reason != UINT32_MAX)
		pcard->current.reason = reason;
	pcard->status = (pcard->status & ~mask) | (status & mask);
	return 1;
}

OCGEXT_API void OCG_ExtSetTurn(OCG_Duel ocg_duel, uint32_t turn) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	pduel->game_field->infos.turn_id = static_cast<int16_t>(turn);
}

OCGEXT_API void OCG_ExtSetLP(OCG_Duel ocg_duel, uint8_t playerid, int32_t lp) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	if(playerid < 2)
		pduel->game_field->player[playerid].lp = lp < 0 ? 0 : lp;
}

// At an Idle or Battle command prompt: drops the prompt and lets the phase compute
// it again (after the state was changed, its lists would be stale). The next
// OCG_DuelProcess sends the new prompt. Returns 0 if the duel is at another prompt.
OCGEXT_API int OCG_ExtRestartCommand(OCG_Duel ocg_duel) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	auto& units = pduel->game_field->core.units;
	if(units.size() < 2 || !pduel->game_field->core.subunits.empty())
		return 0;
	auto first = units.begin();
	auto second = std::next(first);
	Processors::Process<false>* parent = nullptr;
	if(Processors::get_opt_variant<Processors::SelectIdleCmd>(*first)) {
		parent = Processors::get_opt_variant<Processors::IdleCommand>(*second);
	} else if(Processors::get_opt_variant<Processors::SelectBattleCmd>(*first)) {
		parent = Processors::get_opt_variant<Processors::BattleCommand>(*second);
	}
	if(!parent || parent->step != 1)
		return 0;
	units.pop_front();
	parent->step = 0;
	return 1;
}

OCGEXT_API void OCG_ExtGetInfo(OCG_Duel ocg_duel, uint32_t* out5) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	auto& f = *pduel->game_field;
	out5[0] = f.infos.turn_id;
	out5[1] = f.infos.turn_player;
	out5[2] = f.infos.phase;
	out5[3] = static_cast<uint32_t>(f.player[0].lp);
	out5[4] = static_cast<uint32_t>(f.player[1].lp);
}

// Analysis aid: turns the read log on or off and returns what it holds since the
// last call (pairs of card id and a mask of the properties the game read: code 1,
// type 2, Level 4, Rank 8, attribute 16, race 32, ATK 64, DEF 128, archetypes 256,
// Link 512), clearing it. Reads while answering queries are not logged.
OCGEXT_API uint32_t OCG_ExtReadLog(OCG_Duel ocg_duel, uint32_t enable, uint32_t* out, uint32_t cap) {
	auto* pduel = static_cast<duel*>(ocg_duel);
	pduel->read_log_on = enable != 0;
	uint32_t n = 0;
	for(const auto& it : pduel->read_log) {
		if(n < cap) {
			out[2 * n] = it.first;
			out[2 * n + 1] = it.second;
		}
		++n;
	}
	pduel->read_log.clear();
	return n;
}

// Compiles a Lua script to a binary chunk (with its debug information: error
// messages and tracebacks stay the same), which OCG_LoadScript accepts like the
// source: loading it skips the compiler, which is most of the cost of setting up
// a duel. Writes up to `cap` bytes to `out` and returns the size of the chunk,
// or 0 if the script does not compile (load the source then, for the error).
OCGEXT_API uint32_t OCG_ExtCompileScript(const char* src, uint32_t len, const char* name, uint8_t* out, uint32_t cap) {
	lua_State* L = luaL_newstate();
	if(!L)
		return 0;
	std::string chunk;
	if(luaL_loadbuffer(L, src, len, name) == LUA_OK) {
		lua_dump(L, [](lua_State*, const void* p, size_t sz, void* ud) -> int {
			static_cast<std::string*>(ud)->append(static_cast<const char*>(p), sz);
			return 0;
		}, &chunk, 0);
	}
	lua_close(L);
	if(chunk.size() <= cap)
		std::copy(chunk.begin(), chunk.end(), out);
	return static_cast<uint32_t>(chunk.size());
}
