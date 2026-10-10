//! Unchained Abomination
use super::prelude::*;

pub const ID: u32 = 29479265;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[29479265], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,nil,2,4,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(lcheck)), None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // If a card(s) on the field is destroyed by card effect, except by "Unchained Abomination" (except during the Damage Step): You can target 1 card on the field; destroy it
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // When another monster is destroyed by battle: You can target 1 card on the field; destroy it
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(descon2)));
    card::register_effect(d, c, e2, false);
    // During the End Phase: You can target 1 card on the field; destroy it
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_condition(d, e3, Some(aux_true()));
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    card::register_effect(d, c, e3, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_LINK,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_LINK, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReason(REASON_EFFECT) and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_reason(d, c, REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil) and not (re and re:GetHandler():IsCode(id))
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
        && !match a.re {
            Some(re) => card::is_code(d, effect::get_handler(d, re).nil(), &[ID]),
            None => false,
        }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsReason,1,e:GetHandler(),REASON_BATTLE)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &|d, c| card::is_reason(d, c, REASON_BATTLE), 1, h.into())
}
