//! Red Reign
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 5376159;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish all monsters on the field, except monster(s) with the highest level
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Add this card from GY to hand
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(8) and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_level_above(d, c, 8) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// The `Card.IsAbleToRemove` group of `s.target` and `s.activate`.
fn removable(d: &mut Duel, tp: u8) -> CardSet {
    // Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
/// `Card.GetLevel` as a value function
fn level_of(d: &mut Duel, c: CardId) -> i64 {
    card::get_level(d, c) as i64
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = removable(d, tp);
    // local fdwng,fupg=g:Split(Card.IsFacedown,nil)
    let (fdwng, fupg) = group::split(d, &g, &|d, c| card::is_facedown(d, c), NO_EX);
    // local exg=fupg:GetMaxGroup(Card.GetLevel) or Group.CreateGroup()
    let exg = group::get_max_group(d, &fupg, &level_of).map(|r| r.0).unwrap_or_default();
    // if chk==0 then return #fdwng>0 or #(g-exg)>0 end
    let mut rest = g.clone();
    group::remove_card(&mut rest, &exg);
    if a.chk == Some(0) {
        return !fdwng.is_empty() || !rest.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,(g-exg),#(g-exg),tp,0)
    let n = rest.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rest), n, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let mut g = removable(d, tp);
    // local fg=g:Filter(aux.FaceupFilter(Card.IsLevelAbove,1),nil)
    let fg = group::filter(d, &g, &|d, c| card::is_faceup(d, c) && card::is_level_above(d, c, 1), NO_EX);
    // if #fg>0 then g=g-fg:GetMaxGroup(Card.GetLevel) end
    if !fg.is_empty() {
        let exg = group::get_max_group(d, &fg, &level_of).map(|r| r.0).unwrap_or_default();
        group::remove_card(&mut g, &exg);
    }
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // local ig=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ig = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #ig==0 then return end
    if ig.is_empty() {
        return;
    }
    // ig:ForEach(s.immop,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    for tc in group::iter(d, &ig) {
        immop(d, tc, h);
    }
}
/// `s.immop(tc,c)`
fn immop(d: &mut Duel, tc: CardId, c: CardId) {
    // Unaffected by other card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3100)
    effect::set_description(d, e1, 3100);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| efilter(d, a.e(0), a.e(1))));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    // return te:GetOwner()~=e:GetHandler()
    effect::get_owner(d, te) != Some(effect::get_handler(d, e).nil())
}
/// `s.thcfilter(c,tp)`
fn thcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_SYNCHRO)
    //     and c:IsControler(tp) and c:IsSynchroSummoned()
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_controler(d, c, tp)
        && card::is_synchro_summoned(d, c)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.thcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| thcfilter(d, c, tp), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SendtoHand(c,nil,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
