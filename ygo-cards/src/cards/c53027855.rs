//! Windrose the Elemental Lord
use super::prelude::*;

pub const ID: u32 = 53027855;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // leave
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e4:SetOperation(s.leaveop)
    effect::set_operation(d, e4, Some(operation(leaveop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and
    //     Duel.GetMatchingGroupCount(Card.IsAttribute,c:GetControler(),LOCATION_GRAVE,0,nil,ATTRIBUTE_WIND)==5
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_matching_group_count(d, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_WIND)), p, LOCATION_GRAVE, 0, NO_EX) == 5
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
/// `s.leaveop(e,tp,eg,ep,ev,re,r,rp)`
fn leaveop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsFacedown() then return end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, h) {
        return;
    }
    // local effp=e:GetHandler():GetControler()
    let effp = card::get_controler(d, h);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SKIP_BP)
    effect::set_code(d, e1, EFFECT_SKIP_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // if Duel.GetTurnPlayer()==effp then
    if duel::get_turn_player(d) == effp {
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None);
        effect::set_label(d, e1, &[turn as i64]);
        // e1:SetCondition(s.skipcon)
        effect::set_condition(d, e1, Some(condition(skipcon)));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
    }
    // Duel.RegisterEffect(e1,effp)
    duel::register_effect(d, e1, effp);
}
/// `s.skipcon(e)`
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
