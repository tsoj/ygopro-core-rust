//! The Fabled Unicore
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FABLED], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 44155002;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_FABLED),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_FABLED as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Negate and destroy
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND) then return end
    if a.ep == a.tp || duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) != duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) {
        return;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if Duel.NegateEffect(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.Destroy(rc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(rc), REASON_EFFECT, None, None);
    }
}
