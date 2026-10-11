//! Cyberse Contract Witch
use super::prelude::*;

pub const ID: u32 = 37458564;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[],
    ..ScriptInfo::DEFAULT
};

const TYPES_RITUAL_FUSION_SYNCHRO_XYZ: u32 = TYPE_RITUAL | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 monsters with the same Type
    // Link.AddProcedure(c,nil,2,nil,s.matcheck)
    link::add_procedure(d, c, None, 2, None, Some(group_check(matcheck)), None, None);
    // If this card is Link Summoned, or a monster(s) is Special Summoned to a zone(s) this card points to: You can send 1 Spell ...
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1a, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    // e1a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1a, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    // e1a:SetCost(s.thcost)
    effect::set_cost(d, e1a, Some(cost_target(thcost)));
    // e1a:SetTarget(s.thtg)
    effect::set_target(d, e1a, Some(cost_target(thtg)));
    // e1a:SetOperation(s.thop)
    effect::set_operation(d, e1a, Some(operation(thop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetCondition(aux.zptcon(nil))
    effect::set_condition(d, e1b, Some(aux::zptcon(None)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // You can target 1 Ritual, Fusion, Synchro, or Xyz Monster this card points to; Special Summon 1 monster from your GY ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:CheckSameProperty(Card.GetRace,lc,sumtype,tp)
    group::check_same_property(d, g, &|d, c| card::get_race(d, c)).0
}
/// `s.thcostfilter(c)`
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and c:IsAbleToGraveAsCost()
    card::is_spell(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_able_to_grave_as_cost(d, c)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thcostfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thcostfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualMonster() and c:IsAbleToHand()
    card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.tgfilter(c,e,tp,lg)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lg: &CardSet) -> bool {
    // return c:IsType(TYPES_RITUAL_FUSION_SYNCHRO_XYZ) and lg:IsContains(c) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c:GetRace(),c:GetType()&TYPES_RITUAL_FUSION_SYNCHRO_XYZ)
    card::is_type(d, c, TYPES_RITUAL_FUSION_SYNCHRO_XYZ)
        && group::is_contains(lg, c)
        && card::is_faceup(d, c)
        && {
            let race = card::get_race(d, c);
            let ctype = card::get_type(d, c) & TYPES_RITUAL_FUSION_SYNCHRO_XYZ;
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, race, ctype)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
/// `s.spfilter(c,e,tp,race,card_type)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, race: u64, card_type: u32) -> bool {
    // return c:IsType(TYPES_RITUAL_FUSION_SYNCHRO_XYZ) and c:IsRace(race) and not c:IsType(card_type)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPES_RITUAL_FUSION_SYNCHRO_XYZ)
        && card::is_race(d, c, race)
        && !card::is_type(d, c, card_type)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,e,tp,lg) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, e, tp, &lg);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp,lg) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, e, tp, &lg)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp,lg)
    let _g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, e, tp, &lg)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, tc, e)
        && card::is_faceup(d, tc)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,tc:GetRace(),tc:GetType()&TYPES_RITUAL_FUSION_SYNCHRO_XYZ)
        let race = card::get_race(d, tc);
        let ctype = card::get_type(d, tc) & TYPES_RITUAL_FUSION_SYNCHRO_XYZ;
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, race, ctype)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
