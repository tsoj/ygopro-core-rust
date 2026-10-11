//! Abyss Actor - Trendy Understudy
use super::prelude::*;

pub const ID: u32 = 39024589;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ABYSS_ACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // tohand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
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
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsPendulumSummoned()
    card::is_summon_player(d, c, tp) && card::is_pendulum_summoned(d, c)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // local lv=c:GetLevel()
    // return c:IsFaceup() and c:IsSetCard(SET_ABYSS_ACTOR) and (lv==1 or lv==8) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    let lv = card::get_level(d, c);
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
        && (lv == 1 || lv == 8)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_PZONE,0,2,nil,SET_ABYSS_ACTOR)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])), a.tp, LOCATION_PZONE, 0, 2, NO_EX)
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,mc,c)==0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 == 0 {
        return false;
    }
    // local lv=c:GetLevel()
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsSetCard(SET_ABYSS_ACTOR)
    //     and (lv==1 or lv==8) and c:IsType(TYPE_PENDULUM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let lv = card::get_level(d, c);
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
        && (lv == 1 || lv == 8)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let h = effect::get_handler(d, e).nil();
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if ft>0 then loc=loc|LOCATION_HAND end
    if ft > 0 {
        loc |= LOCATION_HAND;
    }
    // if chk==0 then return loc~=0 and Duel.IsExistingMatchingCard(s.spfilter,tp,loc,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        return loc != 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, h)), tp, loc, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, loc as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_HAND end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_HAND;
    }
    // if loc==0 then return end
    if loc == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let h = effect::get_handler(d, e).nil();
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,loc,0,1,1,nil,e,tp,e:GetHandler())
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, h)), tp, loc, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
