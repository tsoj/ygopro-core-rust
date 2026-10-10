//! Branded in Red
//! Scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 82738008;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], listed_series: &[SET_DESPIA], ..ScriptInfo::DEFAULT };

/// `fparams={aux.FilterBoolFunction(Card.IsLevelAbove,8),Fusion.InHandMat(Card.IsAbleToRemove),s.fextra,Fusion.BanishMaterial,nil,s.stage2}`
fn fparams() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_level_above(d, c, 8))),
        matfilter: Some(fusion::in_hand_mat_with(std::sync::Arc::new(|d, c, _e, tp, chk| {
            card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))
        }))),
        extrafil: Some(std::sync::Arc::new(fextra)),
        extraop: Some(fusion::banish_material()),
        stage2: Some(std::sync::Arc::new(stage2)),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // To hand + potential Fusion Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_TOHAND+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_TOHAND + CATEGORY_REMOVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop(Fusion.SummonEffTG(table.unpack(fparams)),Fusion.SummonEffOP(table.unpack(fparams))))
    let fustg = fusion::summon_eff_tg(d, fparams());
    let fusop = fusion::summon_eff_op(d, fparams());
    effect::set_operation(d, e1, Some(operation(move |d, a| thop(d, a, &fustg, &fusop))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and ((c:IsSetCard(SET_DESPIA) and c:IsMonster()) or c:IsCode(CARD_ALBAZ))
    card::is_able_to_hand(d, c, None) && ((card::is_set_card(d, c, &[SET_DESPIA as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[CARD_ALBAZ]))
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_MZONE, 0, NO_EX);
    (Some(g), None)
}
/// `s.stage2(e,tc,tp,mg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // Cannot attack directly this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3207)
        effect::set_description(d, e1, 3207);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
/// `s.thfilter` target: `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
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
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(fustg,fusop)`: the operation of `e1`
fn thop(d: &mut Duel, a: &EffArgs, fustg: &LuaFn, fusop: &LuaFn) {
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let tc = match duel::first_target(d) {
        Some(tc) if card::is_relate_to_effect(d, tc, a.e) => tc,
        _ => return,
    };
    // if Duel.SendtoHand(tc,nil,REASON_EFFECT)==0 or not tc:IsLocation(LOCATION_HAND) then return end
    if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) == 0 || !card::is_location(d, tc, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - a.tp, tc);
    // if fustg(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if fustg.call(d, &args(a, Some(0))).truthy() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // fusop(e,tp,eg,ep,ev,re,r,rp)
        fusop.call(d, &args(a, None));
    }
}
/// The arguments of an effect function: the 8 event values, and `chk` after them if given.
fn args(a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    v
}
