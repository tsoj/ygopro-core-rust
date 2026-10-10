//! Sky Galloping Gaia the Dragon Champion
use super::prelude::*;

pub const ID: u32 = 2519690;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GAIA_THE_FIERCE_KNIGHT],
    listed_names: &[49328340],
    material_setcode: &[SET_GAIA_THE_FIERCE_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GAIA_THE_FIERCE_KNIGHT),aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GAIA_THE_FIERCE_KNIGHT as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
        ],
    );
    // change name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(CARD_GAIA_CHAMPION)
    effect::set_value(d, e1, CARD_GAIA_CHAMPION as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // search
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetTarget(s.postg)
    effect::set_target(d, e3, Some(cost_target(postg)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(49328340) and c:IsAbleToHand()
    card::is_code(d, c, &[49328340]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE|LOCATION_DECK,0,1,1,nil)
    let f = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_GRAVE | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if chk==0 then return d and d:IsControler(1-tp) and d:IsCanChangePosition() end
    if a.chk == Some(0) {
        return match at {
            Some(x) => card::is_controler(d, x, 1 - tp) && card::is_can_change_position(d, x),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, at.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, _a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    // if d and d:IsRelateToBattle() then
    if let Some(at) = duel::get_attack_target(d) {
        if card::is_relate_to_battle(d, at) {
            // Duel.ChangePosition(d,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
            duel::change_position(d, CardSet::from_card(at), POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), false);
        }
    }
}
