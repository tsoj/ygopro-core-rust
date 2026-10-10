//! Sprind the Irondash Dragon
use super::prelude::*;

pub const ID: u32 = 1906812;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPRINGANS], listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Fallen of Albaz" + 1 Effect Monster Special Summoned this turn
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,aux.FilterBoolFunctionEx(s.filter))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_ALBAZ),
            fusion::Mat::F(fusion::fus_mat(|d, x, _fc, _st, _tp| filter(d, x))),
        ],
    );
    // Move this card you control to another of your Main Monster Zones, then you can destroy all other face-up cards in this card's column
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.seqtg)
    effect::set_target(d, e1, Some(cost_target(seqtg)));
    // e1:SetOperation(s.seqop)
    effect::set_operation(d, e1, Some(operation(seqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register the fact it was sent to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsType(TYPE_EFFECT) and c:IsStatus(STATUS_SPSUMMON_TURN)
    card::is_location(d, c, LOCATION_MZONE) && card::is_type(d, c, TYPE_EFFECT) && card::is_status(d, c, STATUS_SPSUMMON_TURN)
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or c:IsControler(1-tp) then return end
    if !card::is_relate_to_effect(d, c, e) || card::is_controler(d, c, 1 - tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    // local seq=Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,0)
    let seq = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, Some(0), false);
    // if seq then
    // Duel.MoveSequence(c,math.log(seq,2))
    duel::move_sequence(d, c, (seq as f64).log2() as u8, None);
    // local dg=c:GetColumnGroup():Filter(Card.IsFaceup,c,nil)
    let column = card::get_column_group(d, c, 0, 0);
    let dg = group::filter(d, &column, &|d, x| card::is_faceup(d, x), NO_EX);
    // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !dg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Add to your hand, or Special Summon, 1 "Springans" monster or 1 "Fallen of Albaz" from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c,e,tp,ft)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsMonster() and (c:IsSetCard(SET_SPRINGANS) or c:IsCode(CARD_ALBAZ))
    //     and (c:IsAbleToHand() or (c:IsCanBeSpecialSummoned(e,0,tp,false,false) and ft>0))
    card::is_monster(d, c)
        && (card::is_set_card(d, c, &[SET_SPRINGANS as u16]) || card::is_code(d, c, &[CARD_ALBAZ]))
        && (card::is_able_to_hand(d, c, None) || (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && ft > 0))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, e, tp, ft)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,ft):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, e, tp, ft)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sel) {
        // aux.ToHandOrElse(tc,tp,function(c)
        //     return tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and ft>0 end,
        // function(c)
        //     Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
        // aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, _c| card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None) && ft > 0),
            Some(&|d, _sc: &CardSet| duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
            Some(aux::stringid(ID, 3)),
        );
    }
}
