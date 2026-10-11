//! Fortune Lady Wind
use super::prelude::*;

pub const ID: u32 = 82693917;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FORTUNE_LADY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk,def
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(s.value)
    effect::set_value_fn(d, e1, effect_card_value(value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // level up
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCondition(s.lvcon)
    effect::set_condition(d, e3, Some(condition(lvcon)));
    // e3:SetOperation(s.lvop)
    effect::set_operation(d, e3, Some(operation(lvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.value(e,c)
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*300
    card::get_level(d, c) as i64 * 300
}
// function s.lvcon(e,tp,eg,ep,ev,re,r,rp)
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():IsLevelAbove(1) and e:GetHandler():IsLevelBelow(11)
    duel::is_turn_player(d, a.tp) && {
        let h = effect::get_handler(d, a.e).nil();
        card::is_level_above(d, h, 1) && card::is_level_below(d, h, 11)
    }
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) or c:IsLevelAbove(12) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) || card::is_level_above(d, c, 12) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
// aux.FaceupFilter(Card.IsSetCard,SET_FORTUNE_LADY)
fn fortune_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FORTUNE_LADY as u16])
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_FORTUNE_LADY),tp,LOCATION_MZONE,0,nil)
        let ct = duel::get_matching_group_count(d, Some(&fortune_faceup), tp, LOCATION_MZONE, 0, NO_EX) as i64;
        // local dt=Duel.GetMatchingGroupCount(s.filter,tp,0,LOCATION_ONFIELD,nil)
        let dt = duel::get_matching_group_count(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX) as i64;
        // e:SetLabel(ct)
        effect::set_label(d, e, &[ct]);
        // return dt>=ct
        return dt >= ct;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,e:GetLabel(),0,0)
    let label = effect::get_label(d, e);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), label as u8, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_FORTUNE_LADY),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&fortune_faceup), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if ct>#g then return end
    if ct > g.len() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local sg=g:Select(tp,ct,ct,nil)
    let sg = group::select(d, &g, tp, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.HintSelection(sg)
    duel::hint_selection(d, sg.clone(), true);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
