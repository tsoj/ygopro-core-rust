//! Blue-Eyes White Dragon, the White Phantom Beast
use super::prelude::*;

pub const ID: u32 = 30397786;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLUE_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When a card or effect is activated that targets this card on the field (Quick Effect): You can discard 1 card; negate that effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_cost(d, e1, Some(cost::discard(None, false, None, None, None)));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);

    // When this card is added from the Deck to your hand, or your monster is destroyed by battle: You can reveal this card in your hand; Special Summon it
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2a, EVENT_TO_HAND);
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    effect::set_condition(d, e2a, Some(condition(cond_e2a)));
    effect::set_cost(d, e2a, Some(cost::self_reveal()));
    effect::set_target(d, e2a, Some(cost_target(sptg)));
    effect::set_operation(d, e2a, Some(operation(spop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2b, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e2b, LOCATION_HAND);
    effect::set_condition(d, e2b, Some(condition(cond_e2b)));
    card::register_effect(d, c, e2b, false);

    // If this card is Special Summoned from the hand or Deck: You can destroy all monsters your opponent controls, also you cannot attack directly for the rest of this turn, except with "Blue-Eyes" monsters
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(cond_e3)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let tg = tg[0].as_group().cloned();
    // return tg and tg:IsContains(c) and Duel.IsChainDisablable(ev)
    match tg {
        Some(tg) => group::is_contains(&tg, c) && duel::is_chain_disablable(d, a.ev as u8),
        None => false,
    }
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
fn cond_e2a(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_DECK)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_DECK)
}
fn cond_e2b(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsPreviousControler,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_previous_controler(d, c, a.tp), 1, NO_EX)
}
fn cond_e3(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSummonLocation(LOCATION_HAND|LOCATION_DECK)
    let c = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, c, LOCATION_HAND | LOCATION_DECK)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // You cannot attack directly for the rest of this turn, except with "Blue-Eyes" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_BLUE_EYES as u16]))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
