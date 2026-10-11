//! S-Force Rappa Chiyomaru
//! Ｓ－Ｆｏｒｃｅ 乱破小夜丸
use super::prelude::*;

pub const ID: u32 = 22180094;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Each of your opponent's monsters in the same column as one of your "S-Force" monsters can only target monsters in that column for attacks
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1a, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1a, 0, LOCATION_MZONE);
    // e1a:SetTarget(function(e,c) if SForce.ColumnTarget(e,c) then e:SetLabelObject(c) return true end end)
    effect::set_target(d, e1a, Some(effect_card_fn(column_label_target)));
    // e1a:SetValue(function(e,c) local bc=e:GetLabelObject() return bc and not bc:GetColumnGroup():IsContains(c) end)
    effect::set_value_fn(d, e1a, effect_card_fn(column_value));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1b, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // (Quick Effect): You can banish 1 "S-Force" card from your hand; return this card to the hand, and if you do, Special Summon 1 "S-Force" monster from your Deck in Defense Position, except "S-Force Rappa Chiyomaru". You can only use this effect of "S-Force Rappa Chiyomaru" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.Replaceable(s.spcost,s.extracon))
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(spcost), Some(lua_fn(extracon)))));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,c) if SForce.ColumnTarget(e,c) then e:SetLabelObject(c) return true end end`
fn column_label_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    if aux::sforce::column_target().call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
        effect::set_label_object(d, e, Value::Card(c));
        return true;
    }
    false
}
/// `function(e,c) local bc=e:GetLabelObject() return bc and not bc:GetColumnGroup():IsContains(c) end`
fn column_value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    match effect::get_label_object(d, e).as_card() {
        Some(bc) => {
            let cg = card::get_column_group(d, bc, 0, 0);
            !group::is_contains(&cg, c)
        }
        None => false,
    }
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_S_FORCE) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_S_FORCE as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&spcostfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_S_FORCE) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_S_FORCE as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.extracon(base,e,tp,eg,ep,ev,re,r,rp,exc)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let tp = a.p(2);
    let exc = a.card(9);
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,exc,e,tp)
    let ex = match exc {
        Some(x) => x.into(),
        None => NO_EX,
    };
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, ex)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.GetMZoneCount(tp,c)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_HAND)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE) end
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
