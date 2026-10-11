//! Angelechy Enlisted
//! JP name
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 81797573;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1 non-Tuner
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,1)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 1, None, None, None, None, None);
    // You can target 1 opponent's monster in this card's adjacent column; banish it, then change control of this card by moving it to one of your opponent's Main Monster Zones in this card's adjacent columns
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If the control of this face-up card changes: Return this card to the Extra Deck, then the owner of this card Special Summons 1 "Angelechy" monster from their Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_CONTROL_CHANGED)
    effect::set_code(d, e2, EVENT_CONTROL_CHANGED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.get_adjacent_zones(c)
fn get_adjacent_zones(d: &mut Duel, c: CardId) -> u32 {
    // if not c:IsLocation(LOCATION_MZONE) then return 0 end
    if !card::is_location(d, c, LOCATION_MZONE) {
        return 0;
    }
    // local zones=0
    let mut zones: u32 = 0;
    // local seq=c:GetSequence()
    let mut seq = card::get_sequence(d, c);
    // if seq==5 then seq=1 elseif seq==6 then seq=3 end
    if seq == 5 {
        seq = 1;
    } else if seq == 6 {
        seq = 3;
    }
    // if seq>0 then zones=1<<(5-seq) end --left zone
    if seq > 0 {
        zones = 1 << (5 - seq);
    }
    // if seq<4 then zones=zones|(1<<(3-seq)) end --right zone
    if seq < 4 {
        zones |= 1 << (3 - seq);
    }
    // return zones
    zones
}
// function s.banfilter(c,tp,zones)
fn banfilter(d: &mut Duel, c: CardId, tp: u8, zones: u32) -> bool {
    // return c:IsMonster() and c:IsControler(1-tp) and c:IsAbleToRemove() and Duel.GetMZoneCount(1-tp,c,tp,LOCATION_REASON_CONTROL,zones)>0
    card::is_monster(d, c)
        && card::is_controler(d, c, 1 - tp)
        && card::is_able_to_remove(d, c, None, None, None)
        && duel::get_m_zone_count(d, 1 - tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL as u32), Some(zones)).0 > 0
}
// function s.bantg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cg=c:GetColumnGroup(1,1):Sub(c:GetColumnGroup()):Match(s.banfilter,c,tp,s.get_adjacent_zones(c))
    let mut cg = card::get_column_group(d, c, 1, 1);
    let own = card::get_column_group(d, c, 0, 0);
    group::remove_card(&mut cg, &own);
    let zones = get_adjacent_zones(d, c);
    group::match_(d, &mut cg, &|d, x| banfilter(d, x, tp, zones), NO_EX);
    // if chkc then return cg:IsContains(chkc) end
    if let Some(chkc) = a.chkc() {
        return cg.contains(chkc);
    }
    // if chk==0 then return cg:IsExists(Card.IsCanBeEffectTarget,1,nil,e) end
    if a.chk == Some(0) {
        return group::is_exists(d, &cg, &|d, x| card::is_can_be_effect_target(d, x, Some(a.e)), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=cg:FilterSelect(tp,Card.IsCanBeEffectTarget,1,1,nil,e)
    let g = group::filter_select(d, &cg, tp, &|d, x| card::is_can_be_effect_target(d, x, Some(a.e)), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.banop(e,tp,eg,ep,ev,re,r,rp)
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zones=s.get_adjacent_zones(c)
    let zones = get_adjacent_zones(d, c);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0
    //     and (tc:IsType(TYPE_TOKEN) or tc:IsLocation(LOCATION_REMOVED)) and c:IsRelateToEffect(e)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,zones)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && (card::is_type(d, tc, TYPE_TOKEN) || card::is_location(d, tc, LOCATION_REMOVED))
        && card::is_relate_to_effect(d, c, a.e)
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL as u32), Some(zones)).0 > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.GetControl(c,1-tp,0,0,zones,tp)
        duel::get_control(d, c, 1 - tp, 0, 0, Some(zones), Some(tp));
    }
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,c:GetOwner(),LOCATION_EXTRA)
    let owner = card::get_owner(d, c);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, owner, LOCATION_EXTRA as i32);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ANGELECHY as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0
    //     and c:IsLocation(LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_EXTRA)
    {
        // local owner=c:GetOwner()
        let owner = card::get_owner(d, c);
        // Duel.Hint(HINT_SELECTMSG,owner,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, owner, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(owner,s.spfilter,owner,LOCATION_EXTRA,0,1,1,nil,e,owner)
        let g = duel::select_matching_card(d, owner, Some(&|d, x| spfilter(d, x, a.e, owner)), owner, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,owner,owner,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, owner, owner, false, false, POS_FACEUP as u8, None);
        }
    }
}
