//! Mist Archfiend
use super::prelude::*;

pub const ID: u32 = 28601770;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with no tribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // e1:SetOperation(s.ntop)
    effect::set_operation(d, e1, Some(lua_fn(ntop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    let c = a.card(1);
    // if c==nil then return true end
    let Some(c) = c else {
        return true;
    };
    // return minc==0 and c:IsLevelAbove(5) and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0
        && card::is_level_above(d, c, 5)
        && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.ntop(e,tp,eg,ep,ev,re,r,rp,c)`
fn ntop(d: &mut Duel, a: Args) {
    let c = a.card(8).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_TEMP_REMOVE))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_TEMP_REMOVE)), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, h.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 1000);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and Duel.Destroy(c,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && duel::destroy(d, c, REASON_EFFECT, None, None) != 0 {
        // Duel.Damage(tp,1000,REASON_EFFECT)
        duel::damage(d, a.tp, 1000, REASON_EFFECT, false, None);
    }
}
