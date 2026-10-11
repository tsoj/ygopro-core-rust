//! Elemental HERO Flame Wingman - Infernal Rage
use super::prelude::*;

pub const ID: u32 = 93347961;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO, SET_FAVORITE],
    material_setcode: &[SET_ELEMENTAL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Summon procedure
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, x, a| ffilter(d, x, a))), 2)]);
    // Must be Fusion Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e0, aux::fuslimit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Add 1 "Favorite" card from your Deck or GY to your hand
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 7 or lower "Elemental HERO" monster from your Deck or Extra Deck
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
    // e2:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() and e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, h) && card::has_flag_effect(d, h, ID, None)
    })));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, effect_card_value(|d, e, c| {
        valcheck(d, e, c);
        0
    }));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.fusfilter(c,attr,fc,sumtype,tp)`
fn fusfilter(d: &mut Duel, c: CardId, attr: u32, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(attr,fc,sumtype,tp) and not c:IsHasEffect(511002961)
    card::is_attribute_ctx(d, c, attr, Some(fc), sumtype, Some(tp)) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `s.ffilter(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // local attr=c:GetAttribute(fc,sumtype,tp)
    let attr = card::get_attribute_ctx(d, c, Some(a.fc), a.sumtype, Some(a.tp));
    // return c:IsSetCard(SET_ELEMENTAL_HERO,fc,sumtype,tp) and attr>0
    //     and (not sg or not sg:IsExists(s.fusfilter,1,c,attr,fc,sumtype,tp))
    if !(card::is_set_card_ctx(d, c, &[SET_ELEMENTAL_HERO as u16], Some(a.fc), a.sumtype, Some(a.tp)) && attr > 0) {
        return false;
    }
    match a.sg {
        None => true,
        Some(sg) => !group::is_exists(d, sg, &|d, x| fusfilter(d, x, attr, a.fc, a.sumtype, a.tp), 1, c.into()),
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FAVORITE) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_FAVORITE as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nv = aux::necro_valley_filter(|d, c| thfilter(d, c));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.spfilter(c,e,tp,ec)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: CardId) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsLevelBelow(7) and not c:IsSummonableCard()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     and ((c:IsLocation(LOCATION_DECK) and Duel.GetMZoneCount(tp,ec)>0)
    //     or (c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,ec,c)>0))
    if !(card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
        && card::is_level_below(d, c, 7)
        && !card::is_summonable_card(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None))
    {
        return false;
    }
    (card::is_location(d, c, LOCATION_DECK) && duel::get_m_zone_count(d, tp, ec.into(), None, None, None).0 > 0)
        || (card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), ec.into(), duel::ExtraCard::Card(c), None).0 > 0)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, h)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, h)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) {
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // if mg:IsExists(Card.IsType,1,nil,TYPE_NORMAL) then
    if group::is_exists(d, &mg, &|d, x| card::is_type(d, x, TYPE_NORMAL), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_TEMP_REMOVE|RESET_LEAVE),0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_TEMP_REMOVE | RESET_LEAVE)), 0, 1, 0, 0);
    }
}
