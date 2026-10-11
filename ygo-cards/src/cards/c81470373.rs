//! BF－毒風のシムーン
//! Blackwing - Simoon the Poison Wind
use super::prelude::*;

pub const ID: u32 = 81470373;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], listed_names: &[91351370], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 "Black Whirlwind" from your Deck face-up in your Spell & Trap Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.plcost)
    effect::set_cost(d, e1, Some(cost_target(plcost)));
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BLACKWING) and c:IsMonster() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.plcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn plcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,c)
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(91351370) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_code(d, c, &[91351370]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // Can be Normal Summoned without Tributes
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetCode(EFFECT_SUMMON_PROC)
        effect::set_code(d, e1, EFFECT_SUMMON_PROC);
        // e1:SetCondition(s.ntcon)
        effect::set_condition(d, e1, Some(lua_fn(ntcon)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local summonable=c:IsSummonable(true,e1)
        let summonable = card::is_summonable(d, c, true, Some(e1), 0, None);
        // e1:Reset()
        effect::reset(d, e1);
        // return (summonable or c:IsAbleToGrave()) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
        //   and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp)
        return (summonable || card::is_able_to_grave(d, c, None))
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let minc = a.int(2);
    minc == 0 && card::get_level(d, c) > 4 && {
        let p = card::get_controler(d, c);
        duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except DARK monsters
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e0, aux::stringid(ID, 1));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e0:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e0, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetTarget(function(_e,_c) return _c:IsLocation(LOCATION_EXTRA) and not _c:IsAttribute(ATTRIBUTE_DARK) end)
    effect::set_target(d, e0, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_attribute(d, c, ATTRIBUTE_DARK))));
    // e0:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(_e,_c) return not _c:IsOriginalAttribute(ATTRIBUTE_DARK) end)
    aux::add_temp_lizard_check(
        d,
        c,
        a.tp,
        Some(effect_card_fn(|d, _e, c| !card::is_original_attribute(d, c, ATTRIBUTE_DARK))),
        None,
        None,
        None,
        None,
    );
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| plfilter(d, x, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    let Some(tc) = tc else { return };
    if !duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
        return;
    }
    // Send it to the GY during the End Phase and take 1000 damage
    // aux.DelayedOperation(tc,PHASE_END,id,e,tp,s.gyop,nil,0,0,aux.Stringid(id,2))
    aux::delayed_operation(
        d,
        tc,
        PHASE_END,
        ID,
        a.e,
        a.tp,
        Some(std::sync::Arc::new(gyop)),
        None,
        Some(0),
        Some(0),
        Some(aux::stringid(ID, 2)),
        None,
    );
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Can be Normal Summoned without Tributes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local b1=c:IsAbleToGrave()
    let b1 = card::is_able_to_grave(d, c, None);
    // local b2=c:IsSummonable(true,e1)
    let b2 = card::is_summonable(d, c, true, Some(e1), 0, None);
    // if not (b1 or b2) then
    if !(b1 || b2) {
        // e1:Reset()
        effect::reset(d, e1);
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //   {b1,aux.Stringid(id,3)},
    //   {b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if op==1 then
    if op == Some(1) {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    } else {
        // Duel.Summon(tp,c,true,e1)
        duel::summon(d, a.tp, c, true, Some(e1), 0, None);
    }
}
/// `s.gyop(ag,e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, ag: &CardSet, a: &EffArgs) -> bool {
    // Duel.SendtoGrave(ag,REASON_EFFECT)
    duel::send_to_grave(d, ag.clone(), REASON_EFFECT, None, None);
    // Duel.Damage(tp,1000,REASON_EFFECT)
    duel::damage(d, a.tp, 1000, REASON_EFFECT, false, None);
    true
}
