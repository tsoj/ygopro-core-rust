//! Raidraptor's Phantom Knights Claw
use super::prelude::*;

pub const ID: u32 = 13486638;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_XYZ_DRAGON, SET_THE_PHANTOM_KNIGHTS, SET_RAIDRAPTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsAttribute(ATTRIBUTE_DARK)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_attribute(d, c, ATTRIBUTE_DARK)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // local mg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in aux.Next(mg) do g:Merge(tc:GetOverlayGroup()) end
    for tc in group::iter(d, &mg) {
        let ov = card::get_overlay_group(d, tc);
        group::add_card(&mut g, &ov);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVEXYZ)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVEXYZ as u64);
    // local sc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // if sc:IsSetCard({SET_XYZ_DRAGON,SET_THE_PHANTOM_KNIGHTS,SET_RAIDRAPTOR}) then e:SetLabel(1) else e:SetLabel(0) end
    if card::is_set_card(d, sc, &[SET_XYZ_DRAGON as u16, SET_THE_PHANTOM_KNIGHTS as u16, SET_RAIDRAPTOR as u16]) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_DETACH_MATERIAL,e,0,0,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::raise_single_event(d, h, EVENT_DETACH_MATERIAL, Some(a.e), 0, 0, 0, 0);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_RAIDRAPTOR) and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16]) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re)
    //   and Duel.Destroy(eg,REASON_EFFECT)~=0 and e:GetLabel()==1 then
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if !card::is_relate_to_effect(d, rh, a.re.nil()) {
        return;
    }
    if duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) == 0 || effect::get_label(d, a.e) != 1 {
        return;
    }
    // local atk=Duel.GetOperatedGroup():GetFirst():GetBaseAttack()
    let op = duel::get_operated_group(d);
    let first = group::get_first(d, &op).nil();
    let atk = card::get_base_attack(d, first);
    // local g=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
}
