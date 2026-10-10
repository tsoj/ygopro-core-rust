//! Arcana Knight Joker
use super::prelude::*;

pub const ID: u32 = 6150044;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,false,false,CARD_QUEEN_KNIGHT,CARD_JACK_KNIGHT,CARD_KING_KNIGHT)
    fusion::add_proc_mix(d, c, false, false, vec![fusion::Mat::Code(CARD_QUEEN_KNIGHT), fusion::Mat::Code(CARD_JACK_KNIGHT), fusion::Mat::Code(CARD_KING_KNIGHT)]);
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_cost(d, e1, Some(cost_target(discost)));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // return (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and tg and tg:IsContains(c) and Duel.IsChainDisablable(ev)
    (effect::is_monster_effect(d, a.re.nil()) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE))
        && tg.is_some_and(|tg| group::is_contains(&tg, c))
        && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.filter(c,tpe)`
fn filter(d: &mut Duel, c: CardId, tpe: u32) -> bool {
    // return c:IsType(tpe) and c:IsDiscardable()
    card::is_type(d, c, tpe) && card::is_discardable(d, c, None, None)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // local rtype=(re:GetActiveType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP))
    let rtype = effect::get_active_type(d, a.re.nil()) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,rtype) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, rtype)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.filter,1,1,REASON_COST|REASON_DISCARD,nil,rtype)
    duel::discard_hand(d, a.tp, Some(&|d, c| filter(d, c, rtype)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
