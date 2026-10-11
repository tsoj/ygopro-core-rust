//! Witchcrafter Confusion Confession
//! ウィッチクラフト・コンフュージョン
use super::prelude::*;

pub const ID: u32 = 35098357;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WITCHCRAFTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Fusion Monster, including 1+ "Witchcrafter" monster as material
    // local e1=Fusion.CreateSummonEff({handler=c,matfilter=s.mfilter,extrafil=s.fextra})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8, _chk: i64| mfilter(d, c))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            ..Default::default()
        },
    );
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card from GY to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND|LOCATION_MZONE) and c:IsAbleToGrave())
    card::is_location(d, c, LOCATION_HAND | LOCATION_MZONE) && card::is_able_to_grave(d, c, None)
}
/// `s.checkmat(tp,sg,fc)`
fn checkmat(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_WITCHCRAFTER)
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]), 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil),s.checkmat
    let g = duel::get_matching_group(d, Some(&|d, c| mfilter(d, c)), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, NO_EX);
    (Some(g), Some(lua_fn(checkmat)))
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_WITCHCRAFTER),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsTurnPlayer(tp)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_turn_player(d, a.tp)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,tp,REASON_EFFECT)
        duel::send_to_hand(d, c, Some(a.tp), REASON_EFFECT, None);
    }
}
