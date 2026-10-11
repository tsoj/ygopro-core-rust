//! Emblem of the Plunder Patroll
use super::prelude::*;

pub const ID: u32 = 80621422;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PLUNDER_PATROLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_PLUNDER_PATROLL))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Equipped monster gains 500 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be targeted by opponent's card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon 1 "Plunder Patroll" monster from extra deck, equip this card to it
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(operation_fn)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,e,tp,att)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32) -> bool {
    // return c:IsSetCard(SET_PLUNDER_PATROLL) and c:IsAttribute(att) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16])
        && card::is_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    card::is_monster(d, c) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
/// The attributes of the monsters on the field and in the GY (the `att` loop of target and operation)
fn all_attributes(d: &mut Duel, tp: u8) -> u32 {
    // local att=0
    let mut att = 0;
    // for gc in aux.Next(Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,nil)) do
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    for gc in group::iter(d, &g) {
        // att=att|gc:GetAttribute()
        att |= card::get_attribute(d, gc);
    }
    att
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local att=0 (loop)
    let att = all_attributes(d, a.tp);
    // if chk==0 then return att>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp,att) end
    if a.chk == Some(0) {
        return att > 0
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp, att)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetTargetCard(c:GetPreviousEquipTarget())
    // (a nil target is not representable here: the core dereferences a null group for nil)
    let prev = card::get_previous_equip_target(d, c);
    if let Some(prev) = prev {
        duel::set_target_card(d, prev);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c:GetPreviousEquipTarget(),1,0,0)
    let prev = card::get_previous_equip_target(d, c);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, prev.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local att=0 (loop)
    let att = all_attributes(d, a.tp);
    // if att==0 then return end
    if att == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,att):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, att)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local ec=e:GetHandler():GetPreviousEquipTarget()
            let h = effect::get_handler(d, a.e).nil();
            let ec = card::get_previous_equip_target(d, h);
            // if ec and ec:IsRelateToEffect(e) then
            if let Some(ec) = ec {
                if card::is_relate_to_effect(d, ec, a.e) {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // s.equipop(tc,e,tp,ec)
                    equipop(d, tc, a.e, a.tp, ec);
                }
            }
        }
    }
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
        return;
    }
    // Gains 500 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetLabelObject(c)
    effect::set_label_object(d, e2, Value::Card(c));
    // e2:SetValue(s.eqlimit)
    effect::set_value_fn(d, e2, effect_card_fn(eqlimit));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
