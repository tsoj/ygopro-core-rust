/*
 * Extensions to ygopro-core used by the yugioh_AI_2 project.
 *
 * The hooks declared here are called from a handful of patched sites in the
 * core (see ocgcore.patch). When nothing has been queued they are no-ops, so
 * a duel that never calls the OCG_Ext* API behaves exactly like upstream.
 *
 * The belief replay uses them to "cheat": it forces the outcome of shuffles
 * and random events so that its simulated duel reproduces what a player
 * observed, and it reorders hidden locations between processing steps.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
#ifndef OCGEXT_H
#define OCGEXT_H

#include <cstdint>
#include <vector>
#include "containers_fwd.h"

class duel;
class card;
class effect;

namespace ocgext {

enum ForcedIntKind : int {
	FORCED_COIN = 0,
	FORCED_DICE = 1,
	FORCED_KIND_COUNT
};

// Returns a queued forced value for `kind` if there is one, otherwise `fallback`.
int32_t forced_int(duel* pduel, int kind, int32_t fallback);

// Called right before MSG_SHUFFLE_{DECK,HAND,EXTRA} is emitted. `count` is the
// number of leading entries of `list` that take part in the shuffle.
void on_shuffle(duel* pduel, uint8_t playerid, uint8_t location, card_vector& list, uint32_t count);

// Called at the end of Group.RandomSelect, may replace the selection.
void on_random_select(duel* pduel, const card_set& from, card_set& selected);

// Called after Duel.ShuffleSetCard computed its permutation: `ms[i]` goes to
// sequence `seq[i]`. May rewrite `ms`.
void on_shuffle_set_card(duel* pduel, card** ms, const uint8_t* seq, int32_t ct);

// Whether the cost/target checks of `peffect` should be skipped when
// checking if `playerid` can activate it (see OCG_ExtForceActivatable).
bool is_forced_activatable(duel* pduel, effect* peffect, uint8_t playerid);

// Whether `pcard` counts as matching every card filter right now (see
// OCG_ExtSetForcedMatch).
bool is_forced_match(duel* pduel, card* pcard);

// Whether `pcard` counts as matching no card filter right now (see
// OCG_ExtSetForcedNoMatch).
bool is_forced_nomatch(duel* pduel, card* pcard);

// Records that a card filter was evaluated on `pcard` with `result`, if the read
// log is on and the card is hidden from its viewer (see OCG_ExtSetReadLog).
void log_filter_read(duel* pduel, card* pcard, bool result);
bool read_log_on(duel* pduel);

// Records that a forced activation (see OCG_ExtForceActivatable) made the mandatory
// trigger `peffect` activatable where its own checks failed (see OCG_ExtTakeForcedUsed).
void note_forced_used(duel* pduel, effect* peffect);

void on_destroy(duel* pduel);

} // namespace ocgext

#endif /* OCGEXT_H */
