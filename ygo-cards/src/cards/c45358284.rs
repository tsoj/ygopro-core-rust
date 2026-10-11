//! Mirror Ladybug
use super::prelude::*;

pub const ID: u32 = 45358284;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // level change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.lvcon)
    effect::set_condition(d, e2, Some(condition(lvcon)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(lua_fn(descon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,c:GetControler(),LOCATION_MZONE,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(Card.IsMonster,c:GetControler(),LOCATION_GRAVE,0,1,nil)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&|d, cc| card::is_monster(d, cc)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let c = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,c)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, LOCATION_MZONE, 0, c.into());
    // local lvs=g:GetSum(Card.GetLevel)
    let lvs = group::get_sum(d, &g, &|d, cc| card::get_level(d, cc) as i64);
    // if lvs~=0 then
    if lvs != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lvs)
        effect::set_value(d, e1, lvs);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.descon(e)`
fn descon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return e:GetHandler():GetLevel()>12
    let c = effect::get_handler(d, e).nil();
    card::get_level(d, c) > 12
}
