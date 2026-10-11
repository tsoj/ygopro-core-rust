//! Defender of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 82498947;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICE_BARRIER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(effect_card_fn(tg)));
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ICE_BARRIER),e:GetHandler():GetControler(),LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::is_existing_matching_card(
        d,
        Some(&|d, cc| card::is_faceup(d, cc) && card::is_set_card(d, cc, &[SET_ICE_BARRIER as u16])),
        p,
        LOCATION_MZONE,
        0,
        1,
        h.into(),
    )
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetAttack()>=e:GetHandler():GetDefense()
    let h = effect::get_handler(d, e).nil();
    card::get_attack(d, c) >= card::get_defense(d, h)
}
