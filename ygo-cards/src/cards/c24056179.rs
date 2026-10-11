//! Capsule Monster Chess
use super::prelude::*;

pub const ID: u32 = 24056179;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // During the Main Phase: The turn player can target 1 monster in their GY; they place it face-up in their Spell & Trap Zone as a Continuous Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_BOTH_SIDE)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_BOTH_SIDE, 0);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the End Phase: The turn player can send 1 face-up Monster Card they control to the GY; they Special Summon 1 monster from their Deck with the same original Type and Attribute, but 1, 2, or 3 original Levels higher
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_EVENT_PLAYER)
    effect::set_property(d, e2, EFFECT_FLAG_EVENT_PLAYER, 0);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.plfilter(c,tp)
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_monster(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // (The Lua tests `chkc`, which this function never receives: it is a nil global, so that branch never runs.)
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.plfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectTarget(tp,s.plfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, tp, 0);
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
        // Treated as a Continuous Spell
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// function s.spcostfilter(c,e,tp)
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonsterCard() and c:IsFaceup() and c:IsAbleToGraveAsCost() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetOriginalRace(),c:GetOriginalAttribute(),c:GetOriginalLevel())
    if !(card::is_monster_card(d, c) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let race = card::get_original_race(d, c);
    let attr = card::get_original_attribute(d, c);
    let lv = card::get_original_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, race, attr, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,original_race,original_attr,original_lv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, original_race: u64, original_attr: u32, original_lv: u32) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsOriginalRace(original_race) and c:IsOriginalAttribute(original_attr)
    //     and c:IsLevelBetween(original_lv+1,original_lv+3)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_original_race(d, c, original_race)
        && card::is_original_attribute(d, c, original_attr)
        && card::is_level_between(d, c, original_lv + 1, original_lv + 3)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_ONFIELD,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spcostfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spcostfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    // local cd=e:GetChainData()
    // cd.original_race=sc:GetOriginalRace()
    let race = card::get_original_race(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(race as i64));
    // cd.original_attr=sc:GetOriginalAttribute()
    let attr = card::get_original_attribute(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Int(attr as i64));
    // cd.original_lv=sc:GetOriginalLevel()
    let lv = card::get_original_level(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 2, Value::Int(lv as i64));
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local cd=e:GetChainData()
    let race = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u64;
    let attr = chain::effect_data(d, a.e, None, chain::field::CARD + 1).int() as u32;
    let lv = chain::effect_data(d, a.e, None, chain::field::CARD + 2).int() as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,cd.original_race,cd.original_attr,cd.original_lv)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, race, attr, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
