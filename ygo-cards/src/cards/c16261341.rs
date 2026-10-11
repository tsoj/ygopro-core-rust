//! Sacred Arch-Airknight Parshath
use super::prelude::*;

pub const ID: u32 = 16261341;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PARSHATH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_HAND | LOCATION_GRAVE);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.spcon1)
    effect::set_condition(d, e3, Some(condition(spcon1)));
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e4, EVENT_CHAIN_NEGATED);
    // e4:SetCondition(s.spcon2)
    effect::set_condition(d, e4, Some(condition(spcon2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // pierce
    // local e3=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e5, EFFECT_PIERCE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e5, false);
    // search
    // local e4=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e6, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e6, EVENT_BATTLE_DAMAGE);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e6, Some(condition(thcon)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e6, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e6, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e6, false);
}
/// `s.spcon1(e,tp,eg,ep,ev,re,r,rp)`
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsActiveType(TYPE_COUNTER)
    a.rp == a.tp && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && effect::is_active_type(d, a.re.nil(), TYPE_COUNTER)
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local dp=Duel.GetChainInfo(ev,CHAININFO_DISABLE_PLAYER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_DISABLE_PLAYER]);
    let dp_is_tp = matches!(info[0], Value::Int(x) if x == a.tp as i64);
    // return dp==tp and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    dp_is_tp && (effect::is_monster_effect(d, a.re.nil()) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE))
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:IsAbleToRemoveAsCost() and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true,true))
    card::is_race(d, c, RACE_FAIRY)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, true))
}
/// `s.mzfilter(c)`
fn mzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5
    card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let rg = duel::get_matching_group(d, Some(&|d, x| cfilter(d, x)), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ct=-ft+1
    let ct = -ft + 1;
    // if chk==0 then return ft>-2 and #rg>1 and (ft>0 or rg:IsExists(s.mzfilter,ct,nil)) end
    if a.chk == Some(0) {
        return ft > -2 && rg.len() > 1 && (ft > 0 || group::is_exists(d, &rg, &|d, x| mzfilter(d, x), ct as u16, NO_EX));
    }
    // local g=nil
    let mut g: CardSet;
    if ft > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // g=rg:Select(tp,2,2,nil)
        g = group::select(d, &rg, a.tp, 2, 2, false, NO_EX).nil();
    } else if ft == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // g=rg:FilterSelect(tp,s.mzfilter,1,1,nil)
        g = group::filter_select(d, &rg, a.tp, &|d, x| mzfilter(d, x), 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g2=rg:Select(tp,1,1,g:GetFirst())
        let first = group::get_first(d, &g);
        let g2 = group::select(d, &rg, a.tp, 1, 1, false, first.into()).nil();
        // g:Merge(g2)
        group::add_card(&mut g, &g2);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // g=rg:FilterSelect(tp,s.mzfilter,2,2,nil)
        g = group::filter_select(d, &rg, a.tp, &|d, x| mzfilter(d, x), 2, 2, false, NO_EX).nil();
    }
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp
    a.ep == 1 - a.tp
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_PARSHATH) or c:IsType(TYPE_COUNTER)) and c:IsAbleToHand()
    (card::is_set_card(d, c, &[SET_PARSHATH as u16]) || card::is_type(d, c, TYPE_COUNTER)) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
