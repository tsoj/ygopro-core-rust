//! Trickstar Holly Angel
use super::prelude::*;

pub const ID: u32 = 32448765;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_TRICKSTAR),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_TRICKSTAR as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indestructable
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.indtg)
    effect::set_target(d, e3, Some(effect_card_fn(indtg)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // ATK change
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e5, EVENT_DAMAGE);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.atkcon)
    effect::set_condition(d, e5, Some(condition(atkcon)));
    // e5:SetOperation(s.atkop)
    effect::set_operation(d, e5, Some(operation(atkop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.cfilter(c,g)`
fn cfilter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR) and g:IsContains(c)
    card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) && g.contains(c)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // return lg and eg:IsExists(s.cfilter,1,nil,lg)
    // (a Lua group is always truthy, so `lg and` is true)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, &lg), 1, NO_EX)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Damage(1-tp,200,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 200, REASON_EFFECT, false, None);
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR) and e:GetHandler():GetLinkedGroup():IsContains(c)
    if !card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    card::get_linked_group(d, h).contains(c)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and (r&REASON_EFFECT)~=0 and re:IsMonsterEffect() and re:GetHandler():IsSetCard(SET_TRICKSTAR)
    if !(a.ep != a.tp && (a.r & REASON_EFFECT) != 0) {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_set_card(d, rh, &[SET_TRICKSTAR as u16])
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ev)
        effect::set_value(d, e1, a.ev as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
