//! Divine Golden Shadow Dragon Dragluxion
use super::prelude::*;

pub const ID: u32 = 44698398;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY, SET_TACHYON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // Add 1 "Galaxy" or "Tachyon" card from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_EXTRA) end)
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Rank 8 Dragon Xyz Monster with 3000 ATK from your Extra Deck, by using this card you control as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSummonLocation(LOCATION_EXTRA) end`
fn e1_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_GALAXY,SET_TACHYON}) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GALAXY as u16, SET_TACHYON as u16]) && card::is_able_to_hand(d, c, None)
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
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsRank(8) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_XYZ) and c:IsAttack(3000)
    //     and mc:IsCanBeXyzMaterial(c,tp) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_rank(d, c, &[8])
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_attack(d, c, &[3000])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_XYZ | REASON_MATERIAL))
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
        return (pg.is_empty() || (pg.len() == 1 && pg.contains(c)))
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsFaceup() and c:IsRelateToEffect(e) and c:IsControler(tp) and not c:IsImmuneToEffect(e)) then return end
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, a.tp) && !card::is_immune_to_effect(d, c, a.e)) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
    // if #pg>1 or (#pg==1 and not pg:IsContains(c)) then return end
    if pg.len() > 1 || (pg.len() == 1 && !pg.contains(c)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else {
        // if not sc then return end
        return;
    };
    // sc:SetMaterial(c)
    card::set_material(d, sc, CardSet::from_card(c));
    // Duel.Overlay(sc,c)
    duel::overlay(d, sc, c, false);
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
        // local mg=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_EXTRA,0,nil,SET_GALAXY)
        let mg = duel::get_matching_group(d, Some(&|d, x| card::is_set_card(d, x, &[SET_GALAXY as u16])), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // if #mg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !mg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
            // local mc=mg:Select(tp,1,1,nil)
            let mc = group::select(d, &mg, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Overlay(sc,mc)
            duel::overlay(d, sc, mc, false);
        }
    }
}
