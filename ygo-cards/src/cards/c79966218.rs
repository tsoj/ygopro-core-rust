//! Princess Dracaena, the Thirsting Tree
use super::prelude::*;

pub const ID: u32 = 79966218;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can send 1 Level 4 or lower Plant monster from your Deck to the GY, except "Draselea the Blood Tree Dragon Princess"; Special Summon this card from your hand, and if you do, its Level becomes the sent monster's
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.AND(s.selfspcost,s.effcost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost_target(selfspcost), cost_target(effcost)])));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card and 1 Plant monster from your GY; Special Summon 1 Dragon monster from your hand or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.AND(s.dragonspcost,s.effcost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost_target(dragonspcost), cost_target(effcost)])));
    // e2:SetTarget(s.dragonsptg)
    effect::set_target(d, e2, Some(cost_target(dragonsptg)));
    // e2:SetOperation(s.dragonspop)
    effect::set_operation(d, e2, Some(operation(dragonspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return c:IsRace(RACE_DRAGON|RACE_PLANT) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(|d, a: Args| card::is_race(d, a.c(0), RACE_DRAGON | RACE_PLANT)));
}
/// `s.effcost(e,tp,...,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // --You cannot Special Summon the turn you activate either of this card's effects, except Dragon or Plant monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsRaceExcept(RACE_DRAGON|RACE_PLANT) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race_except(d, c, RACE_DRAGON | RACE_PLANT, None, 0, None))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.selfspcostfilter(c)`
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_PLANT) and not c:IsCode(id) and c:IsAbleToGraveAsCost()
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_PLANT) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.selfspcost(e,tp,...,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.selfspcostfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&selfspcostfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.selfspcostfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&selfspcostfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    // e:SetLabel(sc:GetLevel())
    let lv = card::get_level(d, sc);
    effect::set_label(d, a.e, &[lv as i64]);
    true
}
/// `s.selfsptg(e,tp,...,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,...)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(e:GetLabel())
        let lv = effect::get_label(d, a.e);
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.dragonspcostfilter(c,e,tp,handler)`
fn dragonspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, handler: CardId) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsAbleToRemoveAsCost()
    //   and Duel.IsExistingMatchingCard(s.dragonspfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,Group.FromCards(c,handler),e,tp)
    let mut ex = CardSet::from_card(c);
    group::add_card(&mut ex, &CardSet::from_card(handler));
    card::is_race(d, c, RACE_PLANT)
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| dragonspfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, (&ex).into())
}
/// `s.dragonspfilter(c,e,tp)`
fn dragonspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DRAGON) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.dragonspcost(e,tp,...,chk)`
fn dragonspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost()
    //   and Duel.IsExistingMatchingCard(s.dragonspcostfilter,tp,LOCATION_GRAVE,0,1,c,e,tp,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| dragonspcostfilter(d, x, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.dragonspcostfilter,tp,LOCATION_GRAVE,0,1,1,c,e,tp,c)
    let mut g = duel::select_matching_card(d, tp, Some(&|d, x| dragonspcostfilter(d, x, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g+c,POS_FACEUP,REASON_COST)
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.dragonsptg(e,tp,...,chk,chkc)`
fn dragonsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.dragonspfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| dragonspfilter(d, x, a.e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.dragonspop(e,tp,...)`
fn dragonspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.dragonspfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| dragonspfilter(d, x, a.e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
