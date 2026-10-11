//! Brotherhood of the Fire Fist - Wolf
use super::prelude::*;

pub const ID: u32 = 93294869;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FORMATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // e2:SetOperation(s.flipop)
    effect::set_operation(d, e2, Some(operation(flipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FIRE_FORMATION) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FIRE_FORMATION) and c:IsSpell() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16]) && card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter1), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // if e:GetHandler():GetFlagEffect(id)~=0 then
    let h = effect::get_handler(d, e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        // e:SetLabel(1)
        effect::set_label(d, e, &[1]);
        // e:GetHandler():ResetFlagEffect(id)
        card::reset_flag_effect(d, h, ID);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter1), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, tp, g, None, true);
        // local sg=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_DECK,0,nil)
        let sg = duel::get_matching_group(d, Some(&filter2), tp, LOCATION_DECK, 0, NO_EX);
        // if e:GetLabel()==1 and #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if effect::get_label(d, e) == 1 && !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
            // local tg=sg:Select(tp,1,1,nil)
            let tg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            // Duel.SSet(tp,tg)
            duel::s_set(d, tp, tg, None, true);
        }
    }
}
/// `s.flipop(e,tp,eg,ep,ev,re,r,rp)`
fn flipop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,0,0,0)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, 0, 0, 0, 0, 0);
}
