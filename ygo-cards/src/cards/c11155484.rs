//! Crystal Miracle
use super::prelude::*;

pub const ID: u32 = 11155484;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 "Crystal Beast" card and negate activation
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY | CATEGORY_NEGATE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Place 1 "Crystal Beast" in your Spell/Trap Zone
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY | EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_MOVE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(plcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(pltg)));
    effect::set_operation(d, e2, Some(operation(plop)));
    card::register_effect(d, c, e2, false);
}
// s.descon
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsChainNegatable(ev) and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    duel::is_chain_negatable(d, a.ev as u8) && {
        let re = a.re.nil();
        effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
    }
}
// s.destg
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_ONFIELD,0,1,nil)
        let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
        return duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // local dg=nil
    // local ct=0
    let mut dg: Option<CardSet> = None;
    let mut ct = 0;
    // if re:GetHandler():IsRelateToEffect(re) then dg=eg ct=1 end
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        dg = a.eg.clone();
        ct = 1;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1+ct,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, dg, (1 + ct) as u8, a.tp, LOCATION_ONFIELD as i32);
    true
}
// s.desop
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_ONFIELD,0,1,1,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 and Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 && duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
// s.plconfilter(c,tp)
fn plconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsLocation(LOCATION_SZONE) and not c:IsPreviousLocation(LOCATION_SZONE)
    //     and c:IsControler(tp) and c:GetSequence()<5
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_location(d, c, LOCATION_SZONE)
        && !card::is_previous_location(d, c, LOCATION_SZONE)
        && card::is_controler(d, c, tp)
        && card::get_sequence(d, c) < 5
}
// s.plcon
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(s.plconfilter,1,nil,tp)
    match a.eg.clone() {
        Some(eg) => group::is_exists(d, &eg, &|d, c| plconfilter(d, c, a.tp), 1, NO_EX),
        None => false,
    }
}
// s.plfilter(c)
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
// s.pltg
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_HAND,0,1,nil)
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
// s.plop
fn plop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plfilter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil):GetFirst()
    let f = aux::necro_valley_filter(|d, c| plfilter(d, c));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if let Some(tc) = tc {
        if duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // Treat as Continuous Spell
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
