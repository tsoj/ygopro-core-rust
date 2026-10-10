//! Prey of the Jirai Gumo
use super::prelude::*;

pub const ID: u32 = 33055499;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: CARDS_SANGA_KAZEJIN_SUIJIN, ..ScriptInfo::DEFAULT };

/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,2100,100,5,RACE_INSECT,ATTRIBUTE_EARTH)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(2100),
        defense: Some(100),
        level: Some(5),
        race: Some(RACE_INSECT),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Sanga of the Thunder", "Kazejin", or "Suijin" to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=c:GetColumnZone(LOCATION_MZONE)&ZONES_MMZ
    let zone = card::get_column_zone(d, c, LOCATION_MZONE, 0, 0, None) & ZONES_MMZ;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //   and zone>0 and not Duel.IsExistingMatchingCard(aux.IsZone,tp,LOCATION_MZONE,0,1,nil,zone,tp)
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,2100,100,5,RACE_INSECT,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && zone > 0
            && !duel::is_existing_matching_card(d, Some(&|d, cc| aux::is_zone(d, cc, zone, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_player_can_special_summon_monster(d, tp, ID, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=c:GetColumnZone(LOCATION_MZONE)&ZONES_MMZ
    let zone = card::get_column_zone(d, c, LOCATION_MZONE, 0, 0, None) & ZONES_MMZ;
    // if not (c:IsRelateToEffect(e) and zone>0 and Duel.IsPlayerCanSpecialSummonMonster(...)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && zone > 0 && duel::is_player_can_special_summon_monster(d, tp, ID, token_data(), None, None, 0)) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_NORMAL + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP,zone)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, Some(zone));
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // local dg=c:GetColumnGroup():Match(aux.AND(Card.IsControler,Card.IsMonster),nil,1-tp)
    let column = card::get_column_group(d, c, 0, 0);
    let mut dg = group::filter(d, &column, &|d, x| card::is_controler(d, x, 1 - tp) && card::is_monster(d, x), NO_EX);
    // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !dg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // if #dg>1 then
        if dg.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // dg=dg:Select(tp,1,1,nil)
            dg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARDS_SANGA_KAZEJIN_SUIJIN) and c:IsAbleToHand() and (c:IsLocation(LOCATION_DECK) or c:IsFaceup())
    card::is_code(d, c, CARDS_SANGA_KAZEJIN_SUIJIN) && card::is_able_to_hand(d, c, None) && (card::is_location(d, c, LOCATION_DECK) || card::is_faceup(d, c))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_REMOVED) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
