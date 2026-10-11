//! Vision HERO Adoration
//! V・HERO アドレイション
use super::prelude::*;

pub const ID: u32 = 45170821;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], material_setcode: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_HERO),2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_HERO as u16], Some(fc), st, Some(tp)))), 2)]);
    // summon success
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.valtg)
    effect::set_target(d, e2, Some(cost_target(valtg)));
    // e2:SetOperation(s.valop)
    effect::set_operation(d, e2, Some(operation(valop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_HERO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HERO as u16])
}
/// `s.valtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn valtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(s.sfilter,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&sfilter), tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_OPPO as u64);
    // local g1=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    let g1 = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1);
    effect::set_label_object(d, a.e, Value::card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
    // Duel.SelectTarget(tp,s.sfilter,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::select_target(d, tp, Some(&sfilter), tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    true
}
/// `s.valop(e,tp,eg,ep,ev,re,r,rp)`
fn valop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local o=e:GetLabelObject()
    let o = effect::get_label_object(d, a.e).as_card().nil();
    // local s=g:GetFirst()
    // if s==o then s=g:GetNext() end
    let members = group::iter(d, &g);
    let mut s = members.first().copied().nil();
    if s == o {
        s = members.get(1).copied().nil();
    }
    // if s:IsFaceup() and o:IsFaceup() and s:IsRelateToEffect(e) and o:IsRelateToEffect(e) then
    if card::is_faceup(d, s) && card::is_faceup(d, o) && card::is_relate_to_effect(d, s, a.e) && card::is_relate_to_effect(d, o, a.e) {
        // local val=s:GetAttack()*-1
        let val = card::get_attack(d, s) as i64 * -1;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(val)
        effect::set_value(d, e1, val);
        // o:RegisterEffect(e1)
        card::register_effect(d, o, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // o:RegisterEffect(e2)
        card::register_effect(d, o, e2, false);
    }
}
