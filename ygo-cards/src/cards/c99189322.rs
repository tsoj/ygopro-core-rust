//! Arcana Call
use super::prelude::*;

pub const ID: u32 = 99189322;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCANA_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change the effect of 1 "Arcana Force" monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARCANA_FORCE) and c:GetFlagEffect(CARD_REVERSAL_OF_FATE)>0
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16]) && card::get_flag_effect(d, c, CARD_REVERSAL_OF_FATE) > 0
}
// function s.rfilter(c)
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARCANA_FORCE) and c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.rfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| rfilter(d, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| rfilter(d, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), 1, 0, 0);
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, Value::card(first));
    true
}
/// `regfun(tc, val)` after `tc:ResetEffect(RESET_DISABLE,RESET_EVENT)`, with `regfun` the
/// `arcanareg` of the script of `of` (false: it has none)
fn reapply(d: &mut Duel, of: CardId, tc: CardId) -> bool {
    // local regfun=X.arcanareg
    // if not regfun then return end
    let Some(regfun) = crate::card_info(d, of).arcanareg else { return false };
    // local val=Arcana.GetCoinResult(tc)
    let val = aux::arcana::get_coin_result(d, tc);
    // tc:ResetEffect(RESET_DISABLE,RESET_EVENT)
    card::reset_effect(d, tc, RESET_DISABLE, RESET_EVENT);
    // regfun(tc,val)
    regfun(d, tc, val.map_or(0, |v| v as i32));
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local regc=e:GetLabelObject()
    let regc = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc=g:GetFirst()
    // if tc==regc then tc=g:GetNext() end
    let cards = group::iter(d, &g);
    let mut tc = cards.first().copied();
    if tc == regc {
        tc = cards.get(1).copied();
    }
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and s.filter(tc) and regc:IsRelateToEffect(e) then
    let Some(tc) = tc else { return };
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && filter(d, tc)) {
        return;
    }
    let regc = regc.nil();
    if !card::is_relate_to_effect(d, regc, a.e) {
        return;
    }
    // Duel.Remove(regc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, CardSet::from_card(regc), POS_FACEUP as u8, REASON_EFFECT, None, None);
    if !reapply(d, regc, tc) {
        return;
    }
    // tc:RegisterFlagEffect(id,RESETS_STANDARD_DISABLE_PHASE_END,0,1)
    card::register_flag_effect(d, tc, ID, RESETS_STANDARD_DISABLE_PHASE_END, 0, 1, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_label_object(d, e1, Value::Card(tc));
    effect::set_operation(d, e1, Some(operation(rec_effect)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
// function s.rec_effect(e,tp,eg,ep,ev,re,r,rp)
fn rec_effect(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    // if not tc or tc:GetFlagEffect(id)==0 or tc:GetFlagEffect(CARD_REVERSAL_OF_FATE)==0 then return end
    let Some(tc) = effect::get_label_object(d, a.e).as_card() else { return };
    if card::get_flag_effect(d, tc, ID) == 0 || card::get_flag_effect(d, tc, CARD_REVERSAL_OF_FATE) == 0 {
        return;
    }
    // local regfun=tc.arcanareg ... regfun(tc,val)
    reapply(d, tc, tc);
}
