//! Dark End Evaporation Dragon
use super::prelude::*;

pub const ID: u32 = 62002838;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.hspcost)
    effect::set_cost(d, e1, Some(cost_target(hspcost)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(cost_target(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(operation(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.hspcostfilter(c)`
fn hspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(8) and c:IsRace(RACE_DRAGON) and c:IsAbleToRemoveAsCost()
    card::is_level(d, c, &[8]) && card::is_race(d, c, RACE_DRAGON) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.hspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.hspcostfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| hspcostfilter(d, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.hspcostfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| hspcostfilter(d, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp)`
fn hspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // Cannot Special Summon monsters for the rest of the turn, except Dragon monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_DRAGON) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_DRAGON))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// Calls the Lua function `fustg(e,tp,eg,ep,ev,re,r,rp,chk)` (a Fusion Summon target) directly.
fn call_fustg(d: &mut Duel, fustg: &LuaFn, a: &EffArgs, chk: i64) -> bool {
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ];
    fustg.call(d, &args).truthy()
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return e:GetLabel()==2 and chkc:IsLocation(LOCATION_MZONE) and chkc:IsAttackPos() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) == 2 && card::is_location(d, chkc, LOCATION_MZONE) && card::is_attack_pos(d, chkc) && chkc != c;
    }
    // local fustg=Fusion.SummonEffTG()
    let fustg = fusion::summon_eff_tg(d, fusion::SummonEff::default());
    // local b1=fustg(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = call_fustg(d, &fustg, a, 0);
    // local b2=c:IsAttackAbove(500) and c:IsDefenseAbove(500)
    //     and Duel.IsExistingTarget(Card.IsAttackPos,tp,LOCATION_MZONE,LOCATION_MZONE,1,c)
    let b2 = card::is_attack_above(d, c, 500)
        && card::is_defense_above(d, c, 500)
        && duel::is_existing_target(d, Some(&|d, x| card::is_attack_pos(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]).map_or(0, |x| x as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // fustg(e,tp,eg,ep,ev,re,r,rp,1)
        call_fustg(d, &fustg, a, 1);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAttackPos,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
        let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_attack_pos(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
        // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,c,1,tp,-500)
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, a.tp, -500);
        // Duel.SetOperationInfo(0,CATEGORY_DEFCHANGE,c,1,tp,-500)
        duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(CardSet::from_card(c)), 1, a.tp, -500);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Fusion.SummonEffOP()(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fusion::SummonEff::default(), a);
    } else if op == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not (c:IsRelateToEffect(e) and c:IsFaceup() and c:IsAttackAbove(500) and c:IsDefenseAbove(500)) then return end
        if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_attack_above(d, c, 500) && card::is_defense_above(d, c, 500)) {
            return;
        }
        // if c:UpdateAttack(-500)==-500 and c:UpdateDefense(-500)==-500 then
        if card::update_attack(d, c, -500, None, None, None) == -500 && card::update_defense(d, c, -500, None, None, None) == -500 {
            // local tc=Duel.GetFirstTarget()
            let tc = duel::first_target(d).nil();
            // if tc:IsRelateToEffect(e) then
            if card::is_relate_to_effect(d, tc, a.e) {
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            }
        }
    }
}
