//! Offering to the Snake Deity
//! 毒蛇の供物
use super::prelude::*;

pub const ID: u32 = 93217231;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_REPTILE),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_REPTILE)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_REPTILE),tp,LOCATION_MZONE,0,1,1,nil)
    let mut g1 = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_REPTILE)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1).nil();
    effect::set_label_object(d, a.e, Value::Card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g2=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,2,2,nil)
    let g2 = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 2, 2, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,#g1,0,0)
    let n = g1.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, e).as_card().nil();
    // if #sg~=3 or sc:IsFacedown() or not sc:IsRace(RACE_REPTILE) or sc:IsControler(1-tp) then return end
    if sg.len() != 3 || card::is_facedown(d, sc) || !card::is_race(d, sc, RACE_REPTILE) || card::is_controler(d, sc, 1 - tp) {
        return;
    }
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
