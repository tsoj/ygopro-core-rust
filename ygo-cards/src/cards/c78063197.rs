//! Relinquished Fusion
use super::prelude::*;

pub const ID: u32 = 78063197;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EYES_RESTRICT], listed_names: &[64631466], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Fusion Summon
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_EYES_RESTRICT),Card.IsAbleToRemove,s.fextra,Fusion.BanishMaterial,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_EYES_RESTRICT as u16]))),
            // Card.IsAbleToRemove as a matfilter: (c, e, tp, chk) -> position tp, reason chk
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32)))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --equip
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
        return (Some(g), None);
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsAbleToChangeControler()
    //   and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_MZONE,0,1,nil,c,tp)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EFFECT)
        && card::is_able_to_change_controler(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.eqfilter(c,ec,tp)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId, tp: u8) -> bool {
    // local eff={c:GetCardEffect(89785779)}
    let eff = card::get_card_effect(d, c, 89785779);
    // if c:IsFacedown() or ((not c:IsSetCard(SET_EYES_RESTRICT) or not c:IsType(TYPE_FUSION)) and not c:IsCode(64631466)) then return false end
    if card::is_facedown(d, c) || ((!card::is_set_card(d, c, &[SET_EYES_RESTRICT as u16]) || !card::is_type(d, c, TYPE_FUSION)) && !card::is_code(d, c, &[64631466])) {
        return false;
    }
    // for _,te in ipairs(eff) do
    for te in eff {
        // if te:GetValue()(ec,c,tp) then return true end
        let Value::Fn(f) = effect::get_value(d, te) else {
            crate::lua_error("attempt to call a number value");
        };
        if f.call(d, &[Value::Card(ec), Value::Card(c), Value::Int(tp as i64)]).truthy() {
            return true;
        }
    }
    // return false
    false
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,tp) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc1=Duel.GetFirstTarget()
    // if not tc1 then return end
    let Some(tc1) = duel::first_target(d) else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_MZONE,0,1,1,nil,tc1,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter(d, c, tc1, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local tc2=g:GetFirst()
    // if not tc2 then return end
    let Some(tc2) = group::get_first(d, &g) else { return };
    // local te=tc2:GetCardEffect(89785779)
    let te = card::get_card_effect(d, tc2, 89785779).first().copied();
    // if tc1:IsFaceup() and tc1:IsRelateToEffect(e) and tc1:IsControler(1-tp) then
    if card::is_faceup(d, tc1) && card::is_relate_to_effect(d, tc1, a.e) && card::is_controler(d, tc1, 1 - a.tp) {
        let te = te.nil();
        // te:GetOperation()(tc2,te:GetLabelObject(),tp,tc1)
        let op = effect::get_operation(d, te).nil();
        let label_object = effect::get_label_object(d, te);
        op.call(d, &[Value::Card(tc2), label_object, Value::Int(a.tp as i64), Value::Card(tc1)]);
    }
}
