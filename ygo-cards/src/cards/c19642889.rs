//! Koa'ki Meiru Powerhand
use super::prelude::*;

pub const ID: u32 = 19642889;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={36623431}
    listed_names: &[36623431],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cost
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.mtcon)
    effect::set_condition(d, e1, Some(condition(mtcon)));
    // e1:SetOperation(s.mtop)
    effect::set_operation(d, e1, Some(operation(mtop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(36623431) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[36623431]) && card::is_able_to_grave_as_cost(d, c)
}
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and not c:IsPublic()
    card::is_normal_trap(d, c) && !card::is_public(d, c)
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, CardSet::from_card(c), true);
    // local g1=Duel.GetMatchingGroup(s.cfilter1,tp,LOCATION_HAND,0,nil)
    let g1 = duel::get_matching_group(d, Some(&cfilter1), a.tp, LOCATION_HAND, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter2,tp,LOCATION_HAND,0,nil)
    let g2 = duel::get_matching_group(d, Some(&cfilter2), a.tp, LOCATION_HAND, 0, NO_EX);
    // local select=2 (every branch below assigns it)
    let select: i64 = if !g1.is_empty() && !g2.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1),aux.Stringid(id,2))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1), aux::stringid(ID, 2)]) as i64
    } else if !g1.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,2))
        let s = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 2)]) as i64;
        // if select==1 then select=2 end
        if s == 1 { 2 } else { s }
    } else if !g2.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) as i64 + 1
    } else {
        // select=Duel.SelectOption(tp,aux.Stringid(id,2))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]);
        // select=2
        2
    };
    if select == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=g1:Select(tp,1,1,nil)
        let g = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    } else if select == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        // local g=g2:Select(tp,1,1,nil)
        let g = group::select(d, &g2, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    } else {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if bc and bc:IsAttribute(ATTRIBUTE_LIGHT+ATTRIBUTE_DARK) and c:IsFaceup() then
    if let Some(bc) = bc {
        if card::is_attribute(d, bc, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_faceup(d, c) {
            // c:CreateRelation(bc,RESET_EVENT|RESETS_STANDARD)
            card::create_relation(d, c, bc, RESET_EVENT | RESETS_STANDARD);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetCondition(s.discon)
            effect::set_condition(d, e1, Some(lua_fn(discon)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
            // bc:RegisterEffect(e1)
            card::register_effect(d, bc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetCondition(s.discon)
            effect::set_condition(d, e2, Some(lua_fn(discon)));
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
            // bc:RegisterEffect(e2)
            card::register_effect(d, bc, e2, false);
        }
    }
}
// function(e) return e:GetOwner():IsRelateToCard(e:GetHandler()) end
fn discon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let owner = effect::get_owner(d, e).nil();
    let handler = effect::get_handler(d, e).nil();
    card::is_relate_to_card(d, owner, handler)
}
