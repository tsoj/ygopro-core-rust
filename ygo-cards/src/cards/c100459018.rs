//! King of Yamimado
use super::prelude::*;

pub const ID: u32 = 100459018;
const CARD_YAMI: u32 = 59197169;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_YAMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1a, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetValue(aux.indoval)
    effect::set_value_fn(d, e1a, aux::indoval());
    // e1a:SetCondition(function(e) return Duel.IsExistingMatchingCard(Card.IsFacedown,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e1a, Some(lua_fn(e1a_cond)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1b, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1b:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1b, aux::tgoval());
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3a, aux::stringid(ID, 1));
    // e3a:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3a, CATEGORY_DESTROY);
    // e3a:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3a, EFFECT_TYPE_QUICK_O);
    // e3a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3a, EVENT_CHAINING);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3a, 1, ID, 1, 0);
    // e3a:SetCondition(s.descon)
    effect::set_condition(d, e3a, Some(condition(descon)));
    // e3a:SetTarget(s.destg)
    effect::set_target(d, e3a, Some(cost_target(destg)));
    // e3a:SetOperation(s.desop)
    effect::set_operation(d, e3a, Some(operation(desop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3b, EVENT_ATTACK_ANNOUNCE);
    // e3b:SetCondition(s.descon2)
    effect::set_condition(d, e3b, Some(condition(descon2)));
    // e3b:SetOperation(s.desop2)
    effect::set_operation(d, e3b, Some(operation(desop2)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `function(e) return Duel.IsExistingMatchingCard(Card.IsFacedown,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end`
fn e1a_cond(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_facedown(d, x)), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsExistingMatchingCard(Card.IsFacedown,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,2,nil)
    //     or Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_YAMI),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_facedown(d, x)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 2, NO_EX)
        || duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_YAMI])),
            tp,
            LOCATION_ONFIELD,
            LOCATION_ONFIELD,
            1,
            NO_EX,
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil());
    // return re:IsMonsterEffect() and Chain.IsTriggeringLocation(ev,LOCATION_MZONE) and rc:IsRelateToEffect(re)
    //     and rc:GetAttack()<e:GetHandler():GetAttack() and rc:IsFaceup()
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    if !chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy() {
        return false;
    }
    let rc = rc.nil();
    if !card::is_relate_to_effect(d, rc, a.re.nil()) {
        return false;
    }
    let rc_atk = card::get_attack(d, rc);
    let h = effect::get_handler(d, a.e).nil();
    let own_atk = card::get_attack(d, h);
    rc_atk < own_atk && card::is_faceup(d, rc)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil());
    if card::is_relate_to_effect(d, rh.nil(), a.re.nil()) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=Duel.GetAttacker()
    let rc = duel::get_attacker(d).nil();
    // return rc:IsOnField() and rc:GetAttack()<e:GetHandler():GetAttack()
    if !card::is_on_field(d, rc) {
        return false;
    }
    let rc_atk = card::get_attack(d, rc);
    let h = effect::get_handler(d, a.e).nil();
    let own_atk = card::get_attack(d, h);
    rc_atk < own_atk
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, _a: &EffArgs) {
    // local rc=Duel.GetAttacker()
    let rc = duel::get_attacker(d).nil();
    // if rc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, rc) {
        // Duel.Destroy(rc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(rc), REASON_EFFECT, None, None);
    }
}
