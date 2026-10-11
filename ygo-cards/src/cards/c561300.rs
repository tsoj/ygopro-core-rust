//! Poisonous Viper
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 561300;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon it, or if your opponent controls no monsters, Special Summon it to their field in Defense Position instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take 500 damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d: &mut Duel, a: &EffArgs| duel::is_turn_player(d, a.tp))));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,500) end)
    effect::set_target(d, e2, Some(cost_target(e2target)));
    // e2:SetOperation(function(e,tp) Duel.Damage(tp,500,REASON_EFFECT) end)
    effect::set_operation(d, e2, Some(operation(e2op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 Set Spell/Trap your opponent controls
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3a, aux::stringid(ID, 2));
    // e3a:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3a, CATEGORY_DESTROY);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3a:SetCode(EVENT_REMOVE)
    effect::set_code(d, e3a, EVENT_REMOVE);
    // e3a:SetCountLimit(1,id)
    effect::set_count_limit(d, e3a, 1, ID, 0, 0);
    // e3a:SetTarget(s.destg)
    effect::set_target(d, e3a, Some(cost_target(destg)));
    // e3a:SetOperation(s.desop)
    effect::set_operation(d, e3a, Some(operation(desop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3b, EVENT_TO_GRAVE);
    // e3b:SetCondition(function(e) return e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e3b, Some(condition(e3bcond)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 then
        if duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0 {
            // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
            return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
        } else {
            // local opp=1-tp
            let opp = 1 - a.tp;
            // return Duel.GetLocationCount(opp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,opp)
            return duel::get_location_count(d, opp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(opp), None);
        }
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // if Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 then
        if duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0 {
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        } else {
            // Duel.SpecialSummon(c,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, c, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_SZONE) and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_SZONE) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFacedown,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsFacedown,tp,0,LOCATION_SZONE,1,1,nil):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, x| card::is_facedown(d, x)), a.tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    // Duel.SetChainLimit(function(e,ep,tp) return e:GetHandler()~=tc end)
    duel::set_chain_limit(
        d,
        lua_fn(move |d, args| {
            let h = effect::get_handler(d, args.e(0)).nil();
            h != tc
        }),
    );
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of the damage trigger's target
fn e2target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 500);
    true
}
/// `function(e,tp) Duel.Damage(tp,500,REASON_EFFECT) end`
fn e2op(d: &mut Duel, a: &EffArgs) {
    duel::damage(d, a.tp, 500, REASON_EFFECT, false, None);
}
/// `function(e) return e:GetHandler():IsReason(REASON_EFFECT) end`
fn e3bcond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT)
}
