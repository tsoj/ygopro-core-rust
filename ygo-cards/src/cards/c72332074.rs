//! Super Quantal Alphan Spike
use super::prelude::*;

pub const ID: u32 = 72332074;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SUPER_QUANTUM],
    listed_names: &[58753372, 10424147],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle as many cards your opponent controls as possible into the Deck, then your opponent Special Summons 1 monster from their Extra Deck, ignoring its Summoning conditions
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function(e,tp) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_SUPER_QUANTUM),tp,LOCATION_MZONE,0,nil):GetClassCount(Card.GetCode)>=3 end)
    effect::set_condition(d, e1, Some(lua_fn(cond1)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 "Super Quantal Mech Ship Magnacarrier" from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(s.actcost)
    effect::set_cost(d, e2, Some(cost_target(actcost)));
    // e2:SetTarget(s.acttg)
    effect::set_target(d, e2, Some(cost_target(acttg)));
    // e2:SetOperation(s.actop)
    effect::set_operation(d, e2, Some(operation(actop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_SUPER_QUANTUM),tp,LOCATION_MZONE,0,nil):GetClassCount(Card.GetCode)>=3 end`
fn cond1(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUPER_QUANTUM as u16])), tp, LOCATION_MZONE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)>0 and Duel.IsPlayerCanSpecialSummon(1-tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::get_field_group_count(d, a.tp, 0, LOCATION_EXTRA) > 0
            && duel::is_player_can_special_summon(d, 1 - a.tp, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.spfilter(c,e,opp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, opp: u8) -> bool {
    // return c:IsMonster() and Duel.GetLocationCountFromEx(opp,opp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,opp,true,false)
    card::is_monster(d, c)
        && duel::get_location_count_from_ex(d, opp, Some(opp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, opp, true, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local opp=1-tp
    let opp = 1 - a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 and Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and Duel.GetOperatedGroup():IsExists(Card.IsLocation,1,nil,LOCATION_DECK|LOCATION_EXTRA) then
    if !g.is_empty() && duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        let op = duel::get_operated_group(d);
        if group::is_exists(d, &op, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX) {
            // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(opp,s.spfilter,opp,LOCATION_EXTRA,0,1,1,nil,e,opp)
            let sg = duel::select_matching_card(d, opp, Some(&|d, c| spfilter(d, c, a.e, opp)), opp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sg,0,opp,opp,true,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, opp, opp, true, false, POS_FACEUP as u8, None);
            }
        }
    }
}
/// `s.actcostfilter(c)`
fn actcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(58753372) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_code(d, c, &[58753372]) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.actcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn actcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingMatchingCard(s.actcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&actcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.actcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c)
    let mut g = duel::select_matching_card(d, a.tp, Some(&actcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g+c,POS_FACEUP,REASON_COST)
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.actfilter(c,tp)`
fn actfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(10424147) and c:GetActivateEffect():IsActivatable(tp,true,true)
    if !card::is_code(d, c, &[10424147]) {
        return false;
    }
    let e = card::get_activate_effect(d, c).first().copied().nil();
    effect::is_activatable(d, e, tp, true, true)
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.actfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| actfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.actfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| actfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.ActivateFieldSpell(sc,e,tp,eg,ep,ev,re,r,rp)
        duel::activate_field_spell(d, Some(sc), a, None);
    }
}
