//! Elemental HERO Flare Neos
use super::prelude::*;

pub const ID: u32 = 81566151;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO, SET_NEOS, SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_NEOS,89621922)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_NEOS), fusion::Mat::Code(89621922)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // aux.EnableNeosReturn(c)
    aux::enable_neos_return(d, c, None, None, None, None);
    // atkup
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e5, EFFECT_UPDATE_ATTACK);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetValue(s.atkval)
    effect::set_value_fn(d, e5, effect_card_value(atkval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&card::is_able_to_deck_or_extra_as_cost), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.contactop(g,tp)`
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA)
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, h, LOCATION_EXTRA)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsSpellTrap,0,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)*400
    duel::get_matching_group_count(d, Some(&card::is_spell_trap), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64 * 400
}
