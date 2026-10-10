//! BK Cheat Commissioner
//! BK チート・コミッショナー
use super::prelude::*;

pub const ID: u32 = 76589815;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // must attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e3:SetCondition(s.atcon)
    effect::set_condition(d, e3, Some(condition(atcon)));
    // e3:SetValue(aux.imval2)
    effect::set_value_fn(d, e3, aux::imval2());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.cfcon)
    effect::set_condition(d, e3, Some(condition(cfcon)));
    // e3:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e3:SetTarget(s.cftg)
    effect::set_target(d, e3, Some(cost_target(cftg)));
    // e3:SetOperation(s.cfop)
    effect::set_operation(d, e3, Some(operation(cfop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atfilter(c)`
fn atfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BATTLIN_BOXER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16])
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.atfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&atfilter), p, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.cfcon(e,tp,eg,ep,ev,re,r,rp)`
fn cfcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return (a:IsControler(tp) and a~=e:GetHandler() and a:IsSetCard(SET_BATTLIN_BOXER))
    let h = effect::get_handler(d, a.e).nil();
    let first = card::is_controler(d, att, tp) && att != h && card::is_set_card(d, att, &[SET_BATTLIN_BOXER as u16]);
    if first {
        return true;
    }
    // or (at and at:IsControler(tp) and at:IsFaceup() and at~=e:GetHandler() and at:IsSetCard(SET_BATTLIN_BOXER))
    match at {
        Some(at) => card::is_controler(d, at, tp) && card::is_faceup(d, at) && at != h && card::is_set_card(d, at, &[SET_BATTLIN_BOXER as u16]),
        None => false,
    }
}
/// `s.cftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0 && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    true
}
/// `s.cffilter(c)`
fn cffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsSSetable()
    card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.cfop(e,tp,eg,ep,ev,re,r,rp)`
fn cfop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g.clone());
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        // local sg=g:Filter(s.cffilter,nil)
        let sg = group::filter(d, &g, &cffilter, NO_EX);
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
            // local setg=sg:Select(tp,1,1,nil)
            let setg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            // Duel.SSet(tp,setg:GetFirst())
            let first = group::get_first(d, &setg).nil();
            duel::s_set(d, tp, CardSet::from_card(first), None, true);
        }
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
}
