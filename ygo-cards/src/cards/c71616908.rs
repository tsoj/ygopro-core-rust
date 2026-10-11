//! Gem-Knight Amethyst
//! ジェムナイト・アメジス
use super::prelude::*;

pub const ID: u32 = 71616908;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GEM_KNIGHT],
    material_setcode: &[SET_GEM as u32, SET_GEM_KNIGHT as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GEM_KNIGHT),aux.FilterBoolFunctionEx(Card.IsRace,RACE_AQUA))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GEM_KNIGHT as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_AQUA, Some(fc), st, Some(tp)))),
        ],
    );
    // spsummon condition
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    // e2:SetValue(s.splimit)
    effect::set_value_fn(d, e2, lua_fn(splimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA) or (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
    let h = effect::get_handler(d, a.e(0)).nil();
    let st = a.int(3) as u64;
    !card::is_location(d, h, LOCATION_EXTRA) || (st & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (e:GetHandler():GetPreviousLocation()&LOCATION_ONFIELD)~=0
    let h = effect::get_handler(d, a.e).nil();
    (card::get_previous_location(d, h) & LOCATION_ONFIELD) != 0
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFacedown,tp,LOCATION_SZONE,LOCATION_SZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFacedown,tp,LOCATION_SZONE,LOCATION_SZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    // Duel.SendtoHand(g,nil,REASON_EFFECT)
    duel::send_to_hand(d, g, None, REASON_EFFECT, None);
}
