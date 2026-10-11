//! Ｒ．Ｂ．ラムダキャノン
//! R.B. Lambda Cannon
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 81794107;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "R.B." monster from your GY to your hand, except "R.B. Lambda Cannon"
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1a, CATEGORY_TOHAND);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.thtg)
    effect::set_target(d, e1a, Some(cost_target(thtg)));
    // e1a:SetOperation(s.thop)
    effect::set_operation(d, e1a, Some(operation(thop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Destroy this card, and if you do, look at your opponent's hand, then you can Special Summon 1 monster from it to their field, but negate its effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(Cost.PayLP(1200))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1200))));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsMonster() and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_RB as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,tp,REASON_EFFECT)
        duel::send_to_hand(d, tc, Some(a.tp), REASON_EFFECT, None);
    }
}
/// `s.desconfilter(c,ec,lg)`
fn desconfilter(d: &mut Duel, c: CardId, ec: CardId, lg: &CardSet) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsLinkMonster() and c:IsFaceup() and (lg:IsContains(c) or c:GetLinkedGroup():IsContains(ec))
    card::is_set_card(d, c, &[SET_RB as u16])
        && card::is_link_monster(d, c)
        && card::is_faceup(d, c)
        && (lg.contains(c) || card::get_linked_group(d, c).contains(ec))
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsExistingMatchingCard(s.desconfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c,c:GetLinkedGroup())
    let lg = card::get_linked_group(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| desconfilter(d, x, c, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.Destroy(c,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::destroy(d, c, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local hg=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let hg = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #hg==0 then return end
    if hg.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,hg)
    duel::confirm_cards(d, tp, hg.clone());
    // local sg=hg:Filter(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false,POS_FACEUP,1-tp)
    let sg = group::filter(
        d,
        &hg,
        &|d, x| card::is_can_be_special_summoned(d, x, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None),
        NO_EX,
    );
    // if #sg==0 or Duel.GetLocationCount(1-tp,LOCATION_MZONE)==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return Duel.ShuffleHand(1-tp) end
    if sg.is_empty()
        || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 == 0
        || !duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::shuffle_hand(d, 1 - tp);
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=sg:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SpecialSummonStep(sc,0,tp,1-tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, sc, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None) {
        // sc:NegateEffects(c)
        card::negate_effects(d, sc, c, None, false, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
}
