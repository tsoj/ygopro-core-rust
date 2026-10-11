//! Fire Formation - Ingen
//! 炎舞－「隠元」
use super::prelude::*;

pub const ID: u32 = 29143457;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When activated, fusion summon 1 beast-warrior fusion monster
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsRace,RACE_BEASTWARRIOR))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_BEASTWARRIOR))),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target(e1:GetTarget(),e1:GetOperation()))
    let sptg = effect::get_target(d, e1).nil();
    let spop = effect::get_operation(d, e1).nil();
    effect::set_target(d, e1, Some(sp_target(sptg, spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this face-up card is sent to GY, add 1 "Fire Fist" monster from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The arguments of a target function (`e,tp,eg,ep,ev,re,r,rp,chk`) for a call from Lua code.
fn target_args(a: &EffArgs, chk: i64) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ]
}
/// `s.target(sptg,spop)`: the target function that asks whether to Special Summon the Fusion
/// Summon's result (`sptg` and `spop` are the Fusion Summon's own target and operation).
fn sp_target(sptg: LuaFn, spop: LuaFn) -> LuaFn {
    cost_target(move |d, a| {
        // if chk==0 then return true end
        if a.chk == Some(0) {
            return true;
        }
        // if sptg(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if sptg.call(d, &target_args(a, 0)).truthy() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
            // e:SetOperation(spop)
            effect::set_operation(d, a.e, Some(spop.clone()));
            // sptg(e,tp,eg,ep,ev,re,r,rp,1)
            sptg.call(d, &target_args(a, 1));
        } else {
            // e:SetOperation(aux.NULL)
            effect::set_operation(d, a.e, Some(LuaFn::new(|_, _| Value::Nil)));
            // e:SetCategory(0)
            effect::set_category(d, a.e, 0);
        }
        true
    })
}
/// `s.thcon(e,tp,...)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousPosition(POS_FACEUP) and e:GetHandler():IsPreviousLocation(LOCATION_SZONE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_position(d, h, POS_FACEUP) && card::is_previous_location(d, h, LOCATION_SZONE)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_FIRE_FIST) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
