//! Witchcrafter Golem Aruru
use super::prelude::*;

pub const ID: u32 = 71074418;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WITCHCRAFTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon and return a target to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon1)
    effect::set_condition(d, e1, Some(condition(thcon1)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e2, EVENT_BECOME_TARGET);
    // e2:SetCondition(s.thcon2)
    effect::set_condition(d, e2, Some(condition(thcon2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return itself to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.thcon3)
    effect::set_condition(d, e3, Some(condition(thcon3)));
    // e3:SetTarget(s.thtg3)
    effect::set_target(d, e3, Some(cost_target(thtg3)));
    // e3:SetOperation(s.thop3)
    effect::set_operation(d, e3, Some(operation(thop3)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp) and c:IsRace(RACE_SPELLCASTER) and c:IsFaceup()
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_faceup(d, c)
}
/// `s.thcon1(e,tp,eg,ep,ev,re,r,rp)`
fn thcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.tgfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| tgfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.thcon2(e,tp,eg,ep,ev,re,r,rp)`
fn thcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and eg:IsExists(s.tgfilter,1,nil,tp)
    if a.rp != 1 - a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| tgfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToHand() and (c:IsControler(1-tp) or (c:IsSpell() and c:IsSetCard(SET_WITCHCRAFTER)))
    card::is_able_to_hand(d, c, None) && (card::is_controler(d, c, 1 - tp) || (card::is_spell(d, c) && card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16])))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && filter(d, chkc, tp);
    }
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,LOCATION_ONFIELD,1,nil,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_GRAVE, LOCATION_ONFIELD, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,LOCATION_ONFIELD,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_GRAVE, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 and tc:IsRelateToEffect(e) then
    if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
/// `s.thcon3(e,tp,eg,ep,ev,re,r,rp)`
fn thcon3(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.thtg3(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg3(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.thop3(e,tp,eg,ep,ev,re,r,rp)`
fn thop3(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SendtoHand(c,nil,REASON_EFFECT)
    duel::send_to_hand(d, c, None, REASON_EFFECT, None);
}
