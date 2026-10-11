//! Vaylantz Mad Marquess
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 87897777;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, listed_names: &[CARD_VALIANTS_KOENIGWISSEN], listed_series: &[SET_VAYLANTZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Special Summon self
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 excavated "Vaylantz" card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DICE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_DICE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon from Spell/Trap Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DICE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DICE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_MOVE)
    effect::set_code(d, e3, EVENT_MOVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.sspcon)
    effect::set_condition(d, e3, Some(condition(sspcon)));
    // e3:SetTarget(s.ssptg)
    effect::set_target(d, e3, Some(cost_target(ssptg)));
    // e3:SetOperation(s.sspop)
    effect::set_operation(d, e3, Some(operation(sspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spconfilter(c)
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAYLANTZ) and c:IsAttribute(ATTRIBUTE_FIRE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::is_attribute(d, c, ATTRIBUTE_FIRE)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_VALIANTS_KOENIGWISSEN,PLAYER_ALL,LOCATION_FZONE)
    //     or Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_environment(d, CARD_VALIANTS_KOENIGWISSEN, Some(PLAYER_ALL as u8), Some(LOCATION_FZONE))
        || duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // return zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
        return zone != 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local zone=(1<<c:GetSequence())&ZONES_MMZ
    let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
    // if zone~=0 then
    if zone != 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAYLANTZ) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<1 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 1 {
        return;
    }
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    // Duel.ConfirmDecktop(tp,dc)
    duel::confirm_decktop(d, a.tp, dc as u32);
    // local dg=Duel.GetDecktopGroup(tp,dc):Filter(s.thfilter,nil)
    let dg = duel::get_decktop_group(d, a.tp, dc as u32);
    let dg = group::filter(d, &dg, &thfilter, NO_EX);
    // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=dg:Select(tp,1,1,nil)
        let g = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, a.tp);
}
// function s.sspcon(e,tp,eg,ep,ev,re,r,rp)
fn sspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
// function s.sspfilter(c,e,tp)
fn sspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsOriginalType(TYPE_MONSTER) and c:GetSequence()<5
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,(1<<c:GetSequence())&ZONES_MMZ)
    card::is_original_type(d, c, TYPE_MONSTER)
        && card::get_sequence(d, c) < 5
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some((1u32 << card::get_sequence(d, c)) & ZONES_MMZ))
}
// function s.ssptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn ssptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sspfilter,tp,LOCATION_SZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| sspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_SZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_SZONE as i32);
    true
}
// function s.sspop(e,tp,eg,ep,ev,re,r,rp)
fn sspop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.sspfilter,tp,LOCATION_SZONE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| sspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_SZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    // if dc<2 or dc>5 then return end
    if !(2..=5).contains(&dc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,(1<<tc:GetSequence())&ZONES_MMZ)
        let zone = (1u32 << card::get_sequence(d, tc)) & ZONES_MMZ;
        duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
