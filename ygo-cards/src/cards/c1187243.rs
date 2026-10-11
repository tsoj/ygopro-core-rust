//! Tailwind of Gusto
use super::prelude::*;

pub const ID: u32 = 1187243;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GUSTO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_GUSTO))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_set_card(d, x, &[SET_GUSTO as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Cannot be destroyed by the opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 "Gusto" Spell/Trap
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon based on the equipped monster's Level/Rank
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thcfilter(c)`
fn thcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsMonster() and c:IsDiscardable()
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_monster(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, e) = (a.tp, a.e);
    // local h=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return h:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingMatchingCard(s.thcfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, h, None)
            && duel::is_existing_matching_card(d, Some(&thcfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.DiscardHand(tp,s.thcfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&thcfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GUSTO) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GUSTO as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, tc);
    }
}
/// `s.spfilter1(c,e,tp,ec)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: CardId) -> bool {
    // return c:IsSetCard(SET_GUSTO) and not c:IsRace(ec:GetRace()) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let race = card::get_race(d, ec);
    card::is_set_card(d, c, &[SET_GUSTO as u16])
        && !card::is_race(d, c, race)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(1) and c:IsType(TYPE_TUNER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[1])
        && card::is_type(d, c, TYPE_TUNER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, e) = (a.tp, a.e);
    if a.chk == Some(0) {
        // local ec=e:GetHandler():GetEquipTarget()
        let h = effect::get_handler(d, e).nil();
        // if not ec or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return false end
        let ec = match card::get_equip_target(d, h) {
            Some(ec) => ec,
            None => return false,
        };
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
            return false;
        }
        // local lv=math.max(ec:GetLevel(),ec:GetRank())
        let lv = card::get_level(d, ec).max(card::get_rank(d, ec));
        // if lv<=4 then
        if lv <= 4 {
            // return Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_DECK,0,1,nil,e,tp,ec)
            return duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, e, tp, ec)), tp, LOCATION_DECK, 0, 1, NO_EX);
        }
        // return Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,1,nil,e,tp)
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (tp, e) = (a.tp, a.e);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    // if not ec or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return false end
    let ec = match card::get_equip_target(d, h) {
        Some(ec) => ec,
        None => return,
    };
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local lv=math.max(ec:GetLevel(),ec:GetRank())
    let lv = card::get_level(d, ec).max(card::get_rank(d, ec));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local f=s.spfilter1  if lv>=5 then f=s.spfilter2 end
    // local g=Duel.SelectMatchingCard(tp,f,tp,LOCATION_DECK,0,1,1,nil,e,tp,ec)
    let g = if lv >= 5 {
        duel::select_matching_card(d, tp, Some(&|d, x| spfilter2(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
    } else {
        duel::select_matching_card(d, tp, Some(&|d, x| spfilter1(d, x, e, tp, ec)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
    };
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
