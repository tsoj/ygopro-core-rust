//! 十二獣ワイルドボウ
//! Zoodiac Boarbow
use super::prelude::*;

pub const ID: u32 = 74393852;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZOODIAC], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 5 Level 4 monsters (or 1 "Zoodiac" monster you control with a different name)
    // Xyz.AddProcedure(c,nil,4,5,s.ovfilter,aux.Stringid(id,0),nil,s.xyzop)
    xyz::add_procedure(d, c, None, Some(4), 5, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), None, Some(lua_fn(xyzop)), false, None);
    // This card gains the ATK/DEF of all "Zoodiac" monsters attached to it as its materials
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetValue(s.atkval)
    effect::set_value_fn(d, e1a, effect_value(atkval));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // e1b:SetValue(s.defval)
    effect::set_value_fn(d, e1b, effect_value(defval));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // This card can attack directly
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // When this card inflicts battle damage to your opponent while it has 12 or more Xyz Materials: You can send as many cards as possible from your opponent's hand and field to the Graveyard, then change this card to Defense Position.
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ZOODIAC,lc,SUMMON_TYPE_XYZ,tp) and not c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,id)
    card::is_faceup(d, c)
        && card::is_set_card_ctx(d, c, &[SET_ZOODIAC as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && !card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[ID])
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // return true
    true
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:HasNonZeroAttack()
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::has_non_zero_attack(d, c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.atkfilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &|d, c| atkfilter(d, c), NO_EX);
    // return g:GetSum(Card.GetAttack)
    group::get_sum(d, &g, &|d, x| card::get_attack(d, x) as i64)
}
/// `s.deffilter(c)`
fn deffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:HasNonZeroDefense()
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::has_non_zero_defense(d, c)
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.deffilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &|d, c| deffilter(d, c), NO_EX);
    // return g:GetSum(Card.GetDefense)
    group::get_sum(d, &g, &|d, x| card::get_defense(d, x) as i64)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and e:GetHandler():GetOverlayCount()>=12
    let h = effect::get_handler(d, a.e).nil();
    a.ep == 1 - a.tp && card::get_overlay_count(d, h) >= 12
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND|LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, (LOCATION_HAND | LOCATION_ONFIELD) as u32);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND|LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, (LOCATION_HAND | LOCATION_ONFIELD) as u32);
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_grave(d, g, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)==0 then return end
        if group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) == 0 {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
            duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
}
