//! EN - Engage Neo Space
use super::prelude::*;

pub const ID: u32 = 80170678;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS, CARD_POLYMERIZATION],
    listed_series: &[SET_ELEMENTAL_HERO, SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsType(TYPE_FUSION) or c:GetSummonLocation()~=LOCATION_EXTRA
    let c = a.c(0);
    card::is_type(d, c, TYPE_FUSION) || card::get_summon_location(d, c) != LOCATION_EXTRA
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsSetCard(SET_NEO_SPACIAN) or c:IsSetCard(SET_ELEMENTAL_HERO)) and c:IsAbleToGraveAsCost()
    card::is_monster(d, c)
        && (card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) || card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]))
        && card::is_able_to_grave_as_cost(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:GetClassCount(Card.GetLocation)==2 and sg:IsExists(Card.IsSetCard,1,nil,SET_NEO_SPACIAN)
    //     and sg:IsExists(Card.IsSetCard,1,nil,SET_ELEMENTAL_HERO)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,sg,e,tp)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,sg,e,tp)
    let ok = group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) == 2
        && group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]), 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, sg.into())
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, sg.into());
    (ok, false)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (c:IsSetCard(SET_NEO_SPACIAN) or (c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsLevelAbove(5)))
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16])
            || (card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && card::is_level_above(d, c, 5)))
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_able_to_hand(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let rg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0
    //     and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0
            && matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except Fusion Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,s.lizfilter)
    aux::add_temp_lizard_check(d, c, a.tp, Some(lua_fn(lizfilter)), None, None, None, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsType(TYPE_FUSION) and c:IsLocation(LOCATION_EXTRA)
    let c = a.c(1);
    !card::is_type(d, c, TYPE_FUSION) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, a: Args) -> bool {
    // return not c:IsOriginalType(TYPE_FUSION)
    !card::is_original_type(d, a.c(1), TYPE_FUSION)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // if sc:IsCode(CARD_NEOS) then
            if card::is_code(d, sc, &[CARD_NEOS]) {
                // Increase ATK
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(1000)
                effect::set_value(d, e1, 1000);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local tg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
            let tg = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(thfilter)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if #tg>0 then
            if !tg.is_empty() {
                // Duel.SendtoHand(tg,nil,REASON_EFFECT)
                duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,tg)
                duel::confirm_cards(d, 1 - a.tp, tg);
            }
        }
    }
}
