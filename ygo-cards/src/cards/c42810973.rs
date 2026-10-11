//! Junk Archer
use super::prelude::*;

pub const ID: u32 = 42810973;
pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_JUNK_SYNCHRON],
    listed_names: &[CARD_JUNK_SYNCHRON],
    material_setcode: &[SET_SYNCHRON as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Junk Synchron" + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,s.tunerfilter,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| tunerfilter(d, x, sc, st, tp))),
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
    // Banish 1 monster your opponent controls until the End Phase of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tunerfilter(c,lc,stype,tp)`
fn tunerfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    // return c:IsSummonCode(lc,stype,tp,CARD_JUNK_SYNCHRON) or c:IsHasEffect(20932152)
    card::is_summon_code(d, c, lc, stype, tp, &[CARD_JUNK_SYNCHRON])
        || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
            tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
        tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // aux.RemoveUntil(tc,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
        aux::remove_until(
            d,
            tc,
            None,
            REASON_EFFECT,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(aux::default_field_return()),
            None,
            None,
            None,
            None,
            None,
        );
    }
}
