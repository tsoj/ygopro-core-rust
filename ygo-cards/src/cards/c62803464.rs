//! Aussa the Earth Channeler
use super::prelude::*;

pub const ID: u32 = 62803464;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_CHARMER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Search 1 EARTH monster with 1850 or less ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon itself from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.disfilter(c,tp,cc)
fn disfilter(d: &mut Duel, c: CardId, tp: u8, cc: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsDiscardable()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,cc:GetOriginalRace(),c:GetOriginalRace())
    if !(card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_discardable(d, c, None, None)) {
        return false;
    }
    let race1 = card::get_original_race(d, cc);
    let race2 = card::get_original_race(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, race1, race2)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// s.thfilter(c,race1,race2)
fn thfilter(d: &mut Duel, c: CardId, race1: u64, race2: u64) -> bool {
    // return c:IsMonster() and c:IsAttackBelow(1850) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(race1|race2)
    //     and c:IsAbleToHand()
    card::is_monster(d, c)
        && card::is_attack_below(d, c, 1850)
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, race1 | race2)
        && card::is_able_to_hand(d, c, None)
}
// s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsDiscardable() and Duel.IsExistingMatchingCard(s.disfilter,tp,LOCATION_HAND,0,1,c,tp,c) end
    if a.chk == Some(0) {
        if !card::is_discardable(d, c, None, None) {
            return false;
        }
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, x| disfilter(d, x, tp, c)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.disfilter,tp,LOCATION_HAND,0,1,1,c,tp,c)
    let tp = a.tp;
    let mut g = duel::select_matching_card(d, tp, Some(&|d, x| disfilter(d, x, tp, c)), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // e:SetLabel(c:GetOriginalRace(),g:GetFirst():GetOriginalRace())
    let race1 = card::get_original_race(d, c) as i64;
    let first = group::get_first(d, &g).nil();
    let race2 = card::get_original_race(d, first) as i64;
    effect::set_label(d, a.e, &[race1, race2]);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_COST | REASON_DISCARD, None, None);
    true
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local race1,race2=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race1 = labels.first().copied().unwrap_or(0) as u64;
    let race2 = labels.get(1).copied().unwrap_or(0) as u64;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,race1,race2)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x, race1, race2)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
    // --Cannot activate non-EARTH monster effects
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(_,re) return re:IsMonsterEffect() and re:GetHandler():IsAttributeExcept(ATTRIBUTE_EARTH) end)
    effect::set_value_fn(d, e1, lua_fn(cannot_activate_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function(_,re) return re:IsMonsterEffect() and re:GetHandler():IsAttributeExcept(ATTRIBUTE_EARTH) end
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_attribute_except(d, h, ATTRIBUTE_EARTH, None, 0, None)
}
// s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetPreviousAttributeOnField()&ATTRIBUTE_EARTH~=0 and c:IsPreviousControler(tp)
    card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_EARTH != 0 && card::is_previous_controler(d, c, tp)
}
// s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| cfilter(d, x, a.tp), 1, NO_EX)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
