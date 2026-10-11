//! Hubolt the Dark Directory of Death
//! 死相の冥鑑ヒュブロ
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 29081251;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Normal Summon this card without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(function(e,c,minc) ... end)
    effect::set_condition(d, e1, Some(lua_fn(summon_proc_condition)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Normal or Special Summoned: You can send 1 Level 6 or higher Zombie monster from your Deck to the GY, except "Hubolt the Dark Directory of Death", then you can add 1 Level 6 or higher Zombie monster from your GY to your hand
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND)
    effect::set_category(d, e2a, CATEGORY_TOGRAVE + CATEGORY_TOHAND);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetTarget(s.tgtg)
    effect::set_target(d, e2a, Some(cost_target(tgtg)));
    // e2a:SetOperation(s.tgop)
    effect::set_operation(d, e2a, Some(operation(tgop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // If a monster(s) is Special Summoned from the GY while this card is in the Monster Zone, you can: Immediately after this effect resolves, Xyz Summon 1 Zombie Xyz Monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return not eg:IsContains(e:GetHandler()) and eg:IsExists(Card.IsSummonLocation,1,nil,LOCATION_GRAVE) end)
    effect::set_condition(d, e3, Some(condition(xyz_condition)));
    // e3:SetTarget(s.xyztg)
    effect::set_target(d, e3, Some(cost_target(xyztg)));
    // e3:SetOperation(s.xyzop)
    effect::set_operation(d, e3, Some(operation(xyzop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function(e,c,minc)
// if c==nil then return true end
// return minc==0 and c:IsLevelAbove(4) and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
fn summon_proc_condition(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    a.int(2) == 0 && card::is_level_above(d, c, 4) && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(6) and c:IsRace(RACE_ZOMBIE) and not c:IsCode(id) and c:IsAbleToGrave()
    card::is_level_above(d, c, 6) && card::is_race(d, c, RACE_ZOMBIE) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(6) and c:IsRace(RACE_ZOMBIE) and c:IsAbleToHand()
    card::is_level_above(d, c, 6) && card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_hand(d, c, None)
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) then
    let Some(sc) = sc else {
        return;
    };
    if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) <= 0 || !card::is_location(d, sc, LOCATION_GRAVE) {
        return;
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, a.tp);
    // if Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    let nv = aux::necro_valley_filter(|d: &mut Duel, c: CardId| thfilter(d, c));
    if !duel::is_existing_matching_card(d, Some(&nv), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        return;
    }
    if !duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
// function(e,tp,eg,ep,ev,re,r,rp)
// return not eg:IsContains(e:GetHandler()) and eg:IsExists(Card.IsSummonLocation,1,nil,LOCATION_GRAVE)
fn xyz_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let handler = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, handler) && group::is_exists(d, &eg, &|d, c| card::is_summon_location(d, c, LOCATION_GRAVE), 1, NO_EX)
}
// function s.xyzfilter(c)
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsXyzSummonable()
    card::is_race(d, c, RACE_ZOMBIE) && card::is_xyz_summonable(d, c, None, None, 0, 0)
}
// function s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk)
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&xyzfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.xyzop(e,tp,eg,ep,ev,re,r,rp)
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&xyzfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.XyzSummon(tp,sc)
        duel::xyz_summon(d, a.tp, sc, None, None, 0, 0);
    }
}
