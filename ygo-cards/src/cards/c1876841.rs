//! Morphtronic Scannen
use super::prelude::*;

pub const ID: u32 = 1876841;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon itself by banishing 1 "Morphtronic" monster from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| spcon(d, a))));
    effect::set_target(d, e1, Some(lua_fn(|d, a: Args| sptg(d, a))));
    effect::set_operation(d, e1, Some(lua_fn(|d, a: Args| spop(d, a))));
    card::register_effect(d, c, e1, false);
    // Search 1 "Morphtronic" Spell/Trap, then return 1 card to the Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond_a)));
    effect::set_target(d, e2, Some(cost_target(target_a)));
    effect::set_operation(d, e2, Some(operation(operation_a)));
    card::register_effect(d, c, e2, false);
    // Add 1 "Morphtronic" monster from GY to the hand, then return 1 card to the Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_TODECK);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(cond_d)));
    effect::set_target(d, e3, Some(cost_target(target_d)));
    effect::set_operation(d, e3, Some(operation(operation_d)));
    card::register_effect(d, c, e3, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORPHTRONIC) and c:IsMonster() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,c)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, c.into())
}
fn sptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.card(9);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,true,c)
    let ex = c.map_or(NO_EX, |c| c.into());
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_HAND, 0, 1, 1, true, ex);
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // g:KeepAlive()
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g));
            return true;
        }
    }
    false
}
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
fn thdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORPHTRONIC) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn cond_a(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e) return e:GetHandler():IsAttackPos() end
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}
fn target_a(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thdfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn operation_a(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thdfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty()
        && duel::send_to_hand(d, &g, None, REASON_EFFECT, None) > 0
        && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_HAND)
    {
        duel::confirm_cards(d, 1 - tp, &g);
        duel::shuffle_hand(d, tp);
        duel::shuffle_deck(d, tp);
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let dg = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        duel::send_to_deck(d, &dg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
fn thgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_MORPHTRONIC) and c:IsLevelBelow(4) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_level_below(d, c, 4) && card::is_able_to_hand(d, c, None)
}
fn cond_d(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e) return e:GetHandler():IsDefensePos() end
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
fn target_d(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thgfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn operation_d(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thgfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty()
        && duel::send_to_hand(d, &g, None, REASON_EFFECT, None) > 0
        && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_HAND)
    {
        duel::confirm_cards(d, 1 - tp, &g);
        duel::shuffle_hand(d, tp);
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let dg = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        duel::send_to_deck(d, &dg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
